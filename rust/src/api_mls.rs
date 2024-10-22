use kc::identity::Identity;
use kc::openmls_rust_persistent_crypto::OpenMlsRustPersistentCrypto;
use openmls::group::{GroupId, MlsGroup, MlsGroupCreateConfig};
use openmls::key_packages::KeyPackage;
use openmls::prelude::{
    LeafNodeIndex, MlsMessageIn, MlsMessageOut, ProcessedMessageContent, StagedWelcome,
};
use openmls_sqlite_storage::LitePool;
use openmls_traits::types::Ciphersuite;

use anyhow::Result;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

pub(crate) const CIPHERSUITE: Ciphersuite =
    Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;

pub struct MlsStore {
    pub pool: LitePool,
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

pub fn init_mls_db(db_path: String) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let pool = LitePool::open(&db_path, Default::default()).await?;
        let mut store = STORE.lock().await;
        *store = Some(MlsStore { pool });
        Ok(())
    });
    result
}

pub fn create_provider(nostr_id: String) -> Result<OpenMlsRustPersistentCrypto> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;

        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<mls api fn[create_provider]> Can not get store err."))?;

        let provider = OpenMlsRustPersistentCrypto::new(nostr_id, store.pool.clone()).await;

        Ok(provider)
    });
    result
}

pub fn create_identity(
    nostr_id: String,
    provider: &OpenMlsRustPersistentCrypto,
) -> Result<Identity> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let identity = Identity::new(CIPHERSUITE, provider, nostr_id.as_bytes());

        Ok(identity)
    });
    result
}

pub fn create_key_package(
    provider: &OpenMlsRustPersistentCrypto,
    identity: &mut Identity,
) -> Result<KeyPackage> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let key_package = identity.add_key_package(CIPHERSUITE, provider);

        Ok(key_package)
    });
    result
}

pub fn group_create_config() -> Result<MlsGroupCreateConfig> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let config = MlsGroupCreateConfig::builder()
            .use_ratchet_tree_extension(true)
            .build();

        Ok(config)
    });
    result
}

pub fn create_mls_group(
    group_id: String,
    group_create_config: &MlsGroupCreateConfig,
    provider: &OpenMlsRustPersistentCrypto,
    identity: &Identity,
) -> Result<MlsGroup> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let mls_group = MlsGroup::new_with_group_id(
            provider,
            &identity.signer,
            group_create_config,
            GroupId::from_slice(group_id.as_bytes()),
            identity.credential_with_key.clone(),
        )
        .map_err(|_| format_err!("<mls api fn[create_mls_group]> execute err."))?;

        Ok(mls_group)
    });
    result
}

// only add one every time
pub fn add_member(
    alice_mls_group: &mut MlsGroup,
    alice_provider: &OpenMlsRustPersistentCrypto,
    alice_identity: &Identity,
    bob_key_package: KeyPackage,
) -> Result<(MlsMessageOut, MlsMessageOut)> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let (queued_msg, welcome, _) = alice_mls_group.add_members(
            alice_provider,
            &alice_identity.signer,
            &[bob_key_package.into()],
        )?;

        alice_mls_group.merge_pending_commit(alice_provider)?;

        Ok((queued_msg, welcome))
    });
    result
}

// add several every time
pub fn add_members(
    alice_mls_group: &mut MlsGroup,
    alice_provider: &OpenMlsRustPersistentCrypto,
    alice_identity: &Identity,
    key_packages: &[KeyPackage],
) -> Result<(MlsMessageOut, MlsMessageOut)> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let (queued_msg, welcome, _) =
            alice_mls_group.add_members(alice_provider, &alice_identity.signer, key_packages)?;

        alice_mls_group.merge_pending_commit(alice_provider)?;

        Ok((queued_msg, welcome))
    });
    result
}

pub fn join_mls_group(
    welcome: MlsMessageOut,
    bob_provider: &OpenMlsRustPersistentCrypto,
    group_create_config: &MlsGroupCreateConfig,
) -> Result<MlsGroup> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let welcome: MlsMessageIn = welcome.into();
        let welcome = welcome.into_welcome().ok_or_else(|| {
            format_err!(
                "<mls api fn[join_mls_group]> expected the message to be a welcome message."
            )
        })?;

        let bob_mls_group = StagedWelcome::new_from_welcome(
            bob_provider,
            group_create_config.join_config(),
            welcome,
            None,
        )
        .map_err(|_| {
            format_err!("<mls api fn[join_mls_group]> Error creating StagedWelcome from Welcome.")
        })?
        .into_group(bob_provider)
        .map_err(|_| {
            format_err!("<mls api fn[join_mls_group]> Error creating group from StagedWelcome.")
        })?;

        Ok(bob_mls_group)
    });
    result
}

// only group is not null, and other members should execute this
pub fn others_commit_add_member(
    mls_group: &mut MlsGroup,
    queued_msg: MlsMessageOut,
    provider: &OpenMlsRustPersistentCrypto,
) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let alice_processed_message = mls_group.process_message(
            provider,
            queued_msg.into_protocol_message().ok_or_else(|| {
                format_err!("<mls api fn[others_commit_add_member]> Unexpected message type")
            })?,
        )?;

        if let ProcessedMessageContent::StagedCommitMessage(staged_commit) =
            alice_processed_message.into_content()
        {
            mls_group.merge_staged_commit(provider, *staged_commit)?;
        } else {
            Err(anyhow::anyhow!(
                "<mls api fn[others_commit_add_member]> Expected a StagedCommit."
            ))?;
        }

        Ok(())
    });
    result
}

pub fn send_msg(
    alice_mls_group: &mut MlsGroup,
    alice_provider: &OpenMlsRustPersistentCrypto,
    alice_identity: &Identity,
    msg: String,
) -> Result<MlsMessageOut> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let msg_out = alice_mls_group
            .create_message(alice_provider, &alice_identity.signer, msg.as_bytes())
            .map_err(|_| format_err!("<mls api fn[send_msg]> Error send message."))?;
        Ok(msg_out)
    });
    result
}

pub fn decrypt_msg(
    bob_mls_group: &mut MlsGroup,
    bob_provider: &OpenMlsRustPersistentCrypto,
    msg: MlsMessageOut,
) -> Result<String> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let processed_message = bob_mls_group
            .process_message(
                bob_provider,
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
    });
    result
}

pub fn remove_member(
    alice_mls_group: &mut MlsGroup,
    bob_mls_group: &mut MlsGroup,
    alice_identity: &Identity,
    alice_provider: &OpenMlsRustPersistentCrypto,
) -> Result<MlsMessageOut> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        // alice remove bob, so alice should konw bob's mls_group
        let (queued_msg, _welcome, _group_info) = alice_mls_group.remove_members(
            alice_provider,
            &alice_identity.signer,
            &[bob_mls_group.own_leaf_index()],
        )?;

        alice_mls_group.merge_pending_commit(alice_provider)?;

        Ok(queued_msg)
    });
    result
}

pub fn remove_members(
    alice_mls_group: &mut MlsGroup,
    members: &[LeafNodeIndex],
    alice_identity: &Identity,
    alice_provider: &OpenMlsRustPersistentCrypto,
) -> Result<MlsMessageOut> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        // alice remove bob, so alice should konw bob's mls_group
        let (queued_msg, _welcome, _group_info) =
            alice_mls_group.remove_members(alice_provider, &alice_identity.signer, &members)?;

        alice_mls_group.merge_pending_commit(alice_provider)?;

        Ok(queued_msg)
    });
    result
}

pub fn others_commit_remove_member(
    mls_group: &mut MlsGroup,
    queued_msg: MlsMessageOut,
    provider: &OpenMlsRustPersistentCrypto,
) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let processed_message = mls_group.process_message(
            provider,
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
            mls_group.merge_staged_commit(provider, *staged_commit)?;
        } else {
            // unreachable!("Expected a StagedCommit.");
            Err(anyhow::anyhow!(
                "<mls api fn[others_commit_remove_member]> Expected a StagedCommit."
            ))?;
        }

        Ok(())
    });
    result
}

pub fn self_leave(
    mls_group: &mut MlsGroup,
    identity: &Identity,
    provider: &OpenMlsRustPersistentCrypto,
) -> Result<MlsMessageOut> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let queued_msg = mls_group.leave_group(provider, &identity.signer)?;

        Ok(queued_msg)
    });
    result
}

pub fn others_proposal_leave(
    mls_group: &mut MlsGroup,
    queued_msg: MlsMessageOut,
    provider: &OpenMlsRustPersistentCrypto,
) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let processed_message = mls_group.process_message(
            provider,
            queued_msg.into_protocol_message().ok_or_else(|| {
                format_err!("<mls api fn[others_proposal_leave]> Unexpected message type")
            })?,
        )?;

        // Store proposal
        if let ProcessedMessageContent::ProposalMessage(staged_proposal) =
            processed_message.into_content()
        {
            mls_group.store_pending_proposal(&provider.storage, *staged_proposal)?;
        } else {
            unreachable!("<mls api fn[others_proposal_leave]> Expected a QueuedProposal.");
        }

        Ok(())
    });
    result
}

pub fn admin_commit_leave(
    mls_group: &mut MlsGroup,
    identity: &Identity,
    provider: &OpenMlsRustPersistentCrypto,
) -> Result<MlsMessageOut> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        let (queued_msg, _welcome_option, _group_info) =
            mls_group.commit_to_pending_proposals(provider, &identity.signer)?;

        if let Some(staged_commit) = mls_group.pending_commit() {
            let _remove = staged_commit.remove_proposals().next().ok_or_else(|| {
                format_err!("<mls api fn[admin_commit_leave]> Expected a proposal.")
            })?;
            mls_group.merge_staged_commit(provider, staged_commit.clone())?;
        } else {
            unreachable!("<mls api fn[admin_commit_leave]> Expected a StagedCommit.");
        }

        Ok(queued_msg)
    });
    result
}

// expect admin, queued_msg is from admin
pub fn normal_member_commit_leave(
    mls_group: &mut MlsGroup,
    queued_msg: MlsMessageOut,
    provider: &OpenMlsRustPersistentCrypto,
) -> Result<()> {
    let rt = lock_runtime!();
    let result = rt.block_on(async {
        // === Leave operation from normal member's perspective ===
        let processed_message = mls_group.process_message(
            provider,
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
            mls_group.merge_staged_commit(provider, *staged_commit)?;
        } else {
            unreachable!("<mls api fn[normal_member_commit_leave]> Expected a StagedCommit.");
        }

        Ok(())
    });
    result
}
