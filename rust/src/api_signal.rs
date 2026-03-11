pub use signal_store;

#[path = "api_signal.types.rs"]
pub mod types;
pub use types::*;

use anyhow::Result;
use lazy_static::lazy_static;
use signal_store::libsignal_protocol::*;
use signal_store::{KeyChatSignalProtocolStore, LitePool};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

pub struct SignalStore {
    pub pool: LitePool,
    pub store_map: HashMap<KeychatIdentityKeyPair, KeyChatSignalProtocolStore>,
}

lazy_static! {
    static ref STORE: Mutex<Option<SignalStore>> = Mutex::new(None);
}

lazy_static! {
    static ref RUNTIME: Arc<Runtime> =
        Arc::new(Runtime::new().expect("failed to create tokio runtime for signal"));
}

/// init db and KeyChatSignalProtocolStore, this is used for testing
pub fn init(db_path: String, key_pair: KeychatIdentityKeyPair, reg_id: u32) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let key_pair_2: IdentityKeyPair = IdentityKeyPair::new(
        IdentityKey::decode(&key_pair.identity_key)?,
        PrivateKey::deserialize(&key_pair.private_key)?,
    );

    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        if store.is_none() {
            let pool = LitePool::open(&db_path, Default::default()).await?;
            *store = Some(SignalStore {
                pool,
                store_map: HashMap::new(),
            });
            error!("store has not been inited.");
        }

        let map = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[init]> Can not get store err."))?;

        let keychat_store = KeyChatSignalProtocolStore::new(map.pool.clone(), key_pair_2, reg_id)?;
        map.store_map.entry(key_pair).or_insert(keychat_store);

        Ok(())
    });
    result
}

/// init db
pub fn init_signal_db(db_path: String) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let pool = LitePool::open(&db_path, Default::default()).await?;
        let mut store = STORE.lock().await;
        *store = Some(SignalStore {
            store_map: HashMap::new(),
            pool,
        });
        Ok(())
    });
    result
}

/// close db
pub fn close_signal_db() -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[close_signal_db]> Can not get store err.")
        })?;
        store.pool.database().close();
        Ok(())
    });
    result
}

/// init KeyChatSignalProtocolStore
async fn _init_keypair(
    store: &mut SignalStore,
    key_pair: KeychatIdentityKeyPair,
    reg_id: u32,
) -> Result<()> {
    let key_pair_2: IdentityKeyPair = IdentityKeyPair::new(
        IdentityKey::decode(&key_pair.identity_key)?,
        PrivateKey::deserialize(&key_pair.private_key)?,
    );
    let keychat_store = KeyChatSignalProtocolStore::new(store.pool.clone(), key_pair_2, reg_id)?;

    store.store_map.entry(key_pair).or_insert_with(|| {
        info!("fn[_init_keypair] key_pair do not init.");
        keychat_store
    });

    Ok(())
}

/// init KeyChatSignalProtocolStore
pub fn init_keypair(key_pair: KeychatIdentityKeyPair, reg_id: u32) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[init_keypair]> Can not get store err."))?;
        _init_keypair(store, key_pair, reg_id).await?;

        Ok(())
    });
    result
}

pub fn generate_signed_pre_key_api(
    key_pair: KeychatIdentityKeyPair,
    signal_identity_private_key: Vec<u8>,
) -> Result<SignedPreKeyResult> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let bob_identity_private = PrivateKey::deserialize(&signal_identity_private_key)?;
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[generate_signed_pre_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("generate_signed_pre_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!(
                "<signal api fn[generate_signed_pre_key_api]> Can not get store from store_map."
            )
        })?;

        let bob_signed_info = store
            .signed_pre_key_store
            .generate_signed_key(bob_identity_private)
            .await?;
        Ok(SignedPreKeyResult {
            signed_pre_key_id: bob_signed_info.0,
            signed_pre_key_public: bob_signed_info.1.serialize().into(),
            signed_pre_key_signature: bob_signed_info.2,
            signed_pre_key_record: bob_signed_info.3,
        })
    });
    result
}

pub fn get_signed_pre_key_api(
    key_pair: KeychatIdentityKeyPair,
    signed_pre_key_id: u32,
) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[get_signed_pre_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("get_signed_pre_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[get_signed_pre_key_api]> Can not get store from store_map.")
        })?;
        let record = store
            .signed_pre_key_store
            .get_signed_pre_key(signed_pre_key_id.into())
            .await?;
        Ok(record.serialize()?)
    });
    result
}

pub fn store_signed_pre_key_api(
    key_pair: KeychatIdentityKeyPair,
    signed_pre_key_id: u32,
    signed_pre_key_record: Vec<u8>,
) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[store_signed_pre_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("store_signed_pre_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!(
                "<signal api fn[store_signed_pre_key_api]> Can not get store from store_map."
            )
        })?;
        let signed_pre_key_record = SignedPreKeyRecord::deserialize(&signed_pre_key_record)?;
        store
            .signed_pre_key_store
            .save_signed_pre_key(signed_pre_key_id.into(), &signed_pre_key_record)
            .await?;
        Ok(())
    });
    result
}

pub fn generate_kyber_pre_key_api(
    key_pair: KeychatIdentityKeyPair,
    signal_identity_private_key: Vec<u8>,
) -> Result<KyberPreKeyResult> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let bob_identity_private = PrivateKey::deserialize(&signal_identity_private_key)?;
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[generate_kyber_pre_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("generate_kyber_pre_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!(
                "<signal api fn[generate_kyber_pre_key_api]> Can not get store from store_map."
            )
        })?;

        let bob_kyber_info = store
            .kyber_pre_key_store
            .generate_kyber_pre_key(bob_identity_private)
            .await?;
        Ok(KyberPreKeyResult {
            kyber_pre_key_id: bob_kyber_info.0,
            kyber_pre_key_public: bob_kyber_info.1.serialize().into(),
            kyber_pre_key_signature: bob_kyber_info.2,
            kyber_pre_key_record: bob_kyber_info.3,
        })
    });
    result
}

pub fn get_kyber_pre_key_api(
    key_pair: KeychatIdentityKeyPair,
    kyber_pre_key_id: u32,
) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[get_kyber_pre_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("get_kyber_pre_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[get_kyber_pre_key_api]> Can not get store from store_map.")
        })?;
        let record = store
            .kyber_pre_key_store
            .get_kyber_pre_key(kyber_pre_key_id.into())
            .await?;
        Ok(record.serialize()?)
    });
    result
}

pub fn store_kyber_pre_key_api(
    key_pair: KeychatIdentityKeyPair,
    kyber_pre_key_id: u32,
    kyber_pre_key_record: Vec<u8>,
) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[store_kyber_pre_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("store_kyber_pre_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!(
                "<signal api fn[store_kyber_pre_key_api]> Can not get store from store_map."
            )
        })?;
        let kyber_pre_key_record = KyberPreKeyRecord::deserialize(&kyber_pre_key_record)?;
        store
            .kyber_pre_key_store
            .save_kyber_pre_key(kyber_pre_key_id.into(), &kyber_pre_key_record)
            .await?;
        Ok(())
    });
    result
}

pub fn generate_pre_key_api(key_pair: KeychatIdentityKeyPair) -> Result<PreKeyResult> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[generate_pre_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("generate_pre_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[generate_pre_key_api]> Can not get store from store_map.")
        })?;
        let prekey_info = store.pre_key_store.generate_pre_key().await?;
        Ok(PreKeyResult {
            pre_key_id: prekey_info.0,
            pre_key_public: prekey_info.1.serialize().into(),
            pre_key_record: prekey_info.2,
        })
    });
    result
}

pub fn get_pre_key_api(key_pair: KeychatIdentityKeyPair, pre_key_id: u32) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[get_pre_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("get_pre_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[get_pre_key_api]> Can not get store from store_map.")
        })?;
        let record = store.pre_key_store.get_pre_key(pre_key_id.into()).await?;
        Ok(record.serialize()?)
    });
    result
}

pub fn store_pre_key_api(
    key_pair: KeychatIdentityKeyPair,
    pre_key_id: u32,
    pre_key_record: Vec<u8>,
) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[store_pre_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("store_pre_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[store_pre_key_api]> Can not get store from store_map.")
        })?;
        let pre_key_record = PreKeyRecord::deserialize(&pre_key_record)?;
        store
            .pre_key_store
            .save_pre_key(pre_key_id.into(), &pre_key_record)
            .await?;
        Ok(())
    });
    result
}

pub fn process_pre_key_bundle_api(
    key_pair: KeychatIdentityKeyPair,
    remote_address: KeychatProtocolAddress,
    registration_id: u32,
    device_id: u32,
    identity_key: KeychatIdentityKey,
    signed_pre_key_id: u32,
    signed_pre_key_public: Vec<u8>,
    signed_pre_key_signature: Vec<u8>,
    kyber_pre_key_id: u32,
    kyber_pre_key_public: Vec<u8>,
    kyber_pre_key_signature: Vec<u8>,
    pre_key_id: u32,
    pre_key_public: Vec<u8>,
) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let remote_device_id = DeviceId::try_from(remote_address.device_id)?;
        let remote_address = ProtocolAddress::new(remote_address.name, remote_device_id);
        let identity_key = IdentityKey::decode(&identity_key.public_key)?;
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[process_pre_key_bundle_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("process_pre_key_bundle_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!(
                "<signal api fn[process_pre_key_bundle_api]> Can not get store from store_map."
            )
        })?;
        let signed_pre_key_public = PublicKey::deserialize(&signed_pre_key_public)?;
        let pre_key_public = PublicKey::deserialize(&pre_key_public)?;
        let pre_key = Some((pre_key_id.into(), pre_key_public));

        let kyber_pre_key_public = kem::PublicKey::deserialize(&kyber_pre_key_public)?;

        let bob_bundle = PreKeyBundle::new(
            registration_id,
            DeviceId::try_from(device_id)?,
            pre_key,
            signed_pre_key_id.into(),
            signed_pre_key_public,
            signed_pre_key_signature.clone(),
            kyber_pre_key_id.into(),
            kyber_pre_key_public,
            kyber_pre_key_signature.clone(),
            identity_key,
        )?;
        process_prekey_bundle(
            &remote_address,
            &mut store.session_store,
            &mut store.identity_store,
            &bob_bundle,
            SystemTime::now(),
            &mut rand::rng(),
        )
        .await?;
        Ok(())
    });
    result
}

pub fn encrypt_signal(
    key_pair: KeychatIdentityKeyPair,
    ptext: String,
    remote_address: KeychatProtocolAddress,
    is_prekey: Option<bool>,
) -> Result<EncryptResult> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[encrypt_signal]> Can not get store err."))?;
        let remote_device_id = DeviceId::try_from(remote_address.device_id)?;
        let remote_address = ProtocolAddress::new(remote_address.name, remote_device_id);
        if !store.store_map.contains_key(&key_pair) {
            info!("encrypt_signal key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[encrypt_signal]> Can not get store from store_map.")
        })?;

        let cipher_text = message_encrypt(
            ptext.as_bytes(),
            &remote_address,
            &mut store.session_store,
            &mut store.identity_store,
            SystemTime::now(),
            is_prekey,
            &mut rand::rng(),
        )
        .await?;
        Ok(EncryptResult {
            ciphertext: cipher_text.0.serialize().to_vec(),
            receiver_address: cipher_text.1,
            message_keys_hash: cipher_text.2,
            sender_addresses: cipher_text.3,
        })
    });
    result
}

pub fn parse_identity_from_prekey_signal_message(
    ciphertext: Vec<u8>,
) -> Result<PreKeySignalMessageInfo> {
    let ciphertext = PreKeySignalMessage::try_from(ciphertext.as_ref())?;
    let identity = ciphertext.identity_key();
    let signed_pre_key_id = ciphertext.signed_pre_key_id();
    Ok(PreKeySignalMessageInfo {
        identity_key: hex::encode(identity.public_key().serialize()),
        signed_pre_key_id: signed_pre_key_id.into(),
    })
}

pub fn parse_is_prekey_signal_message(ciphertext: Vec<u8>) -> Result<bool> {
    if PreKeySignalMessage::try_from(ciphertext.as_ref()).is_ok() {
        Ok(true)
    } else if SignalMessage::try_from(ciphertext.as_ref()).is_ok() {
        Ok(false)
    } else {
        Err(anyhow::anyhow!(
            "parse_is_prekey_signal_message can not be parsed"
        ))
    }
}

pub fn generate_signal_ids() -> Result<SignalKeyPair> {
    let pair = KeyPair::generate(&mut rand::rng());
    Ok(SignalKeyPair {
        private_key: pair.private_key.serialize(),
        public_key: pair.public_key.serialize().into(),
    })
}

pub fn decrypt_signal(
    key_pair: KeychatIdentityKeyPair,
    ciphertext: Vec<u8>,
    remote_address: KeychatProtocolAddress,
    room_id: u32,
    is_prekey: bool,
) -> Result<DecryptResult> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[decrypt_signal]> can not get store err."))?;
        let remote_device_id = DeviceId::try_from(remote_address.device_id)?;
        let remote_address = ProtocolAddress::new(remote_address.name, remote_device_id);

        if !store.store_map.contains_key(&key_pair) {
            info!("decrypt_signal key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[decrypt_signal]> can not get store from store_map.")
        })?;
        let decrypt_msg = if is_prekey {
            let ciphertext = PreKeySignalMessage::try_from(ciphertext.as_ref())?;
            message_decrypt_prekey(
                &ciphertext,
                &remote_address,
                &mut store.session_store,
                &mut store.identity_store,
                &mut store.ratchet_key_store,
                &mut store.pre_key_store,
                &mut store.signed_pre_key_store,
                &mut store.kyber_pre_key_store,
                room_id,
                &mut rand::rng(),
            )
            .await?
        } else {
            let ciphertext = SignalMessage::try_from(ciphertext.as_ref())?;
            message_decrypt_signal(
                &ciphertext,
                &remote_address,
                &mut store.session_store,
                &mut store.identity_store,
                &mut store.ratchet_key_store,
                room_id,
                &mut rand::rng(),
            )
            .await?
        };
        Ok(DecryptResult {
            plaintext: decrypt_msg.0,
            message_keys_hash: decrypt_msg.1,
            sender_addresses: decrypt_msg.2,
        })
    });
    result
}

// do not used again
pub fn session_contain_alice_addr(
    key_pair: KeychatIdentityKeyPair,
    address: String,
) -> Result<Option<KeychatSignalSession>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[session_contain_alice_addr]> can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("session_contain_alice_addr key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!(
                "<signal api fn[session_contain_alice_addr]> can not get store from store_map."
            )
        })?;
        let session = store
            .session_store
            .session_contain_alice_addr(&address)
            .await?;
        let data = match session {
            Some(ss) => Ok(Some(KeychatSignalSession {
                alice_sender_ratchet_key: ss.alice_sender_ratchet_key,
                address: ss.address,
                device: ss.device,
                bob_sender_ratchet_key: ss.bob_sender_ratchet_key,
                record: ss.record,
                bob_address: ss.bob_address,
                alice_addresses: ss.alice_addresses,
            })),
            None => Ok(None),
        };
        data
    });
    result
}

// do not used again
pub fn update_alice_addr(
    key_pair: KeychatIdentityKeyPair,
    address: String,
    device_id: String,
    alice_addr: String,
) -> Result<bool> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[update_alice_addr]> can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("update_alice_addr key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[update_alice_addr]> can not get store from store_map.")
        })?;
        let flag = store
            .session_store
            .update_alice_addr(&address, &device_id, &alice_addr)
            .await?;
        Ok(flag)
    });
    result
}

pub fn contains_session(
    key_pair: KeychatIdentityKeyPair,
    address: KeychatProtocolAddress,
) -> Result<bool> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[contains_session]> can not get store err.")
        })?;
        let device_id = DeviceId::try_from(address.device_id)?;
        let address = ProtocolAddress::new(address.name, device_id);
        if !store.store_map.contains_key(&key_pair) {
            info!("contains_session key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[contains_session]> can not get store from store_map.")
        })?;
        let flag = store.session_store.contains_session(&address).await?;
        Ok(flag)
    });
    result
}

pub fn delete_session_by_device_id(
    key_pair: KeychatIdentityKeyPair,
    device_id: u32,
) -> Result<bool> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[delete_session_by_device_id]> can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("delete_session_by_device_id key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!(
                "<signal api fn[delete_session_by_device_id]> can not get store from store_map."
            )
        })?;
        let flag = store
            .session_store
            .delete_session_by_device_id(device_id)
            .await?;
        Ok(flag)
    });
    result
}

pub fn delete_session(
    key_pair: KeychatIdentityKeyPair,
    address: KeychatProtocolAddress,
) -> Result<bool> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[delete_session]> can not get store err."))?;
        let device_id = DeviceId::try_from(address.device_id)?;
        let address = ProtocolAddress::new(address.name, device_id);
        if !store.store_map.contains_key(&key_pair) {
            info!("delete_session key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[delete_session]> can not get store from store_map.")
        })?;
        let del = store.session_store.delete_session(&address).await?;
        Ok(del)
    });
    result
}

// do not used again
pub fn get_all_alice_addrs(key_pair: KeychatIdentityKeyPair) -> Result<Vec<String>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[get_all_alice_addrs]> can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("get_all_alice_addrs key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[get_all_alice_addrs]> can not get store from store_map.")
        })?;
        let alice_addrs = store.session_store.get_all_alice_addrs().await?;
        Ok(alice_addrs)
    });
    result
}

pub fn get_session(
    key_pair: KeychatIdentityKeyPair,
    address: String,
    device_id: String,
) -> Result<Option<KeychatSignalSession>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[get_session]> can not get store err."))?;
        if !store.store_map.contains_key(&key_pair) {
            info!("get_session key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[get_session]> can not get store from store_map.")
        })?;
        let session = store
            .session_store
            .get_session(&address, &device_id)
            .await?;
        let result_ss = match session {
            Some(ss) => Ok(Some(KeychatSignalSession {
                alice_sender_ratchet_key: ss.alice_sender_ratchet_key,
                address: ss.address,
                device: ss.device,
                bob_sender_ratchet_key: ss.bob_sender_ratchet_key,
                record: ss.record,
                bob_address: ss.bob_address,
                alice_addresses: ss.alice_addresses,
            })),
            None => Ok(None),
        };
        result_ss
    });
    result
}

/**
 * IdentityStore function
 */

pub fn delete_identity(key_pair: KeychatIdentityKeyPair, address: String) -> Result<bool> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[delete_identity]> can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("delete_identity key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[delete_identity]> can not get store from store_map.")
        })?;
        let flag = store.identity_store.delete_identity(&address).await?;
        Ok(flag)
    });
    result
}

pub fn get_identity(
    key_pair: KeychatIdentityKeyPair,
    address: KeychatProtocolAddress,
) -> Result<Option<KeychatIdentityKey>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[get_identity]> can not get store."))?;
        let device_id = DeviceId::try_from(address.device_id)?;
        let address = ProtocolAddress::new(address.name, device_id);
        if !store.store_map.contains_key(&key_pair) {
            info!("get_identity key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[get_identity]> can not get store from store_map.")
        })?;
        let identity = store.identity_store.get_identity(&address).await?;
        if identity.is_none() {
            return Ok(None);
        }
        Ok(Some(KeychatIdentityKey {
            public_key: identity
                .unwrap()
                .public_key()
                .serialize()
                .to_vec()
                .try_into()
                .map_err(|_| {
                    format_err!("<signal api fn[get_identity]> public_key [u8] to [u8:33] err.")
                })?,
        }))
    });
    result
}
