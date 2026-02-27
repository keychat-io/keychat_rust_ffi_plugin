use anyhow::Ok;
use cashu::nuts::nut00::Proofs;
pub use cashu::{Amount, CurrencyUnit, Id, KeySetInfo, MintInfo, MintUrl};
pub use cdk::amount::{SplitTarget, MSAT_IN_SAT};
use cdk::cdk_database;
use cdk::fees::calculate_fee;
pub use cdk::lightning_invoice::{
    Bolt11Invoice as Invoice, Bolt11InvoiceDescriptionRef as InvoiceDescriptionRef,
};
use cdk::nuts::nut00::ProofsMethods;
use cdk::nuts::{MeltOptions, Token};
use cdk::wallet::types::WalletKey;
use cdk::wallet::ReceiveOptions;
pub use cdk::wallet::{MultiMintWallet, SendOptions, Wallet, WalletBuilder};
pub use cdk::Bolt11Invoice;
use cdk_common::common::ProofInfo;
use cdk_common::database::WalletDatabase;
pub use cdk_common::wallet::{TransactionId, TransactionKind, TransactionStatus};
use cdk_sqlite::WalletSqliteDatabase;
use std::collections::{BTreeMap, HashMap, HashSet};
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

use crate::api_cashu_v1;

fn into_transaction(tx: cdk_common::wallet::Transaction) -> Transaction {
    Transaction {
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
    }
}

fn into_transactions(txs: Vec<cdk_common::wallet::Transaction>) -> Vec<Transaction> {
    txs.into_iter().map(into_transaction).collect()
}

async fn build_multi_mint_wallet(dbpath: &str, seed: &[u8; 64]) -> anyhow::Result<MultiMintWallet> {
    let localstore: Arc<dyn WalletDatabase<Err = cdk_database::Error> + Send + Sync> =
        Arc::new(WalletSqliteDatabase::new(dbpath).await?);

    let mut wallets: Vec<Wallet> = Vec::new();

    let mints = localstore.get_mints().await?;

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
            let builder = WalletBuilder::new()
                .mint_url(mint_url.clone())
                .unit(unit)
                .localstore(localstore.clone())
                .seed(seed);

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

    Ok(MultiMintWallet::new(
        localstore,
        Arc::new(seed.clone()),
        wallets,
    ))
}

async fn try_restore_wallet(wallet: &Wallet) {
    match wallet.restore().await {
        std::result::Result::Ok(restored) => {
            log::info!("restore succeeded: {:?}", restored);
        }
        Err(e) => {
            log::error!("restore failed: {:?}", e);
        }
    }
}

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

pub fn cashu_v1_init_test(
    dbpath: String,
    words: Option<String>,
    tokens: String,
) -> anyhow::Result<()> {
    let _re = api_cashu_v1::cashu_init_test(dbpath, words, tokens)?;
    Ok(())
}

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
    let result = state.rt.block_on(build_multi_mint_wallet(&dbpath, &seed));
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
    let result = state.rt.block_on(build_multi_mint_wallet(&dbpath, &seed));
    result.map(|w| {
        state.wallet = Some(w);
    })
}

pub fn init_cashu(prepare_sats_once_time: u16) -> anyhow::Result<Vec<MintCashu>> {
    let mut state = State::lock()?;
    state.sats = prepare_sats_once_time;
    let w = state.get_wallet()?;
    let result = state.rt.block_on(w.localstore.get_mints())?;
    decode_mint_info(result)
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
    state.update_mnmonic(mnemonic)
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
            mints.push(MintCashu {
                url: k.to_string(),
                active: true,
                time: v.time.unwrap_or(0),
                info: Some(mint_info),
            });
        }
    }
    Ok(mints)
}

pub fn get_mints() -> anyhow::Result<Vec<MintCashu>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let result = state.rt.block_on(w.localstore.get_mints())?;
    decode_mint_info(result)
}

pub fn add_mint(url: String) -> anyhow::Result<()> {
    let mint_url = MintUrl::from_str(&url)?;
    let state = State::lock()?;
    let w = state.get_wallet()?;
    state.rt.block_on(w.localstore.add_mint(mint_url, None))?;
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
            if wallet.get_mint_info().await.is_ok() {
                mints.push(mint_url.clone());
            } else {
                mints.push(format!("{}-failure", mint_url));
                warn!("Failed to get mint info for {}", mint_url);
            }
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
            wallet.get_mint_info().await?;
        }
        w.localstore.update_proofs(proofs, vec![]).await?;
        Ok(())
    });
    Ok(())
}

pub fn get_balances() -> anyhow::Result<String> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    state.rt.block_on(async {
        let unit = CurrencyUnit::from_str("sat")?;
        let mints_map = w.localstore.get_mints().await?;
        let active_urls: HashSet<String> = mints_map
            .keys()
            .map(|u| u.to_string().trim_end_matches('/').to_string())
            .collect();

        let bs = w.get_balances(&unit).await?;
        let filtered_bs = bs
            .into_iter()
            .filter(|(k, _v)| active_urls.contains(k.to_string().trim_end_matches('/')))
            .map(|(k, v)| (k.to_string().trim_end_matches('/').to_string(), v))
            .collect::<BTreeMap<String, Amount>>();

        Ok(serde_json::to_string(&filtered_bs)?)
    })
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

pub async fn get_wallet_by_index(
    multi_mint_wallet: &MultiMintWallet,
    mint_amounts: &[(MintUrl, Amount)],
    mint_number: usize,
    unit: CurrencyUnit,
) -> anyhow::Result<cdk::wallet::Wallet> {
    if mint_number >= mint_amounts.len() {
        bail!("Invalid mint number");
    }
    let wallet_key = WalletKey::new(mint_amounts[mint_number].0.clone(), unit);
    let wallet = multi_mint_wallet
        .get_wallet(&wallet_key)
        .await
        .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;
    Ok(wallet.clone())
}

async fn mint_balances(
    multi_mint_wallet: &MultiMintWallet,
    unit: &CurrencyUnit,
) -> anyhow::Result<Vec<(MintUrl, Amount)>> {
    let wallets: BTreeMap<MintUrl, Amount> = multi_mint_wallet.get_balances(unit).await?;
    Ok(wallets
        .into_iter()
        .filter(|(_, a)| *a > Amount::ZERO)
        .collect())
}

fn check_sufficient_funds(available: Amount, required: Amount) -> anyhow::Result<()> {
    if required.gt(&available) {
        bail!("Not enough funds");
    }
    Ok(())
}

pub fn send_all(mint: String) -> anyhow::Result<Transaction> {
    merge_proofs(10)?;
    _send_all(mint)
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
            bail!("The amount is 0");
        }
        let prepared_send = wallet
            .prepare_send(ps.total_amount()?, SendOptions::default())
            .await?;
        Ok(wallet.send(prepared_send, None).await?)
    })?;

    Ok(into_transaction(tx))
}

pub fn merge_proofs(threshold: u64) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;

    state.rt.block_on(async {
        let mints = w.localstore.get_mints().await?;
        for (mint_url, _info) in mints {
            if let Some(wallet) = w
                .get_wallet(&WalletKey::new(mint_url.clone(), unit.clone()))
                .await
            {
                let proofs = wallet.get_unspent_proofs().await?;
                if proofs.is_empty() {
                    continue;
                }
                let mut groups: HashMap<u64, cashu::Proofs> = HashMap::new();
                for p in proofs {
                    let amt = *p.amount.as_ref();
                    groups.entry(amt).or_insert_with(cashu::Proofs::new).push(p);
                }
                for (_amt, group) in groups.into_iter() {
                    if group.len() as u64 >= threshold {
                        let _ = wallet
                            .swap(None, SplitTarget::default(), group, None, false)
                            .await?;
                    }
                }
            }
        }
        Ok(())
    })
}

pub fn multi_receive(stamps: Vec<String>) -> anyhow::Result<()> {
    let token: Token = Token::from_str(&stamps[0])?;
    let mint_url = token.mint_url()?;
    let unit = token.unit().unwrap_or_default();

    let state = State::lock()?;
    let w = state.get_wallet()?;
    state.rt.block_on(async {
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
                None => wallet.get_mint_keysets().await?,
            };
            all_proofs.extend(token_data.proofs(&keysets_info)?);
        }
        let proofs_state = wallet.check_proofs_spent(all_proofs.clone()).await?;
        let unspent: cashu::Proofs = all_proofs
            .into_iter()
            .zip(proofs_state)
            .filter_map(|(p, s)| (s.state == cashu::State::Unspent).then_some(p))
            .collect();

        let tokens = Token::new(mint_url, unspent, None, unit);
        let _amount = w
            .receive(&tokens.to_string(), ReceiveOptions::default())
            .await;
        Ok(())
    })
}

pub fn receive_token(encoded_token: String) -> anyhow::Result<Transaction> {
    let token: Token = Token::from_str(&encoded_token)?;
    let mint_url = token.mint_url()?;
    let unit = token.unit().unwrap_or_default();

    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(async {
        if w.get_wallet(&WalletKey::new(mint_url.clone(), unit.clone()))
            .await
            .is_none()
        {
            get_or_create_wallet(w, &mint_url, unit).await?;
        }
        Ok(w.receive(&encoded_token, ReceiveOptions::default()).await?)
    })?;

    Ok(into_transaction(tx))
}

pub fn print_proofs(mint: String) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint)?;

    state.rt.block_on(async {
        let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
        let ps = wallet.get_unspent_proofs().await?;
        println!("get_all_proofs len: {:?}", ps.len());
        for p in ps {
            println!(
                "{}: {} {:?}",
                p.amount.as_ref(),
                p.keyset_id,
                p.secret.to_string()
            );
        }
        Ok(())
    })
}

pub fn prepare_one_proofs(mint: String) -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    state.rt.block_on(_prepare_one_proofs(w, mint))
}

async fn _prepare_one_proofs(w: &MultiMintWallet, mint: String) -> anyhow::Result<u64> {
    let denomination: u64 = 1;
    let threshold: u64 = 10;
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
        .ok_or_else(|| anyhow!("Could not find balance for mint: {}", mint_url))?;

    if *mint_amount.as_ref() <= amount {
        return Ok(0);
    }
    let ps0 = wallet.get_unspent_proofs().await?;
    let count_before0 = ps0
        .iter()
        .filter(|p| *p.amount.as_ref() == denomination)
        .count() as u64;
    if count_before0 >= threshold {
        return Ok(count_before0);
    }

    let mut count_before = 0u64;
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
                selected,
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
        rt.block_on(w.get_balances(&unit))?
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
                    return Ok(SendStampsResult {
                        tx: tx.0,
                        is_need_split: tx.1,
                    });
                }
                std::result::Result::Ok(std::result::Result::Err(e)) => {
                    last_err = Some(e);
                }
                std::result::Result::Err(_) => {
                    last_err = Some(anyhow::anyhow!("connection timeout for {}", mint_url));
                }
            }
        }
        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("No available mints")))
    })
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

    let mints_amounts = mint_balances(w, &unit).await?;
    let wallet = get_wallet_by_mint_url(w, &active_mint, unit).await?;

    let mint_url = &wallet.mint_url;
    let mint_amount = mints_amounts
        .iter()
        .find(|(url, _)| url == mint_url)
        .map(|(_, amount)| *amount)
        .ok_or_else(|| anyhow!("Could not find balance for mint: {}", mint_url))?;
    check_sufficient_funds(mint_amount, amount.into())?;

    let ps = wallet.get_unspent_proofs().await?;
    let stamp_cnts = ps.iter().filter(|p| *p.amount.as_ref() == 1).count() as u64;

    let mut use_send_one = false;
    let prepared_send = if amount == 1 && stamp_cnts > 0 {
        match wallet
            .prepare_send_one_with_enough(amount.into(), SendOptions::default())
            .await
        {
            std::result::Result::Ok(p) => {
                use_send_one = true;
                p
            }
            Err(_) => {
                wallet
                    .prepare_send(amount.into(), SendOptions::default())
                    .await?
            }
        }
    } else {
        wallet
            .prepare_send(amount.into(), SendOptions::default())
            .await?
    };

    let tx = if use_send_one {
        wallet.send_one(prepared_send, None).await?
    } else {
        wallet.send(prepared_send, None).await?
    };

    Ok((into_transaction(tx), stamp_cnts < 10))
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
        let mints_amounts = mint_balances(w, &unit).await?;
        let wallet = get_wallet_by_mint_url(w, &active_mint, unit).await?;

        let mint_amount = mints_amounts
            .iter()
            .find(|(url, _)| url == &wallet.mint_url)
            .map(|(_, a)| *a)
            .ok_or_else(|| anyhow!("Could not find balance for mint: {}", wallet.mint_url))?;
        check_sufficient_funds(mint_amount, amount.into())?;

        let prepared = wallet
            .prepare_send(Amount::from(amount), SendOptions::default())
            .await?;

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
            counts.values().any(|c| *c >= per_denom_threshold)
        };

        if !need_merge {
            return wallet.send(prepared, None).await.map_err(Into::into);
        }

        if let Err(e) = wallet.cancel_send(prepared).await {
            log::error!("cancel_send failed, attempting restore: {:?}", e);
            try_restore_wallet(&wallet).await;
            return Err(anyhow::anyhow!(
                "cancel_send failed for mint {}, restore attempted: {}",
                active_mint,
                e
            ));
        }

        let all_proofs = wallet.get_unspent_proofs().await?;
        let total = all_proofs.total_amount()?;
        let inputs: Proofs = all_proofs
            .into_iter()
            .filter(|p| denoms_to_watch.contains(p.amount.as_ref()))
            .collect();

        if inputs.len() >= 2 {
            let keyset_fees = wallet.get_keyset_fees().await?;
            let est_fee =
                calculate_fee(&inputs.count_by_keyset(), &keyset_fees).unwrap_or_default();
            if total >= Amount::from(amount) + est_fee {
                if let Err(e) = wallet
                    .swap(None, SplitTarget::default(), inputs, None, false)
                    .await
                {
                    log::warn!("merge swap failed, attempting restore: {:?}", e);
                    try_restore_wallet(&wallet).await;
                }
            }
        }

        let prepared2 = wallet
            .prepare_send(Amount::from(amount), SendOptions::default())
            .await?;
        wallet.send(prepared2, None).await.map_err(Into::into)
    })?;

    Ok(into_transaction(tx))
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
        let quote = wallet.mint_quote(Amount::from(amount), None).await?;
        Ok(quote.1)
    })?;

    Ok(into_transaction(tx))
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
        debug!(
            "Received {} from mint {}",
            proofs.0.total_amount()?,
            mint_url
        );
        Ok(proofs.1)
    })?;

    Ok(into_transaction(tx))
}

pub fn check_all_mint_quotes() -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    state.rt.block_on(async {
        let check = w.check_all_mint_quotes(None).await?;
        Ok(check.values().map(|v| *v.as_ref()).sum())
    })
}

pub fn check_melt_quote_id(quote_id: String, mint_url: String) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint_url)?;
    state.rt.block_on(async {
        let wallet = get_or_create_wallet(w, &mint_url, unit).await?;
        let re = wallet.melt_quote_status_only(&quote_id).await?;
        log::debug!("melt_quote_status_only: {:?}", re);
        Ok(())
    })
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
            .map(|(_, a)| *a)
            .ok_or_else(|| anyhow::anyhow!("Could not find balance for mint: {}", mint_url))?;

        let available_funds = <cdk::Amount as Into<u64>>::into(mint_amount) * MSAT_IN_SAT;
        let bolt11 = Bolt11Invoice::from_str(&invoice)?;

        let options = if bolt11.amount_milli_satoshis().is_none() {
            let user_amount = amount.unwrap() * MSAT_IN_SAT;
            if user_amount > available_funds {
                bail!("Not enough funds");
            }
            Some(MeltOptions::new_amountless(user_amount))
        } else {
            let invoice_amount = bolt11.amount_milli_satoshis().unwrap();
            if invoice_amount > available_funds {
                bail!("Not enough funds");
            }
            None
        };

        let quote = wallet.melt_quote(bolt11.to_string(), options).await?;
        let melt = wallet.melt(&quote.id).await?;
        if let Some(preimage) = melt.0.preimage {
            info!("Payment preimage: {preimage}");
        }
        Ok(melt.1)
    })?;

    Ok(into_transaction(tx))
}

pub fn check_proofs() -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    state.rt.block_on(async {
        let mints = w.localstore.get_mints().await?;
        let mut errs: Vec<String> = Vec::new();
        for (mint_url, _) in mints {
            let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
            if let Err(e) = wallet.check_proofs_from_mint().await {
                error!("check_proofs mint_url: {} error: {:?}", mint_url, e);
                errs.push(format!("{}: {}", mint_url, e));
            }
        }
        if !errs.is_empty() {
            return Err(anyhow::anyhow!(
                "check_proofs failed for {} mint(s): {}",
                errs.len(),
                errs.join(" | ")
            ));
        }
        Ok(())
    })
}

pub fn check_pending() -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    state.rt.block_on(async {
        let mints = w.localstore.get_mints().await?;
        for (mint_url, _) in mints {
            let wallet = get_or_create_wallet(w, &mint_url, unit.clone()).await?;
            wallet.check_pending_transactions_state().await?;
        }
        Ok(())
    })
}

pub fn check_single_pending(tx_id: String, mint_url: String) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&mint_url)?;
    state.rt.block_on(async {
        let wallet = get_or_create_wallet(w, &mint_url, unit).await?;
        wallet.check_pending_transaction_state(tx_id).await?;
        Ok(())
    })
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

        if tx.status == TransactionStatus::Pending {
            wallet.check_pending_transaction_state(id.clone()).await?;
            tx = w
                .localstore
                .get_transaction(tx_id)
                .await?
                .ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;
        }

        if tx.status == TransactionStatus::Failed {
            wallet.check_failed_transaction(id).await?;
            if let Some(new_tx) = w.localstore.get_transaction(tx_id).await? {
                tx = new_tx;
            }
        }

        Ok(into_transaction(tx))
    })?;

    Ok(tx)
}

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

        if tx.status == TransactionStatus::Failed {
            let wallet = get_wallet_by_mint_url(w, &tx.mint_url.to_string(), unit).await?;
            wallet.check_failed_transaction(id).await?;
            if let Some(new_tx) = w.localstore.get_transaction(tx_id).await? {
                tx = new_tx;
            }
        }

        Ok(into_transaction(tx))
    })?;

    Ok(tx)
}

pub fn get_all_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let txs = state.rt.block_on(w.list_transactions(None))?;
    Ok(into_transactions(txs))
}

pub fn get_cashu_one_sats_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let txs = state.rt.block_on(w.list_transactions(None))?;
    Ok(into_transactions(
        txs.into_iter()
            .filter(|tx| tx.kind == TransactionKind::Cashu && *tx.amount.as_ref() == 1)
            .collect(),
    ))
}

pub fn get_cashu_one_sats_transactions_with_mint(
    mint_url: String,
) -> anyhow::Result<Vec<Transaction>> {
    let mint_norm = mint_url.trim_end_matches('/').to_string();
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let txs = state.rt.block_on(w.list_transactions(None))?;
    Ok(into_transactions(
        txs.into_iter()
            .filter(|tx| {
                tx.kind == TransactionKind::Cashu
                    && *tx.amount.as_ref() == 1
                    && tx.mint_url.to_string().trim_end_matches('/') == mint_norm
            })
            .collect(),
    ))
}

pub fn get_pending_failed_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let txs = state
        .rt
        .block_on(w.list_pending_failed_transactions(None))?;
    Ok(into_transactions(txs))
}

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
    Ok(into_transactions(txs))
}

pub fn get_cashu_transactions_with_offset_mint(
    offset: usize,
    limit: usize,
    mint_url: String,
) -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let txs = state
        .rt
        .block_on(w.list_transactions_with_kind_offset_mint(
            offset,
            limit,
            &mint_url,
            [TransactionKind::Cashu].as_slice(),
            None,
        ))?;
    Ok(into_transactions(txs))
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
    Ok(into_transactions(txs))
}

pub fn get_transactions_with_offset_mint(
    offset: usize,
    limit: usize,
    mint_url: String,
) -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let txs = state
        .rt
        .block_on(w.list_transactions_with_kind_offset_mint(
            offset,
            limit,
            &mint_url,
            [TransactionKind::Cashu, TransactionKind::LN].as_slice(),
            None,
        ))?;
    Ok(into_transactions(txs))
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
    Ok(into_transactions(txs))
}

pub fn get_txs_with_offset_mint_amount(
    offset: usize,
    limit: usize,
    mint_url: String,
    is_one_amount: bool,
) -> anyhow::Result<Vec<Transaction>> {
    let amount = if is_one_amount { Some(1) } else { Some(-1) };
    get_transactions_with_offset_mint_amount(offset, limit, mint_url, amount)
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
    Ok(into_transactions(txs))
}

pub fn get_ln_pending_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let txs = state
        .rt
        .block_on(w.list_pending_transactions_with_kind([TransactionKind::LN].as_slice(), None))?;
    Ok(into_transactions(txs))
}

pub fn get_cashu_pending_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let txs = state.rt.block_on(
        w.list_pending_transactions_with_kind([TransactionKind::Cashu].as_slice(), None),
    )?;
    Ok(into_transactions(txs))
}

pub fn get_pending_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let txs = state.rt.block_on(w.list_pending_transactions_with_kind(
        [TransactionKind::Cashu, TransactionKind::LN].as_slice(),
        None,
    ))?;
    Ok(into_transactions(txs))
}

pub fn get_failed_transactions() -> anyhow::Result<Vec<Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let txs = state.rt.block_on(w.list_failed_transactions_with_kind(
        [TransactionKind::Cashu, TransactionKind::LN].as_slice(),
        None,
    ))?;
    Ok(into_transactions(txs))
}

pub fn remove_transactions(
    unix_timestamp_le: u64,
    _status: TransactionStatus,
) -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    state
        .rt
        .block_on(w.remove_transactions(unix_timestamp_le))?;
    Ok(())
}

pub fn get_pending_transactions_count() -> anyhow::Result<u64> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    state.rt.block_on(async {
        let proofs = w.list_all_pending_proofs().await?;
        Ok(proofs.values().map(|(v, _)| v.len() as u64).sum())
    })
}

pub fn get_all_proofs_data() -> anyhow::Result<(usize, usize, usize)> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    state.rt.block_on(async {
        let all = w.list_all_proofs().await?;
        let spent = w.list_spent_proofs().await?;
        let pending = w.list_pending_proofs().await?;
        Ok((
            all.values().len(),
            spent.values().len(),
            pending.values().len(),
        ))
    })
}

pub fn decode_token(encoded_token: String) -> anyhow::Result<TokenInfo> {
    let token: Token = Token::from_str(&encoded_token)?;
    Ok(TokenInfo {
        mint: token.mint_url()?.to_string(),
        amount: token.value()?.into(),
        unit: token.unit().as_ref().map(|s| s.to_string()),
        memo: token.memo().clone(),
    })
}

pub fn restore(mint_url: String, words: Option<String>) -> anyhow::Result<(u64, u64)> {
    let mint_url = MintUrl::from_str(&mint_url)?;
    let mut state = State::lock()?;
    let unit = CurrencyUnit::from_str("sat")?;

    if let Some(w) = words {
        let mi = MnemonicInfo::with_words(&w)?;
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

        Ok(wallet.restore().await?)
    })?;

    Ok((amount.0.into(), amount.1))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
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

    let status = if invoice.is_expired() {
        InvoiceStatus::Expired
    } else {
        InvoiceStatus::Unpaid
    };
    let ts = (invoice.duration_since_epoch() + invoice.expiry_time()).as_millis();

    Ok(InvoiceInfo {
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
