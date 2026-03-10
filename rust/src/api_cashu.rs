use anyhow::Ok;
use cashu::nuts::nut00::{PaymentMethod, Proofs};
pub use cashu::{Amount, CurrencyUnit, Id, KeySetInfo, MintInfo, MintUrl};
pub use cdk::amount::{SplitTarget, MSAT_IN_SAT};
use cdk::cdk_database;
use cdk::fees::calculate_fee;
pub use cdk::lightning_invoice::{
    Bolt11Invoice as Invoice, Bolt11InvoiceDescriptionRef as InvoiceDescriptionRef,
};
use cdk::nuts::nut00::ProofsMethods;
use cdk::nuts::{MeltOptions, Token};
use cdk::wallet::ReceiveOptions;
pub use cdk::wallet::{SendOptions, Wallet, WalletRepository, WalletRepositoryBuilder};
/// Type alias for backward compatibility with generated FFI code
pub type MultiMintWallet = WalletRepository;
pub use cdk::Bolt11Invoice;
use cdk_common::common::ProofInfo;
use cdk_common::database::WalletDatabase;
pub use cdk_common::wallet::{
    TransactionDirection, TransactionId, TransactionKind, TransactionStatus,
};
use cdk_sqlite::WalletSqliteDatabase;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::{Arc, Mutex as StdMutex, MutexGuard as StdMutexGuard};
use tokio::runtime::{Builder, Runtime};
use tokio::time::Duration;

#[frb(ignore)]
pub struct State {
    rt: Arc<Runtime>,
    wallet: Option<WalletRepository>,
    mnemonic: Option<Arc<MnemonicInfo>>,
    sats: u16,
}

#[path = "api_cashu.types.rs"]
pub mod types;
pub use types::*;

use crate::api_cashu_v1;

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
    pub fn get_wallet(&self) -> anyhow::Result<&WalletRepository> {
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

// for cashu v1 init db and send all
pub fn cashu_v1_init_send_all(
    dbpath: String,
    words: Option<String>,
) -> anyhow::Result<CashuV1ToV2> {
    let re = api_cashu_v1::cashu_v1_init_send_all(dbpath, words)?;
    Ok(CashuV1ToV2 {
        tokens: re.0,
        counters: re.1,
        unavailable_mints: re.2,
    })
}

//cashu_init_test
pub fn cashu_v1_init_test(
    dbpath: String,
    words: Option<String>,
    tokens: String,
) -> anyhow::Result<()> {
    let _re = api_cashu_v1::cashu_init_test(dbpath, words, tokens)?;
    Ok(())
}
// for cashu v1 init db and send all
pub fn cashu_v1_init_proofs(
    dbpath: String,
    words: Option<String>,
) -> anyhow::Result<CashuProofsV1ToV2> {
    let re = api_cashu_v1::cashu_v1_init_proofs(dbpath, words)?;
    Ok(CashuProofsV1ToV2 {
        proofs: re.0,
        counters: re.1,
    })
}

pub fn init_v1_and_get_poorfs_to_v2(
    dbpath_old: String,
    dbpath_new: String,
    words: String,
) -> anyhow::Result<(String, Vec<String>)> {
    let re = api_cashu_v1::cashu_v1_init_proofs(dbpath_old, Some(words.clone()))?;
    init_db(dbpath_new.clone(), words.clone(), false)?;
    init_cashu(32)?;
    let counters = re.1;
    let mints = add_counters(counters.clone())?;
    add_proofs_from_v1(re.0)?;
    for mint in &mints {
        let restore = restore(mint.to_string(), Some(words.clone()))?;
        println!("restore mint {} re: {:?}", mint, restore);
    }
    Ok((counters, mints))
}

// this is for init db and cashu once, only call once
pub fn init_db_cashu_once(dbpath: String) -> anyhow::Result<()> {
    init_db_once(dbpath)?;
    init_cashu(32)?;
    Ok(())
}

fn init_db_once(dbpath: String) -> anyhow::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let words = MnemonicInfo::generate_words(12)?;

    let _ = set_mnemonic(Some(words.clone()));

    let mi = MnemonicInfo::with_words(&words)?;
    let seed = mi.mnemonic().to_seed("");

    let mut state = State::lock()?;

    let fut = async move {
        let localstore: Arc<dyn WalletDatabase<cdk_database::Error> + Send + Sync> =
            Arc::new(WalletSqliteDatabase::new(&*dbpath).await?);

        let wallet_repo = WalletRepositoryBuilder::new()
            .localstore(localstore)
            .seed(seed)
            .build()
            .await?;

        Ok(wallet_repo)
    };

    let result = state.rt.block_on(fut);

    result.map(|w| {
        state.wallet = Some(w);
    })
}

pub fn init_db(dbpath: String, words: String, _dev: bool) -> anyhow::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let _ = set_mnemonic(Some(words.clone()));

    let mi = MnemonicInfo::with_words(&words)?;
    let seed = mi.mnemonic().to_seed("");

    let mut state = State::lock()?;

    let fut = async move {
        let localstore: Arc<dyn WalletDatabase<cdk_database::Error> + Send + Sync> =
            Arc::new(WalletSqliteDatabase::new(&*dbpath).await?);

        let wallet_repo = WalletRepositoryBuilder::new()
            .localstore(localstore)
            .seed(seed)
            .build()
            .await?;

        Ok(wallet_repo)
    };

    let result = state.rt.block_on(fut);

    result.map(|w| {
        state.wallet = Some(w);
    })
}

pub fn init_cashu(prepare_sats_once_time: u16) -> anyhow::Result<Vec<MintCashu>> {
    let mut state = State::lock()?;
    state.sats = prepare_sats_once_time;

    let w = state.get_wallet()?;
    let result = state.rt.block_on(w.localstore.get_mints())?;
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

pub fn get_mints() -> anyhow::Result<Vec<MintCashu>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    // let mut mints = Vec::new();
    let result = state.rt.block_on(w.localstore.get_mints())?;
    let mints = decode_mint_info(result)?;
    Ok(mints)
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
            if wallet.fetch_mint_info().await.is_ok() {
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

fn add_proofs_from_v1(proofs: String) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let proofs: Vec<ProofInfo> = serde_json::from_str(&proofs)?;
    let mut mints = HashSet::new();
    for p in proofs.clone() {
        mints.insert(p.mint_url);
    }
    if proofs.is_empty() {
        warn!("The proofs vector is empty.");
        return Ok(());
    }

    let _tx = state.rt.block_on(async {
        for mint_url in mints {
            let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
            wallet.fetch_mint_info().await?;
        }

        w.localstore.update_proofs(proofs, vec![]).await?;
        Ok(())
    });
    Ok(())
}

fn _add_proofs(proofs: Vec<ProofInfo>) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    if proofs.is_empty() {
        warn!("The proofs vector is empty.");
        return Ok(());
    }
    Ok(state
        .rt
        .block_on(w.localstore.update_proofs(proofs, vec![]))?)
}

pub fn get_balances() -> anyhow::Result<String> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;

    let rt = &state.rt;

    let js = rt.block_on(async {
        let mints_map = w.localstore.get_mints().await?;
        let active_urls: HashSet<String> = mints_map
            .keys()
            .map(|u| u.to_string().trim_end_matches('/').to_string())
            .collect();

        let bs = w.get_balances().await?;

        let filtered_bs = bs
            .into_iter()
            .filter(|(k, _v)| {
                k.unit == unit && active_urls.contains(k.mint_url.to_string().trim_end_matches('/'))
            })
            .map(|(k, v)| (k.mint_url.to_string().trim_end_matches('/').to_string(), v))
            .collect::<std::collections::BTreeMap<String, Amount>>();

        let js = serde_json::to_string(&filtered_bs)?;
        Ok(js)
    })?;

    Ok(js)
}

#[frb(ignore)]
pub fn get_wallet(mint_url: MintUrl, unit: CurrencyUnit) -> anyhow::Result<Wallet> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let wallet = state
        .rt
        .block_on(w.get_wallet(&mint_url, &unit))
        .map_err(|_| anyhow::anyhow!("Wallet not found"))?;

    Ok(wallet)
}

/// Helper function to create or get a wallet
#[frb(ignore)]
async fn get_or_create_wallet(
    wallet_repo: &WalletRepository,
    mint_url: &MintUrl,
    unit: CurrencyUnit,
) -> anyhow::Result<cdk::wallet::Wallet> {
    match wallet_repo.get_wallet(mint_url, &unit).await {
        std::result::Result::Ok(wallet) => Ok(wallet),
        std::result::Result::Err(_) => {
            debug!("Wallet does not exist creating..");
            Ok(wallet_repo
                .create_wallet(mint_url.clone(), unit, None)
                .await?)
        }
    }
}

pub fn send_all(mint: String) -> anyhow::Result<Transaction> {
    let _merge = merge_proofs(10)?;
    let tx = _send_all(mint)?;
    Ok(tx)
}

fn _send_all(mint: String) -> anyhow::Result<Transaction> {
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

        let prepared_send = wallet
            .prepare_send(ps.total_amount()?, SendOptions::default())
            .await?;
        let tx = prepared_send.confirm(None).await?;
        Ok(tx)
    })?;
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
pub fn merge_proofs(threshold: u64) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;

    let fut = async move {
        // Iterate all known mints in localstore
        let mints = w.localstore.get_mints().await?;
        for (mint_url, _info) in mints {
            // Only operate on wallets we already have (skip creating new ones here)
            if let std::result::Result::Ok(wallet) = w.get_wallet(&mint_url, &unit).await {
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

        let amount = wallet
            .receive(&encoded_token, ReceiveOptions::default())
            .await;
        debug!("amount: {:?}", amount);
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
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let tx = wallet
            .receive(&encoded_token, ReceiveOptions::default())
            .await?;
        Ok(tx)
    };

    let tx = state.rt.block_on(fut)?;
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
pub fn print_proofs(mint: String) -> anyhow::Result<()> {
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

    let _ = state.rt.block_on(fut)?;

    Ok(())
}

pub fn prepare_one_proofs(mint: String) -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    state.rt.block_on(async {
        // let fut = _prepare_one_proofs(w, mint.clone());
        // let result = match tokio::time::timeout(Duration::from_secs(20), fut).await {
        //     std::result::Result::Ok(std::result::Result::Ok(tx)) => Ok(tx),
        //     std::result::Result::Ok(std::result::Result::Err(e)) => {
        //         error!("prepare_one_proofs error mint={} err={:?}", mint, e);
        //         Err(e)
        //     }
        //     std::result::Result::Err(_) => {
        //         error!("prepare_one_proofs timeout mint={}", mint);
        //         Err(anyhow::anyhow!(
        //             "prepare_one_proofs connection timeout for {}",
        //             mint
        //         ))
        //     }
        // };
        // result
        _prepare_one_proofs(w, mint.clone()).await
    })
}

async fn _prepare_one_proofs(w: &WalletRepository, mint: String) -> anyhow::Result<u64> {
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
        let active_keyset = wallet.get_active_keyset().await?;
        let active_keyset_ids = vec![active_keyset.id];
        let keyset_fees = wallet.get_keyset_fees_and_amounts().await?;
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

pub fn send_stamp(
    amount: u64,
    mints: Vec<String>,
    info: Option<String>,
) -> anyhow::Result<SendStampsResult> {
    if amount == 0 {
        bail!("can't send amount 0");
    }

    let mut state = State::lock()?;
    let unit = CurrencyUnit::from_str("sat")?;

    let rt = state.rt.clone();

    let balances = {
        let w = state.get_wallet()?;
        let all = rt.block_on(w.get_balances())?;
        all.into_iter()
            .filter(|(k, _)| k.unit == unit)
            .map(|(k, v)| (k.mint_url, v))
            .collect::<std::collections::BTreeMap<MintUrl, Amount>>()
    };

    rt.block_on(async {
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
    })
}

async fn mint_balances(
    wallet_repo: &WalletRepository,
    unit: &CurrencyUnit,
) -> anyhow::Result<Vec<(MintUrl, Amount)>> {
    let balances = wallet_repo.get_balances().await?;

    let mut wallets_vec = Vec::new();

    for (i, (key, amount)) in balances
        .iter()
        .filter(|(k, a)| &k.unit == unit && a > &&Amount::ZERO)
        .enumerate()
    {
        let mint_url = key.mint_url.clone();
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

/// Helper function to get a wallet from the wallet repository by mint URL
async fn get_wallet_by_mint_url(
    wallet_repo: &WalletRepository,
    mint_url_str: &str,
    unit: CurrencyUnit,
) -> anyhow::Result<cdk::wallet::Wallet> {
    let mint_url = MintUrl::from_str(mint_url_str)?;

    let wallet = wallet_repo
        .get_wallet(&mint_url, &unit)
        .await
        .map_err(|_| anyhow::anyhow!("Wallet not found for mint URL: {}", mint_url_str))?;

    Ok(wallet)
}

/// Helper function to get a wallet from the wallet repository
pub async fn get_wallet_by_index(
    wallet_repo: &WalletRepository,
    mint_amounts: &[(MintUrl, Amount)],
    mint_number: usize,
    unit: CurrencyUnit,
) -> anyhow::Result<cdk::wallet::Wallet> {
    validate_mint_number(mint_number, mint_amounts.len())?;

    let mint_url = &mint_amounts[mint_number].0;
    let wallet = wallet_repo
        .get_wallet(mint_url, &unit)
        .await
        .map_err(|_| anyhow::anyhow!("Wallet not found"))?;

    Ok(wallet)
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

use std::time::Instant;

// for debug and test, this will print the time taken for the async operation
async fn _time<F, T>(label: &str, fut: F) -> T
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
        prepared_send.confirm(None).await?
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
        let wallet = get_wallet_by_mint_url(w, &active_mint, unit).await?;

        let mint_url = &wallet.mint_url;
        let mint_amount = mints_amounts?
            .iter()
            .find(|(url, _)| url == mint_url)
            .map(|(_, amount)| *amount)
            .ok_or_else(|| anyhow!("Could not find balance for mint: {}", mint_url));
        let mint_amount = mint_amount?;
        check_sufficient_funds(mint_amount.clone(), amount.into())?;

        // 1) First prepare_send (NOTE: this reserves proofs!)
        let prepared = wallet
            .prepare_send(Amount::from(amount), SendOptions::default())
            .await?;

        // 2) Decide based on ACTUAL selected proofs whether token will be too long by
        // "too many same-denom small proofs".
        //
        // Policy knobs:
        // - watch these denoms
        // - if in (proofs_to_send + proofs_to_swap) any watched denom count >= threshold => merge
        let denoms_to_watch: &[u64] = &[1, 2];
        let per_denom_threshold: usize = 15;

        let need_merge = {
            let mut counts: HashMap<u64, usize> = HashMap::new();
            for p in prepared.proofs_to_send().iter() {
                let d = *p.amount.as_ref();
                if denoms_to_watch.contains(&d) {
                    *counts.entry(d).or_insert(0) += 1;
                }
            }
            let result = counts.values().any(|c| *c >= per_denom_threshold);
            log::debug!(
                "pre-send check: proofs_to_send={} watched={:?} need_merge={}",
                prepared.proofs_to_send().len(),
                counts,
                result
            );
            result
        };

        if !need_merge {
            // Fast path: no merge needed, send directly
            let tx = prepared.confirm(None).await?;
            return Ok(tx);
        }
        // 3) Cancel reserved proofs; if cancel fails, try restore to recover reserved proofs
        if let Err(e) = prepared.cancel().await {
            log::error!(
                "cancel_send failed, attempting restore to recover pending proofs. err: {:?}",
                e
            );
            match wallet.restore().await {
                std::result::Result::Ok(restored) => {
                    log::info!(
                        "restore after cancel_send failure succeeded: {:?}",
                        restored
                    );
                }
                Err(restore_err) => {
                    log::error!(
                        "restore after cancel_send failure also failed: {:?}",
                        restore_err
                    );
                }
            }
            return Err(anyhow::anyhow!(
                "cancel_send failed for mint {} of send, restore attempted: {}",
                active_mint,
                e
            ));
        }

        // 4) Collect all small-denom unspent proofs for merging (single pass)
        let all_proofs = wallet.get_unspent_proofs().await?;
        let total = all_proofs.total_amount()?;

        let inputs: Proofs = all_proofs
            .into_iter()
            .filter(|p| denoms_to_watch.contains(p.amount.as_ref()))
            .collect();

        // 5) Fee guard + merge (best-effort, with restore on failure)
        if inputs.len() >= 2 {
            let keyset_fee_and_amounts = wallet.get_keyset_fees_and_amounts().await?;
            let keyset_fees: HashMap<Id, u64> = keyset_fee_and_amounts
                .iter()
                .map(|(k, v)| (*k, v.fee()))
                .collect();
            let est_fee = calculate_fee(&inputs.count_by_keyset(), &keyset_fees)
                .map(|f| f.total)
                .unwrap_or(Amount::ZERO);

            if total >= Amount::from(amount) + est_fee {
                if let Err(e) = wallet
                    .swap(None, SplitTarget::default(), inputs, None, false)
                    .await
                {
                    log::warn!("pre-send merge swap failed, attempting restore: {:?}", e);
                    match wallet.restore().await {
                        std::result::Result::Ok(restored) => {
                            log::info!("restore after merge failure succeeded: {:?}", restored);
                        }
                        Err(restore_err) => {
                            log::error!(
                                "restore after merge failure also failed: {:?}",
                                restore_err
                            );
                        }
                    }
                }
            } else {
                log::debug!(
                    "skip merge: total={} send={} fee={}",
                    *total.as_ref(),
                    amount,
                    *est_fee.as_ref()
                );
            }
        }

        // 6) Re-prepare and send
        let prepared2 = wallet
            .prepare_send(Amount::from(amount), SendOptions::default())
            .await?;
        let tx = prepared2.confirm(None).await?;
        Ok(tx)
    })?;

    Ok(Transaction {
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
    })
}

pub fn request_mint(amount: u64, active_mint: String) -> anyhow::Result<Transaction> {
    if amount == 0 {
        bail!("can't mint amount 0");
    }
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&active_mint)?;

    let state = State::lock()?;

    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        let wallet = get_or_create_wallet(w, &mint_url, unit).await?;
        let (_quote, tx) = wallet
            .mint_quote(
                PaymentMethod::BOLT11,
                Some(Amount::from(amount)),
                None,
                None,
            )
            .await?;

        Ok(tx)
    })?;
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
pub fn check_all_mint_quotes() -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let tx = state.rt.block_on(async {
        let check = w.check_all_mint_quotes(None).await?;
        let amounts: u64 = *check.as_ref();

        Ok(amounts)
    })?;

    Ok(tx)
}

/// check_melt_quote_id test
pub fn check_melt_quote_id(quote_id: String, mint_url: String) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint_url)?;
    let _tx = state.rt.block_on(async {
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let re = wallet.check_melt_quote_status(&quote_id).await?;
        println!("check_melt_quote_status: {:?}", re);

        Ok(())
    })?;

    Ok(())
}

/// Recover incomplete sagas after crash. Call after init_cashu in background.
pub fn recover_sagas() -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    state.rt.block_on(async {
        let mints = w.localstore.get_mints().await?;
        for (mint_url, _info) in mints {
            let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
            if let Err(e) = wallet.recover_incomplete_sagas().await {
                error!(
                    "recover_incomplete_sagas mint_url: {} error: {:?}",
                    mint_url, e
                );
            }
            if let Err(e) = wallet.mint_unissued_quotes().await {
                error!("mint_unissued_quotes mint_url: {} error: {:?}", mint_url, e);
            }
        }
        Ok(())
    })?;
    Ok(())
}

/// Checks pending proofs for spent status
pub fn check_proofs() -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let _tx = state.rt.block_on(async {
        let mints = w.localstore.get_mints().await?;
        let mut errs: Vec<String> = Vec::new();
        for (mint_url, _info) in mints {
            debug!("check_proofs mint_url: {}", mint_url);
            let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
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
    })?;
    Ok(())
}

/// include ln and cashu, tx status
pub fn check_pending() -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let _tx = state.rt.block_on(async {
        let mut check_map = HashMap::new();
        let mints = w.localstore.get_mints().await?;
        for (mint_url, _info) in mints {
            let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
            let check = wallet.check_pending_transactions_state().await?;
            check_map.insert(mint_url.to_string(), check);
        }
        Ok(check_map)
    })?;

    Ok(())
}

/// include ln and cashu, tx status, use check_transaction instead
pub fn check_single_pending(tx_id: String, mint_url: String) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint_url)?;
    let _tx = state.rt.block_on(async {
        let mut check_map = HashMap::new();
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let check = wallet
            .check_pending_transaction_state(tx_id.clone())
            .await?;
        check_map.insert(mint_url.to_string(), check);
        Ok(check_map)
    })?;

    Ok(())
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
        let quote = wallet
            .melt_quote(PaymentMethod::BOLT11, bolt11.to_string(), options, None)
            .await?;
        info!("melt_quote {quote:?}");

        let prepared_melt = wallet.prepare_melt(&quote.id, HashMap::new()).await?;
        let melt = prepared_melt.confirm().await?;
        info!("Paid invoice: {:?}", melt.state());

        if let Some(preimage) = melt.payment_proof() {
            info!("Payment preimage: {preimage}");
        }

        Ok(melt)
    })?;
    let tx_new = match tx.transaction() {
        Some(cdk_tx) => Transaction {
            id: cdk_tx.id().to_string(),
            mint_url: cdk_tx.mint_url.to_string(),
            io: cdk_tx.direction.clone(),
            kind: cdk_tx.kind.clone(),
            amount: *cdk_tx.amount.as_ref(),
            fee: *cdk_tx.fee.as_ref(),
            unit: Some(cdk_tx.unit.to_string()),
            token: cdk_tx.token.clone(),
            status: cdk_tx.status,
            timestamp: cdk_tx.timestamp,
            metadata: cdk_tx.metadata.clone(),
        },
        None => Transaction {
            id: tx.quote_id().to_string(),
            mint_url: active_mint,
            io: TransactionDirection::Outgoing,
            kind: TransactionKind::LN,
            amount: *tx.amount().as_ref(),
            fee: *tx.fee_paid().as_ref(),
            unit: Some("sat".to_string()),
            token: invoice,
            status: TransactionStatus::Pending,
            timestamp: 0,
            metadata: HashMap::new(),
        },
    };

    Ok(tx_new)
}

// this is just for test
pub fn get_all_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = state.rt.block_on(w.list_transactions(None))?;
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

pub fn get_transactions_with_offset(
    offset: usize,
    limit: usize,
) -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = state.rt.block_on(w.list_transactions_with_kind_offset(
        offset,
        limit,
        [TransactionKind::Cashu, TransactionKind::LN].as_slice(),
        None,
    ))?;

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

fn get_transactions_with_offset_mint_amount(
    offset: usize,
    limit: usize,
    mint_url: String,
    amount: Option<i64>,
) -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = state
        .rt
        .block_on(w.list_transactions_with_kind_amount_offset(
            offset,
            limit,
            &mint_url,
            [TransactionKind::Cashu, TransactionKind::LN].as_slice(),
            None,
            amount,
        ))?;

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

pub fn get_txs_with_offset_mint_amount(
    offset: usize,
    limit: usize,
    mint_url: String,
    is_one_amount: bool,
) -> anyhow::Result<Vec<Transaction>> {
    if is_one_amount {
        get_transactions_with_offset_mint_amount(offset, limit, mint_url, Some(1))
    } else {
        // -1 means not equal to 1 sat
        get_transactions_with_offset_mint_amount(offset, limit, mint_url, Some(-1))
    }
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

pub fn get_ln_pending_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = state
        .rt
        .block_on(w.list_pending_transactions_with_kind([TransactionKind::LN].as_slice(), None))?;
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

pub fn get_cashu_pending_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = state.rt.block_on(
        w.list_pending_transactions_with_kind([TransactionKind::Cashu].as_slice(), None),
    )?;

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

pub fn get_failed_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let txs = state.rt.block_on(w.list_failed_transactions_with_kind(
        [TransactionKind::Cashu, TransactionKind::LN].as_slice(),
        None,
    ))?;

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
pub fn remove_transactions(
    unix_timestamp_le: u64,
    _status: TransactionStatus,
) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let _tx = state
        .rt
        .block_on(w.localstore.remove_transactions(unix_timestamp_le))?;

    Ok(())
}

pub fn get_pending_transactions_count() -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        let proofs = w
            .localstore
            .get_proofs(
                None,
                None,
                Some(vec![cashu::State::Pending, cashu::State::PendingSpent]),
                None,
            )
            .await?;
        Ok(proofs.len() as u64)
    })?;
    Ok(tx)
}

pub fn check_transaction(id: String) -> anyhow::Result<Transaction> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let tx_id = TransactionId::from_str(&id)?;
    let tx = state.rt.block_on(async {
        let mut tx = w
            .localstore
            .get_transaction(tx_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;

        let wallet = get_wallet_by_mint_url(w, &tx.mint_url.to_string(), unit).await?;
        if tx.status == cdk_common::wallet::TransactionStatus::Pending {
            tx = wallet.check_pending_transaction_state(id.clone()).await?;
        }

        if tx.status == cdk_common::wallet::TransactionStatus::Failed {
            tx = wallet.check_failed_transaction(id).await?;
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
    })?;

    Ok(tx)
}

// add failed tx check independently
pub fn check_failed_transaction(id: String) -> anyhow::Result<Transaction> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let tx_id = TransactionId::from_str(&id)?;
    let tx = state.rt.block_on(async {
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
    })?;

    Ok(tx)
}
/// (spents, pendings, all), do not use
pub fn get_all_proofs_data() -> anyhow::Result<(usize, usize, usize)> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        let all_proofs = w.localstore.get_proofs(None, None, None, None).await?;
        let spent_proofs = w
            .localstore
            .get_proofs(None, None, Some(vec![cashu::State::Spent]), None)
            .await?;
        let pending_proofs = w
            .localstore
            .get_proofs(None, None, Some(vec![cashu::State::Pending]), None)
            .await?;
        Ok((all_proofs.len(), spent_proofs.len(), pending_proofs.len()))
    })?;

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
pub fn restore(mint_url: String, words: Option<String>) -> anyhow::Result<(u64, u64)> {
    let mint_url = MintUrl::from_str(&mint_url)?;
    let mut state = State::lock()?;
    let unit = CurrencyUnit::from_str("sat")?;

    if words.is_some() {
        let mi = MnemonicInfo::with_words(&words.unwrap())?;
        let _ = state.update_mnmonic(Some(Arc::new(mi)));
    }

    let w = state.get_wallet()?;

    let amount = state.rt.block_on(async {
        let wallet = get_or_create_wallet(w, &mint_url, unit).await?;

        let amount = wallet.restore().await?;
        Ok(amount)
    })?;

    Ok((*amount.unspent.as_ref(), *amount.spent.as_ref()))
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
