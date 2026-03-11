use anyhow::Result;
use rust::api_nostr::nostr::base64::engine::general_purpose;
use rust::api_nostr::nostr::base64::*;
use rust::api_signal::signal_store::libsignal_protocol::*;
use rust::api_signal::*;

fn main() {
    // let _ = test_kdf();
    // let _= test_state();
    // let _ = test_db();
    // let _ = test_close_db();
    // let _ = test_parse_is_prekey_message2();
    // let _ = test_x3dh_db();
    let _ = test_x4dh_db();
    // let _ = test_multi_add();
}

fn test_parse_prekey() -> Result<()> {
    let content = "NAjZtOSOChIhBcM1rtFjYuE6mdak4ql1iSVVbnaRGLH0Na09ZwN6qmRIGiEFXffW+BrVSqmLimFmOQauD3ehoRY/19Ee6ejQBNqxczoiswE0CiEFfrVjB2BM4qaumzeg2TtTcxLHQuqmoP1S9+DXuw1fPHcQABgAIoABbDOaq43qsrmQuvWEsAIrE1DErRiC5tDtmpfWOk4rT33bi7AkD5EFtVoMG5k4PhvvcVuTyg8BqXr6i1NpN7AMunpuplGn79He8TXMluJ6jZcI7HEKkvO0irXWZsEADnHYcLY/n0qhPk4cHwxQGGJmkp37VYFGyYdd8q71roHVi9RyDE3+VfM5iCgAMNyA+ocM";
    let cipher_text = general_purpose::STANDARD.decode(content).unwrap();
    let re = parse_identity_from_prekey_signal_message(cipher_text).unwrap();
    println!("the result is {:?}", re);
    Ok(())
}

fn test_parse_is_prekey_message() -> Result<()> {
    let content = "NAjZtOSOChIhBcM1rtFjYuE6mdak4ql1iSVVbnaRGLH0Na09ZwN6qmRIGiEFXffW+BrVSqmLimFmOQauD3ehoRY/19Ee6ejQBNqxczoiswE0CiEFfrVjB2BM4qaumzeg2TtTcxLHQuqmoP1S9+DXuw1fPHcQABgAIoABbDOaq43qsrmQuvWEsAIrE1DErRiC5tDtmpfWOk4rT33bi7AkD5EFtVoMG5k4PhvvcVuTyg8BqXr6i1NpN7AMunpuplGn79He8TXMluJ6jZcI7HEKkvO0irXWZsEADnHYcLY/n0qhPk4cHwxQGGJmkp37VYFGyYdd8q71roHVi9RyDE3+VfM5iCgAMNyA+ocM";
    let cipher_text = general_purpose::STANDARD.decode(content).unwrap();
    let re = parse_is_prekey_signal_message(cipher_text).unwrap();
    println!("the result is {:?}", re);
    Ok(())
}

fn test_parse_is_prekey_message2() -> Result<()> {
    let cipher_text = [
        52, 10, 33, 5, 123, 4, 193, 146, 66, 82, 212, 67, 197, 105, 34, 120, 117, 46, 253, 30, 167,
        232, 40, 130, 18, 7, 244, 136, 193, 159, 11, 234, 147, 41, 98, 103, 16, 0, 24, 0, 34, 32,
        109, 233, 105, 107, 190, 190, 223, 158, 66, 52, 98, 155, 43, 172, 89, 253, 228, 231, 182,
        247, 165, 255, 196, 140, 74, 80, 240, 204, 231, 225, 201, 103, 35, 178, 185, 36, 84, 185,
        224, 24,
    ];
    let re = parse_is_prekey_signal_message(cipher_text.to_vec()).unwrap();
    println!("the result is {:?}", re);
    Ok(())
}

fn test_close_db() -> Result<()> {
    let db = "./signal.db";

    let device_id: DeviceId = DeviceId::try_from(1)?;

    let alice_identity_private =
        hex::decode("38e11be5690d3e0600544b87088961c7fd58c041d1a1766ac8fc2a50e3bdde4c")
            .expect("valid hex");
    //alice info
    let alice_identity_public =
        hex::decode("05f6214a72c3b3cd6e4e1fca26ffbd40f5c750d06f4be16f013cda52e2e2e6b30b")
            .expect("valid hex");
    let alice_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: alice_identity_public.as_slice().try_into().unwrap(),
        private_key: alice_identity_private.as_slice().try_into().unwrap(),
    };
    let alice_address = KeychatProtocolAddress {
        name: "05743b1e2894d1df36972e91c12700d0f8d2a81b0cf455f18abe0cf09c41e0944a".to_owned(),
        device_id: device_id.into(),
    };
    init(db.to_string(), alice_identity_key_pair, 0).expect("init error");

    //bob info
    let bob_identity_public =
        hex::decode("05f191f40dff0e56fe8833282f5512cf8f68e28794140f650324220f5ed3ee7e4d")
            .expect("valid hex");
    let bob_identity_private =
        hex::decode("38393385efdc31e5565c20610e665429430f6bfb9320adb4e5cbff680febae6e")
            .expect("valid hex");
    let bob_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: bob_identity_public.as_slice().try_into().unwrap(),
        private_key: bob_identity_private.as_slice().try_into().unwrap(),
    };

    let t = generate_signed_pre_key_api(bob_identity_key_pair, alice_identity_private);
    println!("generate_signed_pre_key_api {:?}", t);
    let c = close_signal_db();
    println!("close_signal_db {:?}", c);
    Ok(())
}

fn test_db() -> Result<()> {
    let db = "./signal.db";

    let device_id: DeviceId = DeviceId::try_from(1)?;

    let alice_identity_private =
        hex::decode("38e11be5690d3e0600544b87088961c7fd58c041d1a1766ac8fc2a50e3bdde4c")
            .expect("valid hex");
    //alice info
    let alice_identity_public =
        hex::decode("05f6214a72c3b3cd6e4e1fca26ffbd40f5c750d06f4be16f013cda52e2e2e6b30b")
            .expect("valid hex");
    let alice_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: alice_identity_public.as_slice().try_into().unwrap(),
        private_key: alice_identity_private.as_slice().try_into().unwrap(),
    };
    let alice_address = KeychatProtocolAddress {
        name: "05743b1e2894d1df36972e91c12700d0f8d2a81b0cf455f18abe0cf09c41e0944a".to_owned(),
        device_id: device_id.into(),
    };
    init(db.to_string(), alice_identity_key_pair, 0).expect("init error");

    //bob info
    let bob_identity_public =
        hex::decode("05f191f40dff0e56fe8833282f5512cf8f68e28794140f650324220f5ed3ee7e4d")
            .expect("valid hex");
    let bob_identity_private =
        hex::decode("38393385efdc31e5565c20610e665429430f6bfb9320adb4e5cbff680febae6e")
            .expect("valid hex");
    let bob_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: bob_identity_public.as_slice().try_into().unwrap(),
        private_key: bob_identity_private.as_slice().try_into().unwrap(),
    };

    let t = generate_signed_pre_key_api(bob_identity_key_pair, alice_identity_private);
    println!("generate_signed_pre_key_api {:?}", t);
    //test alice_identity_key_pair
    let t1 = get_all_alice_addrs(bob_identity_key_pair);
    println!("bob_identity_key_pair alice address {:?}", t1.unwrap());

    //test get_session
    let t1 = get_session(
        alice_identity_key_pair,
        "05743b1e2894d1df36972e91c12700d0f8d2a81b0cf455f18abe0cf09c41e0944a".to_string(),
        "1".to_string(),
    );
    println!("get_session alice {:?}", t1.unwrap());

    //test get_identity
    let t1 = get_identity(alice_identity_key_pair, alice_address);
    println!("get_identity alice {:?}", t1.unwrap());

    Ok(())
}

fn test_state() -> Result<()> {
    let db_path = ".signal_test.db";
    let device_id1: DeviceId = DeviceId::try_from(1)?;
    let device_id2: DeviceId = DeviceId::try_from(2)?;
    let device_id3: DeviceId = DeviceId::try_from(3)?;

    //alice info
    let alice_identity_public =
        hex::decode("051e9e15755cee5707a77c164625ca340fdb56c16b20514e7df4e09d01cd2c7316")
            .expect("valid hex");
    let alice_identity_private =
        hex::decode("70648cfae815fd73ab93c673f6827eec45f6688f8ce5fb73f5444999cc0a506e")
            .expect("valid hex");
    let alice_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: alice_identity_public.as_slice().try_into().unwrap(),
        private_key: alice_identity_private.as_slice().try_into().unwrap(),
    };
    let registration_id_alice = 1;
    let alice_address = KeychatProtocolAddress {
        name: "alice".to_owned(),
        device_id: device_id1.into(),
    };

    //bob info
    let bob_identity_public =
        hex::decode("05f191f40dff0e56fe8833282f5512cf8f68e28794140f650324220f5ed3ee7e4d")
            .expect("valid hex");
    let bob_identity_private =
        hex::decode("38393385efdc31e5565c20610e665429430f6bfb9320adb4e5cbff680febae6e")
            .expect("valid hex");
    let bob_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: bob_identity_public.as_slice().try_into().unwrap(),
        private_key: bob_identity_private.as_slice().try_into().unwrap(),
    };
    let registration_id_bob = 1;
    let bob_address = KeychatProtocolAddress {
        name: "bob".to_owned(),
        device_id: device_id2.into(),
    };

    // tom info 0515e97b26c5cbca6f39dce5cc55db22cd948598d370b87c1ce4919d665aeaab27
    let tom_identity_public =
        hex::decode("0515e97b26c5cbca6f39dce5cc55db22cd948598d370b87c1ce4919d665aeaab27")
            .expect("valid hex");
    let tom_identity_private =
        hex::decode("4875f9558f57bd7629d2792afaaf331ea10e6e8d1cbe28448e3850b923243b5c")
            .expect("valid hex");
    let tom_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: tom_identity_public.as_slice().try_into().unwrap(),
        private_key: tom_identity_private.as_slice().try_into().unwrap(),
    };
    let registration_id_tom = 1;
    let tom_address = KeychatProtocolAddress {
        name: "tom".to_owned(),
        device_id: device_id3.into(),
    };

    /*
     * first alice to bob then  bob to alice
     */
    init(
        db_path.to_owned(),
        alice_identity_key_pair,
        registration_id_alice,
    )
    .expect("init error");

    // test contains_session
    let jack_address = KeychatProtocolAddress {
        name: "jack".to_owned(),
        device_id: device_id3.into(),
    };
    // let t1 = contains_session(alice_identity_key_pair, alice_address);
    // println!("contains alice session {:?}", t1.unwrap());
    // let t2 = contains_session(alice_identity_key_pair, bob_address);
    // println!("contains bob session {:?}", t2.unwrap());
    // let t3 = contains_session(alice_identity_key_pair, tom_address);
    // println!("contains tom session {:?}", t3.unwrap());
    // let t4 = contains_session(alice_identity_key_pair, jack_address);
    // println!("contains tom session {:?}", t4.unwrap());

    //test alice_identity_key_pair
    let t1 = get_all_alice_addrs(alice_identity_key_pair);
    println!("alice_identity_key_pair alice address {:?}", t1.unwrap());

    //test get_session
    let t1 = get_session(
        alice_identity_key_pair,
        "alice".to_string(),
        "1".to_string(),
    );
    // println!("get_session alice {:?}", t1.unwrap());
    let t2 = get_session(alice_identity_key_pair, "bob".to_string(), "1".to_string());
    println!("get_session bob {:?}", t2.unwrap());
    let t3 = get_session(alice_identity_key_pair, "jack".to_string(), "1".to_string());
    println!("get_session jack {:?}", t3.unwrap());

    //test get_identity
    let t1 = get_identity(alice_identity_key_pair, alice_address);
    println!("get_identity alice {:?}", t1.unwrap());

    //test session_contain_alice_addr
    let t1 = session_contain_alice_addr(alice_identity_key_pair, "[24, 183, 53, 185, 139, 94, 101, 110, 119, 67, 170, 108, 194, 110, 121, 100, 186, 192, 86, 170, 208, 170, 141, 96, 112, 6, 139, 7, 191, 87, 161, 70]-[5, 51, 226, 203, 233, 14, 250, 226, 235, 146, 133, 48, 127, 198, 111, 252, 66, 95, 28, 27, 205, 104, 181, 198, 53, 121, 24, 242, 144, 199, 246, 245, 56]".to_string());
    println!("session_contain_alice_addr alice {:?}", t1.unwrap());

    //test update_alice_addr
    let t1 = update_alice_addr(
        alice_identity_key_pair,
        "tom".to_string(),
        "3".to_string(),
        "testing".to_string(),
    );
    println!("update_alice_addr tom {:?}", t1.unwrap());

    //test delete_identity
    let t1 = delete_identity(alice_identity_key_pair, "tom".to_string());
    println!("delete_identity tom {:?}", t1.unwrap());

    //test delete_session
    let t1 = delete_session(alice_identity_key_pair, bob_address);
    println!("delete_session bob {:?}", t1.unwrap());

    //test delete_session_by_device_id
    let t1 = delete_session_by_device_id(alice_identity_key_pair, 3);
    println!("delete_session_by_device_id tom {:?}", t1.unwrap());

    Ok(())
}

fn test_x4dh_db() -> Result<()> {
    let db_path = ".signal_test.db";
    let db_path2 = ".signal_test2.db";
    let db_path3 = ".signal_test3.db";
    let device_id1: DeviceId = DeviceId::try_from(1)?;
    let device_id2: DeviceId = DeviceId::try_from(2)?;
    let device_id3: DeviceId = DeviceId::try_from(3)?;

    //alice info
    let alice_identity_public =
        hex::decode("051e9e15755cee5707a77c164625ca340fdb56c16b20514e7df4e09d01cd2c7316")
            .expect("valid hex");
    let alice_identity_private =
        hex::decode("70648cfae815fd73ab93c673f6827eec45f6688f8ce5fb73f5444999cc0a506e")
            .expect("valid hex");
    let alice_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: alice_identity_public.as_slice().try_into().unwrap(),
        private_key: alice_identity_private.as_slice().try_into().unwrap(),
    };
    let registration_id_alice = 1;
    let alice_address = KeychatProtocolAddress {
        name: "alice".to_owned(),
        device_id: device_id1.into(),
    };

    //bob info
    let bob_identity_public =
        hex::decode("05f191f40dff0e56fe8833282f5512cf8f68e28794140f650324220f5ed3ee7e4d")
            .expect("valid hex");
    let bob_identity_private =
        hex::decode("38393385efdc31e5565c20610e665429430f6bfb9320adb4e5cbff680febae6e")
            .expect("valid hex");
    let bob_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: bob_identity_public.as_slice().try_into().unwrap(),
        private_key: bob_identity_private.as_slice().try_into().unwrap(),
    };
    let registration_id_bob = 1;
    let bob_address = KeychatProtocolAddress {
        name: "bob".to_owned(),
        device_id: device_id2.into(),
    };

    // tom info
    let tom_identity_public =
        hex::decode("0515e97b26c5cbca6f39dce5cc55db22cd948598d370b87c1ce4919d665aeaab27")
            .expect("valid hex");
    let tom_identity_private =
        hex::decode("4875f9558f57bd7629d2792afaaf331ea10e6e8d1cbe28448e3850b923243b5c")
            .expect("valid hex");
    let tom_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: tom_identity_public.as_slice().try_into().unwrap(),
        private_key: tom_identity_private.as_slice().try_into().unwrap(),
    };
    let registration_id_tom = 1;
    let tom_address = KeychatProtocolAddress {
        name: "tom".to_owned(),
        device_id: device_id3.into(),
    };

    /*
     * first alice to bob then  bob to alice
     */
    init(
        db_path.to_owned(),
        alice_identity_key_pair,
        registration_id_alice,
    )
    .expect("init error");
    let bob_signed_info =
        generate_signed_pre_key_api(alice_identity_key_pair, bob_identity_private.clone())?;

    let bob_signed_id = bob_signed_info.signed_pre_key_id;
    println!("bob_sign_id {:?}", bob_signed_id);
    let bob_signed_key_public = bob_signed_info.signed_pre_key_public;
    let bob_signed_signature = bob_signed_info.signed_pre_key_signature;

    let bob_kyber_info = generate_kyber_pre_key_api(alice_identity_key_pair, bob_identity_private)?;
    let bob_kyber_id = bob_kyber_info.kyber_pre_key_id;
    println!("bob_kyber_id {:?}", bob_kyber_id);
    let bob_kyber_key_public = bob_kyber_info.kyber_pre_key_public;
    let bob_kyber_signature = bob_kyber_info.kyber_pre_key_signature;

    let bob_prekey_info = generate_pre_key_api(alice_identity_key_pair)?;

    process_pre_key_bundle_api(
        alice_identity_key_pair,
        bob_address.clone(),
        registration_id_bob,
        device_id2.into(),
        KeychatIdentityKey {
            public_key: bob_identity_public.as_slice().try_into().unwrap(),
        },
        bob_signed_id.into(),
        bob_signed_key_public,
        bob_signed_signature,
        bob_kyber_id.into(),
        bob_kyber_key_public,
        bob_kyber_signature,
        bob_prekey_info.pre_key_id.into(),
        bob_prekey_info.pre_key_public,
    )
    .unwrap();

    let alice2bob_msg = "Alice to Bob";
    // alice to bob
    let alice2bob_encrypt = encrypt_signal(
        alice_identity_key_pair,
        alice2bob_msg.to_string(),
        bob_address.clone(),
        None,
    )
    .unwrap();
    init(
        db_path2.to_owned(),
        bob_identity_key_pair,
        registration_id_bob,
    )
    .expect("init error");

    let alice2bob_bob_decrypt = decrypt_signal(
        bob_identity_key_pair,
        alice2bob_encrypt.ciphertext,
        alice_address.clone(),
        1,
        true,
    )
    .unwrap();
    println!(
        "alice2bob_bob_decrypt {:?}",
        String::from_utf8(alice2bob_bob_decrypt.plaintext).expect("valid utf8")
    );
    let bobs_response_to_alice = "Bob response to Alice";
    // bob to Alice
    let bobs_response_to_alice_encrypt = encrypt_signal(
        bob_identity_key_pair,
        bobs_response_to_alice.to_string(),
        alice_address.clone(),
        None,
    )
    .unwrap();

    println!(
        "bobs_response_to_alice_encrypt {:?}",
        bobs_response_to_alice_encrypt
    );
    // alice decrypt bob
    let alice_decrypts_from_bob = decrypt_signal(
        alice_identity_key_pair,
        bobs_response_to_alice_encrypt.ciphertext,
        bob_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "alice_decrypts_from_bob {:?}",
        String::from_utf8(alice_decrypts_from_bob.plaintext).expect("valid utf8")
    );

    /*
     * second alice to tom then  tom to alice
     */
    let tom_info =
        generate_signed_pre_key_api(alice_identity_key_pair, tom_identity_private.clone())?;
    let tom_prekey = generate_pre_key_api(alice_identity_key_pair)?;

    let tom_sign_id = tom_info.signed_pre_key_id;
    println!("tom_sign_id {:?}", tom_sign_id);
    let tom_signed_key_public = tom_info.signed_pre_key_public;
    let tom_signed_signature = tom_info.signed_pre_key_signature;

    let tom_kyber_info = generate_kyber_pre_key_api(alice_identity_key_pair, tom_identity_private)?;
    let tom_kyber_id = tom_kyber_info.kyber_pre_key_id;
    println!("tom_kyber_id {:?}", tom_kyber_id);
    let tom_kyber_key_public = tom_kyber_info.kyber_pre_key_public;
    let tom_kyber_signature = tom_kyber_info.kyber_pre_key_signature;

    process_pre_key_bundle_api(
        alice_identity_key_pair,
        tom_address.clone(),
        registration_id_tom,
        device_id3.into(),
        KeychatIdentityKey {
            public_key: tom_identity_public.as_slice().try_into().unwrap(),
        },
        tom_sign_id,
        tom_signed_key_public,
        tom_signed_signature,
        tom_kyber_id.into(),
        tom_kyber_key_public,
        tom_kyber_signature,
        tom_prekey.pre_key_id.into(),
        tom_prekey.pre_key_public,
    )
    .unwrap();

    let alice2tom_msg = "Alice to Tom";
    // alice to tom
    let alice2tom_encrypt = encrypt_signal(
        alice_identity_key_pair,
        alice2tom_msg.to_string(),
        tom_address.clone(),
        None,
    )
    .unwrap();
    init(
        db_path.to_owned(),
        tom_identity_key_pair,
        registration_id_tom,
    )
    .expect("init error");
    // tom decrypt Alice
    let alice2tom_tom_decrypt = decrypt_signal(
        tom_identity_key_pair,
        alice2tom_encrypt.ciphertext,
        alice_address.clone(),
        2,
        true,
    )
    .unwrap();
    println!(
        "alice2tom_tom_decrypt {:?}",
        String::from_utf8(alice2tom_tom_decrypt.plaintext).expect("valid utf8")
    );
    let tom_response_to_alice = "Tom response to Alice";
    // tom to Alice
    let tom_response_to_alice_encrypt = encrypt_signal(
        tom_identity_key_pair,
        tom_response_to_alice.to_string(),
        alice_address.clone(),
        None,
    )
    .unwrap();
    // alice decrypt tom
    let alice_decrypts_from_tom = decrypt_signal(
        alice_identity_key_pair,
        tom_response_to_alice_encrypt.ciphertext,
        tom_address.clone(),
        2,
        false,
    )
    .unwrap();
    println!(
        "alice_decrypts_from_tom {:?}",
        String::from_utf8(alice_decrypts_from_tom.plaintext).expect("valid utf8")
    );

    /*
     * third alice to bob then  bob to alice
     */
    let alice2bob_msg2 = "Alice to Bob again";
    // alice to bob
    let alice2bob_encrypt2 = encrypt_signal(
        alice_identity_key_pair,
        alice2bob_msg2.to_string(),
        bob_address.clone(),
        None,
    )
    .unwrap();
    // bob decrypt Alice
    let alice2bob_bob_decrypt2 = decrypt_signal(
        bob_identity_key_pair,
        alice2bob_encrypt2.ciphertext,
        alice_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "alice2bob_bob_decrypt2 {:?}",
        String::from_utf8(alice2bob_bob_decrypt2.plaintext).expect("valid utf8")
    );
    let bobs_response_to_alice2 = "Bob response to Alice again";
    // bob to Alice
    let bobs_response_to_alice_encrypt2 = encrypt_signal(
        bob_identity_key_pair,
        bobs_response_to_alice2.to_string(),
        alice_address.clone(),
        None,
    )
    .unwrap();
    // alice decrypt bob
    let alice_decrypts_from_bob2 = decrypt_signal(
        alice_identity_key_pair,
        bobs_response_to_alice_encrypt2.ciphertext,
        bob_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "alice_decrypts_from_bob2 {:?}",
        String::from_utf8(alice_decrypts_from_bob2.plaintext).expect("valid utf8")
    );

    /*
     * forth tom to alice then  alice to tom
     */

    let alice_response_to_tom = "Alice response to Tom";
    // alice to tom
    let alice_response_to_tom_encrypt = encrypt_signal(
        alice_identity_key_pair,
        alice_response_to_tom.to_string(),
        tom_address.clone(),
        None,
    )
    .unwrap();
    // tom decrypt alice
    let tom_decrypts_from_alice = decrypt_signal(
        tom_identity_key_pair,
        alice_response_to_tom_encrypt.ciphertext,
        alice_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "tom_decrypts_from_alice {:?}",
        String::from_utf8(tom_decrypts_from_alice.plaintext).expect("valid utf8")
    );

    let tom2alice_msg = "Tom to Alice";
    // tom to alice
    let tom2alice_encrypt = encrypt_signal(
        tom_identity_key_pair,
        tom2alice_msg.to_string(),
        alice_address.clone(),
        None,
    )
    .unwrap();
    // alice decrypt tom
    let tom2alice_tom_decrypt = decrypt_signal(
        alice_identity_key_pair,
        tom2alice_encrypt.ciphertext,
        tom_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "tom2alice_tom_decrypt {:?}",
        String::from_utf8(tom2alice_tom_decrypt.plaintext).expect("valid utf8")
    );
    Ok(())
}

fn test_multi_add() -> Result<()> {
    let db_path = ".signal_test.db";
    let device_id1: DeviceId = DeviceId::try_from(1)?;
    let device_id2: DeviceId = DeviceId::try_from(2)?;
    let device_id3: DeviceId = DeviceId::try_from(3)?;

    //alice info
    let alice_identity_public =
        hex::decode("051e9e15755cee5707a77c164625ca340fdb56c16b20514e7df4e09d01cd2c7316")
            .expect("valid hex");
    let alice_identity_private =
        hex::decode("70648cfae815fd73ab93c673f6827eec45f6688f8ce5fb73f5444999cc0a506e")
            .expect("valid hex");
    let alice_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: alice_identity_public.as_slice().try_into().unwrap(),
        private_key: alice_identity_private.as_slice().try_into().unwrap(),
    };
    let registration_id_alice = 1;
    let alice_address = KeychatProtocolAddress {
        name: "alice".to_owned(),
        device_id: device_id1.into(),
    };

    //bob info
    let bob_identity_public =
        hex::decode("05f191f40dff0e56fe8833282f5512cf8f68e28794140f650324220f5ed3ee7e4d")
            .expect("valid hex");
    let bob_identity_private =
        hex::decode("38393385efdc31e5565c20610e665429430f6bfb9320adb4e5cbff680febae6e")
            .expect("valid hex");
    let bob_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: bob_identity_public.as_slice().try_into().unwrap(),
        private_key: bob_identity_private.as_slice().try_into().unwrap(),
    };
    let registration_id_bob = 1;
    let bob_address = KeychatProtocolAddress {
        name: "bob".to_owned(),
        device_id: device_id2.into(),
    };

    // tom info
    let tom_identity_public =
        hex::decode("0515e97b26c5cbca6f39dce5cc55db22cd948598d370b87c1ce4919d665aeaab27")
            .expect("valid hex");
    let tom_identity_private =
        hex::decode("4875f9558f57bd7629d2792afaaf331ea10e6e8d1cbe28448e3850b923243b5c")
            .expect("valid hex");
    let tom_identity_key_pair = KeychatIdentityKeyPair {
        identity_key: tom_identity_public.as_slice().try_into().unwrap(),
        private_key: tom_identity_private.as_slice().try_into().unwrap(),
    };
    let registration_id_tom = 1;
    let tom_address = KeychatProtocolAddress {
        name: "tom".to_owned(),
        device_id: device_id3.into(),
    };

    /*
     * first alice to bob then  bob to alice
     */
    init(
        db_path.to_owned(),
        alice_identity_key_pair,
        registration_id_alice,
    )
    .expect("init error");
    let bob_info = generate_signed_pre_key_api(alice_identity_key_pair, bob_identity_private)?;

    let bob_signed_id = bob_info.signed_pre_key_id;
    println!("bob_sign_id {:?}", bob_signed_id);
    let bob_signed_key_public = bob_info.signed_pre_key_public;
    let bob_signed_signature = bob_info.signed_pre_key_signature;

    let bob_prekey_info = generate_pre_key_api(alice_identity_key_pair)?;

    // process_pre_key_bundle_api(
    //     alice_identity_key_pair,
    //     bob_address.clone(),
    //     registration_id_bob,
    //     device_id2.into(),
    //     KeychatIdentityKey {
    //         public_key: bob_identity_public.as_slice().try_into().unwrap(),
    //     },
    //     bob_signed_id.into(),
    //     bob_signed_key_public.clone(),
    //     bob_signed_signature.clone(),
    //     bob_prekey_info.pre_key_id.into(),
    //     bob_prekey_info.clone().pre_key_public,
    // )
    // .unwrap();

    let alice2bob_msg = "Alice to Bob";
    // alice to bob
    let alice2bob_encrypt = encrypt_signal(
        alice_identity_key_pair,
        alice2bob_msg.to_string(),
        bob_address.clone(),
        Some(false),
    )
    .unwrap();

    let _ = delete_session(alice_identity_key_pair, bob_address.clone());

    // process_pre_key_bundle_api(
    //     alice_identity_key_pair,
    //     bob_address.clone(),
    //     registration_id_bob,
    //     device_id2.into(),
    //     KeychatIdentityKey {
    //         public_key: bob_identity_public.as_slice().try_into().unwrap(),
    //     },
    //     bob_signed_id.into(),
    //     bob_signed_key_public,
    //     bob_signed_signature,
    //     bob_prekey_info.pre_key_id.into(),
    //     bob_prekey_info.pre_key_public,
    // )
    // .unwrap();

    let alice2bob_msg = "Alice to Bob";
    // alice to bob
    let alice2bob_encrypt = encrypt_signal(
        alice_identity_key_pair,
        alice2bob_msg.to_string(),
        bob_address.clone(),
        Some(false),
    )
    .unwrap();

    let alice2bob_bob_decrypt = decrypt_signal(
        bob_identity_key_pair,
        alice2bob_encrypt.ciphertext,
        alice_address.clone(),
        1,
        true,
    )
    .unwrap();
    println!(
        "alice2bob_bob_decrypt {:?}",
        String::from_utf8(alice2bob_bob_decrypt.plaintext).expect("valid utf8")
    );

    // let alice2bob_msg2 = "Alice to Bob 2";
    // // alice to bob
    // let alice2bob_encrypt2 = encrypt_signal(
    //     alice_identity_key_pair,
    //     alice2bob_msg2.to_string(),
    //     bob_address.clone(),
    //     Some(true)
    // )
    //     .unwrap();
    // println!("hhhhhh");
    //
    // let alice2bob_bob_decrypt2 = decrypt_signal(
    //     bob_identity_key_pair,
    //     alice2bob_encrypt2.ciphertext,
    //     alice_address.clone(),
    //     1,
    //     false,
    // )
    //     .unwrap();
    // println!(
    //     "alice2bob_bob_decrypt {:?}",
    //     String::from_utf8(alice2bob_bob_decrypt2.plaintext).expect("valid utf8")
    // );

    Ok(())
}
