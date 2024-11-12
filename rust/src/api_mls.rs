use anyhow::Result;
use bincode;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

pub use kc::identity::Identity;
pub use kc::openmls_rust_persistent_crypto::OpenMlsRustPersistentCrypto;
pub use openmls::group::{GroupId, MlsGroup, MlsGroupCreateConfig, MlsGroupJoinConfig};
pub use openmls_sqlite_storage::MLSLitePool;
pub use openmls_traits::OpenMlsProvider;

#[path = "api_mls.user.rs"]
pub mod user;
pub use user::*;

// must be ignore, otherwise will be error when rust to dart
#[frb(ignore)]
pub struct MlsStore {
    pub pool: MLSLitePool,
    pub user: HashMap<String, User>,
}

lazy_static! {
    static ref STORE: Mutex<Option<MlsStore>> = Mutex::new(None);
}

lazy_static! {
    static ref RUNTIME: Arc<StdMutex<Runtime>> = Arc::new(StdMutex::new(
        Runtime::new().expect("failed to create tokio runtime")
    ));
}

macro_rules! lock_runtime {
    () => {
        match RUNTIME.lock() {
            Ok(lock) => lock,
            Err(err) => {
                let err: anyhow::Error = anyhow!("Failed to lock the runtime mutex: {}", err);
                return Err(err.into());
            }
        }
    };
}

// when init db then create the user, every user show init it
pub fn init_mls_db(db_path: String, nostr_id: String) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        if store.is_none() {
            let pool = MLSLitePool::open(&db_path, Default::default()).await?;
            *store = Some(MlsStore {
                pool,
                user: HashMap::new(),
            });
            error!("store has not been inited.");
        }
        let map = store
            .as_mut()
            .ok_or_else(|| format_err!("<fn[init_mls_db]> Can not get store err."))?;
        // first load from db, if none then create new user
        let user_op = User::load(nostr_id.clone(), map.pool.clone()).await?;
        if user_op.is_none() {
            let user = User::new(nostr_id.clone(), map.pool.clone()).await;
            map.user.entry(nostr_id).or_insert(user);
        } else {
            map.user.entry(nostr_id).or_insert(user_op.unwrap());
        }
        Ok(())
    });
    result
}

pub fn get_export_secret(nostr_id: String, group_id: String) -> Result<Vec<u8>> {
    let rt = lock_runtime!();
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
    let rt = lock_runtime!();
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
    let rt = lock_runtime!();
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
        let serialized: Vec<u8> = bincode::serialize(&key_package)?;
        Ok(serialized)
    });
    result
}

pub fn create_group_config() -> Result<Vec<u8>> {
    let rt = lock_runtime!();
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
    let rt = lock_runtime!();
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

/*
   Group IDs SHOULD be constructed in such a way that
   there is an overwhelmingly low probability of honest group creators generating the same group ID,
   even without assistance from the Delivery Service. This can be done, for example,
   by making the group ID a freshly generated random value of size KDF.Nh.
   The Delivery Service MAY attempt to ensure that group IDs are globally unique
   by rejecting the creation of new groups with a previously used ID.
*/

// when create group, then return the group join config
pub fn create_mls_group(nostr_id: String, group_id: String) -> Result<Vec<u8>> {
    let rt = lock_runtime!();
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
        let group_config = user.create_mls_group(group_id.clone())?;
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
    let rt = lock_runtime!();
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

pub fn self_commit(nostr_id: String, group_id: String) -> Result<()> {
    let rt = lock_runtime!();
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
pub fn join_mls_group(
    nostr_id: String,
    group_id: String,
    welcome: Vec<u8>,
    group_join_config: Vec<u8>,
) -> Result<()> {
    let rt = lock_runtime!();
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
        user.join_mls_group(group_id, welcome, group_join_config)?;
        user.update(nostr_id, false).await?;
        Ok(())
    });
    result
}

// only group is not null, and other members should execute this
pub fn others_commit_normal(nostr_id: String, group_id: String, queued_msg: Vec<u8>) -> Result<()> {
    let rt = lock_runtime!();
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

pub fn send_msg(nostr_id: String, group_id: String, msg: String) -> Result<(Vec<u8>, Vec<u8>)> {
    let rt = lock_runtime!();
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

pub fn decrypt_msg(nostr_id: String, group_id: String, msg: Vec<u8>) -> Result<(String, String)> {
    let rt = lock_runtime!();
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
pub fn get_lead_node_index(nostr_id_admin: String, nostr_id_common: String, group_id: String) -> Result<Vec<u8>> {
    let rt = lock_runtime!();
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
    let rt = lock_runtime!();
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
    let rt = lock_runtime!();
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
    let rt = lock_runtime!();
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
    let rt = lock_runtime!();
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
    let rt = lock_runtime!();
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
    let rt = lock_runtime!();
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
    Ok(())
}

pub fn admin_proposal_leave(nostr_id: String, group_id: String) -> Result<Vec<u8>> {
    let rt = lock_runtime!();
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
    let rt = lock_runtime!();
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
