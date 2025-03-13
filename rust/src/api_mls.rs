use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

pub use kc::identity::Identity;
use kc::openmls_rust_persistent_crypto::JsonCodec;
pub use kc::openmls_rust_persistent_crypto::OpenMlsRustPersistentCrypto;
pub use openmls::group::{GroupId, MlsGroup, MlsGroupCreateConfig, MlsGroupJoinConfig};
use openmls::prelude::tls_codec::Deserialize;
use openmls::prelude::tls_codec::Serialize;
use openmls::prelude::ContentType;
use openmls::prelude::MlsMessageBodyIn;
use openmls::prelude::MlsMessageIn;
pub use openmls_sqlite_storage::{Connection, SqliteStorageProvider};
pub use openmls_traits::OpenMlsProvider;

#[path = "api_mls.user.rs"]
pub mod user;
pub use user::*;

// must be ignore, otherwise will be error when rust to dart
#[frb(ignore)]
pub struct MlsStore {
    pub user: HashMap<String, User>,
}

lazy_static! {
    static ref STORE: Mutex<Option<MlsStore>> = Mutex::new(None);
}

lazy_static! {
    static ref RUNTIME: Arc<Runtime> =
        Arc::new(Runtime::new().expect("failed to create tokio runtime for mls"));
}

// when init db then create the user, every user show init it
pub fn init_mls_db(db_path: String, nostr_id: String) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let connection = Connection::open(db_path)?;
        let mut storage = SqliteStorageProvider::<JsonCodec, Connection>::new(connection);
        storage
            .initialize()
            .map_err(|e| format_err!("<MlsUser fn[new]> Failed to initialize storage: {}", e))?;

        let provider = OpenMlsRustPersistentCrypto::new(storage).await;

        if store.is_none() {
            *store = Some(MlsStore {
                user: HashMap::new(),
            });
            error!("store has not been inited.");
        }
        let map = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[init_mls_db]> Can not get store err."))?;
        // first load from db, if none then create new user

        let user = User::load(provider, nostr_id.clone()).await?;
        map.user.entry(nostr_id).or_insert(User { mls_user: user });
        Ok(())
    });
    result
}

pub fn get_export_secret(nostr_id: String, group_id: String) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[get_export_secret]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[get_export_secret]> key_pair do not init.");
            return Err(format_err!("<fn[get_export_secret]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[get_export_secret]> Can not get store from user."))?;
        let export_secret = user.get_export_secret(group_id)?;
        Ok(export_secret)
    });
    result
}

pub fn get_tree_hash(nostr_id: String, group_id: String) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[get_tree_hash]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[get_tree_hash]> key_pair do not init.");
            return Err(format_err!("<fn[get_tree_hash]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[get_tree_hash]> Can not get store from user."))?;
        let export_secret = user.get_tree_hash(group_id)?;
        Ok(export_secret)
    });
    result
}

// only join new group that need to create it
pub fn create_key_package(nostr_id: String) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[create_key_package]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[create_key_package]> key_pair do not init.");
            return Err(format_err!(
                "<fn[create_key_package]> nostr_id do not init."
            ));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[create_key_package]> Can not get store from user."))?;
        let key_package = user.create_key_package()?;
        user.update(nostr_id, true).await?;
        // let serialized: Vec<u8> = bincode::serialize(&key_package)?;
        let serialized = key_package.tls_serialize_detached()?;
        Ok(serialized)
    });
    result
}

pub fn delete_key_package(nostr_id: String, key_package: Vec<u8>) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[create_key_package]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[create_key_package]> key_pair do not init.");
            return Err(format_err!(
                "<fn[create_key_package]> nostr_id do not init."
            ));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[create_key_package]> Can not get store from user."))?;
        user.delete_key_package(key_package)?;
        user.update(nostr_id, true).await
    });
    result
}

pub fn create_group_config() -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let config = MlsGroupCreateConfig::builder()
            .use_ratchet_tree_extension(true)
            .build();
        let serialized: Vec<u8> = bincode::serialize(&config)?;
        Ok(serialized)
    });
    result
}

pub fn get_group_config(nostr_id: String, group_id: String) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[get_group_config]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[get_group_config]> key_pair do not init.");
            return Err(format_err!("<fn[get_group_config]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[get_group_config]> Can not get store from user."))?;
        let config = user.get_group_config(group_id.clone())?;
        Ok(config)
    });
    result
}

// return  group name, description, admin_pubkeys and relays info
pub fn get_group_extension(
    nostr_id: String,
    group_id: String,
) -> Result<(Vec<u8>, Vec<u8>, Vec<Vec<u8>>, Vec<Vec<u8>>)> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[get_group_config]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[get_group_config]> key_pair do not init.");
            return Err(format_err!("<fn[get_group_config]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[get_group_config]> Can not get store from user."))?;
        let extension = user.get_group_extension(group_id.clone())?;
        Ok((
            extension.name,
            extension.description,
            extension.admin_pubkeys,
            extension.relays,
        ))
    });
    result
}

// return vec<String> of group members
pub fn get_group_members(nostr_id: String, group_id: String) -> Result<Vec<String>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[get_group_members]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[get_group_members]> key_pair do not init.");
            return Err(format_err!("<fn[get_group_members]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[get_group_members]> Can not get store from user."))?;
        let members = user.get_group_members(group_id.clone())?;
        Ok(members)
    });
    result
}

/*
   Group IDs SHOULD be constructed in such a way that
   there is an overwhelmingly low probability of honest group creators generating the same group ID,
   even without assistance from the Delivery Service. This can be done, for example,
   by making the group ID a freshly generated random value of size KDF.Nh.
   The Delivery Service MAY attempt to ensure that group IDs are globally unique
   by rejecting the creation of new groups with a previously used ID.
*/

// when create group, then return the group join config
// note: admin_pubkeys_hex is a vec, but only one admin in Keychat
pub fn create_mls_group(
    nostr_id: String,
    group_id: String,
    description: String,
    admin_pubkeys_hex: Vec<String>,
    group_relays: Vec<String>,
) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[create_mls_group]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[create_mls_group]> key_pair do not init.");
            return Err(format_err!("<fn[create_mls_group]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[create_mls_group]> Can not get store from user."))?;
        let group_config = user.create_mls_group(
            group_id.clone(),
            description,
            group_name,
            admin_pubkeys_hex,
            group_relays,
        )?;
        user.update(nostr_id, false).await?;
        Ok(group_config)
    });
    result
}

// add several friends every time
pub fn add_members(
    nostr_id: String,
    group_id: String,
    key_packages: Vec<Vec<u8>>,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[add_members]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[add_members]> key_pair do not init.");
            return Err(format_err!("<fn[add_members]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[add_members]> Can not get store from user."))?;
        let (queued_msg, welcome) = user.add_members(group_id, key_packages)?;
        Ok((queued_msg, welcome))
    });
    result
}

/**
* PrivateMessage
    ContentType::Application = 1
    ContentType::Proposal = 2
    ContentType::Commit = 3
* Welcome = 4
* GroupInfo = 5
* KeyPackage = 6  Not use
* PublicMessage = 0
*/
pub fn parse_mls_msg_type(data: Vec<u8>) -> Result<u8> {
    let queued_msg = MlsMessageIn::tls_deserialize_exact(&data)?;
    match queued_msg.extract() {
        MlsMessageBodyIn::PrivateMessage(private_msg) => {
            println!("Received a private message");
            match private_msg.content_type() {
                ContentType::Application => {
                    return Ok(1);
                }
                ContentType::Proposal => {
                    return Ok(2);
                }
                ContentType::Commit => {
                    return Ok(3);
                }
            }
        }
        MlsMessageBodyIn::Welcome(_welcome_msg) => {
            println!("Received a welcome message");
            return Ok(4);
        }
        MlsMessageBodyIn::GroupInfo(_group_info) => {
            println!("Received a group_info message");
            return Ok(5);
        }
        MlsMessageBodyIn::KeyPackage(_key_package) => {
            println!("Received a key_package message");
            return Ok(6);
        }
        _ => {
            println!("Received a public message, ignore.");
            return Ok(0);
        }
    }
}

pub fn self_commit(nostr_id: String, group_id: String) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[self_commit]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[self_commit]> key_pair do not init.");
            return Err(format_err!("<fn[self_commit]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[self_commit]> Can not get store from user."))?;
        user.self_commit(group_id)?;
        Ok(())
    });
    result
}

// others join the group
pub fn join_mls_group(nostr_id: String, group_id: String, welcome: Vec<u8>) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[join_mls_group]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[join_mls_group]> nostr_id do not init.");
            return Err(format_err!("<fn[join_mls_group]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[join_mls_group]> Can not get store from user."))?;
        user.join_mls_group(group_id, welcome)?;
        user.update(nostr_id, false).await?;
        Ok(())
    });
    result
}

pub fn delete_group(nostr_id: String, group_id: String) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[delete_group]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[delete_group]> nostr_id do not init.");
            return Err(format_err!("<fn[delete_group]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[delete_group]> Can not get store from user."))?;
        user.delete_group(group_id)?;
        user.update(nostr_id, false).await?;
        Ok(())
    });
    result
}

// only group is not null, and other members should execute this
pub fn others_commit_normal(nostr_id: String, group_id: String, queued_msg: Vec<u8>) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[others_commit_normal]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[others_commit_normal]> nostr_id do not init.");
            return Err(format_err!(
                "<fn[others_commit_normal]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<fn[others_commit_normal]> Can not get store from user.")
        })?;
        user.others_commit_normal(group_id, queued_msg)?;
        Ok(())
    });
    result
}

pub fn send_msg(
    nostr_id: String,
    group_id: String,
    msg: String,
) -> Result<(Vec<u8>, Option<Vec<u8>>)> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[send_msg]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[send_msg]> nostr_id do not init.");
            return Err(format_err!("<fn[send_msg]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[send_msg]> Can not get store from user."))?;
        Ok(user.send_msg(group_id, msg)?)
    });
    result
}

pub fn decrypt_msg(
    nostr_id: String,
    group_id: String,
    msg: Vec<u8>,
) -> Result<(String, String, Option<Vec<u8>>)> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[decrypt_msg]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[decrypt_msg]> nostr_id do not init.");
            return Err(format_err!("<fn[decrypt_msg]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[decrypt_msg]> Can not get store from user."))?;
        Ok(user.decrypt_msg(group_id, msg)?)
    });
    result
}

// when remove remembers, should use this lead node
pub fn get_lead_node_index(
    nostr_id_admin: String,
    nostr_id_common: String,
    group_id: String,
) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[get_lead_node_index]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id_admin) {
            error!("<fn[get_lead_node_index]> nostr_id do not init.");
            return Err(format_err!(
                "<fn[get_lead_node_index]> nostr_id do not init."
            ));
        }
        let user = store
            .user
            .get_mut(&nostr_id_admin)
            .ok_or_else(|| format_err!("<fn[get_lead_node_index]> Can not get store from user."))?;
        Ok(user.get_lead_node_index(nostr_id_common, group_id)?)
    });
    result
}

pub fn remove_members(
    nostr_id: String,
    group_id: String,
    members: Vec<Vec<u8>>,
) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[remove_members]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[remove_members]> nostr_id do not init.");
            return Err(format_err!("<fn[remove_members]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[remove_members]> Can not get store from user."))?;
        Ok(user.remove_members(group_id, members)?)
    });
    result
}

pub fn others_commit_remove_member(
    nostr_id: String,
    group_id: String,
    queued_msg: Vec<u8>,
) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<fn[others_commit_remove_member]> Can not get store err.")
        })?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[others_commit_remove_member]> nostr_id do not init.");
            return Err(format_err!(
                "<fn[others_commit_remove_member]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<fn[others_commit_remove_member]> Can not get store from user.")
        })?;
        user.others_commit_remove_member(group_id, queued_msg)?;
        Ok(())
    });
    result
}

pub fn self_leave(nostr_id: String, group_id: String) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[self_leave]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[self_leave]> nostr_id do not init.");
            return Err(format_err!("<fn[self_leave]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[self_leave]> Can not get store from user."))?;
        Ok(user.self_leave(group_id)?)
    });
    result
}

pub fn self_update(nostr_id: String, group_id: String) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[self_update]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[self_update]> nostr_id do not init.");
            return Err(format_err!("<fn[self_update]> nostr_id do not init."));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[self_update]> Can not get store from user."))?;
        Ok(user.self_update(group_id)?)
    });
    result
}

pub fn others_proposal_leave(
    nostr_id: String,
    group_id: String,
    queued_msg: Vec<u8>,
) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[others_proposal_leave]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[others_proposal_leave]> nostr_id do not init.");
            return Err(format_err!(
                "<fn[others_proposal_leave]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<fn[others_proposal_leave]> Can not get store from user.")
        })?;
        user.others_proposal_leave(group_id, queued_msg)?;
        Ok(())
    });
    result
}

pub fn admin_commit_leave(nostr_id: String, group_id: String) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[admin_commit_leave]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[admin_commit_leave]> nostr_id do not init.");
            return Err(format_err!(
                "<fn[admin_commit_leave]> nostr_id do not init."
            ));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[admin_commit_leave]> Can not get store from user."))?;
        Ok(user.admin_commit_leave(group_id)?)
    });
    result
}

pub fn admin_proposal_leave(nostr_id: String, group_id: String) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[admin_commit_leave]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[admin_commit_leave]> nostr_id do not init.");
            return Err(format_err!(
                "<fn[admin_commit_leave]> nostr_id do not init."
            ));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<fn[admin_commit_leave]> Can not get store from user."))?;
        Ok(user.admin_proposal_leave(group_id)?)
    });
    result
}
// expect admin, queued_msg is from admin
pub fn normal_member_commit_leave(
    nostr_id: String,
    group_id: String,
    queued_msg: Vec<u8>,
) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<fn[normal_member_commit_leave]> Can not get store err.")
        })?;
        if !store.user.contains_key(&nostr_id) {
            error!("<fn[normal_member_commit_leave]> nostr_id do not init.");
            return Err(format_err!(
                "<fn[normal_member_commit_leave]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<fn[normal_member_commit_leave]> Can not get store from user.")
        })?;
        user.normal_member_commit_leave(group_id, queued_msg)?;
        Ok(())
    });
    result
}
