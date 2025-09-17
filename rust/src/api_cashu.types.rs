use flutter_rust_bridge::frb;
// use strum::{AsRefStr, Display, EnumIs, EnumString, IntoStaticStr};
pub use cashu::{Amount, CurrencyUnit, MintUrl};
pub use cdk_common::wallet::{TransactionDirection, TransactionKind, TransactionStatus};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub mint_url: String,
    pub io: TransactionDirection,
    pub kind: TransactionKind,
    pub amount: u64,
    pub fee: u64,
    pub unit: Option<String>,
    pub token: String,
    pub status: TransactionStatus,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}

#[frb(mirror(TransactionStatus))]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum _TransactionStatus {
    Pending,
    Success,
    Failed,
    Expired,
}

#[frb(mirror(TransactionDirection))]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum _TransactionDirection {
    Incoming,
    Outgoing,
    Split,
}

#[frb(mirror(TransactionKind))]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum _TransactionKind {
    Cashu,
    LN,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MintCashu {
    pub url: String,
    pub active: bool,
    pub time: u64,
    pub info: Option<MintCashuInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MintCashuInfo {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_long: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motd: Option<String>,
    #[serde(default)]
    pub contact: Vec<ContactCashu>,
    pub nuts: HashMap<String, bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactCashu {
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub info: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CashuV1ToV2 {
    pub tokens: Vec<String>,
    pub counters: String,
}

pub use bip39::Mnemonic;
use bitcoin::bip32::Xpriv;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MnemonicInfo {
    mnemonic: Mnemonic,
    key: Xpriv,
    pubkey: String,
}

impl MnemonicInfo {
    pub fn new(mnemonic: &Mnemonic) -> anyhow::Result<Self> {
        let (key, pubkey) = get_keys(&mnemonic)?;
        Ok(Self {
            mnemonic: mnemonic.clone(),
            key,
            pubkey,
        })
    }
    pub fn with_words(words: &str) -> anyhow::Result<Self> {
        let mnemonic = words.parse()?;
        Self::new(&mnemonic)
    }
    pub fn generate_words(words: usize) -> anyhow::Result<String> {
        let mnemonic = Mnemonic::generate(words)?;
        Ok(mnemonic.to_string())
    }
    pub fn pubkey(&self) -> &str {
        &self.pubkey
    }
    pub fn mnemonic(&self) -> &Mnemonic {
        &self.mnemonic
    }
}

/// m / 129372' / 0' / keyset_k_int' / counter' / secret||r
/// m / 129372' / 0'
fn get_keys(mnemonic: &Mnemonic) -> anyhow::Result<(Xpriv, String)> {
    use bitcoin::bip32::{DerivationPath, Xpriv};
    use bitcoin::Network;
    use cashu::SECP256K1;

    let path: DerivationPath = "m/129372'/0'".parse().unwrap();

    let seed: [u8; 64] = mnemonic.to_seed("");
    let bip32_root_key = Xpriv::new_master(Network::Bitcoin, &seed)?;
    let derived_xpriv = bip32_root_key.derive_priv(&SECP256K1, &path)?;
    let ident = derived_xpriv
        .to_keypair(&SECP256K1)
        .public_key()
        .to_string();
    Ok((bip32_root_key, ident))
}
