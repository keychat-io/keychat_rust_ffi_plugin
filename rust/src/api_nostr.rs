pub use bip39;
use bitcoin::bech32;
pub use nostr;

use bitcoin::secp256k1::hashes::{sha256, Hash};
use bitcoin::secp256k1::Secp256k1;
use nostr::nips::nip04;
use nostr::nips::nip06::FromMnemonic;
use nostr::nips::nip19::{FromBech32, ToBech32};
use nostr::nips::nip44;
use nostr::nips::nip47::NostrWalletConnectURI;
use nostr::secp256k1::PublicKey as PB256;
use nostr::types::Timestamp;
use nostr::{
    Event, EventBuilder, EventId, JsonUtil, Keys, Kind, PublicKey, SecretKey, Tag, UnsignedEvent,
};
use serde::Serialize;
use sha1::{Digest, Sha1};
use signal_store::libsignal_protocol::{PrivateKey, PublicKey as PB};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub kind: u16,
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
    let secret_key = keys.secret_key();

    let result = Secp256k1Account {
        pubkey: public_key.to_string(),
        prikey: keys.secret_key().display_secret().to_string(),
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

pub fn generate_from_mnemonic(password: Option<String>) -> anyhow::Result<Secp256k1Account> {
    let mnemonic = bip39::Mnemonic::generate(12)?;

    let mnemonic_words = mnemonic.to_string();
    let res = import_from_phrase(mnemonic_words, password, None);
    res
}

pub fn generate_simple() -> anyhow::Result<Secp256k1SimpleAccount> {
    let keys = Keys::generate();
    let public_key = keys.public_key();
    let result: Secp256k1SimpleAccount = Secp256k1SimpleAccount {
        pubkey: public_key.to_string(),
        prikey: keys.secret_key().display_secret().to_string(),
    };
    Ok(result)
}

pub fn import_key(sender_keys: String) -> anyhow::Result<Secp256k1Account> {
    let keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let secret_key = keys.secret_key();
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
pub fn import_from_phrase(
    phrase: String,
    password: Option<String>,
    account: Option<u32>,
) -> anyhow::Result<Secp256k1Account> {
    let keys: Keys = Keys::from_mnemonic_with_account(&phrase, password.as_ref(), account)?;
    let public_key = keys.public_key();
    let secret_key = keys.secret_key();
    let (signing_key, verifying_key) =
        generate_curve25519_keypair(phrase.clone(), password.clone(), account)?;

    let result = Secp256k1Account {
        mnemonic: Some(phrase.to_string()),
        pubkey: public_key.to_string(),
        prikey: keys.secret_key().display_secret().to_string(),
        pubkey_bech32: public_key.to_bech32()?,
        prikey_bech32: secret_key.to_bech32()?,
        curve25519_sk_hex: Some(hex::encode(&signing_key)),
        curve25519_pk_hex: Some(hex::encode(&verifying_key)),
        curve25519_sk: Some(signing_key.to_vec()),
        curve25519_pk: Some(verifying_key.to_vec()),
    };
    Ok(result)
}

pub fn import_from_phrase_with(
    phrase: String,
    password: Option<String>,
    offset: u32,
    count: u32,
) -> anyhow::Result<Vec<Secp256k1Account>> {
    let mut accounts = Vec::with_capacity(count as _);
    for i in 0..count {
        let pos = offset + i;
        let account = import_from_phrase(phrase.clone(), password.clone(), Some(pos))?;
        accounts.push(account);
    }

    Ok(accounts)
}

#[frb(sync)]
pub fn get_hex_pubkey_by_bech32(bech32: String) -> String {
    if !bech32.starts_with("npub") {
        return bech32;
    }
    let public_key = PublicKey::from_bech32(&bech32).expect("bech32 to public key error");
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
pub fn get_bech32_prikey_by_hex(hex: String) -> String {
    if hex.starts_with("nsec") {
        return hex;
    }
    let key = SecretKey::from_hex(&hex).expect("hex to secret key error");
    key.to_bech32().expect("prikey key to bech32 error")
}

#[frb(sync)]
pub fn decode_bech32(content: String) -> anyhow::Result<String> {
    use bitcoin::bech32;
    let res = bech32::decode(content.as_ref())?;
    let data = bech32::convert_bits(&res.1, 5, 8, false)?;
    Ok(String::from_utf8(data)?)
}

#[frb(sync)]
pub fn encode_bech32(hrp: String, data: String) -> anyhow::Result<String> {
    use bitcoin::bech32::u5;
    let data_bytes = data.as_bytes();
    let converted = bech32::convert_bits(data_bytes, 8, 5, true)?;
    // Convert Vec<u8> to Vec<u5>
    let converted_u5: Vec<u5> = converted
        .into_iter()
        .map(u5::try_from_u8)
        .collect::<Result<_, _>>()?;
    let encoded = bitcoin::bech32::encode(hrp.as_str(), converted_u5, bech32::Variant::Bech32)?;
    Ok(encoded)
}

#[frb(sync)]
pub fn get_hex_prikey_by_bech32(bech32: String) -> String {
    if !bech32.starts_with("nsec") {
        return bech32;
    }
    let key = SecretKey::from_bech32(&bech32).expect("bech32 to secret key error");
    let result = key.display_secret().to_string();
    result
}

#[frb(sync)]
pub fn get_hex_pubkey_by_prikey(prikey: String) -> anyhow::Result<String> {
    let keys: Keys = nostr::Keys::parse(&prikey)?;
    let public_key = keys.public_key();
    Ok(public_key.to_string())
}

pub async fn create_gift_json(
    kind: u16,
    sender_keys: String,
    receiver_pubkey: String,
    content: String,
    reply: Option<String>,
    expiration_timestamp: Option<u64>,
    timestamp_tweaked: Option<bool>,
) -> anyhow::Result<String> {
    let sender_keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let receiver = get_xonly_pubkey(receiver_pubkey)?;

    // get unsigned
    let rumor = create_unsigned_event(kind, &sender_keys, &receiver, content, reply)?;
    let mut ts = rumor.created_at;

    // rumor -> 13
    let seal: Event = EventBuilder::seal(&sender_keys, &receiver, rumor)
        .await?
        .sign(&sender_keys)
        .await?;

    //  13-> 1059
    // EventBuilder::gift_wrap_from_seal(&receiver, &seal, expiration_timestamp.map(Into::into))?;
    let keys: Keys = Keys::generate();
    let content: String = nostr::nips::nip44::encrypt(
        keys.secret_key(),
        &receiver,
        seal.as_json(),
        Default::default(),
    )?;

    let mut tags: Vec<Tag> = Vec::with_capacity(1 + usize::from(expiration_timestamp.is_some()));
    tags.push(Tag::public_key(receiver.clone()));
    if let Some(timestamp) = expiration_timestamp {
        tags.push(Tag::expiration(timestamp.into()));
    }

    if timestamp_tweaked.unwrap_or(true) {
        ts = Timestamp::tweaked(nostr::nips::nip59::RANGE_RANDOM_TIMESTAMP_TWEAK);
    }

    let gift = EventBuilder::new(Kind::GiftWrap, content)
        .tags(tags)
        .custom_created_at(ts)
        .sign(&keys)
        .await?;

    let json = gift.as_json();
    Ok(json)
}

#[frb(ignore)]
fn create_unsigned_event(
    kind: u16,
    sender_keys: &Keys,
    receiver: &PublicKey,
    content: String,
    reply: Option<String>,
) -> anyhow::Result<UnsignedEvent> {
    let mut tags = vec![Tag::public_key(*receiver)];
    if let Some(reply) = reply {
        let e = EventId::from_hex(&reply)?;
        tags.push(e.into())
    }

    let event = EventBuilder::new(kind.into(), content).tags(tags);

    let mut this = event.build(sender_keys.public_key());
    // UnsignedEvent's compute_id not public
    this.id = Some(EventId::new(
        &this.pubkey,
        &this.created_at,
        &this.kind,
        &this.tags.as_slice(),
        &this.content,
    ));
    Ok(this)
}

pub fn decrypt_gift(
    sender_keys: String,
    receiver: String,
    content: String,
) -> anyhow::Result<NostrEvent> {
    let alice_keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let receiver = get_xonly_pubkey(receiver)?;

    let seal_json = nostr::nips::nip44::decrypt(alice_keys.secret_key(), &receiver, &content)?;
    let seal = nostr::Event::from_json(&seal_json)?;
    seal.verify()?;
    let receiver = seal.pubkey;
    //fix bug with amber app signEvent
    let content = seal.content.replace(" ", "+");
    let rumor_json = nostr::nips::nip44::decrypt(alice_keys.secret_key(), &receiver, &content)?;
    let rumor = UnsignedEvent::from_json(&rumor_json)?;

    // Clients MUST verify if pubkey of the kind:13 is the same pubkey on the kind:14
    ensure!(
        seal.pubkey == rumor.pubkey,
        "the public key of seal isn't equal the rumor's"
    );

    let tags = rumor
        .tags
        .iter()
        .map(|t| t.clone().to_vec().to_owned())
        .collect();
    let ne: NostrEvent = NostrEvent {
        tags,
        id: rumor.id.as_ref().map(|s| s.to_string()).unwrap_or_default(),
        content: rumor.content,
        created_at: rumor.created_at.as_u64(),
        kind: rumor.kind.as_u16(),
        sig: String::new(),
        pubkey: rumor.pubkey.to_string(),
    };
    Ok(ne)
}

// encrypt message and return event string
pub async fn get_encrypt_event(
    sender_keys: String,
    receiver_pubkey: String,
    content: String,
    reply: Option<String>,
) -> anyhow::Result<String> {
    let alice_keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let pubkey = get_xonly_pubkey(receiver_pubkey)?;
    let mut _reply = None;

    if let Some(reply) = reply {
        _reply = Some(EventId::from_hex(&reply)?);
    }
    let encrypted = nip04::encrypt(alice_keys.secret_key(), &pubkey, content)?;

    let mut tags: Vec<Tag> = vec![];
    tags.push(Tag::public_key(pubkey));

    let alice_encrypted_msg = EventBuilder::new(Kind::from_u16(4), encrypted)
        .tags(tags)
        .sign(&alice_keys)
        .await?;
    Ok(alice_encrypted_msg.as_json())
}

// generate event, but do not encrypt content
pub async fn get_unencrypt_event(
    sender_keys: String,
    receiver_pubkeys: Vec<String>,
    content: String,
    kind: u16,
    additional_tags: Option<Vec<Vec<String>>>,
) -> anyhow::Result<String> {
    let mut tags: Vec<Tag> = vec![];

    // Add pubkey tags
    for p in receiver_pubkeys {
        let pubkey = get_xonly_pubkey(p)?;
        tags.push(Tag::public_key(pubkey));
    }

    // Add additional tags if provided
    if let Some(additional_tags) = additional_tags {
        for tag_data in additional_tags {
            tags.push(nostr::Tag::parse(&tag_data)?);
        }
    }

    let alice_keys: Keys = nostr::Keys::parse(&sender_keys)?;

    let event = EventBuilder::new(Kind::from(kind), content)
        .tags(tags)
        .sign(&alice_keys)
        .await?;
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
    let result = nip04::encrypt(alice_keys.secret_key(), &pubkey, content)?;
    Ok(result)
}

pub fn encrypt_nip44(
    sender_keys: String,
    receiver_pubkey: String,
    content: String,
) -> anyhow::Result<String> {
    let alice_keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let pubkey = get_xonly_pubkey(receiver_pubkey)?;
    let result = nip44::encrypt(
        alice_keys.secret_key(),
        &pubkey,
        content,
        nip44::Version::V2,
    )?;
    Ok(result)
}

pub fn decrypt(
    sender_keys: String,
    receiver_pubkey: String,
    content: String,
) -> anyhow::Result<String> {
    let alice_keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let pubkey = get_xonly_pubkey(receiver_pubkey)?;
    let result = nip04::decrypt(alice_keys.secret_key(), &pubkey, content)?;
    Ok(result)
}

pub fn decrypt_nip44(
    secret_key: String,
    public_key: String,
    content: String,
) -> anyhow::Result<String> {
    let alice_keys: Keys = nostr::Keys::parse(&secret_key)?;
    let pubkey = get_xonly_pubkey(public_key)?;
    let result = nip44::decrypt(alice_keys.secret_key(), &pubkey, content)?;
    Ok(result)
}

pub async fn sign_event(
    sender_keys: String,
    content: String,
    created_at: u64,
    kind: u16,
    tags: Vec<Vec<String>>,
) -> anyhow::Result<String> {
    let tags: Vec<Tag> = tags
        .into_iter()
        .map(|t| nostr::Tag::parse(&t).unwrap())
        .collect();
    let alice_keys: Keys = nostr::Keys::parse(&sender_keys)?;
    let event = EventBuilder::new(nostr::Kind::from(kind), content).tags(tags);
    let event_result = event
        .clone()
        .custom_created_at(Timestamp::from(created_at))
        .sign(&alice_keys)
        .await?;
    let result = event_result.as_json();
    Ok(result.to_string())
}

pub fn decrypt_event(sender_keys: String, json: String) -> anyhow::Result<String> {
    let event = nostr::event::Event::from_json(json)?;
    let pubkey = event
        .tags
        .first()
        .ok_or_else(|| format_err!("empty tags"))?
        .clone()
        .to_vec();

    let decrypted_string = decrypt(
        sender_keys,
        pubkey[1].to_string(),
        event.content.to_string(),
    );
    decrypted_string
}

pub fn verify_event(json: String) -> anyhow::Result<NostrEvent> {
    let event = nostr::Event::from_json(json)?;
    event.verify()?;

    let tags = event
        .tags
        .iter()
        .map(|t| t.clone().to_vec().to_owned())
        .collect();
    let ne: NostrEvent = NostrEvent {
        tags,
        id: event.id.to_string(),
        content: event.content.clone(),
        created_at: event.created_at.as_u64(),
        kind: event.kind.as_u16(),
        sig: serde_json::to_string(&event.sig)?,
        pubkey: event.pubkey.to_string(),
    };
    Ok(ne)
}

fn get_xonly_pubkey(pubkey: String) -> anyhow::Result<PublicKey> {
    let bob_pubkey: PublicKey = pubkey.parse()?;
    Ok(bob_pubkey)
}
// Sign a message using Schnorr signature
pub fn sign_schnorr(private_key: String, content: String) -> anyhow::Result<String> {
    let secp = Secp256k1::new();
    let message = bitcoin::secp256k1::Message::from_hashed_data::<sha256::Hash>(content.as_bytes());
    let keypair = bitcoin::secp256k1::KeyPair::from_seckey_str(&secp, &private_key)?;
    let signature = secp.sign_schnorr(&message, &keypair);
    Ok(signature.to_string())
}
// verify sign
pub fn verify_schnorr(
    pubkey: String,
    sig: String,
    content: String,
    hash: bool,
) -> anyhow::Result<bool> {
    let pk = &bitcoin::secp256k1::XOnlyPublicKey::from_str(&pubkey)?;
    let sig = sig.parse()?;

    let secp = Secp256k1::new();

    let message = if hash {
        bitcoin::secp256k1::Message::from_hashed_data::<sha256::Hash>(content.as_bytes())
    } else {
        let message = hex::decode(&content)?;
        bitcoin::secp256k1::Message::from_slice(message.as_ref())?
    };

    // println!("{:?}", secp.verify_schnorr(&sig, &message, &pk));
    let _result: () = secp.verify_schnorr(&sig, &message, &pk)?;
    Ok(true)
}

pub fn generate_curve25519_keypair(
    mnemonic_words: String,
    password: Option<String>,
    pos: Option<u32>,
) -> Result<(Vec<u8>, Vec<u8>), anyhow::Error> {
    let mnemonic = bip39::Mnemonic::parse_in_normalized(bip39::Language::English, &mnemonic_words)?;
    let seed = mnemonic.to_seed(password.unwrap_or_default());
    use bitcoin::bip32::{DerivationPath, ExtendedPrivKey as Xpriv};
    let root_key = Xpriv::new_master(bitcoin::Network::Bitcoin, &seed)?;
    // let path = DerivationPath::from_str("m/44'/1237'/0'/0/0")?;
    // let path: DerivationPath = "m/44'/1238'/0'/0/0".parse()?;
    let account: u32 = pos.unwrap_or_default();
    let path: DerivationPath = format!("m/44'/1238'/{}'/0/0", account).parse()?;
    let ctx = bitcoin::key::Secp256k1::new();
    let child_xprv = root_key.derive_priv(&ctx, &path)?;

    let private_key = PrivateKey::deserialize(&child_xprv.private_key.secret_bytes())?;
    let public_key = private_key.public_key()?;
    // len is 32 33
    Ok((private_key.serialize(), public_key.serialize().into()))
}

pub fn curve25519_sign(secret_key: Vec<u8>, message: Vec<u8>) -> Result<String, anyhow::Error> {
    use bip39::rand_core::OsRng;

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
    let secp = nostr::secp256k1::Secp256k1::new();
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

pub fn generate_seed_from_key(seed_key: Vec<u8>) -> Result<String, anyhow::Error> {
    let mut secrets = Vec::with_capacity(32 * 5);
    secrets.extend_from_slice(&[0xFFu8; 32]);
    secrets.extend_from_slice(&seed_key);

    let secret_hash = sha256::Hash::hash(&secrets).to_string();
    let secret_hash_64 = &secret_hash[0..64];
    let secp = nostr::secp256k1::Secp256k1::new();
    let secret_key =
        nostr::secp256k1::SecretKey::from_slice(hex::decode(secret_hash_64)?.as_slice())?;
    let public_key = PB256::from_secret_key(&secp, &secret_key);
    let x_public_key = public_key.x_only_public_key().0.serialize();
    let result = hex::encode(x_public_key);
    Ok(result)
}

pub fn nip47_encode_uri(
    pubkey: String,
    relay: String,
    secret: String,
    lud16: Option<String>,
) -> Result<String, anyhow::Error> {
    let pubkey = PublicKey::from_str(&pubkey)?;
    let relay_url = nostr::RelayUrl::parse(&relay)?;
    let secret = SecretKey::from_str(&secret)?;
    let uri = NostrWalletConnectURI::new(pubkey, relay_url, secret, lud16);
    Ok(uri.to_string())
}

// pub fn nip47_serialize_request() {
//     use nostr::nips::nip47::*;
//     let request = Request {
//       method: Method::PayInvoice,
//       params: RequestParams::PayInvoice(PayInvoiceRequest { id: None, invoice: "lnbc210n1pj99rx0pp5ehevgz9nf7d97h05fgkdeqxzytm6yuxd7048axru03fpzxxvzt7shp5gv7ef0s26pw5gy5dpwvsh6qgc8se8x2lmz2ev90l9vjqzcns6u6scqzzsxqyz5vqsp".to_string(), amount: None }),
//   };

//     assert_eq!(Request::from_json(request.as_json()).unwrap(), request);

//     assert_eq!(request.as_json(), "{\"method\":\"pay_invoice\",\"params\":{\"invoice\":\"lnbc210n1pj99rx0pp5ehevgz9nf7d97h05fgkdeqxzytm6yuxd7048axru03fpzxxvzt7shp5gv7ef0s26pw5gy5dpwvsh6qgc8se8x2lmz2ev90l9vjqzcns6u6scqzzsxqyz5vqsp\"}}");
// }

pub fn nip47_parse_request(request: String) -> Result<String, anyhow::Error> {
    // let request = "{\"params\":{\"invoice\":\"lnbc210n1pj99rx0pp5ehevgz9nf7d97h05fgkdeqxzytm6yuxd7048axru03fpzxxvzt7shp5gv7ef0s26pw5gy5dpwvsh6qgc8se8x2lmz2ev90l9vjqzcns6u6scqzzsxqyz5vqsp5rdjyt9jr2avv2runy330766avkweqp30ndnyt9x6dp5juzn7q0nq9qyyssq2mykpgu04q0hlga228kx9v95meaqzk8a9cnvya305l4c353u3h04azuh9hsmd503x6jlzjrsqzark5dxx30s46vuatwzjhzmkt3j4tgqu35rms\"},\"method\":\"pay_invoice\"}";
    use nostr::nips::nip47::*;
    let request = Request::from_json(request).unwrap();

    assert_eq!(request.method, Method::PayInvoice);

    if let RequestParams::PayInvoice(pay) = request.params {
        Ok(pay.invoice)
    } else {
        panic!("Invalid request params")
    }
}

pub fn sha256_hash(data: String) -> String {
    // Use the bitcoin secp256k1 sha256 implementation
    let hash = sha256::Hash::hash(data.as_bytes());
    // Convert the hash to a hex string
    hash.to_string()
}

pub fn sha256_hash_bytes(data: Vec<u8>) -> String {
    let hash = sha256::Hash::hash(&data);
    // Convert the hash to a hex string
    hash.to_string()
}

pub fn sha1_hash(data: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
