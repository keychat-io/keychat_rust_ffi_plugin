use bip39::rand_core::OsRng;
use nostr::bitcoin::secp256k1::Secp256k1;
use nostr::nips::nip04;
use nostr::nips::nip06::FromMnemonic;
use nostr::nips::nip19::{FromBech32, ToBech32};
use nostr::secp256k1::hashes::{sha256, Hash};
use nostr::secp256k1::schnorr::Signature as SchnorrSignature;
use nostr::secp256k1::Message;
use nostr::secp256k1::PublicKey as PB256;
use nostr::{EventBuilder, EventId, JsonUtil, Keys, Kind, PublicKey, SecretKey, Tag};
use serde::Serialize;
use signal_store::libsignal_protocol::{PrivateKey, PublicKey as PB};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secp256k1Account {
    pub mnemonic: Option<String>,
    pub pubkey: String,
    pub prikey: String,
    pub pubkey_bech32: String,
    pub prikey_bech32: String,
    pub curve25519_sk: Option<Vec<u8>>,
    pub curve25519_pk: Option<Vec<u8>>,
    pub curve25519_sk_hex: Option<String>,
    pub curve25519_pk_hex: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secp256k1SimpleAccount {
    pub pubkey: String,
    pub prikey: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NostrEvent {
    /// Id
    pub id: String,
    /// Author
    pub pubkey: String,
    /// Timestamp (seconds)
    pub created_at: u64,
    /// Kind
    pub kind: u64,
    /// Vector of [`Tag`]
    pub tags: Vec<Vec<String>>,
    /// Content
    pub content: String,
    /// Signature
    pub sig: String,
}

pub fn generate_secp256k1() -> anyhow::Result<Secp256k1Account> {
    let keys = Keys::generate();
    let public_key = keys.public_key();
    let secret_key = keys.secret_key()?;

    let result = Secp256k1Account {
        pubkey: public_key.to_string(),
        prikey: keys.secret_key()?.display_secret().to_string(),
        pubkey_bech32: public_key.to_bech32()?,
        prikey_bech32: secret_key.to_bech32()?,
        mnemonic: None,
        curve25519_sk: None,
        curve25519_pk: None,
        curve25519_sk_hex: None,
        curve25519_pk_hex: None,
    };
    Ok(result)
}

pub fn generate_from_mnemonic() -> anyhow::Result<Secp256k1Account> {
    let mnemonic = bip39::Mnemonic::generate(12)?;

    let mnemonic_words = mnemonic.to_string();
    let res = import_from_phrase(mnemonic_words);
    res
}

pub fn generate_simple() -> anyhow::Result<Secp256k1SimpleAccount> {
    let keys = Keys::generate();
    let public_key = keys.public_key();
    let result: Secp256k1SimpleAccount = Secp256k1SimpleAccount {
        pubkey: public_key.to_string(),
        prikey: keys.secret_key()?.display_secret().to_string(),
    };
    Ok(result)
}

pub fn import_key(sender_keys: String) -> anyhow::Result<Secp256k1Account> {
    let keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let secret_key = keys.secret_key()?;
    let public_key = keys.public_key();

    let result = Secp256k1Account {
        pubkey: public_key.to_string(),
        prikey: sender_keys,
        pubkey_bech32: public_key.to_bech32()?,
        prikey_bech32: secret_key.to_bech32()?,
        mnemonic: None,
        curve25519_sk: None,
        curve25519_pk: None,
        curve25519_sk_hex: None,
        curve25519_pk_hex: None,
    };
    Ok(result)
}

// import from nmernonic
pub fn import_from_phrase(phrase: String) -> anyhow::Result<Secp256k1Account> {
    let keys: Keys = Keys::from_mnemonic(&phrase, None)?;
    let public_key = keys.public_key();
    let secret_key = keys.secret_key()?;
    let (signing_key, verifying_key) = generate_curve25519_keypair(phrase.clone())?;

    let result = Secp256k1Account {
        mnemonic: Some(phrase.to_string()),
        pubkey: public_key.to_string(),
        prikey: keys.secret_key()?.display_secret().to_string(),
        pubkey_bech32: public_key.to_bech32()?,
        prikey_bech32: secret_key.to_bech32()?,
        curve25519_sk_hex: Some(hex::encode(&signing_key)),
        curve25519_pk_hex: Some(hex::encode(&verifying_key)),
        curve25519_sk: Some(signing_key.to_vec()),
        curve25519_pk: Some(verifying_key.to_vec()),
    };
    Ok(result)
}

#[frb(sync)]
pub fn get_hex_pubkey_by_bech32(bech32: String) -> String {
    if !bech32.starts_with("npub") {
        return bech32;
    }
    let public_key = PublicKey::from_bech32(bech32).expect("bech32 to public key error");
    let result = public_key.to_string();
    result
}

#[frb(sync)]
pub fn get_bech32_pubkey_by_hex(hex: String) -> String {
    if hex.starts_with("npub") {
        return hex;
    }
    let pubkey = get_xonly_pubkey(hex).expect("get_xonly_pubkey from hex error");
    pubkey.to_bech32().expect("public key to bech32 error")
}

#[frb(sync)]
pub fn get_hex_prikey_by_bech32(bech32: String) -> String {
    if !bech32.starts_with("nsec") {
        return bech32;
    }
    let key = SecretKey::from_bech32(bech32).expect("bech32 to secret key error");
    let result = key.display_secret().to_string();
    result
}

pub fn get_hex_pubkey_by_prikey(prikey: String) -> anyhow::Result<String> {
    let keys: Keys = nostr::Keys::parse(prikey)?;
    let public_key = keys.public_key();
    Ok(public_key.to_string())
}

// encrypt message and return event string
pub fn get_encrypt_event(
    sender_keys: String,
    receiver_pubkey: String,
    content: String,
    reply: Option<String>,
) -> anyhow::Result<String> {
    let alice_keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let pubkey = get_xonly_pubkey(receiver_pubkey)?;
    let mut _reply = None;

    if let Some(reply) = reply {
        _reply = Some(EventId::from_hex(reply)?);
    }

    let alice_encrypted_msg =
        EventBuilder::encrypted_direct_msg(&alice_keys, pubkey, content, _reply)?
            .to_event(&alice_keys)?;
    Ok(alice_encrypted_msg.as_json())
}

// generate event, but do not encrypt content
pub fn get_unencrypt_event(
    sender_keys: String,
    receiver_pubkey: String,
    content: String,
    reply: Option<String>,
) -> anyhow::Result<String> {
    let pubkey = get_xonly_pubkey(receiver_pubkey)?;

    let mut tags: Vec<Tag> = vec![Tag::PublicKey {
        public_key: pubkey,
        relay_url: None,
        alias: None,
        uppercase: false,
    }];
    if let Some(reply) = reply {
        tags.push(Tag::Event {
            event_id: EventId::from_hex(reply)?,
            relay_url: None,
            marker: None,
        });
    }
    let alice_keys: Keys = nostr::Keys::parse(&sender_keys)?;

    let event =
        EventBuilder::new(Kind::EncryptedDirectMessage, content, tags).to_event(&alice_keys)?;
    Ok(event.as_json())
}

// encrypt message
pub fn encrypt(
    sender_keys: String,
    receiver_pubkey: String,
    content: String,
) -> anyhow::Result<String> {
    let alice_keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let pubkey = get_xonly_pubkey(receiver_pubkey)?;
    let result = nip04::encrypt(alice_keys.secret_key()?, &pubkey, content)?;
    Ok(result)
}

pub fn decrypt(
    sender_keys: String,
    receiver_pubkey: String,
    content: String,
) -> anyhow::Result<String> {
    let alice_keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let pubkey = get_xonly_pubkey(receiver_pubkey)?;
    let result = nip04::decrypt(alice_keys.secret_key()?, &pubkey, content)?;
    Ok(result)
}

pub fn set_metadata(sender_keys: String, content: String) -> anyhow::Result<String> {
    let alice_keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let tags = [];
    let event = EventBuilder::new(nostr::Kind::Metadata, content, tags);
    let result = event.to_event(&alice_keys)?.as_json();
    Ok(result)
}

pub fn decrypt_event(sender_keys: String, json: String) -> anyhow::Result<String> {
    let event = nostr::event::Event::from_json(json)?;
    let pubkey = event
        .tags
        .first()
        .ok_or_else(|| format_err!("empty tags"))?
        .as_vec();

    let decrypted_string = decrypt(
        sender_keys,
        pubkey[1].to_string(),
        event.content.to_string(),
    );
    decrypted_string
}

pub fn verify_event(json: String) -> anyhow::Result<NostrEvent> {
    let event: nostr::Event = nostr::Event::from_json(json)?;
    let ne: NostrEvent = NostrEvent {
        id: event.id.to_string(),
        content: event.content.clone(),
        tags: vec![event
            .tags
            .first()
            .expect("get event tags first error")
            .as_vec()],
        created_at: event.created_at.as_u64(),
        kind: event.kind.as_u64(),
        sig: serde_json::to_string(&event.sig)?,
        pubkey: event.pubkey.to_string(),
    };
    Ok(ne)
}

fn get_xonly_pubkey(pubkey: String) -> anyhow::Result<PublicKey> {
    let bob_pubkey: PublicKey = pubkey.parse()?;
    Ok(bob_pubkey)
}

// sign schnorr
pub fn sign_schnorr(sender_keys: String, content: String) -> anyhow::Result<String> {
    let sk: Keys = nostr::Keys::parse(sender_keys)?;
    let secp = Secp256k1::new();
    let message = Message::from(sha256::Hash::hash(content.as_bytes()));
    let sig: SchnorrSignature = secp.sign_schnorr(&message, &sk.key_pair(&secp)?);
    Ok(sig.to_string())
}

// type Aes128Cbc = Cbc<Aes128, Pkcs7>;
// use aes::Aes128;
// use base64::engine::general_purpose;
// use base64::{self, Engine};

// use block_modes::block_padding::Pkcs7;
// use block_modes::{BlockMode, Cbc};
// pub fn aes_encrypt(content: &str, key: &str, iv: &str) -> String {
//     let cipher = Aes128Cbc::new_from_slices(key.as_bytes(), iv.as_bytes()).unwrap();
//     let cipher_text = cipher.encrypt_vec(content.as_bytes());
//     general_purpose::STANDARD.encode(&cipher_text)
// }

// pub fn aes_decrypt(cipher_text: &str, key: &str, iv: &str) -> String {
//     let cipher_text = general_purpose::STANDARD.decode(cipher_text).unwrap();
//     let cipher = Aes128Cbc::new_from_slices(key.as_bytes(), iv.as_bytes()).unwrap();
//     let decrypted_text = cipher.decrypt_vec(&cipher_text).unwrap();
//     String::from_utf8(decrypted_text).unwrap()
// }

// pub fn aes_encrypt_bytes(content: Vec<u8>, key: &str, iv: &str) -> Vec<u8> {
//     let cipher = Aes128Cbc::new_from_slices(key.as_bytes(), iv.as_bytes()).unwrap();
//     cipher.encrypt_vec(content.as_slice())
// }

// pub fn aes_decrypt_bytes(cipher_text: Vec<u8>, key: &str, iv: &str) -> Vec<u8> {
//     let cipher = Aes128Cbc::new_from_slices(key.as_bytes(), iv.as_bytes()).unwrap();
//     cipher.decrypt_vec(&cipher_text).unwrap()
// }

// verify sign
pub fn verify_schnorr(
    pubkey: String,
    sig: String,
    content: String,
    hash: bool,
) -> anyhow::Result<bool> {
    let pk = get_xonly_pubkey(pubkey)?;
    let sig = sig.parse()?;

    let secp = Secp256k1::new();

    let message = if hash {
        Message::from_hashed_data::<sha256::Hash>(content.as_bytes())
    } else {
        let message = hex::decode(&content)?;
        Message::from_slice(message.as_ref())?
    };

    // println!("{:?}", secp.verify_schnorr(&sig, &message, &pk));
    let _result: () = secp.verify_schnorr(&sig, &message, &pk)?;
    Ok(true)
}

pub fn generate_curve25519_keypair(
    mnemonic_words: String,
) -> Result<(Vec<u8>, Vec<u8>), anyhow::Error> {
    let mnemonic = bip39::Mnemonic::parse_in_normalized(bip39::Language::English, &mnemonic_words)?;
    let seed = mnemonic.to_seed("");
    use bitcoin::bip32::{DerivationPath, ExtendedPrivKey as Xpriv};
    let root_key = Xpriv::new_master(bitcoin::Network::Bitcoin, &seed)?;
    // let path = DerivationPath::from_str("m/44'/1237'/0'/0/0")?;
    let path: DerivationPath = "m/44'/1238'/0'/0/0".parse()?;
    let ctx = bitcoin::key::Secp256k1::new();
    let child_xprv = root_key.derive_priv(&ctx, &path)?;

    let private_key = PrivateKey::deserialize(&child_xprv.private_key.secret_bytes())?;
    let public_key = private_key.public_key()?;
    // len is 32 33
    Ok((private_key.serialize(), public_key.serialize().into()))
}

pub fn curve25519_sign(secret_key: Vec<u8>, message: Vec<u8>) -> Result<String, anyhow::Error> {
    let signing_key = PrivateKey::deserialize(&secret_key)?;
    let sig = signing_key.calculate_signature(&message, &mut OsRng)?;
    let to_hex = hex::encode(sig.as_ref());
    Ok(to_hex)
}

pub fn curve25519_get_pubkey(prikey: String) -> Result<String, anyhow::Error> {
    let private_key_hex = hex::decode(prikey)?;
    let private_key = PrivateKey::deserialize(&private_key_hex)?;
    let public_key = private_key.public_key()?;
    Ok(hex::encode(public_key.serialize()))
}

pub fn curve25519_verify(
    public_key: Vec<u8>,
    message: Vec<u8>,
    sig: String,
) -> Result<bool, anyhow::Error> {
    let verify_key = PB::deserialize(&public_key)?;
    let sig_vec = hex::decode(sig)?;
    let result = verify_key.verify_signature(&message, &sig_vec)?;
    Ok(result)
}

pub fn generate_seed_from_ratchetkey_pair(seed_key: String) -> Result<String, anyhow::Error> {
    let split_to_arr = seed_key.split_once('-');
    let private: Vec<u8> = signal_store::decode_str_to_bytes(split_to_arr.expect("split error").0)?;
    let public: Vec<u8> = signal_store::decode_str_to_bytes(split_to_arr.expect("split error").1)?;
    let alice_private = PrivateKey::deserialize(&private)?;
    let bob_public = PB::deserialize(&public)?;

    let mut secrets = Vec::with_capacity(32 * 5);
    secrets.extend_from_slice(&[0xFFu8; 32]);
    secrets.extend_from_slice(&alice_private.calculate_agreement(&bob_public)?);

    let secret_hash = sha256::Hash::hash(&secrets).to_string();
    let secret_hash_64 = &secret_hash[0..64];
    let secp = Secp256k1::new();
    let secret_key =
        nostr::secp256k1::SecretKey::from_slice(hex::decode(secret_hash_64)?.as_slice())?;
    let public_key = PB256::from_secret_key(&secp, &secret_key);
    let x_public_key = public_key.x_only_public_key().0.serialize();
    let result = hex::encode(x_public_key);
    Ok(result)
}

// this use secp256k1::hashes::{Hash, sha256};
pub fn generate_message_key_hash(seed_key: String) -> Result<String, anyhow::Error> {
    let split_to_arr = seed_key.split_once('-');
    let cipher_key: Vec<u8> =
        signal_store::decode_str_to_bytes(split_to_arr.expect("split error").0)?;
    let iv: Vec<u8> = signal_store::decode_str_to_bytes(split_to_arr.expect("split error").1)?;
    let mut secrets = Vec::with_capacity(32 * 5);
    secrets.extend_from_slice(&iv);
    secrets.extend_from_slice(&cipher_key);
    let msg_hash = sha256::Hash::hash(&secrets).to_string();
    let result = msg_hash[0..64].to_owned();
    Ok(result)
}
