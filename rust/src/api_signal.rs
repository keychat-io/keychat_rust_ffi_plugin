pub use signal_store;

use anyhow::Result;
use lazy_static::lazy_static;
use rand::rngs::OsRng;
use signal_store::libsignal_protocol::*;
use signal_store::{KeyChatSignalProtocolStore, LitePool};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

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
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[close_signal_db]> Can not get store err."))?;
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

//bob_signed_id, bob_signed_public, bob_signed_signature, record
pub fn generate_signed_key_api(
    key_pair: KeychatIdentityKeyPair,
    signal_identity_private_key: Vec<u8>,
) -> Result<(u32, Vec<u8>, Vec<u8>, Vec<u8>)> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let bob_identity_private = PrivateKey::deserialize(&signal_identity_private_key)?;
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[generate_signed_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("generate_signed_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!(
                "<signal api fn[generate_signed_key_api]> Can not get store from store_map."
            )
        })?;

        let bob_signed_info = store
            .signed_pre_key_store
            .generate_signed_key(bob_identity_private)
            .await?;
        // bob_sign_id, pair.public_key, bob_signed_signature, record
        Ok((
            bob_signed_info.0,
            bob_signed_info.1.serialize().into(),
            bob_signed_info.2,
            bob_signed_info.3,
        ))
    });
    result
}

pub fn get_signed_key_api(key_pair: KeychatIdentityKeyPair, signed_key_id: u32) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[get_signed_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("get_signed_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[get_signed_key_api]> Can not get store from store_map.")
        })?;
        let record = store
            .signed_pre_key_store
            .get_signed_pre_key(signed_key_id.into())
            .await?;
        Ok(record.serialize()?)
    });
    result
}

pub fn store_signed_key_api(
    key_pair: KeychatIdentityKeyPair,
    signed_key_id: u32,
    record: Vec<u8>,
) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[store_signed_key_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("store_signed_key_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[store_signed_key_api]> Can not get store from store_map.")
        })?;
        let signed_key_record = SignedPreKeyRecord::deserialize(&record)?;
        store
            .signed_pre_key_store
            .save_signed_pre_key(signed_key_id.into(), &signed_key_record)
            .await?;
        Ok(())
    });
    result
}

//bob_prekey_id, bob_prekey_public, record
pub fn generate_prekey_api(key_pair: KeychatIdentityKeyPair) -> Result<(u32, Vec<u8>, Vec<u8>)> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[generate_prekey_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("generate_prekey_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[generate_prekey_api]> Can not get store from store_map.")
        })?;
        let prekey_info = store.pre_key_store.generate_pre_key().await?;
        // prekey_id, pair.public_key, record
        Ok((
            prekey_info.0,
            prekey_info.1.serialize().into(),
            prekey_info.2,
        ))
    });
    result
}

pub fn get_prekey_api(key_pair: KeychatIdentityKeyPair, prekey_id: u32) -> Result<Vec<u8>> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[get_prekey_api]> Can not get store err."))?;
        if !store.store_map.contains_key(&key_pair) {
            info!("get_prekey_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[get_prekey_api]> Can not get store from store_map.")
        })?;
        let record = store.pre_key_store.get_pre_key(prekey_id.into()).await?;
        Ok(record.serialize()?)
    });
    result
}

pub fn store_prekey_api(
    key_pair: KeychatIdentityKeyPair,
    prekey_id: u32,
    record: Vec<u8>,
) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[store_prekey_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("store_prekey_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[store_prekey_api]> Can not get store from store_map.")
        })?;
        let prekey_record = PreKeyRecord::deserialize(&record)?;
        store
            .pre_key_store
            .save_pre_key(prekey_id.into(), &prekey_record)
            .await?;
        Ok(())
    });
    result
}

pub fn process_prekey_bundle_api(
    key_pair: KeychatIdentityKeyPair,
    remote_address: KeychatProtocolAddress,
    reg_id: u32,
    device_id: u32,
    identity_key: KeychatIdentityKey,
    bob_signed_id: u32,
    bob_signed_public: Vec<u8>,
    bob_siged_sig: Vec<u8>,
    bob_prekey_id: u32,
    bob_prekey_public: Vec<u8>,
) -> Result<()> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut csprng = OsRng;
        let remote_address =
            ProtocolAddress::new(remote_address.name, remote_address.device_id.into());
        let identity_key = IdentityKey::decode(&identity_key.public_key)?;
        let mut store = STORE.lock().await;
        let store = store.as_mut().ok_or_else(|| {
            format_err!("<signal api fn[process_prekey_bundle_api]> Can not get store err.")
        })?;
        if !store.store_map.contains_key(&key_pair) {
            info!("process_prekey_bundle_api key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!(
                "<signal api fn[process_prekey_bundle_api]> Can not get store from store_map."
            )
        })?;
        let bob_signed_public = PublicKey::deserialize(&bob_signed_public)?;
        let bob_prekey_public = PublicKey::deserialize(&bob_prekey_public)?;
        let bob_prekey = Some((bob_prekey_id.into(), bob_prekey_public));
        let bob_bundle = PreKeyBundle::new(
            reg_id,
            device_id.into(),
            bob_prekey,
            bob_signed_id.into(),
            bob_signed_public,
            bob_siged_sig,
            identity_key,
        )?;
        process_prekey_bundle(
            &remote_address,
            &mut store.session_store,
            &mut store.identity_store,
            &bob_bundle,
            SystemTime::now(),
            &mut csprng,
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
) -> Result<(Vec<u8>, Option<String>, String, Option<Vec<String>>)> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[encrypt_signal]> Can not get store err."))?;
        let remote_address =
            ProtocolAddress::new(remote_address.name, remote_address.device_id.into());
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
        )
        .await?;
        // encrypt msg, my_receiver_addr, msg_keys_hash, alice_addrs_pre
        Ok((
            cipher_text.0.serialize().to_vec(),
            cipher_text.1,
            cipher_text.2,
            cipher_text.3,
        ))
    });
    result
}

pub fn parse_identity_from_prekey_signal_message(ciphertext: Vec<u8>) -> Result<(String, u32)> {
    let ciphertext = PreKeySignalMessage::try_from(ciphertext.as_ref())?;
    let identity = ciphertext.identity_key();
    let signed_pre_key_id = ciphertext.signed_pre_key_id();
    Ok((
        hex::encode(identity.public_key().serialize()),
        signed_pre_key_id.into(),
    ))
}

pub fn parse_is_prekey_signal_message(ciphertext: Vec<u8>) -> Result<bool> {
    if PreKeySignalMessage::try_from(ciphertext.as_ref()).is_ok() {
        Ok(true)
    } else if SignalMessage::try_from(ciphertext.as_ref()).is_ok() {
        Ok(false)
    } else {
        Err(anyhow::anyhow!(
            "parse_is_prekey_signal_message can not be pared"
        ))
    }
}

pub fn generate_signal_ids() -> Result<(Vec<u8>, Vec<u8>)> {
    let mut csprng = OsRng;
    let pair = KeyPair::generate(&mut csprng);
    Ok((
        pair.private_key.serialize(),
        pair.public_key.serialize().into(),
    ))
}

pub fn decrypt_signal(
    key_pair: KeychatIdentityKeyPair,
    ciphertext: Vec<u8>,
    remote_address: KeychatProtocolAddress,
    room_id: u32,
    is_prekey: bool,
) -> Result<(Vec<u8>, String, Option<Vec<String>>)> {
    let rt = RUNTIME.as_ref();
    let result = rt.block_on(async {
        let mut csprng = OsRng;
        let mut store = STORE.lock().await;
        let store = store
            .as_mut()
            .ok_or_else(|| format_err!("<signal api fn[decrypt_signal]> can not get store err."))?;
        let remote_address =
            ProtocolAddress::new(remote_address.name, remote_address.device_id.into());

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
                &mut csprng,
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
                &mut csprng,
            )
            .await?
        };
        // decrypt msg, msg_keys_hash, alice_addr_pre
        Ok((decrypt_msg.0, decrypt_msg.1, decrypt_msg.2))
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
        let address = ProtocolAddress::new(address.name, address.device_id.into());
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
            .ok_or_else(|| format_err!("<signal api fn[delete_session]> cat not get store err."))?;
        let address = ProtocolAddress::new(address.name, address.device_id.into());
        if !store.store_map.contains_key(&key_pair) {
            info!("delete_session key_pair do not init.");
            _init_keypair(store, key_pair, 0).await?;
        }
        let store = store.store_map.get_mut(&key_pair).ok_or_else(|| {
            format_err!("<signal api fn[delete_session]> cat not get store from store_map.")
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
        let address = ProtocolAddress::new(address.name, address.device_id.into());
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
