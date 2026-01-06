use anyhow::Ok;
pub use cashu::{Amount, CurrencyUnit, Id, KeySetInfo, MintInfo, MintUrl};
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
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::{Arc, Mutex as StdMutex, MutexGuard as StdMutexGuard};
use tokio::runtime::{Builder, Runtime};
use tokio::time::Duration;

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

pub async fn init_db(dbpath: String, words: String, _dev: bool) -> anyhow::Result<()> {
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
            let mut units = if let Some(mint_info) = mint_info {
                mint_info.supported_units().into_iter().cloned().collect()
            } else {
                vec![CurrencyUnit::Sat]
            };
            if units.is_empty() {
                units.push(CurrencyUnit::Sat);
            }

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
        let multi_mint_wallet = MultiMintWallet::new(localstore, Arc::new(seed), wallets);

        Ok(multi_mint_wallet)
    };

    fut.await.map(|w| {
        state.wallet = Some(w);
    })
}

pub async fn init_cashu(prepare_sats_once_time: u16) -> anyhow::Result<Vec<MintCashu>> {
    let mut state = State::lock()?;
    state.sats = prepare_sats_once_time;

    let w = state.get_wallet()?;
    let result = w.localstore.get_mints().await?;
    let mints = decode_mint_info(result)?;

    Ok(mints)
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

fn decode_mint_info(
    mint_info: HashMap<MintUrl, Option<MintInfo>>,
) -> anyhow::Result<Vec<MintCashu>> {
    let mut mints = Vec::new();
    for (k, v) in mint_info {
        if let Some(v) = v {
            let mint_info = MintCashuInfo {
                name: v.name.unwrap_or_default(),
                version: v.version.map(|v| v.to_string()).unwrap_or_default(),
                pubkey: v.pubkey.map(|v| v.to_string()),
                description: v.description,
                description_long: v.description_long,
                motd: v.motd,
                contact: v
                    .contact
                    .unwrap_or_default()
                    .into_iter()
                    .map(|contact_info| ContactCashu {
                        method: contact_info.method,
                        info: contact_info.info,
                    })
                    .collect(),
                nuts: {
                    let mut nuts_map = HashMap::new();
                    // Check each nut and add to map if supported
                    if !v.nuts.nut04.methods.is_empty() || !v.nuts.nut04.disabled {
                        nuts_map.insert("nut04".to_string(), true);
                    }
                    if !v.nuts.nut05.methods.is_empty() || !v.nuts.nut05.disabled {
                        nuts_map.insert("nut05".to_string(), true);
                    }
                    if v.nuts.nut07.supported {
                        nuts_map.insert("nut07".to_string(), true);
                    }
                    if v.nuts.nut08.supported {
                        nuts_map.insert("nut08".to_string(), true);
                    }
                    if v.nuts.nut09.supported {
                        nuts_map.insert("nut09".to_string(), true);
                    }
                    if v.nuts.nut10.supported {
                        nuts_map.insert("nut10".to_string(), true);
                    }
                    if v.nuts.nut11.supported {
                        nuts_map.insert("nut11".to_string(), true);
                    }
                    if v.nuts.nut12.supported {
                        nuts_map.insert("nut12".to_string(), true);
                    }
                    if v.nuts.nut14.supported {
                        nuts_map.insert("nut14".to_string(), true);
                    }
                    if !v.nuts.nut15.methods.is_empty() {
                        nuts_map.insert("nut15".to_string(), true);
                    }
                    if !v.nuts.nut17.supported.is_empty() {
                        nuts_map.insert("nut17".to_string(), true);
                    }
                    if v.nuts.nut19.ttl.is_some() || !v.nuts.nut19.cached_endpoints.is_empty() {
                        nuts_map.insert("nut19".to_string(), true);
                    }
                    if v.nuts.nut20.supported {
                        nuts_map.insert("nut20".to_string(), true);
                    }
                    if v.nuts.nut21.is_some() {
                        nuts_map.insert("nut21".to_string(), true);
                    }
                    if v.nuts.nut22.is_some() {
                        nuts_map.insert("nut22".to_string(), true);
                    }
                    nuts_map
                },
            };
            let mint = MintCashu {
                url: k.to_string(),
                active: true,
                time: v.time.unwrap_or(0),
                info: Some(mint_info),
            };
            mints.push(mint);
        }
    }
    Ok(mints)
}

pub async fn get_mints() -> anyhow::Result<Vec<MintCashu>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    // let mut mints = Vec::new();
    let result = w.localstore.get_mints().await?;
    let mints = decode_mint_info(result)?;
    Ok(mints)
}

pub async fn add_mint(url: String) -> anyhow::Result<()> {
    let mint_url = MintUrl::from_str(&url)?;
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let _result = w.localstore.add_mint(mint_url, None).await?;

    Ok(())
}

pub async fn remove_mint(url: String) -> anyhow::Result<()> {
    let mint_url = MintUrl::from_str(&url)?;

    let state = State::lock()?;
    let w = state.get_wallet()?;

    w.localstore.remove_mint(mint_url).await?;

    Ok(())
}
#[derive(Debug, Serialize, Deserialize)]
struct MintCounterRecord {
    mint: String,
    keysetid: String,
    max_counter: u32,
}

pub fn add_counters(counters: String) -> anyhow::Result<Vec<String>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let records: Vec<MintCounterRecord> = serde_json::from_str(&counters)?;
    let mut map: HashMap<String, Vec<KeySetInfo>> = HashMap::new();
    let mut keysetid_to_counter: HashMap<Id, u32> = HashMap::new();
    let mut mints = Vec::new();

    for record in records {
        let id = Id::from_str(&record.keysetid).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let keyset_info = KeySetInfo {
            id,
            unit: CurrencyUnit::Sat,
            active: true,
            input_fee_ppk: 0,
            final_expiry: None,
        };
        keysetid_to_counter.insert(id, record.max_counter);

        map.entry(record.mint.clone())
            .or_insert_with(Vec::new)
            .push(keyset_info);
    }
    let _tx = state.rt.block_on(async {
        for (mint_url, keyset_infos) in map {
            let mint = MintUrl::from_str(&mint_url.trim())?;
            let wallet = get_or_create_wallet(w, &mint, unit.clone()).await?;
            if wallet.get_mint_info().await.is_ok() {
                mints.push(mint_url.clone());
            } else {
                let tmp = format!("{}-failure", mint_url);
                mints.push(tmp);
                warn!("Failed to get mint info for {}", mint_url);
            }

            // if w.localstore.get_mint(mint_url.clone()).await?.is_none() {
            //     w.localstore.add_mint(mint_url.clone(), None).await?;
            // }

            w.localstore
                .add_mint_keysets(mint.clone(), keyset_infos)
                .await?;
        }

        for (id, counter) in keysetid_to_counter {
            w.localstore.increment_keyset_counter(&id, counter).await?;
        }
        Ok(())
    });

    Ok(mints)
}

// ? direct use map?
pub async fn get_balances() -> anyhow::Result<String> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let bs = w.get_balances(&CurrencyUnit::Sat).await?;
    let bs = bs
        .into_iter()
        // .filter(|(k, _v)| k.unit() == CURRENCY_UNIT_SAT)
        // .map(|(k, v)| (k.mint().to_owned(), v))
        .collect::<std::collections::BTreeMap<_, _>>();
    let js = serde_json::to_string(&bs)?;

    Ok(js)
}

#[frb(ignore)]
pub async fn get_wallet(mint_url: MintUrl, unit: CurrencyUnit) -> anyhow::Result<Wallet> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let wallet_key = WalletKey::new(mint_url.clone(), unit);

    let wallet = w
        .get_wallet(&wallet_key)
        .await
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

pub async fn send_all(mint: String) -> anyhow::Result<Transaction> {
    merge_proofs(10).await?;
    let tx = _send_all(mint).await?;
    Ok(tx)
}

async fn _send_all(mint: String) -> anyhow::Result<Transaction> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint)?;

    let result = async move {
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let ps = wallet.get_unspent_proofs().await?;
        if *ps.total_amount()?.as_ref() == 0 {
            let err: anyhow::Error = format_err!("The amount is 0");
            return Err(err.into());
        }

        let prepared_send = wallet
            .prepare_send(ps.total_amount()?, SendOptions::default())
            .await?;
        let tx = wallet.send(prepared_send, None).await?;
        Ok(tx)
    };
    let tx = result.await?;
    let tx_new = Transaction {
        id: tx.id().to_string(),
        mint_url: tx.mint_url.to_string(),
        io: tx.direction,
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
pub async fn merge_proofs(threshold: u64) -> anyhow::Result<()> {
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
                    if group.len() as u64 >= threshold {
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

    fut.await?;
    Ok(())
}

/// inner used, this is for receive stamps every multi times
/// need diff mint url put in a like map<url, token>
pub async fn multi_receive(stamps: Vec<String>) -> anyhow::Result<()> {
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
        debug!("amount: {:?}", amount);
        Ok(())
    };

    fut.await?;

    Ok(())
}

pub async fn receive_token(encoded_token: String) -> anyhow::Result<Transaction> {
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

    let tx = fut.await?;
    let tx_new = Transaction {
        id: tx.id().to_string(),
        mint_url: tx.mint_url.to_string(),
        io: tx.direction,
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
pub async fn print_proofs(mint: String) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint)?;

    let fut = async move {
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let ps = wallet.get_unspent_proofs().await?;
        // let ps = wallet.get_all_proofs().await?;
        println!("get_all_proofs len: {:?}", ps.len());

        for p in ps {
            println!(
                "{}: {} {:?}",
                p.amount.as_ref(),
                p.keyset_id,
                p.secret.to_string(),
                // p.y()?,
            );
        }
        Ok(())
    };

    fut.await?;

    Ok(())
}

pub async fn prepare_one_proofs(mint: String) -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    _prepare_one_proofs(w, mint.clone()).await
}

async fn _prepare_one_proofs(w: &MultiMintWallet, mint: String) -> anyhow::Result<u64> {
    // split to 1 sat
    let denomination: u64 = 1;
    // if less then 10, prepare to 32
    let threshold: u64 = 10;
    // total prepare amount 32
    let amount: u64 = 32;

    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint)?;

    let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
    let mints_amounts = mint_balances(w, &unit).await?;
    let mint_url = &wallet.mint_url;
    let mint_amount = mints_amounts
        .iter()
        .find(|(url, _)| url == mint_url)
        .map(|(_, amount)| *amount)
        .ok_or_else(|| anyhow!("Could not find balance for mint: {}", mint_url));
    let mint_amount = mint_amount?;
    // if balance less then include equal 32, return directly 0
    if *mint_amount.as_ref() <= amount {
        debug!("balance less then 32 amount");
        return Ok(0);
    }
    let ps0 = wallet.get_unspent_proofs().await?;
    let count_before0 = ps0
        .iter()
        .filter(|p| *p.amount.as_ref() == denomination)
        .count() as u64;
    // more than 10, no need execute prepare
    if count_before0 >= threshold {
        debug!("have enough denomination proofs, no need to prepare");
        return Ok(count_before0);
    }
    let mut count_before = 0u64;
    // first check proofs state, but this will take more time
    // let _check = wallet.check_all_pending_proofs().await?;
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
    Ok(32 - count_before)
}

pub async fn send_stamp(
    amount: u64,
    mints: Vec<String>,
    info: Option<String>,
) -> anyhow::Result<SendStampsResult> {
    if amount == 0 {
        bail!("can't send amount 0");
    }

    let mut state = State::lock()?;
    let unit = CurrencyUnit::from_str("sat")?;

    let w = state.get_wallet()?;

    let balances = w.get_balances(&unit).await?;

    let fut = async move {
        let mut mints_first = Vec::new();
        let mut mints_second = Vec::new();
        for (k, _v) in balances.into_iter().filter(|(_, v)| *v >= amount.into()) {
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

        let mut last_err: Option<anyhow::Error> = None;

        for mint_url in mints_first.iter().chain(mints_second.iter()) {
            let mint_str = mint_url.to_string();
            let fut = _send_one(&mut state, amount, mint_str.clone(), info.clone());
            match tokio::time::timeout(Duration::from_secs(20), fut).await {
                std::result::Result::Ok(std::result::Result::Ok(tx)) => {
                    debug!("send_stamp success {} {}", mint_url, amount);
                    let result = SendStampsResult {
                        tx: tx.0,
                        is_need_split: tx.1,
                    };
                    return Ok(result);
                }
                std::result::Result::Ok(std::result::Result::Err(e)) => {
                    debug!(
                        "send_stamp error mint={} amount={} err={:?}",
                        mint_url, amount, e
                    );
                    last_err = Some(e);
                }
                std::result::Result::Err(_) => {
                    debug!("send_stamp timeout mint={} amount={}", mint_url, amount);
                    last_err = Some(anyhow::anyhow!("connection timeout for {}", mint_url));
                }
            }
        }

        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("No available mints")))
    };
    fut.await
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

pub async fn send(
    amount: u64,
    active_mint: String,
    _info: Option<String>,
) -> anyhow::Result<Transaction> {
    if amount == 0 {
        bail!("can't send amount 0");
    }
    let mut state = State::lock()?;
    _send(&mut state, amount, active_mint, None).await
}

use std::time::Instant;

async fn time<F, T>(label: &str, fut: F) -> T
where
    F: std::future::Future<Output = T>,
{
    let t0 = Instant::now();
    let out = fut.await;
    println!("{} took {} ms", label, t0.elapsed().as_millis());
    out
}

async fn _send_one(
    state: &mut StdMutexGuard<'static, State>,
    amount: u64,
    active_mint: String,
    _info: Option<String>,
) -> anyhow::Result<(Transaction, bool)> {
    if amount == 0 {
        bail!("can't send amount 0");
    }
    let unit = CurrencyUnit::from_str("sat")?;
    let w = state.get_wallet()?;

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

    let mint_amount = mint_amount?;
    check_sufficient_funds(mint_amount.clone(), amount.into())?;
    // prepare one proofs if less than 10, and set prepare amount to 32
    // if amount == 1 && *mint_amount.as_ref() > 32 && state.sats > 1 {
    //     let _ = prepare_one_proofs_back(w, 10, state.sats.into(), active_mint).await?;
    // }

    let ps = wallet.get_unspent_proofs().await?;
    let stamp_cnts = ps.iter().filter(|p| *p.amount.as_ref() == 1).count() as u64;

    // add if have one stamp proof, then take it directly from prepare_send_one_with_enough, else use normal prepare_send
    let mut use_send_one = false;
    let prepared_send = if amount == 1 && stamp_cnts > 0 {
        debug!("use prepare_send_one_with_enough");
        match wallet
            .prepare_send_one_with_enough(amount.into(), SendOptions::default())
            .await
        {
            std::result::Result::Ok(p) => {
                use_send_one = true;
                p
            }
            Err(e) => {
                debug!("prepare_send_one_with_enough failed, fallback: {}", e);
                wallet
                    .prepare_send(amount.into(), SendOptions::default())
                    .await?
            }
        }
    } else {
        debug!("use normal prepare_send");
        wallet
            .prepare_send(amount.into(), SendOptions::default())
            .await?
    };

    let tx = if use_send_one {
        debug!("use send_one");
        wallet.send_one(prepared_send, None).await?
    } else {
        debug!("use send");
        wallet.send(prepared_send, None).await?
    };

    let tx_new = Transaction {
        id: tx.id().to_string(),
        mint_url: tx.mint_url.to_string(),
        io: tx.direction,
        kind: tx.kind,
        amount: *tx.amount.as_ref(),
        fee: *tx.fee.as_ref(),
        unit: Some(tx.unit.to_string()),
        token: tx.token,
        status: tx.status,
        timestamp: tx.timestamp,
        metadata: tx.metadata,
    };
    Ok((tx_new, stamp_cnts < 10))
}

async fn _send(
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

    let result = async move {
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

        let mint_amount = mint_amount?;
        check_sufficient_funds(mint_amount.clone(), amount.into())?;
        // prepare one proofs if less than 10, and set prepare amount to 32
        // if amount == 1 && *mint_amount.as_ref() > 32 && state.sats > 1 {
        //     let _ = prepare_one_proofs_back(w, 10, state.sats.into(), active_mint).await?;
        // }
        let prepared_send = wallet
            .prepare_send(amount.into(), SendOptions::default())
            .await?;
        let tx = wallet.send(prepared_send, None).await?;
        Ok(tx)
    };
    let tx = result.await?;
    let tx_new = Transaction {
        id: tx.id().to_string(),
        mint_url: tx.mint_url.to_string(),
        io: tx.direction,
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

pub async fn request_mint(amount: u64, active_mint: String) -> anyhow::Result<Transaction> {
    if amount == 0 {
        bail!("can't mint amount 0");
    }
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&active_mint)?;

    let state = State::lock()?;

    let w = state.get_wallet()?;

    let result = async move {
        let wallet = get_or_create_wallet(w, &mint_url, unit).await?;
        let quote = wallet.mint_quote(Amount::from(amount), None).await?;
        Ok(quote)
    };
    let tx = result.await?;
    let tx = tx.1;
    let tx_new = Transaction {
        id: tx.id().to_string(),
        mint_url: tx.mint_url.to_string(),
        io: tx.direction,
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

/// don not used in flutter, and put it in check_pending
pub async fn mint_token(
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

    let tx = async move {
        let wallet = get_or_create_wallet(w, &mint_url, unit).await?;
        let proofs = wallet.mint(&quote_id, SplitTarget::default(), None).await?;

        let receive_amount = proofs.0.total_amount()?;

        debug!("Received {receive_amount} from mint {mint_url}");
        Ok(proofs.1)
    }
    .await?;

    let tx_new = Transaction {
        id: tx.id().to_string(),
        mint_url: tx.mint_url.to_string(),
        io: tx.direction,
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

/// this need call every init melt mint, do not used in flutter
pub async fn check_all_mint_quotes() -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let tx = async move {
        let check = w.check_all_mint_quotes(None).await?;
        let amounts: u64 = check.values().map(|v| *v.as_ref()).sum();

        Ok(amounts)
    }
    .await?;

    Ok(tx)
}

/// check_melt_quote_id test
pub async fn check_melt_quote_id(quote_id: String, mint_url: String) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint_url)?;
    let _tx = async move {
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let re = wallet.melt_quote_status_only(&quote_id).await?;
        println!("melt_quote_status_only: {:?}", re);

        Ok(())
    }
    .await?;

    Ok(())
}

/// Checks pending proofs for spent status
pub async fn check_proofs() -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    async move {
        let mints = w.localstore.get_mints().await?;
        let mut errs: Vec<String> = Vec::new();
        for (mint_url, _info) in mints {
            debug!("check_proofs mint_url: {}", mint_url);
            let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
            // wallet.check_proofs_from_mint().await?;
            if let Err(e) = wallet.check_proofs_from_mint().await {
                error!("check_proofs mint_url: {} error: {:?}", mint_url, e);
                errs.push(format!("{}: {}", mint_url, e));
            }
        }
        // throw err
        if !errs.is_empty() {
            return Err(anyhow::anyhow!(format!(
                "check_proofs failed for {} mint(s): {}",
                errs.len(),
                errs.join(" | ")
            )));
        }
        Ok(())
    }
    .await?;
    Ok(())
}

/// include ln and cashu, tx status
pub async fn check_pending() -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    async move {
        let mut check_map = HashMap::new();
        let mints = w.localstore.get_mints().await?;
        for (mint_url, _info) in mints {
            let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
            let check = wallet.check_pending_transactions_state().await?;
            check_map.insert(mint_url.to_string(), check);
        }
        Ok(check_map)
    }
    .await?;

    Ok(())
}

/// include ln and cashu, tx status, use check_transaction instead
pub async fn check_single_pending(tx_id: String, mint_url: String) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint_url)?;
    let _tx = async move {
        let mut check_map = HashMap::new();
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let check = wallet
            .check_pending_transaction_state(tx_id.clone())
            .await?;
        check_map.insert(mint_url.to_string(), check);
        Ok(check_map)
    }
    .await?;

    Ok(())
}

pub async fn melt(
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

    let tx = async move {
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
        info!("melt_quote {quote:?}");

        let melt = wallet.melt(&quote.id).await?;
        info!("Paid invoice: {}", melt.0.state);

        if let Some(preimage) = melt.0.preimage {
            info!("Payment preimage: {preimage}");
        }

        Ok(melt.1)
    }
    .await?;
    let tx_new = Transaction {
        id: tx.id().to_string(),
        mint_url: tx.mint_url.to_string(),
        io: tx.direction,
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

pub async fn get_all_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = w.list_transactions(None).await?;
    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            id: tx.id().to_string(),
            mint_url: tx.mint_url.to_string(),
            io: tx.direction,
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

pub async fn get_pending_failed_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = w.list_pending_failed_transactions(None).await?;
    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            id: tx.id().to_string(),
            mint_url: tx.mint_url.to_string(),
            io: tx.direction,
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
pub async fn get_cashu_transactions_with_offset(
    offset: usize,
    limit: usize,
) -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = w
        .list_transactions_with_kind_offset(
            offset,
            limit,
            [TransactionKind::Cashu].as_slice(),
            None,
        )
        .await?;

    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            id: tx.id().to_string(),
            mint_url: tx.mint_url.to_string(),
            io: tx.direction,
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

pub async fn get_transactions_with_offset(
    offset: usize,
    limit: usize,
) -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = w
        .list_transactions_with_kind_offset(
            offset,
            limit,
            [TransactionKind::Cashu, TransactionKind::LN].as_slice(),
            None,
        )
        .await?;

    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            id: tx.id().to_string(),
            mint_url: tx.mint_url.to_string(),
            io: tx.direction,
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

pub async fn get_ln_transactions_with_offset(
    offset: usize,
    limit: usize,
) -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = w
        .list_transactions_with_kind_offset(offset, limit, [TransactionKind::LN].as_slice(), None)
        .await?;
    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            id: tx.id().to_string(),
            mint_url: tx.mint_url.to_string(),
            io: tx.direction,
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

pub async fn get_ln_pending_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = w
        .list_pending_transactions_with_kind([TransactionKind::LN].as_slice(), None)
        .await?;

    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            id: tx.id().to_string(),
            mint_url: tx.mint_url.to_string(),
            io: tx.direction,
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

pub async fn get_cashu_pending_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = w
        .list_pending_transactions_with_kind([TransactionKind::Cashu].as_slice(), None)
        .await?;

    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            id: tx.id().to_string(),
            mint_url: tx.mint_url.to_string(),
            io: tx.direction,
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

pub async fn get_pending_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = w
        .list_pending_transactions_with_kind(
            [TransactionKind::Cashu, TransactionKind::LN].as_slice(),
            None,
        )
        .await?;

    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            id: tx.id().to_string(),
            mint_url: tx.mint_url.to_string(),
            io: tx.direction,
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

pub async fn get_failed_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = w
        .list_failed_transactions_with_kind(
            [TransactionKind::Cashu, TransactionKind::LN].as_slice(),
            None,
        )
        .await?;

    let mut txs_new = Vec::new();
    for tx in txs {
        let tx_new = Transaction {
            id: tx.id().to_string(),
            mint_url: tx.mint_url.to_string(),
            io: tx.direction,
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
pub async fn remove_transactions(
    unix_timestamp_le: u64,
    _status: TransactionStatus,
) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let _tx = w.remove_transactions(unix_timestamp_le).await?;

    Ok(())
}

pub async fn get_pending_transactions_count() -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = async move {
        let proofs = w.list_all_pending_proofs().await?;
        let counts = proofs.values().map(|(v, _)| v.len() as u64).sum();
        Ok(counts)
    }
    .await?;
    Ok(tx)
}

pub async fn check_transaction(id: String) -> anyhow::Result<Transaction> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let tx_id = TransactionId::from_str(&id)?;
    let tx = async move {
        let mut tx = w
            .localstore
            .get_transaction(tx_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;

        let wallet = get_wallet_by_mint_url(w, &tx.mint_url.to_string(), unit).await?;
        if tx.status == cdk_common::wallet::TransactionStatus::Pending {
            let _check = wallet.check_pending_transaction_state(id.clone()).await?;
            tx = w
                .localstore
                .get_transaction(tx_id)
                .await?
                .ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;
        }

        if tx.status == cdk_common::wallet::TransactionStatus::Failed {
            let _check = wallet.check_failed_transaction(id).await?;
            // may be the tx has been removed after check, so may be not found again
            if let Some(new_tx) = w.localstore.get_transaction(tx_id).await? {
                tx = new_tx;
            } else {
                warn!("Transaction not found again, keep original tx: {}", tx_id);
            }
        }

        let tx_new = Transaction {
            id: tx.id().to_string(),
            mint_url: tx.mint_url.to_string(),
            io: tx.direction,
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
    .await?;

    Ok(tx)
}

// add failed tx check independently
pub async fn check_failed_transaction(id: String) -> anyhow::Result<Transaction> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let tx_id = TransactionId::from_str(&id)?;
    let tx = async move {
        let mut tx = w
            .localstore
            .get_transaction(tx_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;

        let wallet = get_wallet_by_mint_url(w, &tx.mint_url.to_string(), unit).await?;

        if tx.status == cdk_common::wallet::TransactionStatus::Failed {
            let _check = wallet.check_failed_transaction(id).await?;
            // may be the tx has been removed after check, so may be not found again
            if let Some(new_tx) = w.localstore.get_transaction(tx_id).await? {
                tx = new_tx;
            } else {
                warn!("Transaction not found again, keep original tx: {}", tx_id);
            }
        }

        let tx_new = Transaction {
            id: tx.id().to_string(),
            mint_url: tx.mint_url.to_string(),
            io: tx.direction,
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
    .await?;

    Ok(tx)
}
/// (spents, pendings, all)
pub async fn get_all_proofs_data() -> anyhow::Result<(usize, usize, usize)> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = async move {
        let all_proofs = w.list_all_proofs().await?;
        let spent_proofs = w.list_spent_proofs().await?;
        let pending_proofs = w.list_pending_proofs().await?;
        Ok((
            all_proofs.values().len(),
            spent_proofs.values().len(),
            pending_proofs.values().len(),
        ))
    }
    .await?;

    Ok(tx)
}

pub fn decode_token(encoded_token: String) -> anyhow::Result<TokenInfo> {
    let token: Token = Token::from_str(&encoded_token)?;

    Ok(TokenInfo {
        // encoded_token,
        mint: token.mint_url()?.to_string(),
        amount: token.value()?.into(),
        unit: token.unit().as_ref().map(|s| s.to_string()),
        memo: token.memo().clone(),
    })
}

/// sleepms_after_check_a_batch for (code: 429): {"detail":"Rate limit exceeded."}
pub async fn restore(mint_url: String, words: Option<String>) -> anyhow::Result<(u64, u64)> {
    let mint_url = MintUrl::from_str(&mint_url)?;
    let mut state = State::lock()?;
    let unit = CurrencyUnit::from_str("sat")?;

    if words.is_some() {
        let mi = MnemonicInfo::with_words(&words.unwrap())?;
        let _ = state.update_mnmonic(Some(Arc::new(mi)));
    }

    let w = state.get_wallet()?;

    let amount = async move {
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
    }
    .await?;

    Ok((amount.0.into(), amount.1))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    // pub encoded_token: String,
    pub mint: String,
    pub amount: u64,
    pub unit: Option<String>,
    pub memo: Option<String>,
}

pub fn decode_invoice(encoded_invoice: String) -> anyhow::Result<InvoiceInfo> {
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
        true => InvoiceStatus::Expired,
        false => InvoiceStatus::Unpaid,
    };

    let ts = (invoice.duration_since_epoch() + invoice.expiry_time()).as_millis();
    debug!(
        "{:?}+{:?}={}",
        invoice.duration_since_epoch(),
        invoice.expiry_time(),
        ts
    );

    Ok(InvoiceInfo {
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
pub struct InvoiceInfo {
    // pub bolt11: String,
    pub amount: u64,
    pub expiry_ts: u64,
    pub hash: String,
    pub memo: Option<String>,
    pub mint: Option<String>,
    pub status: InvoiceStatus,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Paid,
    Unpaid,
    Expired,
}
