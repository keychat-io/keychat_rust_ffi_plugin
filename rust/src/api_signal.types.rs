#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct KeychatIdentityKeyPair {
    pub identity_key: [u8; 33],
    pub private_key: [u8; 32],
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct KeychatProtocolAddress {
    pub name: String,
    pub device_id: u32,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct KeychatIdentityKey {
    pub public_key: [u8; 33],
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct KeychatSignalSession {
    pub alice_sender_ratchet_key: Option<String>,
    pub address: String,
    pub device: u32,
    pub bob_sender_ratchet_key: Option<String>,
    pub record: String,
    pub bob_address: Option<String>,
    pub alice_addresses: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SignedPreKeyResult {
    pub signed_pre_key_id: u32,
    pub signed_pre_key_public: Vec<u8>,
    pub signed_pre_key_signature: Vec<u8>,
    pub signed_pre_key_record: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PreKeyResult {
    pub pre_key_id: u32,
    pub pre_key_public: Vec<u8>,
    pub pre_key_record: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct EncryptResult {
    pub ciphertext: Vec<u8>,
    pub receiver_address: Option<String>,
    pub message_keys_hash: String,
    pub sender_addresses: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct DecryptResult {
    pub plaintext: Vec<u8>,
    pub message_keys_hash: String,
    pub sender_addresses: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct PreKeySignalMessageInfo {
    pub identity_key: String,
    pub signed_pre_key_id: u32,
}

#[derive(Debug, Clone)]
pub struct SignalKeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}
