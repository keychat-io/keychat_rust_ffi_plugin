use kc::identity::Identity;
use kc::openmls_rust_persistent_crypto::OpenMlsRustPersistentCrypto;
use openmls::group::{GroupId, MlsGroup, MlsGroupCreateConfig};
use openmls::key_packages::KeyPackage;
use openmls::prelude::tls_codec::Deserialize;
use openmls::prelude::{LeafNodeIndex, MlsMessageIn, ProcessedMessageContent, StagedWelcome};
use openmls_sqlite_storage::LitePool;
use openmls_traits::types::Ciphersuite;

use anyhow::Result;
use bincode;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use std::{cell::RefCell, collections::HashMap, str};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

pub(crate) const CIPHERSUITE: Ciphersuite =
    Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;

#[derive(Debug)]
pub struct Group {
    mls_group: RefCell<MlsGroup>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(skip)]
    pub(crate) groups: RefCell<HashMap<String, Group>>,
    group_list: HashSet<String>,
    pub(crate) identity: RefCell<Identity>,
    #[serde(skip)]
    provider: OpenMlsRustPersistentCrypto,
}

impl User {
    /// Create a new user with the given name and a fresh set of credentials.
    pub async fn new(username: String, pool: LitePool) -> Self {
        let crypto = OpenMlsRustPersistentCrypto::new(username.clone(), pool).await;
        let out = Self {
            groups: RefCell::new(HashMap::new()),
            group_list: HashSet::new(),
            identity: RefCell::new(Identity::new(
                CIPHERSUITE,
                &crypto,
                username.clone().as_bytes(),
            )),
            provider: crypto,
        };
        out
    }

    pub fn get_export_secret(&self, group_name: String) -> Result<Vec<u8>> {
        let mut groups = self.groups.borrow_mut();
        let group = match groups.get_mut(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let mls_group = group.mls_group.borrow_mut();
        let export_secret = mls_group.export_secret(&self.provider, "", &[], 32)?;
        Ok(export_secret)
    }

    pub fn create_key_package(&mut self) -> Result<KeyPackage> {
        let key_package = self
            .identity
            .borrow_mut()
            .add_key_package(CIPHERSUITE, &self.provider);
        Ok(key_package)
    }

    pub fn create_mls_group(
        &mut self,
        group_name: String,
        group_create_config: Vec<u8>,
    ) -> Result<MlsGroup> {
        let group_create_config: MlsGroupCreateConfig = bincode::deserialize(&group_create_config)?;

        let mls_group = MlsGroup::new_with_group_id(
            &self.provider,
            &self.identity.borrow().signer,
            &group_create_config,
            GroupId::from_slice(group_name.as_bytes()),
            self.identity.borrow().credential_with_key.clone(),
        )?;
        let group = Group {
            mls_group: RefCell::new(mls_group.clone()),
        };

        if self.groups.borrow().contains_key(&group_name) {
            panic!("Group '{}' existed already", group_name);
        }

        self.groups.borrow_mut().insert(group_name.clone(), group);
        self.group_list.insert(group_name);

        Ok(mls_group)
    }

    pub fn add_members(
        &mut self,
        group_name: String,
        key_packages: Vec<Vec<u8>>,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        let mut kps: Vec<KeyPackage> = Vec::new();
        for kp in key_packages {
            let kp: KeyPackage = bincode::deserialize(&kp)?;
            kps.push(kp);
        }
        // Build a proposal with this key package and do the MLS bits.
        let mut groups = self.groups.borrow_mut();
        let group = match groups.get_mut(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let mut mls_group = group.mls_group.borrow_mut();
        let (queued_msg, welcome, _) =
            mls_group.add_members(&self.provider, &self.identity.borrow().signer, &kps)?;

        mls_group.merge_pending_commit(&self.provider)?;
        let serialized_queued_msg: Vec<u8> = queued_msg.to_bytes()?;
        let serialized_welcome: Vec<u8> = welcome.to_bytes()?;

        Ok((serialized_queued_msg, serialized_welcome))
    }

    pub fn bob_join_mls_group(
        &mut self,
        group_name: String,
        welcome: Vec<u8>,
        group_create_config: Vec<u8>,
    ) -> Result<MlsGroup> {
        let group_create_config: MlsGroupCreateConfig = bincode::deserialize(&group_create_config)?;
        let welcome = MlsMessageIn::tls_deserialize_exact(&welcome)?;
        let welcome = welcome.into_welcome().ok_or_else(|| {
            format_err!(
                "<mls api fn[bob_join_mls_group]> expected the message to be a welcome message."
            )
        })?;

        let bob_mls_group = StagedWelcome::new_from_welcome(
            &self.provider,
            group_create_config.join_config(),
            welcome,
            None,
        )
        .map_err(|_| {
            format_err!(
                "<mls api fn[bob_join_mls_group]> Error creating StagedWelcome from Welcome."
            )
        })?
        .into_group(&self.provider)
        .map_err(|_| {
            format_err!("<mls api fn[bob_join_mls_group]> Error creating group from StagedWelcome.")
        })?;

        let group = Group {
            mls_group: RefCell::new(bob_mls_group.clone()),
        };

        if self.groups.borrow().contains_key(&group_name) {
            panic!("Group '{}' existed already", group_name);
        }

        self.groups.borrow_mut().insert(group_name.clone(), group);
        self.group_list.insert(group_name);

        Ok(bob_mls_group)
    }

    // only group is not null, and other members should execute this
    pub fn others_commit_add_member(
        &mut self,
        group_name: String,
        queued_msg: Vec<u8>,
    ) -> Result<()> {
        let mut groups = self.groups.borrow_mut();
        let group = match groups.get_mut(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let mut mls_group = group.mls_group.borrow_mut();
        let queued_msg = MlsMessageIn::tls_deserialize_exact(&queued_msg)?;
        let alice_processed_message = mls_group.process_message(
            &self.provider,
            queued_msg.into_protocol_message().ok_or_else(|| {
                format_err!("<mls api fn[others_commit_add_member]> Unexpected message type")
            })?,
        )?;

        if let ProcessedMessageContent::StagedCommitMessage(staged_commit) =
            alice_processed_message.into_content()
        {
            mls_group.merge_staged_commit(&self.provider, *staged_commit)?;
        } else {
            Err(anyhow::anyhow!(
                "<mls api fn[others_commit_add_member]> Expected a StagedCommit."
            ))?;
        }

        Ok(())
    }

    pub fn send_msg(&mut self, group_name: String, msg: String) -> Result<Vec<u8>> {
        let mut groups = self.groups.borrow_mut();
        let group = match groups.get_mut(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let msg_out = group
            .mls_group
            .borrow_mut()
            .create_message(
                &self.provider,
                &self.identity.borrow().signer,
                msg.as_bytes(),
            )
            .map_err(|_| format_err!("<mls api fn[send_msg]> Error send message."))?;
        let serialized_msg_out: Vec<u8> = msg_out.to_bytes()?;
        Ok(serialized_msg_out)
    }

    pub fn decrypt_msg(&mut self, group_name: String, msg: Vec<u8>) -> Result<String> {
        let mut groups = self.groups.borrow_mut();
        let group = match groups.get_mut(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let msg = MlsMessageIn::tls_deserialize_exact(&msg)?;
        let processed_message = group
            .mls_group
            .borrow_mut()
            .process_message(
                &self.provider,
                msg.into_protocol_message()
                    .ok_or_else(|| format_err!("Unexpected message type"))?,
            )
            .map_err(|_| format_err!("<mls api fn[send_msg]> Error decrypt message."))?;
        if let ProcessedMessageContent::ApplicationMessage(application_message) =
            processed_message.into_content()
        {
            let text = String::from_utf8(application_message.into_bytes()).unwrap();
            Ok(text)
        } else {
            Err(anyhow::anyhow!(
                "<mls api fn[decrypt_msg]> Unexpected application_message."
            ))
        }
    }

    pub fn get_lead_node_index(&mut self, group_name: String) -> Result<Vec<u8>> {
        let groups = self.groups.borrow();
        let group = match groups.get(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let lead_node_index = group.mls_group.borrow_mut().own_leaf_index();
        let lead_node_index_vec = bincode::serialize(&lead_node_index)?;
        Ok(lead_node_index_vec)
    }

    pub fn remove_members(&mut self, group_name: String, members: Vec<Vec<u8>>) -> Result<Vec<u8>> {
        let mut groups = self.groups.borrow_mut();
        let group = match groups.get_mut(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let mut mls_group = group.mls_group.borrow_mut();
        let mut leaf_nodes: Vec<LeafNodeIndex> = Vec::new();
        for m in members {
            let m: LeafNodeIndex = bincode::deserialize(&m)?;
            leaf_nodes.push(m);
        }
        // alice remove bob, so alice should konw bob's mls_group
        let (queued_msg, _welcome, _group_info) = mls_group.remove_members(
            &self.provider,
            &self.identity.borrow().signer,
            &leaf_nodes,
        )?;

        mls_group.merge_pending_commit(&self.provider)?;
        let serialized_queued_msg: Vec<u8> = queued_msg.to_bytes()?;
        Ok(serialized_queued_msg)
    }

    pub fn others_commit_remove_member(
        &mut self,
        group_name: String,
        queued_msg: Vec<u8>,
    ) -> Result<()> {
        let mut groups = self.groups.borrow_mut();
        let group = match groups.get_mut(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let mut mls_group = group.mls_group.borrow_mut();
        let queued_msg = MlsMessageIn::tls_deserialize_exact(&queued_msg)?;
        let processed_message = mls_group.process_message(
            &self.provider,
            queued_msg.into_protocol_message().ok_or_else(|| {
                format_err!("<mls api fn[others_commit_remove_member]> Unexpected message type")
            })?,
        )?;

        // Check that we receive the correct proposal
        if let ProcessedMessageContent::StagedCommitMessage(staged_commit) =
            processed_message.into_content()
        {
            let _remove = staged_commit.remove_proposals().next().ok_or_else(|| {
                format_err!("<mls api fn[others_commit_remove_member]> Expected a proposal.")
            })?;

            // Merge staged Commit
            mls_group.merge_staged_commit(&self.provider, *staged_commit)?;
        } else {
            // unreachable!("Expected a StagedCommit.");
            Err(anyhow::anyhow!(
                "<mls api fn[others_commit_remove_member]> Expected a StagedCommit."
            ))?;
        }

        Ok(())
    }

    pub fn self_leave(&mut self, group_name: String) -> Result<Vec<u8>> {
        let mut groups = self.groups.borrow_mut();
        let group = match groups.get_mut(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let queued_msg = group
            .mls_group
            .borrow_mut()
            .leave_group(&self.provider, &self.identity.borrow().signer)?;

        let serialized_queued_msg: Vec<u8> = queued_msg.to_bytes()?;
        Ok(serialized_queued_msg)
    }

    pub fn others_proposal_leave(&mut self, group_name: String, queued_msg: Vec<u8>) -> Result<()> {
        let mut groups = self.groups.borrow_mut();
        let group = match groups.get_mut(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let mut mls_group = group.mls_group.borrow_mut();
        let queued_msg = MlsMessageIn::tls_deserialize_exact(&queued_msg)?;
        let processed_message = mls_group.process_message(
            &self.provider,
            queued_msg.into_protocol_message().ok_or_else(|| {
                format_err!("<mls api fn[others_proposal_leave]> Unexpected message type")
            })?,
        )?;

        // Store proposal
        if let ProcessedMessageContent::ProposalMessage(staged_proposal) =
            processed_message.into_content()
        {
            mls_group.store_pending_proposal(&self.provider.storage, *staged_proposal)?;
        } else {
            unreachable!("<mls api fn[others_proposal_leave]> Expected a QueuedProposal.");
        }

        Ok(())
    }

    pub fn admin_commit_leave(&mut self, group_name: String) -> Result<Vec<u8>> {
        let mut groups = self.groups.borrow_mut();
        let group = match groups.get_mut(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let mut mls_group = group.mls_group.borrow_mut();
        let (queued_msg, _welcome_option, _group_info) = mls_group
            .commit_to_pending_proposals(&self.provider, &self.identity.borrow().signer)?;

        // if let Some(staged_commit) = mls_group.pending_commit() {
        //     let _remove = staged_commit.remove_proposals().next().ok_or_else(|| {
        //         format_err!("<mls api fn[admin_commit_leave]> Expected a proposal.")
        //     })?;
        //     mls_group
        //         .merge_staged_commit(&self.provider, staged_commit.clone())?;
        // } else {
        //     unreachable!("<mls api fn[admin_commit_leave]> Expected a StagedCommit.");
        // }
        let staged_commit = mls_group.pending_commit().cloned().ok_or_else(|| {
            format_err!("<mls api fn[admin_commit_leave]> Expected a StagedCommit.")
        })?;
        let _remove = staged_commit
            .remove_proposals()
            .next()
            .ok_or_else(|| format_err!("<mls api fn[admin_commit_leave]> Expected a proposal."))?;
        mls_group.merge_staged_commit(&self.provider, staged_commit)?;

        let serialized_queued_msg: Vec<u8> = queued_msg.to_bytes()?;
        Ok(serialized_queued_msg)
    }

    // expect admin, queued_msg is from admin
    pub fn normal_member_commit_leave(
        &mut self,
        group_name: String,
        queued_msg: Vec<u8>,
    ) -> Result<()> {
        let mut groups = self.groups.borrow_mut();
        let group = match groups.get_mut(&group_name) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {group_name} known.")),
        };
        let mut mls_group = group.mls_group.borrow_mut();
        let queued_msg = MlsMessageIn::tls_deserialize_exact(&queued_msg)?;
        // === Leave operation from normal member's perspective ===
        let processed_message = mls_group.process_message(
            &self.provider,
            queued_msg.into_protocol_message().ok_or_else(|| {
                format_err!("<mls api fn[normal_member_commit_leave]> Unexpected message type")
            })?,
        )?;

        // Check that we received the correct proposals
        if let ProcessedMessageContent::StagedCommitMessage(staged_commit) =
            processed_message.into_content()
        {
            let _remove = staged_commit.remove_proposals().next().ok_or_else(|| {
                format_err!("<mls api fn[normal_member_commit_leave]> Expected a proposal.")
            })?;
            // Merge staged Commit
            mls_group.merge_staged_commit(&self.provider, *staged_commit)?;
        } else {
            unreachable!("<mls api fn[normal_member_commit_leave]> Expected a StagedCommit.");
        }

        Ok(())
    }

    pub async fn save() {}
    pub async fn load() {}
    // pub async fn get_latest(group_name: String) -> Result<User>{

    // }
}

pub struct MlsStore {
    pub pool: LitePool,
    // pub user: User,
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

// when init db then create the user
pub fn init_mls_db(db_path: String, nostr_id: String) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        if store.is_none() {
            let pool = LitePool::open(&db_path, Default::default()).await?;
            // let user = User::new(nostr_id, pool.clone()).await;
            // *store = Some(MlsStore { pool, user });
            *store = Some(MlsStore {
                pool,
                user: HashMap::new(),
            });
            error!("store has not been inited.");
        }
        let map = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[init]> Can not get store err."))?;
        let user = User::new(nostr_id.clone(), map.pool.clone()).await;

        map.user.entry(nostr_id).or_insert(user);
        // println!("init_mls_db {:?}", map.user.len());

        Ok(())
    });
    result
}

pub fn get_export_secret(nostr_id: String, group_name: String) -> Result<Vec<u8>> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<mls api fn[create_key_package]> Can not get store err.")
        })?;
        if !store.user.contains_key(&nostr_id) {
            error!("create_key_package key_pair do not init.");
            return Err(format_err!(
                "<signal api fn[create_key_package]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[create_key_package]> Can not get store from user.")
        })?;

        let export_secret = user.get_export_secret(group_name)?;
        Ok(export_secret)
    });
    result
}

pub fn create_key_package(nostr_id: String) -> Result<Vec<u8>> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<mls api fn[create_key_package]> Can not get store err.")
        })?;
        if !store.user.contains_key(&nostr_id) {
            error!("create_key_package key_pair do not init.");
            return Err(format_err!(
                "<signal api fn[create_key_package]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[create_key_package]> Can not get store from user.")
        })?;

        let key_package = user.create_key_package()?;
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

pub fn create_mls_group(
    nostr_id: String,
    group_name: String,
    group_create_config: Vec<u8>,
) -> Result<MlsGroup> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<mls api fn[create_mls_group]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("create_key_package key_pair do not init.");
            return Err(format_err!(
                "<signal api fn[create_mls_group]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[create_mls_group]> Can not get store from user.")
        })?;
        let mls_group = user.create_mls_group(group_name.clone(), group_create_config)?;
        Ok(mls_group)
    });
    result
}

// add several friends every time
pub fn add_members(
    nostr_id: String,
    group_name: String,
    key_packages: Vec<Vec<u8>>,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<mls api fn[add_members]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("create_key_package key_pair do not init.");
            return Err(format_err!(
                "<signal api fn[add_members]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[add_members]> Can not get store from user.")
        })?;
        let (queued_msg, welcome) = user.add_members(group_name, key_packages)?;

        Ok((queued_msg, welcome))
    });
    result
}

pub fn bob_join_mls_group(
    nostr_id: String,
    group_name: String,
    welcome: Vec<u8>,
    group_create_config: Vec<u8>,
) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<mls api fn[bob_join_mls_group]> Can not get store err.")
        })?;
        if !store.user.contains_key(&nostr_id) {
            error!("bob_join_mls_group nostr_id do not init.");
            return Err(format_err!(
                "<signal api fn[bob_join_mls_group]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[bob_join_mls_group]> Can not get store from user.")
        })?;
        let _mls_group = user.bob_join_mls_group(group_name, welcome, group_create_config)?;

        Ok(())
    });
    result
}

// only group is not null, and other members should execute this
pub fn others_commit_add_member(
    nostr_id: String,
    group_name: String,
    queued_msg: Vec<u8>,
) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<mls api fn[others_commit_add_member]> Can not get store err.")
        })?;
        if !store.user.contains_key(&nostr_id) {
            error!("others_commit_add_member nostr_id do not init.");
            return Err(format_err!(
                "<signal api fn[others_commit_add_member]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[others_commit_add_member]> Can not get store from user.")
        })?;

        user.others_commit_add_member(group_name, queued_msg)?;
        Ok(())
    });
    result
}

pub fn send_msg(nostr_id: String, group_name: String, msg: String) -> Result<Vec<u8>> {
    let rt = lock_runtime!();

    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<mls api fn[send_msg]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("send_msg nostr_id do not init.");
            return Err(format_err!(
                "<signal api fn[send_msg]> nostr_id do not init."
            ));
        }
        let user = store
            .user
            .get_mut(&nostr_id)
            .ok_or_else(|| format_err!("<signal api fn[send_msg]> Can not get store from user."))?;
        Ok(user.send_msg(group_name, msg)?)
    });
    result
}

pub fn decrypt_msg(nostr_id: String, group_name: String, msg: Vec<u8>) -> Result<String> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<mls api fn[decrypt_msg]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("decrypt_msg nostr_id do not init.");
            return Err(format_err!(
                "<signal api fn[decrypt_msg]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[decrypt_msg]> Can not get store from user.")
        })?;
        Ok(user.decrypt_msg(group_name, msg)?)
    });
    result
}

pub fn get_lead_node_index(nostr_id: String, group_name: String) -> Result<Vec<u8>> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<mls api fn[get_lead_node_index]> Can not get store err.")
        })?;
        if !store.user.contains_key(&nostr_id) {
            error!("get_lead_node_index nostr_id do not init.");
            return Err(format_err!(
                "<signal api fn[get_lead_node_index]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[get_lead_node_index]> Can not get store from user.")
        })?;
        Ok(user.get_lead_node_index(group_name)?)
    });
    result
}

pub fn remove_members(
    nostr_id: String,
    group_name: String,
    members: Vec<Vec<u8>>,
) -> Result<Vec<u8>> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<mls api fn[remove_members]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("remove_members nostr_id do not init.");
            return Err(format_err!(
                "<signal api fn[remove_members]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[remove_members]> Can not get store from user.")
        })?;
        Ok(user.remove_members(group_name, members)?)
    });
    result
}

pub fn others_commit_remove_member(
    nostr_id: String,
    group_name: String,
    queued_msg: Vec<u8>,
) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<mls api fn[others_commit_remove_member]> Can not get store err.")
        })?;
        if !store.user.contains_key(&nostr_id) {
            error!("others_commit_remove_member nostr_id do not init.");
            return Err(format_err!(
                "<signal api fn[others_commit_remove_member]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[others_commit_remove_member]> Can not get store from user.")
        })?;

        user.others_commit_remove_member(group_name, queued_msg)?;
        Ok(())
    });
    result
}

pub fn self_leave(nostr_id: String, group_name: String) -> Result<Vec<u8>> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<mls api fn[self_leave]> Can not get store err."))?;
        if !store.user.contains_key(&nostr_id) {
            error!("self_leave nostr_id do not init.");
            return Err(format_err!(
                "<signal api fn[self_leave]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[self_leave]> Can not get store from user.")
        })?;
        Ok(user.self_leave(group_name)?)
    });
    result
}

pub fn others_proposal_leave(
    nostr_id: String,
    group_name: String,
    queued_msg: Vec<u8>,
) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<mls api fn[others_proposal_leave]> Can not get store err.")
        })?;
        if !store.user.contains_key(&nostr_id) {
            error!("others_proposal_leave nostr_id do not init.");
            return Err(format_err!(
                "<signal api fn[others_proposal_leave]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[others_proposal_leave]> Can not get store from user.")
        })?;
        user.others_proposal_leave(group_name, queued_msg)?;
        Ok(())
    });
    result
}

pub fn admin_commit_leave(nostr_id: String, group_name: String) -> Result<Vec<u8>> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<mls api fn[admin_commit_leave]> Can not get store err.")
        })?;
        if !store.user.contains_key(&nostr_id) {
            error!("admin_commit_leave nostr_id do not init.");
            return Err(format_err!(
                "<signal api fn[admin_commit_leave]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[admin_commit_leave]> Can not get store from user.")
        })?;
        Ok(user.admin_commit_leave(group_name)?)
    });
    result
}

// expect admin, queued_msg is from admin
pub fn normal_member_commit_leave(
    nostr_id: String,
    group_name: String,
    queued_msg: Vec<u8>,
) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<mls api fn[normal_member_commit_leave]> Can not get store err.")
        })?;
        if !store.user.contains_key(&nostr_id) {
            error!("normal_member_commit_leave nostr_id do not init.");
            return Err(format_err!(
                "<signal api fn[normal_member_commit_leave]> nostr_id do not init."
            ));
        }
        let user = store.user.get_mut(&nostr_id).ok_or_else(|| {
            format_err!("<signal api fn[normal_member_commit_leave]> Can not get store from user.")
        })?;
        user.normal_member_commit_leave(group_name, queued_msg)?;

        Ok(())
    });
    result
}
