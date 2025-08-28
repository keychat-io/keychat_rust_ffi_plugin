use anyhow::Ok;
pub use cashu::{Amount, CurrencyUnit, MintUrl};
pub use cdk::amount::{SplitTarget, MSAT_IN_SAT};
use cdk::cdk_database;
pub use cdk::lightning_invoice::{
    Bolt11Invoice as Invoice, Bolt11InvoiceDescriptionRef as InvoiceDescriptionRef,
};
use cdk::nuts::nut00::ProofsMethods;
use cdk::nuts::{MeltOptions, Token};
use cdk::wallet::types::WalletKey;
use cdk::wallet::ReceiveOptions;
pub use cdk::wallet::{MultiMintWallet, SendOptions, Wallet, WalletBuilder};
pub use cdk::Bolt11Invoice;
use cdk_common::database::WalletDatabase;
pub use cdk_common::wallet::{TransactionId, TransactionKind, TransactionStatus};
use cdk_sqlite::WalletSqliteDatabase;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::{Arc, Mutex as StdMutex, MutexGuard as StdMutexGuard};
use tokio::runtime::{Builder, Runtime};

#[frb(ignore)]
pub struct State {
    rt: Arc<Runtime>,
    wallet: Option<MultiMintWallet>,
    mnemonic: Option<Arc<MnemonicInfo>>,
    sats: u16,
}

#[path = "api_cashu.types.rs"]
pub mod types;
pub use types::*;

impl State {
    fn new() -> anyhow::Result<Self> {
        let this = Self {
            rt: Builder::new_current_thread().enable_all().build()?.into(),
            wallet: None,
            mnemonic: None,
            sats: 0,
        };

        Ok(this)
    }

    #[frb(ignore)]
    pub fn mnemonic(&self) -> Option<&Arc<MnemonicInfo>> {
        self.mnemonic.as_ref()
    }

    #[frb(ignore)]
    pub fn update_mnmonic(&mut self, mnemonic: Option<Arc<MnemonicInfo>>) -> anyhow::Result<bool> {
        if self.mnemonic == mnemonic {
            return Ok(false);
        }
        let has = std::mem::replace(&mut self.mnemonic, mnemonic);
        Ok(has.is_some())
    }

    #[frb(ignore)]
    pub fn get_wallet(&self) -> anyhow::Result<&MultiMintWallet> {
        if self.wallet.is_none() {
            let err: anyhow::Error = format_err!("Wallet not init");
            debug!("get_wallet none");
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

pub fn init_db(dbpath: String, words: String, _dev: bool) -> anyhow::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let _ = set_mnemonic(Some(words.clone()));
    // let _ = add_mint("https://8333.space:3338/".to_string());

    let mi = MnemonicInfo::with_words(&words)?;
    let seed = mi.mnemonic().to_seed("");

    let mut state = State::lock()?;

    let fut = async move {
        let localstore: Arc<dyn WalletDatabase<Err = cdk_database::Error> + Send + Sync> =
            Arc::new(WalletSqliteDatabase::new(&dbpath).await?);

        let mut wallets: Vec<Wallet> = Vec::new();

        let mints = localstore.get_mints().await?;
        if mints.is_empty() {}

        for (mint_url, mint_info) in mints {
            let units = if let Some(mint_info) = mint_info {
                mint_info.supported_units().into_iter().cloned().collect()
            } else {
                vec![CurrencyUnit::Sat]
            };

            for unit in units {
                let mint_url_clone = mint_url.clone();
                let builder = WalletBuilder::new()
                    .mint_url(mint_url_clone.clone())
                    .unit(unit)
                    .localstore(localstore.clone())
                    .seed(&seed);

                let wallet = builder.build()?;

                let wallet_clone = wallet.clone();

                tokio::spawn(async move {
                    if let Err(err) = wallet_clone.get_mint_info().await {
                        error!(
                            "Could not get mint quote for {}, {}",
                            wallet_clone.mint_url, err
                        );
                    }
                });

                wallets.push(wallet);
            }
        }
        // println!("wallets is {:?}", wallets);
        let multi_mint_wallet = MultiMintWallet::new(localstore, Arc::new(seed), wallets);

        Ok(multi_mint_wallet)
    };

    let result = state.rt.block_on(fut);

    result.map(|w| {
        state.wallet = Some(w);
    })
}

pub fn init_cashu(prepare_sats_once_time: u16) -> anyhow::Result<Vec<String>> {
    let mut state = State::lock()?;
    state.sats = prepare_sats_once_time;

    let w = state.get_wallet()?;
    let mints = state.rt.block_on(w.localstore.get_mints())?;
    // mints.values_mut().for_each(|v| *v = None);
    let only_keys: Vec<String> = mints.into_keys().map(|k| k.to_string()).collect();
    Ok(only_keys)
}

#[frb(ignore)]
pub fn get_mnemonic_info() -> anyhow::Result<Option<String>> {
    let state = State::lock()?;
    let mnemonic = state.mnemonic();
    let mi = mnemonic.map(|m| m.pubkey().to_string());
    Ok(mi)
}

pub fn set_mnemonic(words: Option<String>) -> anyhow::Result<bool> {
    if words.is_none() {
        return Ok(false);
    }
    let mut mnemonic = None;
    if let Some(s) = words {
        let mi = MnemonicInfo::with_words(&s)?;
        mnemonic = Some(Arc::new(mi))
    }

    let mut state = State::lock()?;
    let has = state.update_mnmonic(mnemonic);
    has
}

pub fn get_mints() -> anyhow::Result<HashMap<String, ()>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let mints = state.rt.block_on(w.localstore.get_mints())?;
    let only_keys: std::collections::HashMap<String, _> =
        mints.into_keys().map(|k| (k.to_string(), ())).collect();
    Ok(only_keys)
}

pub fn add_mint(url: String) -> anyhow::Result<()> {
    let mint_url = MintUrl::from_str(&url)?;
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let _result = state.rt.block_on(w.localstore.add_mint(mint_url, None))?;

    Ok(())
}

pub fn remove_mint(url: String) -> anyhow::Result<()> {
    let mint_url = MintUrl::from_str(&url)?;

    let state = State::lock()?;
    let w = state.get_wallet()?;

    let _result = state.rt.block_on(w.localstore.remove_mint(mint_url));

    Ok(())
}

// ? direct use map?
pub fn get_balances() -> anyhow::Result<String> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let bs = state.rt.block_on(w.get_balances(&CurrencyUnit::Sat))?;
    let bs = bs
        .into_iter()
        // .filter(|(k, _v)| k.unit() == CURRENCY_UNIT_SAT)
        // .map(|(k, v)| (k.mint().to_owned(), v))
        .collect::<std::collections::BTreeMap<_, _>>();
    let js = serde_json::to_string(&bs)?;

    Ok(js)
}

#[frb(ignore)]
pub fn get_wallet(mint_url: MintUrl, unit: CurrencyUnit) -> anyhow::Result<Wallet> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let wallet_key = WalletKey::new(mint_url.clone(), unit);

    let wallet = state
        .rt
        .block_on(w.get_wallet(&wallet_key))
        .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;

    Ok(wallet)
}

/// Helper function to create or get a wallet
#[frb(ignore)]
async fn get_or_create_wallet(
    multi_mint_wallet: &MultiMintWallet,
    mint_url: &MintUrl,
    unit: CurrencyUnit,
) -> anyhow::Result<cdk::wallet::Wallet> {
    match multi_mint_wallet
        .get_wallet(&WalletKey::new(mint_url.clone(), unit.clone()))
        .await
    {
        Some(wallet) => Ok(wallet.clone()),
        None => {
            debug!("Wallet does not exist creating..");
            multi_mint_wallet
                .create_and_add_wallet(&mint_url.to_string(), unit, None)
                .await
        }
    }
}

pub fn send_all(mint: String) -> anyhow::Result<Transaction> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint)?;

    let tx = state.rt.block_on(async {
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let ps = wallet.get_unspent_proofs().await?;
        if *ps.total_amount()?.as_ref() == 0 {
            let err: anyhow::Error = format_err!("The amount is 0");
            return Err(err.into());
        }

        // // let total_amount: u64 =  *ps.total_amount()?.as_ref();
        // let keyset_fees = wallet.get_keyset_fees().await?;
        // let fee =
        //     calculate_fee(&ps.count_by_keyset(), &keyset_fees).unwrap_or_default();
        // let net_amount = ps.total_amount()? - fee;
        // println!("net_amount is {:?}, fee is {:?}", net_amount, fee);

        let prepared_send = wallet
            .prepare_send(ps.total_amount()?, SendOptions::default())
            .await?;
        let tx = wallet.send(prepared_send, None).await?;
        Ok(tx)
    })?;
    let tx_new = Transaction {
        mint_url: tx.mint_url.to_string(),
        direction: tx.direction,
        kind: tx.kind,
        amount: *tx.amount.as_ref(),
        fee: *tx.fee.as_ref(),
        unit: Some(tx.unit.to_string()),
        token: tx.token,
        status: tx.status,
        timestamp: tx.timestamp,
        metadata: tx.metadata,
    };

    Ok(tx_new)
}

/// default set 20
pub fn merge_proofs(thershold: u64) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;

    let fut = async move {
        // Iterate all known mints in localstore
        let mints = w.localstore.get_mints().await?;
        for (mint_url, _info) in mints {
            // Only operate on wallets we already have (skip creating new ones here)
            if let Some(wallet) = w
                .get_wallet(&WalletKey::new(mint_url.clone(), unit.clone()))
                .await
            {
                // Get unspent proofs for this wallet
                let proofs = wallet.get_unspent_proofs().await?;
                if proofs.is_empty() {
                    continue;
                }

                // Group proofs by denomination (amount)
                let mut groups: HashMap<u64, cashu::Proofs> = HashMap::new();
                for p in proofs {
                    let amt = *p.amount.as_ref();
                    groups.entry(amt).or_insert_with(cashu::Proofs::new).push(p);
                }

                // For each denomination with count > 20, merge to minimal outputs
                for (_amt, group) in groups.into_iter() {
                    if group.len() as u64 >= thershold {
                        debug!("need merge");
                        // Default target merges to least number of power-of-two outputs
                        let _ = wallet
                            .swap(None, SplitTarget::default(), group, None, false)
                            .await?;
                    }
                }
            }
        }
        Ok(())
    };

    state.rt.block_on(fut)?;
    Ok(())
}

/// inner used, this is for receive stamps every multi times
/// need diff mint url put in a like map<url, token>
pub fn multi_receive(stamps: Vec<String>) -> anyhow::Result<()> {
    let token: Token = Token::from_str(&stamps[0])?;
    let mint_url = token.mint_url()?;
    let unit = token.unit().unwrap_or_default();

    let state = State::lock()?;
    let w = state.get_wallet()?;
    let fut = async move {
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let mut all_proofs = Vec::new();
        for token_str in stamps {
            let token_data = Token::from_str(&token_str)?;
            let keysets_info = match w
                .localstore
                .get_mint_keysets(token_data.mint_url()?)
                .await?
            {
                Some(keysets_info) => keysets_info,
                // Hit the keysets endpoint if we don't have the keysets for this Mint
                None => wallet.get_mint_keysets().await?,
            };
            let proofs = token_data.proofs(&keysets_info)?;
            all_proofs.extend(proofs);
        }
        // must checkout proofs with unspent, this is check for http post
        let proofs_state = wallet.check_proofs_spent(all_proofs.clone()).await?;
        let unspent: cashu::Proofs = all_proofs
            .into_iter()
            .zip(proofs_state)
            .filter_map(|(p, s)| (s.state == cashu::State::Unspent).then_some(p))
            .collect();

        let tokens = Token::new(mint_url, unspent, None, unit);
        let encoded_token = tokens.to_string();

        let amount = w.receive(&encoded_token, ReceiveOptions::default()).await;
        println!("amount: {:?}", amount);
        Ok(())
    };

    let _amount = state.rt.block_on(fut)?;

    Ok(())
}

pub fn receive_token(encoded_token: String) -> anyhow::Result<Transaction> {
    let token: Token = Token::from_str(&encoded_token)?;
    let mint_url = token.mint_url()?;

    let unit = token.unit().unwrap_or_default();
    let state = State::lock()?;

    let w = state.get_wallet()?;

    let fut = async move {
        if w.get_wallet(&WalletKey::new(mint_url.clone(), unit.clone()))
            .await
            .is_none()
        {
            get_or_create_wallet(w, &mint_url, unit).await.unwrap();
        }
        w.receive(&encoded_token, ReceiveOptions::default()).await
    };

    let tx = state.rt.block_on(fut)?;
    let tx_new = Transaction {
        mint_url: tx.mint_url.to_string(),
        direction: tx.direction,
        kind: tx.kind,
        amount: *tx.amount.as_ref(),
        fee: *tx.fee.as_ref(),
        unit: Some(tx.unit.to_string()),
        token: tx.token,
        status: tx.status,
        timestamp: tx.timestamp,
        metadata: tx.metadata,
    };

    Ok(tx_new)
}

/// inner used
pub fn print_proofs(mint: String) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint)?;

    let fut = async move {
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let ps = wallet.get_unspent_proofs().await?;
        println!("get_all_proofs len: {:?}", ps.len());

        for p in ps {
            println!(
                "{}: {} {}",
                p.amount.as_ref(),
                p.keyset_id,
                p.secret.to_string(),
            );
        }
        Ok(())
    };

    let _ = state.rt.block_on(fut)?;

    Ok(())
}

pub fn prepare_one_proofs(amount: u64, mint: String) -> anyhow::Result<u64> {
    let denomination: u64 = 1;

    let state = State::lock()?;
    let w = state.get_wallet()?;
    if amount == 0 {
        bail!("can't mint amount 0");
    }
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint)?;

    let a = state.rt.block_on(async {
        let mut count_before = 0u64;
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let mut ps = wallet.get_unspent_proofs().await?;

        ps.retain(|p| {
            let is = *p.amount.as_ref() == denomination;
            if is {
                count_before += 1;
            }
            !is
        });

        if count_before * denomination < amount {
            let rest_amount = amount - count_before * denomination;
            let active_keyset_ids = wallet
                .get_active_mint_keysets()
                .await?
                .into_iter()
                .map(|keyset| keyset.id)
                .collect();
            let keyset_fees = wallet.get_keyset_fees().await?;
            let selected = Wallet::select_proofs(
                rest_amount.into(),
                ps,
                &active_keyset_ids,
                &keyset_fees,
                true,
            )?;

            wallet
                .swap_denomination(
                    denomination.into(),
                    Some(rest_amount.into()),
                    selected.clone(),
                    false,
                )
                .await?;
        }
        Ok(count_before)
    })?;

    Ok(a)
}

pub fn send_stamp(
    amount: u64,
    mints: Vec<String>,
    info: Option<String>,
) -> anyhow::Result<Transaction> {
    if amount == 0 {
        bail!("can't send amount 0");
    }

    let mut state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;

    let bs = state.rt.block_on(w.get_balances(&unit))?;

    let mut mints_first = Vec::new();
    let mut mints_second = Vec::new();
    for (k, _v) in bs.into_iter().filter(|(_k, _v)| *_v >= amount.into()) {
        // let mint_url: MintUrl = k;
        let k_str = k.to_string();
        if mints
            .iter()
            .any(|m| m.trim_end_matches('/') == k_str.trim_end_matches('/'))
        {
            mints_first.push(k);
        } else {
            mints_second.push(k);
        }
    }

    for mint_url in mints_first.iter().chain(mints_second.iter()) {
        let tx = _send(&mut state, amount, mint_url.to_string(), info.clone());
        debug!("send_stamp {} {} got: {:?}", mint_url, amount, tx);

        if tx.is_err() && get_wallet(mint_url.clone(), unit.clone()).is_err() {
            error!(
                "send_stamp {} {} failed: {:?}",
                mint_url.to_string(),
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
        .ok_or_else(|| cdk_common::Error::InsufficientFunds)?;
    let tx = send(amount, mint_url.to_string(), info.clone())?;
    Ok(tx)
}

async fn mint_balances(
    multi_mint_wallet: &MultiMintWallet,
    unit: &CurrencyUnit,
) -> anyhow::Result<Vec<(MintUrl, Amount)>> {
    let wallets: BTreeMap<MintUrl, Amount> = multi_mint_wallet.get_balances(unit).await?;

    let mut wallets_vec = Vec::with_capacity(wallets.len());

    for (i, (mint_url, amount)) in wallets
        .iter()
        .filter(|(_, a)| a > &&Amount::ZERO)
        .enumerate()
    {
        let mint_url = mint_url.clone();
        debug!("{i}: {mint_url} {amount} {unit}");
        wallets_vec.push((mint_url, *amount))
    }
    Ok(wallets_vec)
}

// Helper function to check if there are enough funds for an operation
fn check_sufficient_funds(available: Amount, required: Amount) -> anyhow::Result<()> {
    if required.gt(&available) {
        bail!("Not enough funds");
    }
    Ok(())
}

/// Helper function to get a wallet from the multi-mint wallet by mint URL
async fn get_wallet_by_mint_url(
    multi_mint_wallet: &MultiMintWallet,
    mint_url_str: &str,
    unit: CurrencyUnit,
) -> anyhow::Result<cdk::wallet::Wallet> {
    let mint_url = MintUrl::from_str(mint_url_str)?;

    let wallet_key = WalletKey::new(mint_url.clone(), unit);
    let wallet = multi_mint_wallet
        .get_wallet(&wallet_key)
        .await
        .ok_or_else(|| anyhow::anyhow!("Wallet not found for mint URL: {}", mint_url_str))?;

    Ok(wallet.clone())
}

/// Helper function to get a wallet from the multi-mint wallet
pub async fn get_wallet_by_index(
    multi_mint_wallet: &MultiMintWallet,
    mint_amounts: &[(MintUrl, Amount)],
    mint_number: usize,
    unit: CurrencyUnit,
) -> anyhow::Result<cdk::wallet::Wallet> {
    validate_mint_number(mint_number, mint_amounts.len())?;

    let wallet_key = WalletKey::new(mint_amounts[mint_number].0.clone(), unit);
    let wallet = multi_mint_wallet
        .get_wallet(&wallet_key)
        .await
        .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;

    Ok(wallet.clone())
}

/// Helper function to validate a mint number against available mints
pub fn validate_mint_number(mint_number: usize, mint_count: usize) -> anyhow::Result<()> {
    if mint_number >= mint_count {
        bail!("Invalid mint number");
    }
    Ok(())
}

pub fn send(
    amount: u64,
    active_mint: String,
    _info: Option<String>,
) -> anyhow::Result<Transaction> {
    if amount == 0 {
        bail!("can't send amount 0");
    }
    let mut state = State::lock()?;
    _send(&mut state, amount, active_mint, None)
}

fn _send(
    state: &mut StdMutexGuard<'static, State>,
    amount: u64,
    active_mint: String,
    _info: Option<String>,
) -> anyhow::Result<Transaction> {
    if amount == 0 {
        bail!("can't send amount 0");
    }

    let unit = CurrencyUnit::from_str("sat")?;

    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        let mints_amounts = mint_balances(w, &unit).await;
        // Get wallet either by mint URL or by index
        let wallet = get_wallet_by_mint_url(w, &active_mint, unit).await?;

        // Find the mint amount for the selected wallet to check if we have sufficient funds
        let mint_url = &wallet.mint_url;
        let mint_amount = mints_amounts?
            .iter()
            .find(|(url, _)| url == mint_url)
            .map(|(_, amount)| *amount)
            .ok_or_else(|| anyhow!("Could not find balance for mint: {}", mint_url));

        check_sufficient_funds(mint_amount?, amount.into())?;
        let prepared_send = wallet
            .prepare_send(amount.into(), SendOptions::default())
            .await?;
        let tx = wallet.send(prepared_send, None).await?;
        Ok(tx)
    })?;
    let tx_new = Transaction {
        mint_url: tx.mint_url.to_string(),
        direction: tx.direction,
        kind: tx.kind,
        amount: *tx.amount.as_ref(),
        fee: *tx.fee.as_ref(),
        unit: Some(tx.unit.to_string()),
        token: tx.token,
        status: tx.status,
        timestamp: tx.timestamp,
        metadata: tx.metadata,
    };

    Ok(tx_new)
}

pub fn request_mint(amount: u64, active_mint: String) -> anyhow::Result<String> {
    if amount == 0 {
        bail!("can't mint amount 0");
    }
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&active_mint)?;

    let state = State::lock()?;

    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        let wallet = get_or_create_wallet(w, &mint_url, unit).await?;
        let quote = wallet.mint_quote(Amount::from(amount), None).await?;
        // let mut subscription = wallet
        //     .subscribe(WalletSubscription::Bolt11MintQuoteState(vec![quote
        //         .id
        //         .clone()]))
        //     .await;

        // while let Some(msg) = subscription.recv().await {
        //     if let NotificationPayload::MintQuoteBolt11Response(response) = msg {
        //         if response.state == MintQuoteState::Paid {
        //             break;
        //         }
        //     }
        // }
        // let request = quote.request;
        // println!("request mint {:?}", quote.request);
        // let proofs = wallet.mint(&quote_id, SplitTarget::default(), None).await?;

        // let receive_amount = proofs.total_amount()?;

        // debug!("Received {receive_amount} from mint {mint_url}");
        Ok(quote.request)
    })?;

    Ok(tx)
}

/// this need call every init melt mint
pub fn check_all_mint_quotes() -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let tx = state.rt.block_on(async {
        let check = w.check_all_mint_quotes(None).await?;
        let amounts: u64 = check.values().map(|v| *v.as_ref()).sum();

        Ok(amounts)
    })?;

    Ok(tx)
}

/// Checks pending proofs for spent status
pub fn check_proofs() -> anyhow::Result<HashMap<String, u64>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    // let mint_url = MintUrl::from_str(&active_mint)?;
    let tx = state.rt.block_on(async {
        let mut check_map = HashMap::new();
        let mints = w.localstore.get_mints().await?;
        for (mint_url, _info) in mints {
            let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
            let check = wallet.check_all_pending_proofs().await?;
            check_map.insert(mint_url.to_string(), *check.as_ref());
        }
        Ok(check_map)
    })?;

    Ok(tx)
}

/// include ln and cashu
pub fn check_pending() -> anyhow::Result<HashMap<String, (u64, u64)>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let tx = state.rt.block_on(async {
        let mut check_map = HashMap::new();
        let mints = w.localstore.get_mints().await?;
        for (mint_url, _info) in mints {
            let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
            let check = wallet.check_proofs_tx_spent_state().await?;
            check_map.insert(mint_url.to_string(), check);
        }
        Ok(check_map)
    })?;

    Ok(tx)
}

pub fn mint_token(
    amount: u64,
    quote_id: String,
    active_mint: String,
) -> anyhow::Result<Transaction> {
    if amount == 0 {
        bail!("can't mint amount 0");
    }

    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&active_mint)?;

    let state = State::lock()?;

    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        let wallet = get_or_create_wallet(w, &mint_url, unit).await?;
        let proofs = wallet.mint(&quote_id, SplitTarget::default(), None).await?;

        let receive_amount = proofs.0.total_amount()?;

        debug!("Received {receive_amount} from mint {mint_url}");
        Ok(proofs.1)
    })?;

    let tx_new = Transaction {
        mint_url: tx.mint_url.to_string(),
        direction: tx.direction,
        kind: tx.kind,
        amount: *tx.amount.as_ref(),
        fee: *tx.fee.as_ref(),
        unit: Some(tx.unit.to_string()),
        token: tx.token,
        status: tx.status,
        timestamp: tx.timestamp,
        metadata: tx.metadata,
    };

    Ok(tx_new)
}

pub fn melt(
    invoice: String,
    active_mint: String,
    amount: Option<u64>,
) -> anyhow::Result<Transaction> {
    if amount == Some(0) {
        bail!("can't melt amount 0");
    }

    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&active_mint)?;

    let state = State::lock()?;

    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let mints_amounts = mint_balances(w, &unit).await?;
        let mint_amount = mints_amounts
            .iter()
            .find(|(url, _)| url == &mint_url)
            .map(|(_, amount)| *amount)
            .ok_or_else(|| anyhow::anyhow!("Could not find balance for mint: {}", mint_url))?;

        let available_funds = <cdk::Amount as Into<u64>>::into(mint_amount) * MSAT_IN_SAT;

        let bolt11 = Bolt11Invoice::from_str(&invoice)?;
        // Determine payment amount and options
        let options = if bolt11.amount_milli_satoshis().is_none() {
            let user_amount = amount.unwrap() * MSAT_IN_SAT;

            if user_amount > available_funds {
                bail!("Not enough funds");
            }

            Some(MeltOptions::new_amountless(user_amount))
        } else {
            // Check if invoice amount exceeds available funds
            let invoice_amount = bolt11.amount_milli_satoshis().unwrap();
            if invoice_amount > available_funds {
                bail!("Not enough funds");
            }
            None
        };
        // Process payment
        let quote = wallet.melt_quote(bolt11.to_string(), options).await?;
        info!("{quote:?}");

        let melt = wallet.melt(&quote.id).await?;
        info!("Paid invoice: {}", melt.0.state);

        if let Some(preimage) = melt.0.preimage {
            info!("Payment preimage: {preimage}");
        }

        Ok(melt.1)
    })?;
    let tx_new = Transaction {
        mint_url: tx.mint_url.to_string(),
        direction: tx.direction,
        kind: tx.kind,
        amount: *tx.amount.as_ref(),
        fee: *tx.fee.as_ref(),
        unit: Some(tx.unit.to_string()),
        token: tx.token,
        status: tx.status,
        timestamp: tx.timestamp,
        metadata: tx.metadata,
    };

    Ok(tx_new)
}

pub fn get_all_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = state.rt.block_on(w.list_transactions(None))?;
    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            mint_url: tx.mint_url.to_string(),
            direction: tx.direction,
            kind: tx.kind,
            amount: *tx.amount.as_ref(),
            fee: *tx.fee.as_ref(),
            unit: Some(tx.unit.to_string()),
            token: tx.token,
            status: tx.status,
            timestamp: tx.timestamp,
            metadata: tx.metadata,
        };
        txs_new.push(tx_new);
    }

    Ok(txs_new)
}

// incloud normal_tx ln_tx melt_tx mint_tx
pub fn get_cashu_transactions_with_offset(
    offset: usize,
    limit: usize,
) -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = state.rt.block_on(w.list_transactions_with_kind_offset(
        offset,
        limit,
        [TransactionKind::Cashu].as_slice(),
        None,
    ))?;

    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            mint_url: tx.mint_url.to_string(),
            direction: tx.direction,
            kind: tx.kind,
            amount: *tx.amount.as_ref(),
            fee: *tx.fee.as_ref(),
            unit: Some(tx.unit.to_string()),
            token: tx.token,
            status: tx.status,
            timestamp: tx.timestamp,
            metadata: tx.metadata,
        };
        txs_new.push(tx_new);
    }

    Ok(txs_new)
}

pub fn get_ln_transactions_with_offset(
    offset: usize,
    limit: usize,
) -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = state.rt.block_on(w.list_transactions_with_kind_offset(
        offset,
        limit,
        [TransactionKind::LN].as_slice(),
        None,
    ))?;
    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            mint_url: tx.mint_url.to_string(),
            direction: tx.direction,
            kind: tx.kind,
            amount: *tx.amount.as_ref(),
            fee: *tx.fee.as_ref(),
            unit: Some(tx.unit.to_string()),
            token: tx.token,
            status: tx.status,
            timestamp: tx.timestamp,
            metadata: tx.metadata,
        };
        txs_new.push(tx_new);
    }

    Ok(txs_new)
}

pub fn get_ln_pending_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = state
        .rt
        .block_on(w.list_pending_transactions_with_kind([TransactionKind::LN].as_slice(), None))?;
    // let mut txs = Vec::with_capacity(tx.iter().filter(|x| x.is_ln()).count());
    // for t in tx {
    //     if let Transaction::LN(t) = t {
    //         txs.push(t);
    //     }
    // }
    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            mint_url: tx.mint_url.to_string(),
            direction: tx.direction,
            kind: tx.kind,
            amount: *tx.amount.as_ref(),
            fee: *tx.fee.as_ref(),
            unit: Some(tx.unit.to_string()),
            token: tx.token,
            status: tx.status,
            timestamp: tx.timestamp,
            metadata: tx.metadata,
        };
        txs_new.push(tx_new);
    }

    Ok(txs_new)
}

pub fn get_cashu_pending_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = state.rt.block_on(
        w.list_pending_transactions_with_kind([TransactionKind::Cashu].as_slice(), None),
    )?;

    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            mint_url: tx.mint_url.to_string(),
            direction: tx.direction,
            kind: tx.kind,
            amount: *tx.amount.as_ref(),
            fee: *tx.fee.as_ref(),
            unit: Some(tx.unit.to_string()),
            token: tx.token,
            status: tx.status,
            timestamp: tx.timestamp,
            metadata: tx.metadata,
        };
        txs_new.push(tx_new);
    }

    Ok(txs_new)
}

/// remove transaction.time() <= unix_timestamp_le and kind is the status, timestamp must be second
pub fn remove_transactions(
    unix_timestamp_le: u64,
    _status: TransactionStatus,
) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let _tx = state
        .rt
        .block_on(w.remove_transactions(unix_timestamp_le))?;

    Ok(())
}

pub fn get_pending_transactions_count() -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        let proofs = w.list_all_pending_proofs().await?;
        let counts = proofs.values().map(|(v, _)| v.len() as u64).sum();
        Ok(counts)
    })?;
    Ok(tx)
}

pub fn check_transaction(id: String) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let id = TransactionId::from_str(&id)?;
    let tx = state.rt.block_on(async {
        let tx = w
            .localstore
            .get_transaction(id)
            .await?
            .ok_or(cdk_common::Error::TransactionNotFound)?;

        if tx.direction != cdk_common::wallet::TransactionDirection::Outgoing {
            return Err(cdk_common::Error::InvalidTransactionDirection);
        }
        let wallet = get_wallet_by_mint_url(w, &tx.mint_url.to_string(), unit).await;
        let wallet = wallet.unwrap();

        wallet.revert_transaction(id).await
    })?;

    Ok(tx)
}

/// (spents, pendings, all)
pub fn get_all_proofs_data() -> anyhow::Result<(usize, usize, usize)> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        let all_proofs = w.list_all_proofs().await?;
        let spent_proofs = w.list_spent_proofs().await?;
        let pending_proofs = w.list_pending_proofs().await?;
        Ok((
            all_proofs.values().len(),
            spent_proofs.values().len(),
            pending_proofs.values().len(),
        ))
    })?;

    Ok(tx)
}

pub fn decode_token(encoded_token: String) -> anyhow::Result<TokenInfoV2> {
    let token: Token = Token::from_str(&encoded_token)?;

    Ok(TokenInfoV2 {
        // encoded_token,
        mint: token.mint_url()?.to_string(),
        amount: token.value()?.into(),
        unit: token.unit(),
        memo: token.memo().clone(),
    })
}

/// sleepms_after_check_a_batch for (code: 429): {"detail":"Rate limit exceeded."}
pub fn restore(mint_url: String, words: Option<String>) -> anyhow::Result<u64> {
    let mint_url = MintUrl::from_str(&mint_url)?;
    let mut state = State::lock()?;
    let unit = CurrencyUnit::from_str("sat")?;

    if words.is_some() {
        let mi = MnemonicInfo::with_words(&words.unwrap())?;
        let _ = state.update_mnmonic(Some(Arc::new(mi)));
    }

    let w = state.get_wallet()?;

    let amount = state.rt.block_on(async {
        let wallet = match w
            .get_wallet(&WalletKey::new(mint_url.clone(), unit.clone()))
            .await
        {
            Some(wallet) => wallet.clone(),
            None => {
                w.create_and_add_wallet(&mint_url.to_string(), unit, None)
                    .await?
            }
        };

        let amount = wallet.restore().await?;
        Ok(amount)
    })?;

    Ok(amount.into())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfoV2 {
    // pub encoded_token: String,
    pub mint: String,
    pub amount: u64,
    pub unit: Option<CurrencyUnit>,
    pub memo: Option<String>,
}

pub fn decode_invoice(encoded_invoice: String) -> anyhow::Result<InvoiceInfoV2> {
    let encoded_invoice = encoded_invoice.replace("lightning:", "");
    let invoice: Invoice = encoded_invoice.parse()?;

    let amount = invoice
        .amount_milli_satoshis()
        .ok_or_else(|| format_err!("amount_milli_satoshis null"))?;

    let memo = match invoice.description() {
        InvoiceDescriptionRef::Direct(memo) => Some(memo.to_string()),
        InvoiceDescriptionRef::Hash(_) => None,
    };

    let status = match invoice.is_expired() {
        true => InvoiceStatusV2::Expired,
        false => InvoiceStatusV2::Unpaid,
    };

    let ts = (invoice.duration_since_epoch() + invoice.expiry_time()).as_millis();
    debug!(
        "{:?}+{:?}={}",
        invoice.duration_since_epoch(),
        invoice.expiry_time(),
        ts
    );

    Ok(InvoiceInfoV2 {
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
pub struct InvoiceInfoV2 {
    // pub bolt11: String,
    pub amount: u64,
    pub expiry_ts: u64,
    pub hash: String,
    pub memo: Option<String>,
    pub mint: Option<String>,
    pub status: InvoiceStatusV2,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InvoiceStatusV2 {
    Paid,
    Unpaid,
    Expired,
}
