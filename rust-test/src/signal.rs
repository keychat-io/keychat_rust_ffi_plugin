use anyhow::Result;
use nostr::base64::engine::general_purpose;
use nostr::base64::*;
use rust::api_signal::signal_store::libsignal_protocol::*;
use rust::api_signal::*;

fn main() {
    // let _ = test_x3dh_db();
    // let _= test_state();
    // let _ = test_db();
    let _ = test_parse_prekey();
}

fn test_parse_prekey() -> Result<()> {
    let content = "NAjZtOSOChIhBcM1rtFjYuE6mdak4ql1iSVVbnaRGLH0Na09ZwN6qmRIGiEFXffW+BrVSqmLimFmOQauD3ehoRY/19Ee6ejQBNqxczoiswE0CiEFfrVjB2BM4qaumzeg2TtTcxLHQuqmoP1S9+DXuw1fPHcQABgAIoABbDOaq43qsrmQuvWEsAIrE1DErRiC5tDtmpfWOk4rT33bi7AkD5EFtVoMG5k4PhvvcVuTyg8BqXr6i1NpN7AMunpuplGn79He8TXMluJ6jZcI7HEKkvO0irXWZsEADnHYcLY/n0qhPk4cHwxQGGJmkp37VYFGyYdd8q71roHVi9RyDE3+VfM5iCgAMNyA+ocM";
    let cipher_text = general_purpose::STANDARD.decode(content).unwrap();
    let re = parse_identity_from_prekey_signal_message(cipher_text).unwrap();
    println!("the result is {:?}", re);
    Ok(())
}

fn test_db() -> Result<()> {
    let db = "./signal.db";

    let device_id: DeviceId = 1.into();

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

    //test alice_identity_key_pair
    let t1 = get_all_alice_addrs(alice_identity_key_pair);
    println!("alice_identity_key_pair alice address {:?}", t1.unwrap());

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
    let device_id1: DeviceId = 1.into();
    let device_id2: DeviceId = 2.into();
    let device_id3: DeviceId = 3.into();

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
fn test_x3dh_db() -> Result<()> {
    let db_path = ".signal_test.db";
    let db_path2 = ".signal_test2.db";
    let db_path3 = ".signal_test3.db";
    let device_id1: DeviceId = 1.into();
    let device_id2: DeviceId = 2.into();
    let device_id3: DeviceId = 3.into();

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
    let bob_info = generate_signed_key_api(alice_identity_key_pair, bob_identity_private)?;

    let bob_signed_id = bob_info.0;
    println!("bob_sign_id {:?}", bob_signed_id);
    let bob_signed_key_public = bob_info.1;
    let bob_signed_signature = bob_info.2;

    let bob_prekey_info = generate_prekey_api(alice_identity_key_pair)?;

    process_prekey_bundle_api(
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
        bob_prekey_info.0.into(),
        bob_prekey_info.1,
    )
    .unwrap();

    let alice2bob_msg = "Alice to Bob";
    // alice to bob
    let alice2bob_encrypt = encrypt_signal(
        alice_identity_key_pair,
        alice2bob_msg.to_string(),
        bob_address.clone(),
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
        alice2bob_encrypt.0,
        alice_address.clone(),
        1,
        true,
    )
    .unwrap();
    println!(
        "alice2bob_bob_decrypt {:?}",
        String::from_utf8(alice2bob_bob_decrypt.0).expect("valid utf8")
    );
    let bobs_response_to_alice = "Bob response to Alice";
    // bob to Alice
    let bobs_response_to_alice_encrypt = encrypt_signal(
        bob_identity_key_pair,
        bobs_response_to_alice.to_string(),
        alice_address.clone(),
    )
    .unwrap();
    // alice decrypt bob
    let alice_decrypts_from_bob = decrypt_signal(
        alice_identity_key_pair,
        bobs_response_to_alice_encrypt.0,
        bob_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "alice_decrypts_from_bob {:?}",
        String::from_utf8(alice_decrypts_from_bob.0).expect("valid utf8")
    );

    /*
     * second alice to tom then  tom to alice
     */
    let tom_info = generate_signed_key_api(alice_identity_key_pair, tom_identity_private)?;
    let tom_prekey = generate_prekey_api(alice_identity_key_pair)?;

    let tom_sign_id = tom_info.0;
    println!("tom_sign_id {:?}", tom_sign_id);
    let tom_signed_key_public = tom_info.1;
    let tom_signed_signature = tom_info.2;
    process_prekey_bundle_api(
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
        tom_prekey.0.into(),
        tom_prekey.1,
    )
    .unwrap();

    let alice2tom_msg = "Alice to Tom";
    // alice to tom
    let alice2tom_encrypt = encrypt_signal(
        alice_identity_key_pair,
        alice2tom_msg.to_string(),
        tom_address.clone(),
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
        alice2tom_encrypt.0,
        alice_address.clone(),
        2,
        true,
    )
    .unwrap();
    println!(
        "alice2tom_tom_decrypt {:?}",
        String::from_utf8(alice2tom_tom_decrypt.0).expect("valid utf8")
    );
    let tom_response_to_alice = "Tom response to Alice";
    // tom to Alice
    let tom_response_to_alice_encrypt = encrypt_signal(
        tom_identity_key_pair,
        tom_response_to_alice.to_string(),
        alice_address.clone(),
    )
    .unwrap();
    // alice decrypt tom
    let alice_decrypts_from_tom = decrypt_signal(
        alice_identity_key_pair,
        tom_response_to_alice_encrypt.0,
        tom_address.clone(),
        2,
        false,
    )
    .unwrap();
    println!(
        "alice_decrypts_from_tom {:?}",
        String::from_utf8(alice_decrypts_from_tom.0).expect("valid utf8")
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
    )
    .unwrap();
    // bob decrypt Alice
    let alice2bob_bob_decrypt2 = decrypt_signal(
        bob_identity_key_pair,
        alice2bob_encrypt2.0,
        alice_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "alice2bob_bob_decrypt2 {:?}",
        String::from_utf8(alice2bob_bob_decrypt2.0).expect("valid utf8")
    );
    let bobs_response_to_alice2 = "Bob response to Alice again";
    // bob to Alice
    let bobs_response_to_alice_encrypt2 = encrypt_signal(
        bob_identity_key_pair,
        bobs_response_to_alice2.to_string(),
        alice_address.clone(),
    )
    .unwrap();
    // alice decrypt bob
    let alice_decrypts_from_bob2 = decrypt_signal(
        alice_identity_key_pair,
        bobs_response_to_alice_encrypt2.0,
        bob_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "alice_decrypts_from_bob2 {:?}",
        String::from_utf8(alice_decrypts_from_bob2.0).expect("valid utf8")
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
    )
    .unwrap();
    // tom decrypt alice
    let tom_decrypts_from_alice = decrypt_signal(
        tom_identity_key_pair,
        alice_response_to_tom_encrypt.0,
        alice_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "tom_decrypts_from_alice {:?}",
        String::from_utf8(tom_decrypts_from_alice.0).expect("valid utf8")
    );

    let tom2alice_msg = "Tom to Alice";
    // tom to alice
    let tom2alice_encrypt = encrypt_signal(
        tom_identity_key_pair,
        tom2alice_msg.to_string(),
        alice_address.clone(),
    )
    .unwrap();
    // alice decrypt tom
    let tom2alice_tom_decrypt = decrypt_signal(
        alice_identity_key_pair,
        tom2alice_encrypt.0,
        tom_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "tom2alice_tom_decrypt {:?}",
        String::from_utf8(tom2alice_tom_decrypt.0).expect("valid utf8")
    );
    Ok(())
}

fn test_x3dh_db2() -> Result<()> {
    let db_path = ".signal_test.db";
    let db_path2 = ".signal_test2.db";
    let db_path3 = ".signal_test3.db";
    let device_id1: DeviceId = 1.into();
    let device_id2: DeviceId = 2.into();
    let device_id3: DeviceId = 3.into();

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
    let bob_info = generate_signed_key_api(alice_identity_key_pair, bob_identity_private)?;

    let bob_signed_id = bob_info.0;
    println!("bob_sign_id {:?}", bob_signed_id);
    let bob_signed_key_public = bob_info.1;
    let bob_signed_signature = bob_info.2;

    let bob_prekey_info = generate_prekey_api(alice_identity_key_pair)?;

    process_prekey_bundle_api(
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
        bob_prekey_info.0.into(),
        bob_prekey_info.1,
    )
    .unwrap();

    let alice2bob_msg = "Alice to Bob";
    // alice to bob
    let alice2bob_encrypt = encrypt_signal(
        alice_identity_key_pair,
        alice2bob_msg.to_string(),
        bob_address.clone(),
    )
    .unwrap();
    init(
        db_path2.to_owned(),
        bob_identity_key_pair,
        registration_id_bob,
    )
    .expect("init error");
    // bob decrypt Alice
    let alice2bob_bob_decrypt = decrypt_signal(
        bob_identity_key_pair,
        alice2bob_encrypt.0,
        alice_address.clone(),
        1,
        true,
    )
    .unwrap();
    println!(
        "alice2bob_bob_decrypt {:?}",
        String::from_utf8(alice2bob_bob_decrypt.0).expect("valid utf8")
    );
    let bobs_response_to_alice = "Bob response to Alice";
    // bob to Alice
    let bobs_response_to_alice_encrypt = encrypt_signal(
        bob_identity_key_pair,
        bobs_response_to_alice.to_string(),
        alice_address.clone(),
    )
    .unwrap();
    // alice decrypt bob
    let alice_decrypts_from_bob = decrypt_signal(
        alice_identity_key_pair,
        bobs_response_to_alice_encrypt.0,
        bob_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "alice_decrypts_from_bob {:?}",
        String::from_utf8(alice_decrypts_from_bob.0).expect("valid utf8")
    );

    let bobs_response_to_alice2 = "Bob response to Alice2";
    // bob to Alice
    let bobs_response_to_alice_encrypt2 = encrypt_signal(
        bob_identity_key_pair,
        bobs_response_to_alice2.to_string(),
        alice_address.clone(),
    )
    .unwrap();
    // alice decrypt bob
    let alice_decrypts_from_bob2 = decrypt_signal(
        alice_identity_key_pair,
        bobs_response_to_alice_encrypt2.0,
        bob_address.clone(),
        1,
        false,
    )
    .unwrap();
    println!(
        "alice_decrypts_from_bob2 {:?}",
        String::from_utf8(alice_decrypts_from_bob2.0).expect("valid utf8")
    );

    Ok(())
}

// fn test_double_db5() -> Result<()> {

//     let db_path1 = ".signal_test.db";
//     let db_path2 = ".signal_test2.db";
//     let device_id1: DeviceId = 1.into();
//     let device_id2: DeviceId = 2.into();

//     let alice_identity_public =
//     hex::decode("a1551afd418c6af0666a082e6b827aa5a80757e99a0ae6c46dfcc43230769af9")
//         .expect("valid hex");
//     let alice_identity_private =
//     hex::decode("ef229182ad155c8e868467b23ef38efbd512913dddec4f825fe5f36fcdb99665")
//         .expect("valid hex");
//     let alice_identity_key_pair =
//         KeychatIdentityKeyPair{
//             identity_key: alice_identity_public.as_slice().try_into().unwrap(),
//             private_key: alice_identity_private.as_slice().try_into().unwrap(),};

//     let jack_identity_public = hex::decode("f6fbb2f9e1d36ed79f52497bfd820af8083d72e3176907aae935b9aebfb88743")
//         .expect("valid hex");
//     let jack_identity_private = hex::decode("c1fd3a136155723b66ce4d80e506cd31c6687ca3188daa2ce4632be1d1737e04")
//         .expect("valid hex");
//     let jack_identity_key_pair =
//         KeychatIdentityKeyPair{
//             identity_key: jack_identity_public.as_slice().try_into().unwrap(),
//             private_key: jack_identity_private.as_slice().try_into().unwrap(),};

//     let bob_identity_public =
//     hex::decode("6b5f947a51058a35df37bfc025ee23a463661f95a0e9b53dc3a35abfd439c4aa")
//         .expect("valid hex");
//     let bob_identity_private =
//     hex::decode("0838f95e3c48a1bece4948b8295b91f5f25b4b5c6b67ac3b922b510b9ff7dfb6")
//         .expect("valid hex");
//     let bob_identity_key_pair =
//         KeychatIdentityKeyPair{
//             identity_key: bob_identity_public.as_slice().try_into().unwrap(),
//             private_key: bob_identity_private.as_slice().try_into().unwrap(),};

//     let tom_identity_public =
//         hex::decode("d1c08c3cdfb094936a2591cfb12a693989b0064db6f91e97add0f00c5622bc56")
//         .expect("valid hex");
//     let tom_identity_private =
//         hex::decode("11e78c0a0b60dd8a7bdc21884a332978153b6229d22abec2ce2c88575b80e0e7")
//         .expect("valid hex");
//     let tom_identity_key_pair =
//         KeychatIdentityKeyPair{
//             identity_key: tom_identity_public.as_slice().try_into().unwrap(),
//             private_key: tom_identity_private.as_slice().try_into().unwrap(),};

//     let alice_address = KeychatProtocolAddress{name: "alice".to_owned(), device_id: device_id1.into()};
//     let alice_address2 = KeychatProtocolAddress{name: "alice".to_owned(), device_id: device_id2.into()};
//     let jack_address = KeychatProtocolAddress{name: "jack".to_owned(), device_id: device_id1.into()};
//     let jack_address2 = KeychatProtocolAddress{name: "jack".to_owned(), device_id: device_id2.into()};

//     let bob_address = KeychatProtocolAddress{name: "bob".to_owned(), device_id: device_id1.into()};
//     let bob_address2 = KeychatProtocolAddress{name: "bob".to_owned(), device_id: device_id2.into()};
//     let tom_address = KeychatProtocolAddress{name: "tom".to_owned(), device_id: device_id1.into()};
//     let tom_address2 = KeychatProtocolAddress{name: "tom".to_owned(), device_id: device_id2.into()};

//     let registration_id_alice = 1;
//     let registration_id_bob = 2;
//     let registration_id_tom = 3;
//     let registration_id_jack = 4;
// //     let db_path1 = ".signal_test3.db";
// //     let db_path2 = ".signal_test4.db";
// //     let device_id1: DeviceId = 1.into();
// //     let device_id2: DeviceId = 2.into();

// //     let alice_identity_public =
// //     hex::decode("a1551afd418c6af0666a082e6b827aa5a80757e99a0ae6c46dfcc43230769af9")
// //         .expect("valid hex");
// //     let alice_identity_private =
// //     hex::decode("ef229182ad155c8e868467b23ef38efbd512913dddec4f825fe5f36fcdb99665")
// //         .expect("valid hex");
// //     let alice_identity_key_pair =
// //         KeychatIdentityKeyPair{
// //             identity_key: alice_identity_public.as_slice().try_into().unwrap(),
// //             private_key: alice_identity_private.as_slice().try_into().unwrap(),};

// //     let jack_identity_public = hex::decode("f6fbb2f9e1d36ed79f52497bfd820af8083d72e3176907aae935b9aebfb88743")
// //         .expect("valid hex");
// //     let jack_identity_private = hex::decode("c1fd3a136155723b66ce4d80e506cd31c6687ca3188daa2ce4632be1d1737e04")
// //         .expect("valid hex");
// //     let jack_identity_key_pair =
// //         KeychatIdentityKeyPair{
// //             identity_key: jack_identity_public.as_slice().try_into().unwrap(),
// //             private_key: jack_identity_private.as_slice().try_into().unwrap(),};

// //     let bob_identity_public =
// //     hex::decode("6b5f947a51058a35df37bfc025ee23a463661f95a0e9b53dc3a35abfd439c4aa")
// //         .expect("valid hex");
// //     let bob_identity_private =
// //     hex::decode("0838f95e3c48a1bece4948b8295b91f5f25b4b5c6b67ac3b922b510b9ff7dfb6")
// //         .expect("valid hex");
// //     let bob_identity_key_pair =
// //         KeychatIdentityKeyPair{
// //             identity_key: bob_identity_public.as_slice().try_into().unwrap(),
// //             private_key: bob_identity_private.as_slice().try_into().unwrap(),};

// //     let tom_identity_public =
// //         hex::decode("d1c08c3cdfb094936a2591cfb12a693989b0064db6f91e97add0f00c5622bc56")
// //         .expect("valid hex");
// //     let tom_identity_private =
// //         hex::decode("11e78c0a0b60dd8a7bdc21884a332978153b6229d22abec2ce2c88575b80e0e7")
// //         .expect("valid hex");
// //     let tom_identity_key_pair =
// //         KeychatIdentityKeyPair{
// //             identity_key: tom_identity_public.as_slice().try_into().unwrap(),
// //             private_key: tom_identity_private.as_slice().try_into().unwrap(),};

// //     let alice_address = KeychatProtocolAddress{name: "alice".to_owned(), device_id: device_id1.into()};
// //     let alice_address2 = KeychatProtocolAddress{name: "alice".to_owned(), device_id: device_id2.into()};
// //     let jack_address = KeychatProtocolAddress{name: "jack".to_owned(), device_id: device_id1.into()};
// //     let jack_address2 = KeychatProtocolAddress{name: "jack".to_owned(), device_id: device_id2.into()};

// //     let bob_address = KeychatProtocolAddress{name: "bob".to_owned(), device_id: device_id1.into()};
// //     let bob_address2 = KeychatProtocolAddress{name: "bob".to_owned(), device_id: device_id2.into()};
// //     let tom_address = KeychatProtocolAddress{name: "tom".to_owned(), device_id: device_id1.into()};
// //     let tom_address2 = KeychatProtocolAddress{name: "tom".to_owned(), device_id: device_id2.into()};

// //     let registration_id_alice = 1;
// //     let registration_id_bob = 2;
// //     let registration_id_tom = 3;
// //     let registration_id_jack = 4;

//     let _ = init(db_path1.to_owned(), alice_identity_key_pair, registration_id_alice);
//     process_prekey_bundle_api(bob_address.clone(),registration_id_bob, device_id1.into(),
//         KeychatIdentityKey{public_key: bob_identity_public.as_slice().try_into().unwrap()}).unwrap();
//     let alice2bob_msg = "Alice to Bob";
//     let alice2bob_encrypt = encrypt_signal(alice2bob_msg.to_string(), bob_address.clone()).unwrap();

//     let _ = init(db_path2.to_owned(), bob_identity_key_pair, registration_id_bob);
//     process_prekey_bundle_api(alice_address.clone(),registration_id_alice, device_id1.into(),
//         KeychatIdentityKey{public_key: alice_identity_public.as_slice().try_into().unwrap()}).unwrap();
//     let alice2bob_bob_decrypt = decrypt_signal(
//         alice2bob_encrypt.0,
//         alice_address.clone(),
//         1,
//         true
//     ).unwrap();
//     println!("alice2bob_bob_decrypt {:?}", String::from_utf8(alice2bob_bob_decrypt).expect("valid utf8"));
//     let bobs_response_to_alice = "Bob response to Alice";
//     let bobs_response_to_alice_encrypt = encrypt_signal(bobs_response_to_alice.to_string(), alice_address.clone()).unwrap();

//     let _ = init(db_path1.to_owned(), alice_identity_key_pair, registration_id_alice);
//     let alice_decrypts_from_bob = decrypt_signal(bobs_response_to_alice_encrypt.0, bob_address.clone(), 1, false).unwrap();
//     println!("alice_decrypts_from_bob {:?}", String::from_utf8(alice_decrypts_from_bob).expect("valid utf8"));

//     let _ = init(db_path1.to_owned(), alice_identity_key_pair, registration_id_alice);
//     process_prekey_bundle_api(tom_address.clone(),registration_id_tom, device_id1.into(),
//         KeychatIdentityKey{public_key: tom_identity_public.as_slice().try_into().unwrap()}).unwrap();
//     let alice_2tom_msg = "Alice to Tom";
//     let alice_2tom_msg_encrypt = encrypt_signal(alice_2tom_msg.to_string(), tom_address.clone()).unwrap();

//     let _ = init(db_path2.to_owned(), tom_identity_key_pair, registration_id_tom);
//     process_prekey_bundle_api(alice_address2.clone(),registration_id_alice, device_id2.into(),
//         KeychatIdentityKey{public_key: alice_identity_public.as_slice().try_into().unwrap()}).unwrap();
//      let alice2tom_tom_decrypt = decrypt_signal(
//         alice_2tom_msg_encrypt.0,
//         alice_address2.clone(),
//         1,
//         true
//     ).unwrap();
//     println!("alice2tom_tom_decrypt {:?}", String::from_utf8(alice2tom_tom_decrypt).expect("valid utf8"));
//     let tom_response_to_alice = "Tom responce to Alice";
//     let tom_response_to_alice_encrypt = encrypt_signal(tom_response_to_alice.to_string(), alice_address2.clone()).unwrap();

//     let _ = init(db_path1.to_owned(), alice_identity_key_pair, registration_id_alice);
//     let alice_decrypts_from_tom = decrypt_signal(tom_response_to_alice_encrypt.0, tom_address.clone(), 2, false).unwrap();
//     println!("alice_decrypts_from_tom {:?}", String::from_utf8(alice_decrypts_from_tom).expect("valid utf8"));

//     let _ = init(db_path1.to_owned(), jack_identity_key_pair, registration_id_jack);
//     process_prekey_bundle_api(bob_address2.clone(),registration_id_bob, device_id2.into(),
//         KeychatIdentityKey{public_key: bob_identity_public.as_slice().try_into().unwrap()}).unwrap();
//     let jack2bob_msg = "Jack to Bob";
//     let jack2bob_encrypt = encrypt_signal(jack2bob_msg.to_string(), bob_address2.clone()).unwrap();

//     let _ = init(db_path2.to_owned(), bob_identity_key_pair, registration_id_bob);
//     process_prekey_bundle_api(jack_address.clone(),registration_id_bob, device_id1.into(),
//         KeychatIdentityKey{public_key: jack_identity_public.as_slice().try_into().unwrap()}).unwrap();
//     let jack2bob_bob_decrypt = decrypt_signal(
//         jack2bob_encrypt.0,
//         jack_address.clone(),
//         2,
//         true
//     ).unwrap();
//     println!("jack2bob_bob_decrypt {:?}", String::from_utf8(jack2bob_bob_decrypt).expect("valid utf8"));
//     let bobs_response_to_jack = "Bob response to Jack";
//     let bobs_response_to_jack_encrypt = encrypt_signal(bobs_response_to_jack.to_string(), jack_address.clone()).unwrap();

//     let _ = init(db_path1.to_owned(), jack_identity_key_pair, registration_id_jack);
//     let jack_decrypts_from_bob = decrypt_signal(bobs_response_to_jack_encrypt.0, bob_address2.clone(), 1, false).unwrap();
//     println!("jack_decrypts_from_bob {:?}", String::from_utf8(jack_decrypts_from_bob).expect("valid utf8"));

//     let _ = init(db_path1.to_owned(), jack_identity_key_pair, registration_id_jack);
//     process_prekey_bundle_api(tom_address2.clone(),registration_id_tom, device_id2.into(),
//         KeychatIdentityKey{public_key: tom_identity_public.as_slice().try_into().unwrap()}).unwrap();
//     let jack_2tom_msg = "Jack to Tom";
//     let jack_2tom_msg_encrypt = encrypt_signal(jack_2tom_msg.to_string(), tom_address2.clone()).unwrap();

//     let _ = init(db_path2.to_owned(), tom_identity_key_pair, registration_id_tom);
//     process_prekey_bundle_api(jack_address2.clone(),registration_id_jack, device_id2.into(),
//         KeychatIdentityKey{public_key: jack_identity_public.as_slice().try_into().unwrap()}).unwrap();
//     let jack2tom_tom_decrypt = decrypt_signal(
//         jack_2tom_msg_encrypt.0,
//         jack_address2.clone(),
//         2,
//         true
//     ).unwrap();
//     println!("jack2tom_tom_decrypt {:?}", String::from_utf8(jack2tom_tom_decrypt).expect("valid utf8"));
//     let tom_response_to_jack = "Tom responce to Jack";
//     let tom_response_to_jack_encrypt = encrypt_signal(tom_response_to_jack.to_string(), jack_address2.clone()).unwrap();

//     let _ = init(db_path1.to_owned(), jack_identity_key_pair, registration_id_jack);
//     let jack_decrypts_from_tom = decrypt_signal(tom_response_to_jack_encrypt.0, tom_address2.clone(), 2, false).unwrap();
//     println!("jack_decrypts_from_tom {:?}", String::from_utf8(jack_decrypts_from_tom).expect("valid utf8"));

//     // let alice_response = "hello";
//     // let alice_outgoing = encrypt_signal(alice_response.to_string(), bob_address.clone()).unwrap();
//     // println!("alice_outgoing receive_addr {:?}", alice_outgoing.1);
//     // let bob_decrypts = decrypt_signal(alice_outgoing.0, alice_address.clone(), 1, false).unwrap();
//     // println!("bob_decrypts {:?}", String::from_utf8(bob_decrypts).expect("valid utf8"));

//     // let session1 = get_session("alice".to_string(), "1".to_string()).unwrap();
//     // let session2 = get_session("bob".to_string(), "1".to_string()).unwrap();

//     // let session5 = get_session("alice".to_string(), "1".to_string()).unwrap();
//     // println!("session5 {:?}", session5.unwrap().alice_addresses);

//     // let session3 = get_session("bob".to_string(), "1".to_string()).unwrap();
//     // println!("{:?}, {:?}", session1.clone().unwrap().bob_address, session1.clone().unwrap().address);
//     // println!("{:?}, {:?}", session2.clone().unwrap().alice_addresses, session2.clone().unwrap().address);
//     // println!("{:?}, {:?}", session3.clone().unwrap().alice_addresses, session3.clone().unwrap().address);

//     Ok(())
// }

//     // let session5 = get_session("alice".to_string(), "1".to_string()).unwrap();
//     // println!("session5 {:?}", session5.unwrap().alice_addresses);

//     // let session3 = get_session("bob".to_string(), "1".to_string()).unwrap();
//     // println!("{:?}, {:?}", session1.clone().unwrap().bob_address, session1.clone().unwrap().address);
//     // println!("{:?}, {:?}", session2.clone().unwrap().alice_addresses, session2.clone().unwrap().address);
//     // println!("{:?}, {:?}", session3.clone().unwrap().alice_addresses, session3.clone().unwrap().address);

//     Ok(())
// }
