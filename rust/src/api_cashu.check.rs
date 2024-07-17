pub use std::collections::HashSet as Set;

use cashu_wallet::store::UnitedStore;
use cashu_wallet::UniError;
use cashu_wallet::UniErrorFrom;
use cashu_wallet::UnitedWallet;

#[frb(ignore)]
/// (spents, pendings, all)
pub async fn check_proofs_in_database<S>(
    this: &UnitedWallet<S>,
) -> Result<(usize, usize, usize), UniError<S::Error>>
where
    S: UnitedStore + Clone + Send + Sync + 'static,
    UniError<S::Error>: UniErrorFrom<S>,
{
    let ps = this.store().get_all_proofs().await?;

    let all = ps.values().map(|p| p.len()).sum();
    let mut set = Set::new();
    let mut spents = 0usize;

    let batch_size = 64;
    for (k, txs) in ps.iter() {
        let mint_url = k.mint().parse()?;
        let wallet = this.get_wallet_optional(&mint_url)?;
        if wallet.is_none() {
            continue;
        }
        let wallet = wallet.unwrap();

        for ps in txs.chunks(batch_size) {
            let state = wallet.check_proofs(ps).await?.states;

            for (idx, b) in state.into_iter().enumerate() {
                if b.state == cashu_wallet::cashu::nuts::nut07::State::Pending {
                    let tx = &ps[idx];

                    set.insert(tx.raw.secret.as_str().to_owned());
                }

                if b.state == cashu_wallet::cashu::nuts::nut07::State::Spent {
                    let tx = &ps[idx];
                    let txs = std::array::from_ref(tx).as_slice();
                    this.store().delete_proofs(&mint_url, txs).await.ok();
                    spents += 1;
                }
            }
        }
    }

    let res = Ok((spents, set.len(), all));
    {
        let mut lock = STATE.lock().expect("failed to lock pending proofs");
        *lock = set.clone();
    }

    res
}

use std::sync::Mutex as StdMutex;
lazy_static! {
    static ref STATE: StdMutex<Set<String>> = StdMutex::new(Default::default());
}

use cashu_wallet_sqlite::StoreError;
use cashu_wallet_sqlite::Tables;

#[frb(ignore)]
#[derive(Debug, Clone)]
pub struct LitePool(cashu_wallet_sqlite::LitePool);
impl LitePool {
    /// https://docs.rs/sqlx-sqlite/0.7.1/sqlx_sqlite/struct.SqliteConnectOptions.html#impl-FromStr-for-SqliteConnectOptions
    pub async fn open(dbpath: &str, tables: Tables) -> Result<LitePool, StoreError> {
        let this = cashu_wallet_sqlite::LitePool::open(dbpath, tables).await?;
        Ok(Self(this))
    }
}

impl AsRef<cashu_wallet_sqlite::LitePool> for LitePool {
    fn as_ref(&self) -> &cashu_wallet_sqlite::LitePool {
        &self.0
    }
}

use cashu_wallet::store::MintUrlWithUnitOwned;
use cashu_wallet::types::{Mint, Transaction, TransactionKind, TransactionStatus};
use cashu_wallet::wallet::{ProofExtended, ProofsExtended, Record};
use cashu_wallet::Url;
use std::collections::BTreeMap as Map;

#[async_trait]
impl UnitedStore for LitePool {
    type Error = StoreError;

    async fn delete_proofs(
        &self,
        mint_url: &Url,
        proofs: &[ProofExtended],
    ) -> Result<(), Self::Error> {
        self.as_ref().delete_proofs(mint_url, proofs).await
    }
    async fn add_proofs(
        &self,
        mint_url: &Url,
        proofs: &[ProofExtended],
    ) -> Result<(), Self::Error> {
        self.as_ref().add_proofs(mint_url, proofs).await
    }
    async fn get_proofs_limit_unit(
        &self,
        mint_url: &Url,
        unit: &str,
    ) -> Result<ProofsExtended, Self::Error> {
        let mut ps = self.as_ref().get_proofs_limit_unit(mint_url, unit).await?;
        {
            let lock = STATE.lock().expect("failed to lock pending proofs.");
            ps.retain(|p| !lock.contains(p.raw.secret.as_str()));
        }
        Ok(ps)
    }
    async fn get_proofs(&self, mint_url: &Url) -> Result<Map<String, ProofsExtended>, Self::Error> {
        self.as_ref().get_proofs(mint_url).await
    }
    async fn get_all_proofs(
        &self,
    ) -> Result<Map<MintUrlWithUnitOwned, ProofsExtended>, Self::Error> {
        self.as_ref().get_all_proofs().await
    }
    // counter records
    async fn add_counter(&self, records: &Record) -> Result<(), Self::Error> {
        self.as_ref().add_counter(records).await
    }
    async fn delete_counters(&self, mint_url: &Url) -> Result<(), Self::Error> {
        self.as_ref().delete_counters(mint_url).await
    }
    async fn get_counters(&self, mint_url: &Url, pubkey: &str) -> Result<Vec<Record>, Self::Error> {
        self.as_ref().get_counters(mint_url, pubkey).await
    }
    //
    async fn migrate(&self) -> Result<(), Self::Error> {
        self.as_ref().migrate().await
    }
    //
    // mints
    async fn add_mint(&self, mint: &Mint) -> Result<(), Self::Error> {
        self.as_ref().add_mint(mint).await
    }
    async fn get_mint(&self, mint_url: &str) -> Result<Option<Mint>, Self::Error> {
        self.as_ref().get_mint(mint_url).await
    }
    async fn get_mints(&self) -> Result<Vec<Mint>, Self::Error> {
        self.as_ref().get_mints().await
    }
    //
    // tx
    async fn delete_transactions(
        &self,
        status: &[TransactionStatus],
        unix_timestamp_ms_le: u64,
    ) -> Result<u64, Self::Error> {
        self.as_ref()
            .delete_transactions(status, unix_timestamp_ms_le)
            .await
    }
    async fn add_transaction(&self, tx: &Transaction) -> Result<(), Self::Error> {
        self.as_ref().add_transaction(tx).await
    }
    async fn get_transaction(&self, txid: &str) -> Result<Option<Transaction>, Self::Error> {
        self.as_ref().get_transaction(txid).await
    }
    async fn get_transactions(
        &self,
        status: &[TransactionStatus],
    ) -> Result<Vec<Transaction>, Self::Error> {
        self.as_ref().get_transactions(status).await
    }
    async fn get_pending_transactions(&self) -> Result<Vec<Transaction>, Self::Error> {
        self.as_ref().get_pending_transactions().await
    }
    async fn get_all_transactions(&self) -> Result<Vec<Transaction>, Self::Error> {
        self.as_ref().get_all_transactions().await
    }
    async fn get_transactions_with_offset(
        &self,
        offset: usize,
        limit: usize,
        kinds: &[TransactionKind],
    ) -> Result<Vec<Transaction>, Self::Error> {
        self.as_ref()
            .get_transactions_with_offset(offset, limit, kinds)
            .await
    }
}
