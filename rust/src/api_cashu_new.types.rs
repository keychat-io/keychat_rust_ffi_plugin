use flutter_rust_bridge::frb;
// use strum::{AsRefStr, Display, EnumIs, EnumString, IntoStaticStr};

use cdk_common::wallet::{Transaction, TransactionDirection, TransactionKind, TransactionStatus};

#[frb(mirror(TransactionStatus))]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
//
// #[derive(Display, AsRefStr, IntoStaticStr, EnumIs, EnumString)]
pub enum _TransactionStatus {
    Pending,
    Success,
    Failed,
    Expired,
}

#[frb(mirror(TransactionDirection))]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
//
// #[derive(Display, AsRefStr, IntoStaticStr, EnumIs, EnumString)]
pub enum _TransactionDirection {
    In,
    Out,
    Split,
}

// #[frb(mirror(TransactionKind))]
// #[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
// //
// // #[derive(Display, AsRefStr, IntoStaticStr, EnumIs, EnumString)]
// pub enum _TransactionKind {
//     Cashu,
//     LN,
// }

// #[frb(mirror(Transaction))]
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// //
// // #[derive(EnumIs)]
// #[serde(tag = "kind")]
// pub enum _Transaction {
//     Cashu(CashuTransaction),
//     LN(LNTransaction),
// }

// #[frb(mirror(CashuTransaction))]
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct _CashuTransaction {
//     pub id: String,
//     pub status: TransactionStatus,
//     pub io: TransactionDirection,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     pub info: Option<String>,
//     pub time: u64,
//     pub amount: u64,
//     pub fee: Option<u64>,
//     pub mint: String,
//     pub token: String,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     pub unit: Option<String>,
// }

// #[frb(mirror(LNTransaction))]
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct _LNTransaction {
//     pub status: TransactionStatus,
//     pub io: TransactionDirection,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     pub info: Option<String>,
//     pub time: u64,
//     pub amount: u64,
//     pub fee: Option<u64>,
//     pub mint: String,
//     // invoice
//     pub pr: String,
//     pub hash: String,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     pub unit: Option<String>,
// }

// pub use cashu_wallet::types::{
//     Contact, Mint, MintInfo, NutSupported, Nuts, PaymentMethod, PaymentMethodSettings,
// };

// #[frb(mirror(Mint))]
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct _Mint {
//     pub url: String,
//     pub active: bool,
//     pub time: u64,
//     pub info: Option<MintInfo>,
// }

// /// NUT-06: Mint information
// // https://github.com/cashubtc/nuts/blob/main/06.md
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[frb(mirror(MintInfo))]
// pub struct _MintInfo {
//     pub name: String,
//     pub version: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pubkey: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub description: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub description_long: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub motd: Option<String>,
//     #[serde(default)]
//     pub contact: Vec<Contact>,
//     pub nuts: Nuts,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[frb(mirror(Contact))]
// pub struct _Contact {
//     #[serde(default)]
//     pub method: String,
//     #[serde(default)]
//     pub info: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[frb(mirror(Nuts))]
// pub struct _Nuts {
//     #[serde(default, rename = "4")]
//     pub nut04: PaymentMethodSettings,
//     #[serde(default, rename = "5")]
//     pub nut05: PaymentMethodSettings,
//     #[serde(default, rename = "7")]
//     pub nut07: NutSupported,
//     #[serde(default, rename = "8")]
//     pub nut08: NutSupported,
//     #[serde(default, rename = "9")]
//     pub nut09: NutSupported,
//     #[serde(default, rename = "10")]
//     pub nut10: NutSupported,
//     #[serde(default, rename = "11")]
//     pub nut11: NutSupported,
//     #[serde(default, rename = "12")]
//     pub nut12: NutSupported,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[frb(mirror(PaymentMethod))]
// pub struct _PaymentMethod {
//     pub method: String,
//     pub unit: String,
//     #[serde(default)]
//     pub min_amount: i64,
//     #[serde(default)]
//     pub max_amount: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[frb(mirror(PaymentMethodSettings))]
// pub struct _PaymentMethodSettings {
//     #[serde(default)]
//     pub methods: Vec<PaymentMethod>,
//     pub disabled: bool,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[frb(mirror(NutSupported))]
// pub struct _NutSupported {
//     pub supported: bool,
// }

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