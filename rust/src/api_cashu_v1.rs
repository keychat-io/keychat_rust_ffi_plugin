use cashu_wallet::store::UnitedStore;
use cashu_wallet::wallet::AmountHelper;
use cashu_wallet::wallet::HttpOptions;
use cashu_wallet::wallet::MnemonicInfo;
use cashu_wallet::wallet::ProofsHelper;
use cashu_wallet::wallet::Token;
use cashu_wallet::wallet::WalletError;
use cashu_wallet::wallet::CURRENCY_UNIT_SAT;
use cashu_wallet_sqlite::StoreError;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use std::sync::MutexGuard as StdMutexGuard;
use tokio::runtime::{Builder, Runtime};

use cashu_wallet::types::{
    CashuTransaction, LNTransaction, Mint as MintV1, MintInfo as MintInfoV1,
    Transaction as TransactionV1, TransactionDirection as TransactionDirectionV1,
    TransactionKind as TransactionKindV1, TransactionStatus as TransactionStatusV1,
};

use cashu_wallet_sqlite::LitePool;
type Wallet = cashu_wallet::UnitedWallet<LitePool>;
use cashu_wallet;

#[frb(ignore)]
pub struct State {
    rt: Arc<Runtime>,
    wallet: Option<Wallet>,
    sats: u16,
    futs: Option<WalletFuts>,
}

#[path = "api_cashu.check.rs"]
mod check;
#[path = "api_cashu_v1.types.rs"]
mod types;
// use types::*;

impl State {
    fn new() -> anyhow::Result<Self> {
        let this = Self {
            rt: Builder::new_current_thread().enable_all().build()?.into(),
            wallet: None,
            futs: None,
            sats: 0,
        };

        Ok(this)
    }

    #[frb(ignore)]
    pub fn get_wallet(&self) -> anyhow::Result<&Wallet> {
        if self.wallet.is_none() {
            let err: anyhow::Error = format_err!("Wallet not init");
            return Err(err.into());
        }

        Ok(self.wallet.as_ref().unwrap())
    }

    // ignore for test
    #[frb(ignore)]
    pub fn lock() -> anyhow::Result<StdMutexGuard<'static, Self>> {
        STATE
            .lock()
            .map_err(|e| format_err!("Failed to lock the state: {}", e))
    }
}

lazy_static! {
    static ref STATE: StdMutex<State> =
        StdMutex::new(State::new().expect("failed to create tokio runtime"));
}

// the only pub fn that v2 can call
pub fn cashu_v1_init_send_all(
    dbpath: String,
    words: Option<String>,
) -> anyhow::Result<Vec<String>> {
    init_db(dbpath, words, false)?;
    init_cashu(32)?;
    let mints = get_mints()?;
    let mut tokens: Vec<String> = Vec::new();
    for m in mints {
        let url = m.url;
        let (is_charge, amount) = get_balance(url.clone())?;

        if (!is_charge && amount > 0) || amount > 2 {
            let tx = send_all(url, None)?;
            tokens.push(tx.content().to_string());
        }
    }

    Ok(tokens)
}

fn init_db(dbpath: String, words: Option<String>, dev: bool) -> anyhow::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut mnemonic = None;
    if let Some(s) = words {
        let mi = MnemonicInfo::with_words(&s)?;
        mnemonic = Some(Arc::new(mi))
    }

    let mut state = State::lock()?;

    let c = HttpOptions::new()
        .connection_verbose(true)
        .timeout_connect_ms(3000)
        .timeout_get_ms(5000)
        .timeout_swap_ms(0)
        .connection_verbose(true);

    let fut = async move {
        use cashu_wallet_sqlite::sqlx::{self, sqlite::SqliteLockingMode};

        // prevent other thread open it
        let mut lockmode = SqliteLockingMode::Exclusive;
        if dev {
            lockmode = SqliteLockingMode::Normal;
        }

        let opts = dbpath
            .parse::<sqlx::sqlite::SqliteConnectOptions>()?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            // or normal
            .synchronous(sqlx::sqlite::SqliteSynchronous::Full)
            .locking_mode(lockmode);

        info!("SqlitePool open: {:?}", opts);
        let db = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(opts)
            .await?;

        let store = LitePool::new(db, Default::default()).await?;
        let w = Wallet::with_mnemonic(store, c, mnemonic);

        // not return error
        let mut futs = WalletFuts::new();
        let res = load_mints_from_database_background(&w, &mut futs).await;
        info!("load_mints_from_database_background init_db: {:?}", res);

        Ok((w, futs))
    };

    let result = state.rt.block_on(fut);

    result.map(|(w, futs)| {
        state.wallet = Some(w);
        state.futs = Some(futs);
    })
}

fn close_db() -> anyhow::Result<bool> {
    let state = State::lock()?;
    if state.wallet.is_none() {
        return Ok(false);
    }

    let w = state.get_wallet()?;

    state.rt.block_on(w.store().database().close());
    Ok(true)
}

fn init_cashu(prepare_sats_once_time: u16) -> anyhow::Result<Vec<MintV1>> {
    let mut state = State::lock()?;
    state.sats = prepare_sats_once_time;

    try_load_mints(&mut state, true)?;

    let w = state.get_wallet()?;
    let mints = state.rt.block_on(w.mints())?;
    Ok(mints)
}

/// -> (okcount, trycount)
fn try_load_mints(
    state: &mut StdMutexGuard<'static, State>,
    wait: bool,
) -> anyhow::Result<(usize, usize)> {
    if state.futs.as_ref().map(|x| x.is_empty()) == Some(true) {
        return Ok((0, 0));
    }

    let mut futs = state.futs.take();
    if futs.is_none() {
        futs = Some(Default::default());
    }

    let mints = {
        let w = state.get_wallet()?;

        let futs = &mut futs;
        state.rt.clone().block_on(async move {
            let mut err = None;
            let futs = futs.as_mut().unwrap();
            if futs.is_empty() {
                let res = load_mints_from_database_background(w, futs).await;
                error!(
                    "load_mints_from_database_background futs.is_empty: {:?}",
                    res
                );
            }

            let all = futs.len();
            let mut okcount = 0usize;
            while let Some(res) = if wait {
                futs.join_next().await
            } else {
                futs.try_join_next()
            } {
                match res.map_err(|e| e.into()).and_then(|res| res) {
                    Ok(m) => {
                        info!("load_mints.got wallet 0: {}", m.client().url().as_str());
                        // not overwrite
                        if w.contains(m.client().url())? {
                            continue;
                        }

                        let m = Arc::new(m);
                        let r = w
                            .add_mint_with_units(
                                m.client().url().clone(),
                                false,
                                &[CURRENCY_UNIT_SAT],
                                Some(m),
                            )
                            .await;
                        info!("load_mints.got wallet 1: {:?}", r);
                        if let Err(e) = r {
                            err = Some(e.into());
                        } else {
                            okcount += 1;
                        }
                    }
                    Err(e) => {
                        warn!("load_mints.got wallet e: {}", e);
                        err = Some(e);
                    }
                }
            }

            if let Some(e) = err {
                bail!(e);
            }

            Ok((okcount, all))
        })
    };

    // fill for next call
    if mints.is_err() {
        let w = state.get_wallet()?;
        let res = state.rt.block_on(load_mints_from_database_background(
            w,
            futs.as_mut().unwrap(),
        ));
        warn!("load_mints_from_database_background fill: {:?}", res);
        res?;
    }
    // empty for prevent recall
    state.futs = futs;

    mints
}

use cashu_wallet::wallet::Wallet as WalletForMint;
use tokio::task::JoinSet;
type WalletFuts = JoinSet<anyhow::Result<WalletForMint>>;
/// load active mints from database::get_mints
#[doc(hidden)]
async fn load_mints_from_database_background(
    w: &Wallet,
    futs: &mut WalletFuts,
) -> anyhow::Result<(usize, usize)> {
    let count = {
        let mints = w.mints().await?;

        let mut count = 0usize;
        for m in &mints {
            let mint_url: cashu_wallet::Url = m.url.parse()?;

            // skip exist
            if w.get_wallet(&mint_url).is_ok() {
                continue;
            }

            // reduce the rate for sqlx error when init_cashu: pool timed out while waiting for an open connection
            let mut records = None;
            if let Some(mn) = w.mnemonic() {
                let rs = w.store().get_counters(&mint_url, mn.pubkey()).await?;
                records = Some(rs);
            }

            let client = cashu_wallet::wallet::MintClient::new(
                mint_url.clone(),
                w.http_options().as_ref().clone(),
            )?;

            let menomic = w.mnemonic().cloned();
            let store = w.store().clone();
            let w = async move {
                WalletForMint::new(client, None, None, menomic, &store, records)
                    .await
                    .map_err(|e| e.into())
            };
            count += 1;
            futs.spawn(w);
        }

        (count, mints.len())
    };

    Ok(count)
}

// ignore for test
// add by 2.0.0-dev.9
#[frb(ignore)]
fn get_mnemonic_info() -> anyhow::Result<Option<String>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let mi = w.mnemonic().map(|m| m.pubkey().to_string());
    Ok(mi)
}

fn set_mnemonic(words: Option<String>) -> anyhow::Result<bool> {
    let mut mnemonic = None;
    if let Some(s) = words {
        let mi = MnemonicInfo::with_words(&s)?;
        mnemonic = Some(Arc::new(mi))
    }

    let mut state = State::lock()?;

    let rt = state.rt.clone();
    let w = state
        .wallet
        .as_mut()
        .ok_or_else(|| format_err!("wallet not init"))?;

    let has = rt.block_on(async move {
        // for replace mnemonic
        let old = w.update_mnmonic(mnemonic).await?;

        Ok(old)
    });
    has
}

fn get_mints() -> anyhow::Result<Vec<MintV1>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let mints = state.rt.block_on(w.mints())?;

    Ok(mints)
}

fn add_mint(url: String) -> anyhow::Result<bool> {
    let u: cashu_wallet::Url = url.parse()?;

    let state = State::lock()?;
    let w = state.get_wallet()?;

    let result = state
        .rt
        .block_on(w.add_mint_with_units(u, true, &[CURRENCY_UNIT_SAT], None))?;

    Ok(result)
}

fn remove_mint(url: String) -> anyhow::Result<Option<String>> {
    let u: cashu_wallet::Url = url.parse()?;

    let state = State::lock()?;
    let w = state.get_wallet()?;
    let fut = async move {
        let ok = w.remove_mint(&u).await?;
        Ok(ok.then_some(u.as_str().to_owned()))
    };

    let result = state.rt.block_on(fut);

    result
}

// ? direct use map?
fn get_balances() -> anyhow::Result<String> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let bs = state.rt.block_on(w.get_balances())?;
    let bs = bs
        .into_iter()
        .filter(|(k, _v)| k.unit() == CURRENCY_UNIT_SAT)
        .map(|(k, v)| (k.mint().to_owned(), v))
        .collect::<std::collections::BTreeMap<_, _>>();
    let js = serde_json::to_string(&bs)?;

    Ok(js)
}

fn get_balance(mint: String) -> anyhow::Result<(bool, u64)> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let mint_url: cashu_wallet::Url = mint.parse()?;
    let is_charge = {
        let wallet_opt = w.get_wallet_optional(&mint_url)?;
        let keysetinfo = wallet_opt
            .as_ref()
            .map(|w| w.keysetinfo.clone())
            .unwrap_or_default();
        keysetinfo.iter().any(|k| k.input_fee_ppk > 0)
    };

    let bs = state.rt.block_on(w.get_balance(&mint_url))?;

    let bs = bs
        .into_iter()
        .filter(|(k, _v)| k == CURRENCY_UNIT_SAT)
        .collect::<std::collections::BTreeMap<_, _>>();
    let v = bs.values().next().copied().unwrap_or(0);

    Ok((is_charge, v))
}

fn receive_token(encoded_token: String) -> anyhow::Result<Vec<TransactionV1>> {
    let token: Token = encoded_token.parse()?;
    let token = token.into_v3()?;

    let mut state = State::lock()?;
    try_load_mints(&mut state, false).ok();

    let w = state.get_wallet()?;
    let fut = async move {
        for t in &token.token {
            if !w.contains(&t.mint)? {
                w.add_mint_with_units(t.mint.clone(), false, &[CURRENCY_UNIT_SAT], None)
                    .await?;
            }
        }

        let mut txs = vec![];
        w.receive_tokens_full_limit_unit(&token.into(), &mut txs, &[CURRENCY_UNIT_SAT])
            .await
            .map(|_| txs)
    };

    let txs = state.rt.block_on(fut)?;

    Ok(txs)
}

#[frb(ignore)]
fn prepare_one_proofs(amount: u64, mint: String) -> anyhow::Result<u64> {
    let u: cashu_wallet::Url = mint.parse()?;

    let state = State::lock()?;
    let w = state.get_wallet()?;

    let a = state
        .rt
        .block_on(w.prepare_one_proofs(&u, amount, Some(CURRENCY_UNIT_SAT)))?;

    Ok(a)
}

fn send_stamp(
    amount: u64,
    mints: Vec<String>,
    info: Option<String>,
) -> anyhow::Result<TransactionV1> {
    if amount == 0 {
        bail!("can't send amount 0");
    }

    let mut state = State::lock()?;

    let bs = state.rt.block_on(state.get_wallet()?.get_balances())?;

    let mut mints_first = Vec::new();
    let mut mints_second = Vec::new();
    for (k, _v) in bs
        .into_iter()
        .filter(|(k, _v)| k.unit() == CURRENCY_UNIT_SAT && *_v >= amount)
    {
        let mint_url: MintUrl = k.mint().parse()?;
        if mints
            .iter()
            .any(|m| m.trim_end_matches('/') == k.mint().trim_end_matches('/'))
        {
            mints_first.push(mint_url);
        } else {
            mints_second.push(mint_url);
        }
    }

    for mint_url in mints_first.iter().chain(mints_second.iter()) {
        let tx = __send(&mut state, amount, &mint_url, info.clone());
        debug!("send_stamp {} {} got: {:?}", mint_url, amount, tx);

        if tx.is_err() && !(state.get_wallet()?.contains(&mint_url)?) {
            error!(
                "send_stamp {} {} failed: {:?}",
                mint_url.as_str(),
                amount,
                tx
            );
            continue;
        } else {
            return tx;
        }
    }

    // last try ?
    let mint_url = mints_first
        .iter()
        .chain(mints_second.iter())
        .next()
        .ok_or_else(|| WalletError::insufficant_funds())?;
    let tx = __send(&mut state, amount, &mint_url, info.clone());
    tx
}

fn send(amount: u64, active_mint: String, info: Option<String>) -> anyhow::Result<TransactionV1> {
    if amount == 0 {
        bail!("can't send amount 0");
    }

    let mint_url: cashu_wallet::Url = active_mint.parse()?;
    let mut state = State::lock()?;
    __send(&mut state, amount, &mint_url, info)
}

use cashu_wallet::wallet::MintUrl;
#[frb(ignore)]
fn __send(
    state: &mut StdMutexGuard<'static, State>,
    amount: u64,
    mint_url: &MintUrl,
    info: Option<String>,
) -> anyhow::Result<TransactionV1> {
    try_load_mints(state, false).ok();

    let sats = state.sats;
    let w = state.get_wallet()?;

    let fut = async move {
        use cashu_wallet::wallet::SplitProofsGeneric;

        let mut wallet = w.get_wallet_optional(&mint_url)?;

        let mut ps = w
            .store()
            .get_proofs_limit_unit(&mint_url, CURRENCY_UNIT_SAT)
            .await?;
        let psv = ps.sum().to_u64();

        let mut keysetinfo = wallet
            .as_ref()
            .map(|w| w.keysetinfo.clone())
            .unwrap_or_default();

        let (mut select, mut sum_fee_ppk) = cashu_wallet::select_send_proofs_with_fee::<
            cashu_wallet_sqlite::StoreError,
        >(&keysetinfo, amount, &mut ps)?;
        if amount == 1 && sats > 1 && (&ps[..=select]).sum().to_u64() > 1 {
            let change = std::cmp::min(psv, sats.into());

            if wallet.is_none() {
                w.add_mint_with_units(mint_url.clone(), false, &[CURRENCY_UNIT_SAT], None)
                    .await?;
                wallet = Some(w.get_wallet(&mint_url)?);
            }

            keysetinfo = wallet
                .as_ref()
                .map(|w| w.keysetinfo.clone())
                .unwrap_or_default();

            let coins = w
                .prepare_one_proofs(&mint_url, change, Some(CURRENCY_UNIT_SAT))
                .await;
            info!("prepare_one_proofs min({},{}) got: {:?}", psv, sats, coins);
            coins?;

            ps = w
                .store()
                .get_proofs_limit_unit(&mint_url, CURRENCY_UNIT_SAT)
                .await?;
            (select, sum_fee_ppk) = cashu_wallet::select_send_proofs_with_fee::<
                cashu_wallet_sqlite::StoreError,
            >(&keysetinfo, amount, &mut ps)?;
        }

        let pss = &ps[..=select];
        let mut need_swap = false;
        let tokens = if pss.sum().to_u64() == amount {
            SplitProofsGeneric::new(pss.to_owned(), 0)
        } else {
            if wallet.is_none() {
                w.add_mint_with_units(mint_url.clone(), false, &[CURRENCY_UNIT_SAT], None)
                    .await?;
                wallet = Some(w.get_wallet(&mint_url)?);
            }
            need_swap = true;
            wallet
                .as_ref()
                .unwrap()
                .send(
                    amount.into(),
                    sum_fee_ppk.into(),
                    pss,
                    Some(CURRENCY_UNIT_SAT),
                    w.store(),
                )
                .await?
        };

        w.store().add_proofs(&mint_url, tokens.keep()).await?;
        w.store().delete_proofs(&mint_url, pss).await?;

        // clear dleq for token size
        let (mut ps, send_start_idx) = tokens.into_inner();
        let ps = &mut ps[send_start_idx..];
        ps.iter_mut().for_each(|p| p.raw.dleq = None);
        let cashu_tokens = WalletForMint::proofs_to_token(
            &*ps,
            mint_url.clone(),
            None,
            Some(CURRENCY_UNIT_SAT),
            true,
        )?;

        let mut tx: TransactionV1 = CashuTransaction::new(
            TransactionStatusV1::Pending,
            TransactionDirectionV1::Out,
            amount,
            mint_url.as_str(),
            if need_swap { Some(sum_fee_ppk) } else { None },
            &cashu_tokens,
            None,
            Some(CURRENCY_UNIT_SAT),
        )
        .into();
        *tx.info_mut() = info;

        w.store().add_transaction(&tx).await?;
        Ok::<_, cashu_wallet::UniError<cashu_wallet_sqlite::StoreError>>(tx)
    };
    let tx = state.rt.block_on(fut)?;

    Ok(tx)
}

fn send_all(active_mint: String, info: Option<String>) -> anyhow::Result<TransactionV1> {
    let mint_url: cashu_wallet::Url = active_mint.parse()?;
    let mut state = State::lock()?;
    __send_all(&mut state, &mint_url, info)
}

#[frb(ignore)]
fn __send_all(
    state: &mut StdMutexGuard<'static, State>,
    mint_url: &MintUrl,
    info: Option<String>,
) -> anyhow::Result<TransactionV1> {
    try_load_mints(state, false).ok();

    let w = state.get_wallet()?;

    let fut = async move {
        let mut wallet = w.get_wallet_optional(&mint_url)?;
        let ps = w
            .store()
            .get_proofs_limit_unit(&mint_url, CURRENCY_UNIT_SAT)
            .await?;
        let total_amount = ps.sum().to_u64();
        // if total_amount < 1 {
        //     bail!("Balance must great then 1.");
        // }

        let keysetinfo = wallet
            .as_ref()
            .map(|w| w.keysetinfo.clone())
            .unwrap_or_default();

        let mut sum_fee_ppk = 0;
        let mut final_fee = 0;

        for (_idx, proof) in ps.clone().iter().enumerate() {
            if let Some(need_keyset) = keysetinfo.iter().find(|i| i.id == proof.as_ref().keyset_id)
            {
                let input_fee_ppk = need_keyset.input_fee_ppk;
                sum_fee_ppk += input_fee_ppk;
            }
            if sum_fee_ppk > 0 {
                final_fee = (sum_fee_ppk + 999) / 1000;
            }
        }

        debug!("The final fee is {:?}", final_fee);
        if wallet.is_none() {
            w.add_mint_with_units(mint_url.clone(), false, &[CURRENCY_UNIT_SAT], None)
                .await?;
            wallet = Some(w.get_wallet(&mint_url)?);
        }
        let tokens = wallet
            .as_ref()
            .unwrap()
            .send(
                (total_amount - final_fee).into(),
                final_fee.into(),
                &ps,
                Some(CURRENCY_UNIT_SAT),
                w.store(),
            )
            .await?;

        debug!("the token is {:?}", tokens);

        w.store().add_proofs(&mint_url, tokens.keep()).await?;
        w.store().delete_proofs(&mint_url, &ps).await?;

        // clear dleq for token size
        // let (ps, send_start_idx) = tokens.into_inner();
        // let ps = &mut ps[send_start_idx..];
        // ps.iter_mut().for_each(|p| p.raw.dleq = None);
        let cashu_tokens = WalletForMint::proofs_to_token(
            // &*ps,
            tokens.all(),
            mint_url.clone(),
            None,
            Some(CURRENCY_UNIT_SAT),
            true,
        )?;
        debug!("cashu tokens is {:?}", cashu_tokens);

        let mut tx: TransactionV1 = CashuTransaction::new(
            TransactionStatusV1::Pending,
            TransactionDirectionV1::Out,
            total_amount - final_fee,
            mint_url.as_str(),
            Some(final_fee),
            &cashu_tokens,
            None,
            Some(CURRENCY_UNIT_SAT),
        )
        .into();
        *tx.info_mut() = info;

        w.store().add_transaction(&tx).await?;
        Ok::<_, cashu_wallet::UniError<cashu_wallet_sqlite::StoreError>>(tx)
    };
    let tx = state.rt.block_on(fut)?;

    Ok(tx)
}

fn request_mint(amount: u64, active_mint: String) -> anyhow::Result<TransactionV1> {
    if amount == 0 {
        bail!("can't mint amount 0");
    }
    let u: cashu_wallet::Url = active_mint.parse()?;

    let mut state = State::lock()?;
    try_load_mints(&mut state, false).ok();

    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        if !w.contains(&u)? {
            w.add_mint_with_units(u.clone(), false, &[CURRENCY_UNIT_SAT], None)
                .await?;
        }

        w.request_mint(&u, amount, Some(CURRENCY_UNIT_SAT)).await
    })?;

    Ok(tx)
}

fn mint_token(amount: u64, hash: String, active_mint: String) -> anyhow::Result<TransactionV1> {
    if amount == 0 {
        bail!("can't mint amount 0");
    }

    let u: cashu_wallet::Url = active_mint.parse()?;

    let mut state = State::lock()?;
    try_load_mints(&mut state, false).ok();

    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        if !w.contains(&u)? {
            w.add_mint_with_units(u.clone(), false, &[CURRENCY_UNIT_SAT], None)
                .await?;
        }
        w.mint_tokens(&u, amount, hash, Some(CURRENCY_UNIT_SAT))
            .await
    })?;

    Ok(tx)
}

fn melt(
    invoice: String,
    active_mint: String,
    amount: Option<u64>,
) -> anyhow::Result<TransactionV1> {
    if amount == Some(0) {
        bail!("can't melt amount 0");
    }

    let u: cashu_wallet::Url = active_mint.parse()?;

    let mut state = State::lock()?;
    try_load_mints(&mut state, false).ok();

    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        if !w.contains(&u)? {
            w.add_mint_with_units(u.clone(), false, &[CURRENCY_UNIT_SAT], None)
                .await?;
        }
        w.melt(&u, invoice, amount, Some(CURRENCY_UNIT_SAT), None)
            .await
    })?;
    Ok(tx)
}

fn get_transactions() -> anyhow::Result<Vec<TransactionV1>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(w.store().get_all_transactions())?;
    Ok(tx)
}

fn get_transactions_with_offset(offset: usize, limit: usize) -> anyhow::Result<Vec<TransactionV1>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(w.store().get_transactions_with_offset(
        offset,
        limit,
        [TransactionKindV1::Cashu, TransactionKindV1::LN].as_slice(),
    ))?;
    Ok(tx)
}

fn get_cashu_transactions_with_offset(
    offset: usize,
    limit: usize,
) -> anyhow::Result<Vec<CashuTransaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(w.store().get_transactions_with_offset(
        offset,
        limit,
        [TransactionKindV1::Cashu].as_slice(),
    ))?;

    let mut txs = Vec::with_capacity(tx.len());
    for t in tx {
        match t {
            TransactionV1::Cashu(t) => txs.push(t),
            _ => unreachable!("unreachable not CashuTransaction"),
        }
    }

    Ok(txs)
}

fn get_ln_transactions_with_offset(
    offset: usize,
    limit: usize,
) -> anyhow::Result<Vec<LNTransaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(w.store().get_transactions_with_offset(
        offset,
        limit,
        [TransactionKindV1::LN].as_slice(),
    ))?;

    let mut txs = Vec::with_capacity(tx.len());
    for t in tx {
        match t {
            TransactionV1::LN(t) => txs.push(t),
            _ => unreachable!("unreachable not LNTransaction"),
        }
    }

    Ok(txs)
}

fn get_pending_transactions() -> anyhow::Result<Vec<TransactionV1>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(w.store().get_pending_transactions())?;
    Ok(tx)
}

fn get_ln_pending_transactions() -> anyhow::Result<Vec<LNTransaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(w.store().get_pending_transactions())?;
    let mut txs = Vec::with_capacity(tx.iter().filter(|x| x.is_ln()).count());
    for t in tx {
        if let TransactionV1::LN(t) = t {
            txs.push(t);
        }
    }
    Ok(txs)
}

fn get_cashu_pending_transactions() -> anyhow::Result<Vec<CashuTransaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(w.store().get_pending_transactions())?;
    let mut txs = Vec::with_capacity(tx.iter().filter(|x| x.is_cashu()).count());
    for t in tx {
        if let TransactionV1::Cashu(t) = t {
            txs.push(t);
        }
    }
    Ok(txs)
}

/// remove transaction.time() <= unix_timestamp_ms_le and kind is the status
fn remove_transactions(
    unix_timestamp_ms_le: u64,
    kind: TransactionStatusV1,
) -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(
        w.store()
            .delete_transactions(&vec![kind], unix_timestamp_ms_le),
    )?;

    Ok(tx)
}

fn get_pending_transactions_count() -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(w.store().get_pending_transactions())?;
    Ok(tx.len() as _)
}

fn check_pending() -> anyhow::Result<(usize, usize)> {
    let mut state = State::lock()?;
    try_load_mints(&mut state, false).ok();
    let w = state.get_wallet()?;

    let upc_all = state.rt.block_on(w.check_pendings())?;
    Ok(upc_all)
}

fn check_transaction(id: String) -> anyhow::Result<TransactionV1> {
    let mut state = State::lock()?;
    try_load_mints(&mut state, false).ok();
    let w = state.get_wallet()?;

    let fut = async move {
        let mut tx = w
            .store()
            .get_transaction(&id)
            .await?
            .ok_or_else(|| StoreError::Custom(format_err!("tx id not found")))?;

        if tx.is_pending() {
            let u = tx.mint_url().parse()?;
            if !w.contains(&u)? {
                w.add_mint_with_units(u.clone(), false, &[CURRENCY_UNIT_SAT], None)
                    .await?;
            }

            let txs = vec![tx];
            let _res = w.check_pendings_with(txs).await?;

            tx = w
                .store()
                .get_transaction(&id)
                .await?
                .ok_or_else(|| StoreError::Custom(format_err!("tx id not found")))?;
        }
        Ok(tx)
    };

    let tx = state.rt.block_on(fut);
    tx
}

/// (spents, pendings, all)
fn check_proofs() -> anyhow::Result<(usize, usize, usize)> {
    let mut state = State::lock()?;
    try_load_mints(&mut state, true).ok();

    let w = state.get_wallet()?;

    let spa = state.rt.block_on(check::check_proofs_in_database(w));
    warn!("check_proofs.spa: {:?}", spa);
    let spa = spa?;

    Ok(spa)
}

fn decode_token(encoded_token: String) -> anyhow::Result<TokenInfoV1> {
    let token: Token = encoded_token.parse()?;
    let token = token.into_v3()?;

    if token.token.is_empty() {
        bail!("empty token")
    }

    Ok(TokenInfoV1 {
        // encoded_token,
        mint: token.mint0().as_str().to_owned(),
        amount: token.amount(),
        unit: token.unit().map(|s| s.to_owned()),
        memo: token.memo,
    })
}

/// sleepms_after_check_a_batch for (code: 429): {"detail":"Rate limit exceeded."}
fn restore(
    mint: String,
    words: Option<String>,
    sleepms_after_check_a_batch: u64,
) -> anyhow::Result<(u64, usize)> {
    let mint_url: cashu_wallet::Url = mint.parse()?;

    let mut state = State::lock()?;
    try_load_mints(&mut state, false).ok();

    let mut mnemonic = None;
    if let Some(s) = words {
        let mi = MnemonicInfo::with_words(&s)?;
        mnemonic = Some(Arc::new(mi))
    }

    let w = state.get_wallet()?;

    let coins = state.rt.block_on(async {
        if !w.contains(&mint_url)? {
            w.add_mint_with_units(mint_url.clone(), false, &[CURRENCY_UNIT_SAT], None)
                .await?;
        }

        w.restore(
            &mint_url,
            100,
            sleepms_after_check_a_batch,
            &[],
            mnemonic,
            |_, _, _, _, _, _, _, _, _, _, _, _| false,
        )
        .await
    })?;

    Ok((coins.sum().to_u64(), coins.len()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenInfoV1 {
    //  encoded_token: String,
    pub mint: String,
    pub amount: u64,
    pub unit: Option<String>,
    pub memo: Option<String>,
}

use cashu_wallet::cashu::lightning_invoice::{
    Bolt11Invoice as Invoice, Bolt11InvoiceDescription as InvoiceDescription,
};
fn decode_invoice(encoded_invoice: String) -> anyhow::Result<InvoiceInfoV1> {
    let encoded_invoice = encoded_invoice.replace("lightning:", "");
    let invoice: Invoice = encoded_invoice.parse()?;

    let amount = invoice
        .amount_milli_satoshis()
        .ok_or_else(|| format_err!("amount_milli_satoshis null"))?;

    let memo = match invoice.description() {
        InvoiceDescription::Direct(memo) => Some(memo.to_string()),
        InvoiceDescription::Hash(_) => None,
    };

    let status = match invoice.is_expired() {
        true => InvoiceStatusV1::Expired,
        false => InvoiceStatusV1::Unpaid,
    };

    let ts = (invoice.duration_since_epoch() + invoice.expiry_time()).as_millis();
    // println!(
    //     "{:?}+{:?}={}",
    //     invoice.duration_since_epoch(),
    //     invoice.expiry_time(),
    //     ts
    // );

    Ok(InvoiceInfoV1 {
        // bolt11: encoded_invoice,
        amount: amount / 1000,
        hash: invoice.payment_hash().to_string(),
        expiry_ts: ts as _,
        mint: None,
        memo,
        status,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InvoiceInfoV1 {
    // pub bolt11: String,
    pub amount: u64,
    pub expiry_ts: u64,
    pub hash: String,
    pub memo: Option<String>,
    pub mint: Option<String>,
    pub status: InvoiceStatusV1,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum InvoiceStatusV1 {
    Paid,
    Unpaid,
    Expired,
}
