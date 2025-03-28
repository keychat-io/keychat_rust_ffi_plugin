use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResult {
    pub encrypt_msg: String,
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
    pub queued_msg: String,
    pub welcome: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupExtensionResult {
    pub name: Vec<u8>,
    pub description: Vec<u8>,
    pub admin_pubkeys: Vec<Vec<u8>>,
    pub relays: Vec<Vec<u8>>,
    pub status: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommitTypeResult {
    Add,
    Update,
    Remove,
    GroupContextExtensions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitResult {
    pub sender: String,
    pub commit_type: CommitTypeResult,
    pub operated_members: Option<Vec<String>>,
}

pub enum MessageInType {
    Application,
    Proposal,
    Commit,
    Welcome,
    GroupInfo,
    KeyPackage,
    PublicMessage,
    Custom,
}
