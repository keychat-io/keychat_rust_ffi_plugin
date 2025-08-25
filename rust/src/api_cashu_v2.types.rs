use flutter_rust_bridge::frb;
// use strum::{AsRefStr, Display, EnumIs, EnumString, IntoStaticStr};
use cashu::{Amount, CurrencyUnit, MintUrl};
use cdk_common::wallet::{
    TransactionDirection as TransactionDirectionV2, TransactionKind as TransactionKindV2,
};
use std::collections::HashMap;

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct TransactionV2 {
//     pub mint_url: String,
//     pub direction: TransactionDirectionV2,
//     pub kind: TransactionKindV2,
//     pub amount: u64,
//     pub fee: u64,
//     pub unit: Option<String>,
//     pub token: String,
//     pub timestamp: u64,
//     pub metadata: HashMap<String, String>,
// }

#[frb(mirror(TransactionDirectionV2))]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum _TransactionDirection {
    Incoming,
    Outgoing,
    Split,
}

#[frb(mirror(TransactionKindV2))]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum _TransactionKind {
    Cashu,
    LN,
}

#[frb(mirror(TransactionV2))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct _Transaction {
    pub mint_url: MintUrl,
    pub direction: TransactionDirectionV2,
    pub kind: TransactionKindV2,
    pub amount: Amount,
    pub fee: Amount,
    pub unit: CurrencyUnit,
    pub token: String,
    // pub ys: Vec<PublicKey>,
    pub timestamp: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[frb(mirror(MintUrl))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct _MintUrl(pub String);

#[frb(mirror(Amount))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct _Amount(pub u64);

#[frb(mirror(CurrencyUnit))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum _CurrencyUnit {
    /// Sat
    #[default]
    Sat,
    /// Msat
    Msat,
    /// Usd
    Usd,
    /// Euro
    Eur,
    /// Auth
    Auth,
}

// #[frb(mirror(PublicKey))]
// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct _PublicKey {
//     pub bytes: Vec<u8>,
// }

// #[frb(mirror(MintInfo))]
// #[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct _MintInfo {
//     /// name of the mint and should be recognizable
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub name: Option<String>,
//     /// hex pubkey of the mint
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pubkey: Option<PublicKey>,
//     /// implementation name and the version running
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub version: Option<MintVersion>,
//     /// short description of the mint
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub description: Option<String>,
//     /// long description
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub description_long: Option<String>,
//     /// Contact info
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub contact: Option<Vec<ContactInfo>>,
//     /// shows which NUTs the mint supports
//     pub nuts: Nuts,
//     /// Mint's icon URL
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub icon_url: Option<String>,
//     /// Mint's endpoint URLs
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub urls: Option<Vec<String>>,
//     /// message of the day that the wallet must display to the user
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub motd: Option<String>,
//     /// server unix timestamp
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub time: Option<u64>,
//     /// terms of url service of the mint
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub tos_url: Option<String>,
// }

// #[frb(mirror(ContactInfo))]
// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct _ContactInfo {
//     /// Contact Method i.e. nostr
//     pub method: String,
//     /// Contact info i.e. npub...
//     pub info: String,
// }

// #[frb(mirror(Nuts))]
// /// Supported nuts and settings
// #[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct _Nuts {
//     /// NUT04 Settings
//     #[serde(default)]
//     #[serde(rename = "4")]
//     pub nut04: Settings,
//     /// NUT05 Settings
//     #[serde(default)]
//     #[serde(rename = "5")]
//     pub nut05: Settings,
//     /// NUT07 Settings
//     #[serde(default)]
//     #[serde(rename = "7")]
//     pub nut07: SupportedSettings,
//     /// NUT08 Settings
//     #[serde(default)]
//     #[serde(rename = "8")]
//     pub nut08: SupportedSettings,
//     /// NUT09 Settings
//     #[serde(default)]
//     #[serde(rename = "9")]
//     pub nut09: SupportedSettings,
//     /// NUT10 Settings
//     #[serde(rename = "10")]
//     #[serde(default)]
//     pub nut10: SupportedSettings,
//     /// NUT11 Settings
//     #[serde(rename = "11")]
//     #[serde(default)]
//     pub nut11: SupportedSettings,
//     /// NUT12 Settings
//     #[serde(default)]
//     #[serde(rename = "12")]
//     pub nut12: SupportedSettings,
//     /// NUT14 Settings
//     #[serde(default)]
//     #[serde(rename = "14")]
//     pub nut14: SupportedSettings,
//     /// NUT15 Settings
//     #[serde(default)]
//     #[serde(rename = "15")]
//     pub nut15: Settings,
//     /// NUT17 Settings
//     #[serde(default)]
//     #[serde(rename = "17")]
//     pub nut17: SupportedSettings,
//     /// NUT19 Settings
//     #[serde(default)]
//     #[serde(rename = "19")]
//     pub nut19: Settings,
//     /// NUT20 Settings
//     #[serde(default)]
//     #[serde(rename = "20")]
//     pub nut20: SupportedSettings,
//     /// NUT21 Settings
//     #[serde(rename = "21")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[cfg(feature = "auth")]
//     pub nut21: Option<ClearAuthSettings>,
//     /// NUT22 Settings
//     #[serde(rename = "22")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[cfg(feature = "auth")]
//     pub nut22: Option<BlindAuthSettings>,
// }

// #[frb(mirror(Settings))]
// #[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
// pub struct _Settings {
//     /// Number of seconds the responses are cached for
//     pub ttl: Option<u64>,
//     /// Cached endpoints
//     pub cached_endpoints: Vec<CachedEndpoint>,
// }

// #[frb(mirror(SupportedSettings))]
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, Serialize, Deserialize)]
// pub struct _SupportedSettings {
//     /// Setting supported
//     pub supported: bool,
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
