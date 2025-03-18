use anyhow::Result;
use bincode;
use kc::user::MlsUser;

use kc::group_context_extension::NostrGroupDataExtension;
pub use kc::openmls_rust_persistent_crypto::OpenMlsRustPersistentCrypto;
use kc::user::Group;
pub use openmls::group::{GroupId, Member, MlsGroup, MlsGroupCreateConfig, MlsGroupJoinConfig};
use openmls::key_packages::KeyPackage;
use openmls::prelude::tls_codec::{Deserialize, Serialize};
use openmls::prelude::{
    BasicCredential, Capabilities, Extension, Extensions, KeyPackageIn, LeafNode, LeafNodeIndex,
    LeafNodeParameters, MlsMessageIn, ProcessedMessageContent, ProtocolVersion, StagedWelcome,
    UnknownExtension,
};
pub use openmls_sqlite_storage::SqliteStorageProvider;
use openmls_traits::types::Ciphersuite;
pub use openmls_traits::OpenMlsProvider;

pub(crate) const CIPHERSUITE: Ciphersuite =
    Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;

// must be add ignore, otherwise will be error when rust to dart
#[frb(ignore)]
pub struct User {
    pub mls_user: MlsUser,
}

impl User {
    /// Create a new user with the given name and a fresh set of credentials.
    pub(crate) async fn _new(
        provider: OpenMlsRustPersistentCrypto,
        username: String,
    ) -> Result<Self> {
        let mls_user = MlsUser::new(provider, username).await?;
        Ok(Self { mls_user })
    }

    pub(crate) async fn update(&mut self, nostr_id: String, is_identity: bool) -> Result<()> {
        Ok(self.mls_user.update(nostr_id, is_identity).await?)
    }

    pub(crate) async fn load(
        provider: OpenMlsRustPersistentCrypto,
        nostr_id: String,
    ) -> Result<MlsUser> {
        Ok(MlsUser::load(provider, nostr_id).await?)
    }

    pub(crate) fn get_export_secret(&self, group_id: String) -> Result<Vec<u8>> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let export_secret =
            group
                .mls_group
                .export_secret(&self.mls_user.provider, "keychat", b"keychat", 32)?;
        Ok(export_secret)
    }

    pub(crate) fn get_tree_hash(&self, group_id: String) -> Result<Vec<u8>> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let tree_hash = group.mls_group.tree_hash().to_vec();
        Ok(tree_hash)
    }

    pub(crate) fn get_member_extension(&self, group_id: String) -> Result<Vec<LeafNode>> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let leaf_nodes = group
            .mls_group
            .leaf_nodes()
            .cloned()
            .collect::<Vec<LeafNode>>();
        Ok(leaf_nodes)
    }

    pub(crate) fn get_group_config(&self, group_id: String) -> Result<Vec<u8>> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let group_config = group.mls_group.configuration();
        let group_config_vec = bincode::serialize(&group_config)?;
        Ok(group_config_vec)
    }

    pub(crate) fn get_group_extension(&self, group_id: String) -> Result<NostrGroupDataExtension> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let extension = NostrGroupDataExtension::from_group(&group.mls_group)?;
        Ok(extension)
    }

    pub(crate) fn create_key_package(&mut self) -> Result<KeyPackage> {
        let mut identity = self
            .mls_user
            .identity
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let capabilities: Capabilities = identity.create_capabilities()?;
        let key_package =
            identity.add_key_package(CIPHERSUITE, &self.mls_user.provider, capabilities);
        Ok(key_package)
    }

    pub(crate) fn delete_key_package(&mut self, key_package: Vec<u8>) -> Result<()> {
        let kp: KeyPackage = self.parse_key_package(key_package)?;
        let mut identity = self
            .mls_user
            .identity
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        identity.delete_key_package_from_storage(kp, &self.mls_user.provider)
    }

    pub(crate) fn create_mls_group(
        &mut self,
        group_id: String,
        description: String,
        group_name: String,
        admin_pubkeys_hex: Vec<String>,
        group_relays: Vec<String>,
    ) -> Result<Vec<u8>> {
        let identity = self
            .mls_user
            .identity
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;

        let group_data = NostrGroupDataExtension::new(
            group_name.clone(),
            description,
            admin_pubkeys_hex,
            group_relays,
        );
        let serialized_group_data = group_data
            .tls_serialize_detached()
            .expect("Failed to serialize group data");

        let extensions = vec![Extension::Unknown(
            group_data.extension_type(),
            UnknownExtension(serialized_group_data),
        )];
        let capabilities: Capabilities = identity.create_capabilities()?;
        let group_create_config = MlsGroupCreateConfig::builder()
            .capabilities(capabilities)
            .use_ratchet_tree_extension(true)
            .with_group_context_extensions(
                Extensions::from_vec(extensions)
                    .expect("Couldn't convert extensions vec to Object"),
            )
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .build();

        let mls_group = MlsGroup::new_with_group_id(
            &self.mls_user.provider,
            &identity.signer,
            &group_create_config,
            GroupId::from_slice(group_id.as_bytes()),
            identity.credential_with_key.clone(),
        )?;
        let group = Group {
            mls_group: mls_group.clone(),
        };
        {
            let groups = self
                .mls_user
                .groups
                .read()
                .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;

            if groups.contains_key(&group_id) {
                panic!("Group '{}' existed already", group_id);
            }
        }

        self.mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?
            .insert(group_id.clone(), group);
        self.mls_user.group_list.insert(group_id);

        let group_config = group_create_config.join_config();
        let group_config_vec = bincode::serialize(&group_config)?;
        Ok(group_config_vec)
    }

    pub(crate) fn parse_key_package(&self, key_package_bytes: Vec<u8>) -> Result<KeyPackage> {
        let key_package_in = KeyPackageIn::tls_deserialize(&mut key_package_bytes.as_slice())
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        // Validate the signature, ciphersuite, and extensions of the key package
        let key_package = key_package_in
            .validate(self.mls_user.provider.crypto(), ProtocolVersion::Mls10)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(key_package)
    }

    pub(crate) fn add_members(
        &mut self,
        group_id: String,
        key_packages: Vec<Vec<u8>>,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        let mut kps: Vec<KeyPackage> = Vec::new();
        for kp in key_packages {
            // let kp: KeyPackage = bincode::deserialize(&kp)?;
            let kp: KeyPackage = self.parse_key_package(kp)?;
            kps.push(kp);
        }
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let (queued_msg, welcome, _) = group.mls_group.add_members(
            &self.mls_user.provider,
            &self
                .mls_user
                .identity
                .read()
                .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?
                .signer,
            &kps,
        )?;
        // split this for method self_commit
        // mls_group.merge_pending_commit(&self.provider)?;
        let serialized_queued_msg: Vec<u8> = queued_msg.to_bytes()?;
        let serialized_welcome: Vec<u8> = welcome.to_bytes()?;
        Ok((serialized_queued_msg, serialized_welcome))
    }

    pub(crate) fn get_group_members(&self, group_id: String) -> Result<Vec<String>> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };

        let mut members = group.mls_group.members();
        members.try_fold(Vec::new(), |mut vec, member| {
            let pubkey = String::from_utf8(
                BasicCredential::try_from(member.credential)
                    .map_err(|e| anyhow::anyhow!(e.to_string()))?
                    .identity()
                    .to_vec(),
            )
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            vec.push(pubkey);
            Ok(vec)
        })
    }

    pub(crate) fn self_commit(&mut self, group_id: String) -> Result<()> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        group
            .mls_group
            .merge_pending_commit(&self.mls_user.provider)?;
        Ok(())
    }

    pub(crate) fn delete_group(&mut self, group_id: String) -> Result<()> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;

        if groups.contains_key(&group_id) {
            groups.remove(&group_id);
            self.mls_user.group_list.remove(&group_id);
        }
        Ok(())
    }

    pub(crate) fn parse_welcome_message(
        &self,
        welcome: Vec<u8>,
    ) -> Result<(StagedWelcome, NostrGroupDataExtension)> {
        let mls_group_config = MlsGroupJoinConfig::builder()
            .use_ratchet_tree_extension(true)
            .build();
        let welcome = MlsMessageIn::tls_deserialize_exact(&welcome)?;
        let welcome = welcome.into_welcome().ok_or_else(|| {
            format_err!(
                "<mls api fn[join_mls_group]> expected the message to be a welcome message."
            )
        })?;
        let staged_welcome = StagedWelcome::new_from_welcome(
            &self.mls_user.provider,
            &mls_group_config,
            welcome,
            None,
        )
        .map_err(|e| {
            format_err!(
                "<mls api fn[join_mls_group]> Error creating StagedWelcome from Welcome {}.",
                e
            )
        })?;

        let nostr_group_data =
            NostrGroupDataExtension::from_group_context(staged_welcome.group_context())
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok((staged_welcome, nostr_group_data))
    }

    pub(crate) fn join_mls_group(&mut self, group_id: String, welcome: Vec<u8>) -> Result<()> {
        let (staged_welcome, _) = self.parse_welcome_message(welcome)?;

        let bob_mls_group = staged_welcome
            .into_group(&self.mls_user.provider)
            .map_err(|e| {
                format_err!(
                    "<mls api fn[join_mls_group]> Error creating group from StagedWelcome {}.",
                    e
                )
            })?;

        let group = Group {
            mls_group: bob_mls_group.clone(),
        };
        {
            let groups = self
                .mls_user
                .groups
                .read()
                .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
            if groups.contains_key(&group_id) {
                panic!("Group '{}' existed already", group_id);
            }
        }

        self.mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?
            .insert(group_id.clone(), group);
        self.mls_user.group_list.insert(group_id);

        Ok(())
    }

    // this is used for add member and update, only group is not null, and other members should execute this
    pub(crate) fn others_commit_normal(
        &mut self,
        group_id: String,
        queued_msg: Vec<u8>,
    ) -> Result<()> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let queued_msg = MlsMessageIn::tls_deserialize_exact(&queued_msg)?;
        let alice_processed_message = group.mls_group.process_message(
            &self.mls_user.provider,
            queued_msg.into_protocol_message().ok_or_else(|| {
                format_err!("<mls api fn[others_commit_normal]> Unexpected message type")
            })?,
        )?;

        if let ProcessedMessageContent::StagedCommitMessage(staged_commit) =
            alice_processed_message.0.into_content()
        {
            group
                .mls_group
                .merge_staged_commit(&self.mls_user.provider, *staged_commit)?;
        } else {
            Err(anyhow::anyhow!(
                "<mls api fn[others_commit_normal]> Expected a StagedCommit."
            ))?;
        }
        Ok(())
    }

    pub(crate) fn send_msg(
        &mut self,
        group_id: String,
        msg: String,
    ) -> Result<(Vec<u8>, Option<Vec<u8>>)> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let identity = self
            .mls_user
            .identity
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let msg_out = group
            .mls_group
            .create_message(&self.mls_user.provider, &identity.signer, msg.as_bytes())
            .map_err(|e| format_err!("<mls api fn[send_msg]> Error send message {}.", e))?;
        let serialized_msg_out: Vec<u8> = msg_out.0.to_bytes()?;
        // let export_secret =
        //     group
        //         .mls_group
        //         .export_secret(&self.provider, "keychat", b"keychat", 32)?;
        let ratchet_key = msg_out.1;
        Ok((serialized_msg_out, ratchet_key))
    }

    pub(crate) fn decrypt_msg(
        &mut self,
        group_id: String,
        msg: Vec<u8>,
    ) -> Result<(String, String, Option<Vec<u8>>)> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let msg = MlsMessageIn::tls_deserialize_exact(&msg)?;
        let processed_message = group
            .mls_group
            .process_message(
                &self.mls_user.provider,
                msg.into_protocol_message()
                    .ok_or_else(|| format_err!("Unexpected message type"))?,
            )
            .map_err(|e| format_err!("<mls api fn[decrypt_msg]> Error decrypt message {}.", e))?;
        let sender_content = String::from_utf8(
            processed_message
                .0
                .credential()
                .serialized_content()
                .to_vec(),
        )?;
        if let ProcessedMessageContent::ApplicationMessage(application_message) =
            processed_message.0.into_content()
        {
            let text = String::from_utf8(application_message.into_bytes())?;
            Ok((text, sender_content, processed_message.1))
        } else {
            Err(anyhow::anyhow!(
                "<mls api fn[decrypt_msg]> Unexpected application_message."
            ))
        }
    }

    pub(crate) fn _get_own_lead_node_index(&mut self, group_id: String) -> Result<Vec<u8>> {
        let groups = self
            .mls_user
            .groups
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let group = match groups.get(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let lead_node_index = group.mls_group.own_leaf_index();
        let lead_node_index_vec = bincode::serialize(&lead_node_index)?;
        Ok(lead_node_index_vec)
    }

    pub(crate) fn get_lead_node_index(
        &mut self,
        nostr_id_common: String,
        group_id: String,
    ) -> Result<Vec<u8>> {
        let groups = self
            .mls_user
            .groups
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let group = match groups.get(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let members = group.mls_group.members().collect::<Vec<Member>>();
        for member in members {
            let credential = member.credential.serialized_content();
            if String::from_utf8(credential.to_vec())? == nostr_id_common {
                let lead_node_index = member.index;
                return Ok(bincode::serialize(&lead_node_index)?);
            }
        }
        Err(anyhow::anyhow!("No member found with the given nostr_id."))
    }

    pub(crate) fn remove_members(
        &mut self,
        group_id: String,
        members: Vec<Vec<u8>>,
    ) -> Result<Vec<u8>> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let mut leaf_nodes: Vec<LeafNodeIndex> = Vec::new();
        for m in members {
            let m: LeafNodeIndex = bincode::deserialize(&m)?;
            leaf_nodes.push(m);
        }
        let identity = self
            .mls_user
            .identity
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        // alice remove bob, so alice should konw bob's mls_group
        let (queued_msg, _welcome, _group_info) = group.mls_group.remove_members(
            &self.mls_user.provider,
            &identity.signer,
            &leaf_nodes,
        )?;
        // split this for method self_commit
        // group.mls_group.merge_pending_commit(&self.provider)?;
        let serialized_queued_msg: Vec<u8> = queued_msg.to_bytes()?;
        Ok(serialized_queued_msg)
    }

    pub(crate) fn others_commit_remove_member(
        &mut self,
        group_id: String,
        queued_msg: Vec<u8>,
    ) -> Result<()> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let queued_msg = MlsMessageIn::tls_deserialize_exact(&queued_msg)?;
        let processed_message = group.mls_group.process_message(
            &self.mls_user.provider,
            queued_msg.into_protocol_message().ok_or_else(|| {
                format_err!("<mls api fn[others_commit_remove_member]> Unexpected message type")
            })?,
        )?;
        // Check that we receive the correct proposal
        if let ProcessedMessageContent::StagedCommitMessage(staged_commit) =
            processed_message.0.into_content()
        {
            // let _remove = staged_commit.remove_proposals().next().ok_or_else(|| {
            //     format_err!("<mls api fn[others_commit_remove_member]> Expected a proposal.")
            // })?;
            // Merge staged Commit
            group
                .mls_group
                .merge_staged_commit(&self.mls_user.provider, *staged_commit)?;
        } else {
            Err(anyhow::anyhow!(
                "<mls api fn[others_commit_remove_member]> Expected a StagedCommit."
            ))?;
        }
        Ok(())
    }

    pub(crate) fn self_leave(&mut self, group_id: String) -> Result<Vec<u8>> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let identity = self
            .mls_user
            .identity
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let queued_msg = group
            .mls_group
            .leave_group(&self.mls_user.provider, &identity.signer)?;
        let serialized_queued_msg: Vec<u8> = queued_msg.to_bytes()?;
        Ok(serialized_queued_msg)
    }

    pub(crate) fn others_proposal_leave(
        &mut self,
        group_id: String,
        queued_msg: Vec<u8>,
    ) -> Result<()> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let queued_msg = MlsMessageIn::tls_deserialize_exact(&queued_msg)?;
        let processed_message = group.mls_group.process_message(
            &self.mls_user.provider,
            queued_msg.into_protocol_message().ok_or_else(|| {
                format_err!("<mls api fn[others_proposal_leave]> Unexpected message type")
            })?,
        )?;
        // Store proposal
        if let ProcessedMessageContent::ProposalMessage(staged_proposal) =
            processed_message.0.into_content()
        {
            group
                .mls_group
                .store_pending_proposal(&self.mls_user.provider.storage, *staged_proposal)?;
        } else {
            unreachable!("<mls api fn[others_proposal_leave]> Expected a QueuedProposal.");
        }
        Ok(())
    }

    pub(crate) fn admin_commit_leave(&mut self, group_id: String) -> Result<()> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        if let Some(staged_commit) = group.mls_group.pending_commit() {
            // let _remove = staged_commit.remove_proposals().next().ok_or_else(|| {
            //     format_err!("<mls api fn[admin_commit_leave]> Expected a proposal.")
            // })?;
            let staged_commit_clone = staged_commit.clone();
            group
                .mls_group
                .merge_staged_commit(&self.mls_user.provider, staged_commit_clone)?;
        } else {
            unreachable!("<mls api fn[admin_commit_leave]> Expected a StagedCommit.");
        }
        Ok(())
    }

    pub(crate) fn admin_proposal_leave(&mut self, group_id: String) -> Result<Vec<u8>> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let identity = self
            .mls_user
            .identity
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let (queued_msg, _welcome_option, _group_info) = group
            .mls_group
            .commit_to_pending_proposals(&self.mls_user.provider, &identity.signer)?;
        // this use fn admin_commit_leave instead
        // if let Some(staged_commit) = group.mls_group.pending_commit() {
        //     let remove = staged_commit.remove_proposals().next().ok_or_else(|| {
        //         format_err!("<mls api fn[admin_commit_leave]> Expected a proposal.")
        //     })?;
        //     let staged_commit_clone = staged_commit.clone();
        //     group
        //         .mls_group
        //         .merge_staged_commit(&self.provider, staged_commit_clone)?;
        // } else {
        //     unreachable!("<mls api fn[admin_commit_leave]> Expected a StagedCommit.");
        // }
        let serialized_queued_msg: Vec<u8> = queued_msg.to_bytes()?;
        Ok(serialized_queued_msg)
    }

    // expect admin, queued_msg is from admin
    pub(crate) fn normal_member_commit_leave(
        &mut self,
        group_id: String,
        queued_msg: Vec<u8>,
    ) -> Result<()> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let queued_msg = MlsMessageIn::tls_deserialize_exact(&queued_msg)?;
        // === Leave operation from normal member's perspective ===
        let processed_message = group.mls_group.process_message(
            &self.mls_user.provider,
            queued_msg.into_protocol_message().ok_or_else(|| {
                format_err!("<mls api fn[normal_member_commit_leave]> Unexpected message type")
            })?,
        )?;
        // Check that we received the correct proposals
        if let ProcessedMessageContent::StagedCommitMessage(staged_commit) =
            processed_message.0.into_content()
        {
            // let _remove = staged_commit.remove_proposals().next().ok_or_else(|| {
            //     format_err!("<mls api fn[normal_member_commit_leave]> Expected a proposal.")
            // })?;
            // Merge staged Commit
            group
                .mls_group
                .merge_staged_commit(&self.mls_user.provider, *staged_commit)?;
        } else {
            unreachable!("<mls api fn[normal_member_commit_leave]> Expected a StagedCommit.");
        }
        Ok(())
    }

    // self can update it own info, like name description and so on.
    pub(crate) fn self_update(&mut self, group_id: String, extensions: Vec<u8>) -> Result<Vec<u8>> {
        const UNKNOWN_EXTENSION_TYPE: u16 = 0xF233;
        let extension = Extension::Unknown(UNKNOWN_EXTENSION_TYPE, UnknownExtension(extensions));
        let leaf_extensions = Extensions::single(extension.clone());
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let identity = self
            .mls_user
            .identity
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock"))?;
        let commit_message_bundle = group.mls_group.self_update(
            &self.mls_user.provider,
            &identity.signer,
            // LeafNodeParameters::default(),
            LeafNodeParameters::builder()
                .with_extensions(leaf_extensions)
                .build(),
        )?;
        let queued_msg = commit_message_bundle.commit();
        let serialized_queued_msg: Vec<u8> = queued_msg.to_bytes()?;
        Ok(serialized_queued_msg)
    }

    pub(crate) fn get_sender(&self, group_id: String, queued_msg: Vec<u8>) -> Result<Vec<u8>> {
        let mut groups = self
            .mls_user
            .groups
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        let group = match groups.get_mut(&group_id) {
            Some(g) => g,
            _ => return Err(anyhow::anyhow!("No group with name {} known.", group_id)),
        };
        let msg = MlsMessageIn::tls_deserialize_exact(&queued_msg)?;
        let processed_message = group
            .mls_group
            .process_message(
                &self.mls_user.provider,
                msg.into_protocol_message()
                    .ok_or_else(|| format_err!("Unexpected message type"))?,
            )
            .map_err(|e| format_err!("<mls api fn[get_sender]> Error decrypt message {}.", e))?;
        let sender = processed_message
            .0
            .credential()
            .serialized_content()
            .to_vec();
        Ok(sender)
    }
}
