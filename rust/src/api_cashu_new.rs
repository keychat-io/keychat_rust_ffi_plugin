use anyhow::Ok;
use cashu::Amount;
use cashu::MintUrl;
use cdk_common::wallet::TransactionId;
use cdk_sqlite::WalletSqliteDatabase;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use std::sync::MutexGuard as StdMutexGuard;
use tokio::runtime::{Builder, Runtime};

use cashu::CurrencyUnit;
use cdk::cdk_database;
use cdk_common::database::WalletDatabase;


use cdk_common::wallet::{Transaction, TransactionDirection, TransactionKind, TransactionStatus};

use cdk::wallet::{MultiMintWallet, Wallet, WalletBuilder};

#[frb(ignore)]
pub struct State {
    rt: Arc<Runtime>,
    wallet: Option<MultiMintWallet>,
    mnemonic: Option<Arc<MnemonicInfo>>,
    sats: u16,
}

#[path = "api_cashu.check.rs"]
pub mod check;
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
            println!("get_wallet none");
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

pub fn init_db(dbpath: String, words: Option<String>, _dev: bool) -> anyhow::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let _ = set_mnemonic(words.clone());
    // let _ = add_mint("https://8333.space:3338/".to_string());

    let mut seed = None;
    if let Some(s) = words {
        let mi = MnemonicInfo::with_words(&s)?;
        seed = Some(mi.mnemonic().to_seed(""));
    }

    let mut state = State::lock()?;

    let fut = async move {
        let localstore: Arc<dyn WalletDatabase<Err = cdk_database::Error> + Send + Sync> =
            Arc::new(WalletSqliteDatabase::new(&dbpath).await?);

        let mut wallets: Vec<Wallet> = Vec::new();

        let mints = localstore.get_mints().await?;
        if mints.is_empty() {
            
        }

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
                    .seed(&seed.unwrap());

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
        let multi_mint_wallet = MultiMintWallet::new(localstore, Arc::new(seed.unwrap()), wallets);

        Ok(multi_mint_wallet)
    };

    let result = state.rt.block_on(fut);

    result.map(|w| {
        state.wallet = Some(w);
    })
}

use std::collections::HashMap;
pub fn init_cashu(
    prepare_sats_once_time: u16,
) -> anyhow::Result<HashMap<cashu::MintUrl, Option<cdk_common::MintInfo>>> {
    let mut state = State::lock()?;
    state.sats = prepare_sats_once_time;

    let w = state.get_wallet()?;
    let mints = state.rt.block_on(w.localstore.get_mints())?;
    Ok(mints)
}

// ignore for test
// add by 2.0.0-dev.9
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

pub fn get_mints() -> anyhow::Result<HashMap<cashu::MintUrl, Option<cdk_common::MintInfo>>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let mints = state.rt.block_on(w.localstore.get_mints())?;

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

// use cashu_wallet::wallet::CURRENCY_UNIT_SAT;
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

use cdk::wallet::types::WalletKey;
/// Helper function to create or get a wallet
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

use cdk::nuts::Token;
use cdk::wallet::ReceiveOptions;
pub fn receive_token(encoded_token: String) -> anyhow::Result<Amount> {
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

    let amount = state.rt.block_on(fut)?;

    Ok(amount)
}

#[frb(ignore)]
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
        let wallet = get_or_create_wallet(w, &mint_url, unit).await?;
        let ps = wallet.get_unspent_proofs().await?;
        ps.iter().map(|p| {
            let is = *p.amount.as_ref() == denomination;
            if is {
                count_before += 1;
            }
        });
        if count_before * denomination < amount {
            let rest_amount = amount - count_before * denomination;
            let target = vec![cashu::Amount::ONE; rest_amount as usize];
            let split_target = SplitTarget::Values(target.clone());
            let active_keyset_ids = wallet
                .get_active_mint_keysets()
                .await?
                .into_iter()
                .map(|keyset| keyset.id)
                .collect();
            let selected = Wallet::select_proofs(
                rest_amount.into(),
                ps,
                &active_keyset_ids,
                &HashMap::new(),
                true,
            )?;
            wallet
                .swap(
                    Some(rest_amount.into()),
                    split_target,
                    selected,
                    None,
                    false,
                )
                .await?;
        }
        Ok(0)
    })?;

    Ok(a)
}

pub fn send_stamp(amount: u64, mints: Vec<String>, info: Option<String>) -> anyhow::Result<String> {
    if amount == 0 {
        bail!("can't send amount 0");
    }

    let state = State::lock()?;
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
        let tx = send(amount, mint_url.to_string(), info.clone());
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
    let tx = send(amount, mint_url.to_string(), info.clone());
    tx
}

use std::collections::BTreeMap;
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
        println!("{i}: {mint_url} {amount} {unit}");
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

use cdk::wallet::SendOptions;
pub fn send(amount: u64, active_mint: String, _info: Option<String>) -> anyhow::Result<String> {
    if amount == 0 {
        bail!("can't send amount 0");
    }
    let unit = CurrencyUnit::from_str("sat")?;
    let state = State::lock()?;

    let w = state.get_wallet()?;

    let fut = async move {
        let mints_amounts = mint_balances(w, &unit).await;
        // Get wallet either by mint URL or by index
        let wallet = get_wallet_by_mint_url(w, &active_mint, unit).await;

        // Find the mint amount for the selected wallet to check if we have sufficient funds
        let wallet = wallet.unwrap();
        let mint_url = &wallet.mint_url;
        let mint_amount = mints_amounts
            .unwrap()
            .iter()
            .find(|(url, _)| url == mint_url)
            .map(|(_, amount)| *amount)
            .ok_or_else(|| anyhow!("Could not find balance for mint: {}", mint_url));

        check_sufficient_funds(mint_amount.unwrap(), amount.into()).unwrap();
        let prepared_send = wallet
            .prepare_send(amount.into(), SendOptions::default())
            .await;
        wallet.send(prepared_send.unwrap(), None).await
    };
    let token = state.rt.block_on(fut)?;
    Ok(token.to_v3_string())
}

use cdk::amount::SplitTarget;
use cdk::nuts::nut00::ProofsMethods;
use cdk::nuts::{MintQuoteState, NotificationPayload};
use cdk::wallet::WalletSubscription;
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

// this need call every init
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

pub fn mint_token(amount: u64, quote_id: String, active_mint: String) -> anyhow::Result<()> {
    if amount == 0 {
        bail!("can't mint amount 0");
    }

    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&active_mint)?;

    let state = State::lock()?;

    let w = state.get_wallet()?;

    let _tx = state.rt.block_on(async {
        let wallet = get_or_create_wallet(w, &mint_url, unit).await?;
        let proofs = wallet.mint(&quote_id, SplitTarget::default(), None).await?;

        let receive_amount = proofs.total_amount()?;

        debug!("Received {receive_amount} from mint {mint_url}");
        Ok(())
    })?;

    Ok(())
}

use cdk::amount::MSAT_IN_SAT;
use cdk::nuts::MeltOptions;
use cdk::Bolt11Invoice;
pub fn melt(invoice: String, active_mint: String, amount: Option<u64>) -> anyhow::Result<()> {
    if amount == Some(0) {
        bail!("can't melt amount 0");
    }

    let unit = CurrencyUnit::from_str("sat")?;
    let mint_url = MintUrl::from_str(&active_mint)?;

    let state = State::lock()?;

    let w = state.get_wallet()?;

    let _tx = state.rt.block_on(async {
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
        println!("Paid invoice: {}", melt.state);

        if let Some(preimage) = melt.preimage {
            info!("Payment preimage: {preimage}");
        }

        Ok(())
    })?;
    Ok(())
}

pub fn get_all_transactions() -> anyhow::Result<Vec<cdk_common::wallet::Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(w.list_transactions(
        None,
    ))?;

    Ok(tx)
}

// incloud normal_tx ln_tx melt_tx mint_tx
pub fn get_cashu_transactions_with_offset(
    offset: usize,
    limit: usize,
) -> anyhow::Result<Vec<cdk_common::wallet::Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(w.list_transactions_with_kind_offset(
        offset,
        limit,
        [TransactionKind::Cashu].as_slice(),
        None,
    ))?;

    Ok(tx)
}

pub fn get_ln_transactions_with_offset(
    offset: usize,
    limit: usize,
) -> anyhow::Result<Vec<cdk_common::wallet::Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(w.list_transactions_with_kind_offset(
        offset,
        limit,
        [TransactionKind::LN].as_slice(),
        None,
    ))?;

    Ok(tx)
}

pub fn get_ln_pending_transactions() -> anyhow::Result<Vec<cdk_common::wallet::Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state
        .rt
        .block_on(w.list_pending_transactions_with_kind([TransactionKind::LN].as_slice(), None))?;
    // let mut txs = Vec::with_capacity(tx.iter().filter(|x| x.is_ln()).count());
    // for t in tx {
    //     if let Transaction::LN(t) = t {
    //         txs.push(t);
    //     }
    // }
    Ok(tx)
}

pub fn get_cashu_pending_transactions() -> anyhow::Result<Vec<cdk_common::wallet::Transaction>> {
    let state = State::lock()?;
    let w = state.get_wallet()?;

    let tx = state.rt.block_on(
        w.list_pending_transactions_with_kind([TransactionKind::Cashu].as_slice(), None),
    )?;
    
    Ok(tx)
}

/// remove transaction.time() <= unix_timestamp_le and kind is the status, timestamp must be second
pub fn remove_transactions(unix_timestamp_le: u64, kind: TransactionStatus) -> anyhow::Result<()> {
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

/// include ln and cashu
pub fn check_pending() -> anyhow::Result<()> {
    let state = State::lock()?;
    let w = state.get_wallet()?;
    let tx = state.rt.block_on(w.check_pending())?;
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
pub fn check_proofs() -> anyhow::Result<(usize, usize, usize)> {
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

pub fn decode_token(encoded_token: String) -> anyhow::Result<TokenInfo> {
    let token: Token = Token::from_str(&encoded_token)?;

    Ok(TokenInfo {
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
pub struct TokenInfo {
    // pub encoded_token: String,
    pub mint: String,
    pub amount: u64,
    pub unit: Option<CurrencyUnit>,
    pub memo: Option<String>,
}

use cdk::lightning_invoice::{
    Bolt11Invoice as Invoice, Bolt11InvoiceDescriptionRef as InvoiceDescriptionRef,
};

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
