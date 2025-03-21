use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResult {
    pub encrypt_msg: Vec<u8>,
    pub ratchet_key: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptedMessage {
    pub decrypt_msg: String,
    pub sender: String,
    pub ratchet_key: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMembersResult {
    pub queued_msg: Vec<u8>,
    pub welcome: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupExtensionResult {
    pub name: Vec<u8>,
    pub description: Vec<u8>,
    pub admin_pubkeys: Vec<Vec<u8>>,
    pub relays: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommitTypeResult {
    Add,
    Update,
    Remove,
    GroupContextExtensions,
}
