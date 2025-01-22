use rust::api_nostr as nostr;
use serde_json::json;

#[cfg(test)]
mod tests {

    use super::*;
    use nostr::bip39::Mnemonic;

    #[test]
    fn test_generate() {
        let my_keys = nostr::generate_secp256k1();

        println!("my_keys :{:?}", my_keys);

        let my_keys2 = nostr::generate_from_mnemonic(None).unwrap();
        println!("my_keys2 :{:?}", my_keys2.curve25519_pk);
        println!("my_keys2 :{:?}", my_keys2);

        // assert_eq!(re, "Hello, world!");
    }

    #[test]
    fn test_imports() {
        let phrase =
            "blouse only remind galaxy genuine south various mother general duty pave boost";
        let accounts = nostr::import_from_phrase_with(phrase.to_owned(), None, 0, 10).unwrap();
        assert_ne!(accounts[0].prikey, accounts[1].prikey);
        assert_ne!(accounts[0].pubkey, accounts[1].pubkey);
        assert_ne!(accounts[0].pubkey_bech32, accounts[1].pubkey_bech32);
        assert_ne!(accounts[0].curve25519_sk, accounts[1].curve25519_sk);
        assert_ne!(accounts[0].curve25519_pk, accounts[1].curve25519_pk);
        let accounts2 = nostr::import_from_phrase_with(phrase.to_owned(), None, 0, 10).unwrap();
        assert_eq!(accounts, accounts2);
    }

    #[test]
    fn test_import() {
        let phrase =
            "blouse only remind galaxy genuine south various mother general duty pave boost";
        let my_keys = nostr::import_from_phrase(phrase.to_string(), None, None).unwrap();

        assert_eq!(
            my_keys.pubkey,
            "659f7522523c9b3df8d4eddd318f1c4574198e2b93de7a7f68ca7b6b4ee5f295"
        );
        assert_eq!(
            my_keys.prikey,
            "df4dc4fe3326e9e4c774c2df83db8b583a0d83d923b4dae1d10b8779b4ec055c"
        );
        println!("{:?}", my_keys.curve25519_sk_hex);
        assert_eq!(
            my_keys.curve25519_pk_hex,
            Some(
                "05ab03d3e1e2c9ce3b7f7cea79131367ed7160ef769051ba788d6cfec9e251b27a".to_lowercase()
            )
        );

        assert_eq!(
            my_keys.curve25519_sk,
            Some(
                [
                    72, 252, 59, 93, 9, 125, 223, 236, 246, 176, 61, 42, 201, 104, 241, 54, 122, 0,
                    75, 194, 150, 205, 59, 201, 225, 66, 139, 81, 228, 74, 215, 64
                ]
                .to_vec()
            )
        );
    }

    #[test]
    fn test_import2() {
        let phrase =
            "legend claw leader monster swallow uncle resemble reward short name explain tray";
        let my_keys = nostr::import_from_phrase(phrase.to_string(), None, None).unwrap();
        println!("{:?}", my_keys.curve25519_pk);
        println!("{:?}", my_keys.curve25519_pk_hex);
        println!("{:?}", my_keys.curve25519_sk_hex);
        println!("{:?}", my_keys.pubkey);

        // assert_eq!(
        //     my_keys.pubkey,
        //     "659f7522523c9b3df8d4eddd318f1c4574198e2b93de7a7f68ca7b6b4ee5f295"
        // );
        // assert_eq!(
        //     my_keys.prikey,
        //     "df4dc4fe3326e9e4c774c2df83db8b583a0d83d923b4dae1d10b8779b4ec055c"
        // );
        // println!("{:?}",my_keys.curve25519_sk_hex );
        // assert_eq!(
        //     my_keys.curve25519_pk_hex,
        //     Some("1701E4A3AC802B6E2CE246A2F2F0C54A63BC294218C0E262903B68CAD9051F98".to_lowercase())
        // );
        //
        // assert_eq!(
        //     my_keys.curve25519_sk,
        //     Some(
        //         [
        //             73, 252, 59, 93, 9, 125, 223, 236, 246, 176, 61, 42, 201, 104, 241, 54, 122, 0,
        //             75, 194, 150, 205, 59, 201, 225, 66, 139, 81, 228, 74, 215, 64
        //         ]
        //             .to_vec()
        //     )
        // );
    }

    #[test]
    fn get_hex_pubkey() {
        let pubkey_bech32: String =
            "npub14f8usejl26twx0dhuxjh9cas7keav9vr0v8nvtwtrjqx3vycc76qqh9nsy".to_string();
        let pubkey_hex =
            "aa4fc8665f5696e33db7e1a572e3b0f5b3d615837b0f362dcb1c8068b098c7b4".to_string();
        let result = nostr::get_hex_pubkey_by_bech32(pubkey_bech32);

        // println!("result :{:?}", result.unwrap());
        assert_eq!(result, pubkey_hex);
    }

    #[test]
    fn get_hex_pubkey2() {
        const PUBKEY_HEX: &str = "aa4fc8665f5696e33db7e1a572e3b0f5b3d615837b0f362dcb1c8068b098c7b4";
        let result = nostr::get_hex_pubkey_by_bech32(PUBKEY_HEX.to_string());

        // println!("result :{:?}", result.unwrap());
        assert_eq!(result, PUBKEY_HEX);
    }
    const ALICE_SK: &str = "6b911fd37cdf5c81d4c0adb1ab7fa822ed253ab0ad9aa18d77257c88b29b718e";
    // const BOB_SK: &str = "b0a1938dedb4eedb6eb5ea79a7288531123a53cbb6f86a68e59eefd92648b97c";

    const PUBKEY_HEX: &str = "3119e78c156f961669472305706f796abc929e4e2961d82abdf1311b2c10f77b";

    #[test]
    fn get_nip04() {
        let result = nostr::get_encrypt_event(
            ALICE_SK.to_string(),
            PUBKEY_HEX.to_string(),
            "1234".to_string(),
            None,
        )
        .unwrap();
        let res = nostr::verify_event(result.to_string());
        // println!("result :{:?}", result.unwrap());
        assert_eq!(res.unwrap().tags[0][1], PUBKEY_HEX);
    }

    #[test]
    fn get_nip042() {
        let result = nostr::encrypt(
            ALICE_SK.to_string(),
            PUBKEY_HEX.to_string(),
            "1234".to_string(),
        );

        println!("result :{:?}", result.unwrap());
        // assert_eq!(result.unwrap().clone(), PUBKEY_HEX);
    }

    #[test]
    fn get_unencrypt_event() {
        let result = nostr::get_unencrypt_event(
            ALICE_SK.to_string(),
            vec![PUBKEY_HEX.to_string()],
            "1234".to_string(),
            None,
            4,
        );

        println!("result :{:?}", result.unwrap());
        // assert_eq!(result.unwrap().clone(), PUBKEY_HEX);
    }

    #[test]
    fn get_encrypt_event() {
        let result = nostr::get_encrypt_event(
            ALICE_SK.to_string(),
            PUBKEY_HEX.to_string(),
            "1234".to_string(),
            None,
        );

        println!("result :{:?}", result.unwrap());
        // assert_eq!(result.unwrap().clone(), PUBKEY_HEX);
    }

    #[test]
    fn sign_schnorr() {
        let prikey = "dd733a9b4610cd05e8dbf4ef047bef4e7b3ec6b39e94caa1727b1297455c0120";
        let pubkey: &str = "327cc50855cb14db66d9dcd4c797e7341fd5139333760d61a15222aa02677d94";

        let content = "Hello World!";

        let result = nostr::sign_schnorr(prikey.to_string(), content.to_string());

        println!("result :{:?}", result);
        let result2 = nostr::verify_schnorr(
            pubkey.to_string(),
            result.unwrap().clone(),
            content.to_string(),
            true,
        );
        assert_eq!(result2.unwrap(), true);
    }

    #[test]
    fn verify_schnorr() {
        let sig = "fe75c6df44443c85645370e1855ff2e06c82162365497b7faae4b36e75207a3c41e43291d2cfaa9c19def0ce8d89d5e3b97fa16b5810e8f15956079e3ab037d4";
        let content = "Hello World!";
        let pubkey: &str = "327cc50855cb14db66d9dcd4c797e7341fd5139333760d61a15222aa02677d94";

        let result2 = nostr::verify_schnorr(
            pubkey.to_string(),
            sig.to_string(),
            content.to_string(),
            true,
        );
        assert_eq!(result2.unwrap(), true);
    }

    #[test]
    fn decrypt() {
        let prikey = "dd733a9b4610cd05e8dbf4ef047bef4e7b3ec6b39e94caa1727b1297455c0120";
        let pubkey: &str = "327cc50855cb14db66d9dcd4c797e7341fd5139333760d61a15222aa02677d94";

        let content = "Hello World!";
        let encrypted = nostr::encrypt(prikey.to_string(), pubkey.to_string(), content.to_string());
        // let encrypted_content = "92wADtNzdf9FuJNrrrafTA==?iv=OAhsPYoWGn0wDpRl0cqNXw==";
        print!("encrypted :{:?}", encrypted);
        let result = nostr::decrypt(
            prikey.to_string(),
            pubkey.to_string(),
            encrypted.unwrap().to_string(),
        );

        println!("result :{:?}", result);
    }


    #[test]
    fn decrypt_nip44() {
        let prikey = "nsec1vc3d6tmwkq33p86f9r2aqapwm4zv0aur34l4l3fl2vjr6ultgrtqael3up";
        let pubkey: &str = "4a303ead948bd0977084b5afba2d1007179c3d93d8122dc31c7eba7bec0eb058";

        let content = "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";
         // let encrypted_content = "92wADtNzdf9FuJNrrrafTA==?iv=OAhsPYoWGn0wDpRl0cqNXw==";
        let result = nostr::decrypt_nip44(
            prikey.to_string(),
            pubkey.to_string(),
            content.to_string(),
        );

        println!("result :{:?}", result);
    }

    #[test]
    fn decrypt_event() {
        let prikey = "dd733a9b4610cd05e8dbf4ef047bef4e7b3ec6b39e94caa1727b1297455c0120";

        let data = json!({"id":"be46269f89b06f4380e6049fa5664b5cdef0b9d577b37d7d3d8b2d2e42d0dfb1","pubkey":"327cc50855cb14db66d9dcd4c797e7341fd5139333760d61a15222aa02677d94","created_at":1693205651,"kind":4,"tags":[["p","327cc50855cb14db66d9dcd4c797e7341fd5139333760d61a15222aa02677d94"]],"content":"hZAznH6TUFrfHlDYVwpAIw==?iv=iQ5LnIblKwhhKbxalVRNpQ==","sig":"a2fa751ec56da10f2f14d8249b18e5f70e0bfbca2fe3c9e40f1f3252ed64873b7f9e2ab64b4357fef5261418c354a7505b0a774e9e2db9585f0e5d6b71868d60"});
        let res = nostr::decrypt_event(prikey.to_string(), data.to_string());
        print!("res :{:?}", res.unwrap());
    }

    #[test]
    fn verify_event() {
        let event_str: &str = "{\"created_at\":1693230349,\"content\":\"y9POdVGKJoP7joTZrHCYsQ==?iv=X9aFulrnMbcjitzc1r\\/cVg==\",\"kind\":4,\"tags\":[[\"p\",\"d9a2c74b85004e1a669ea0b9f9ceef661afb4e44e22350a92dead8aabc10d62f\"]],\"pubkey\":\"327cc50855cb14db66d9dcd4c797e7341fd5139333760d61a15222aa02677d94\",\"id\":\"38ac2807e5e6f20805ff2831392e0a5a3a9cbdbea90992c41b73139cc74524e4\",\"sig\":\"c60629194299ccf5b5d759bdd2a4de428cbc67f68afefe01a0acf46e3a7eb88c3246d1718b63015d6ef162981b7bbdc2fe3a1acb9bdae901df0900aa3166a05c\"}";
        let res = nostr::verify_event(event_str.to_string());
        print!("res :{:?}", res.unwrap());
    }

    #[test]
    fn get_hex_prikey_by_bech32() {
        let prikey_bech32: &str = "nsec1qn4d6u3uxnyu05wrtepjsw2jpqxk4m0lzdv0e8n6amh0u2yupccsh28kvf";
        let prikey_hex: &str = "04eadd723c34c9c7d1c35e43283952080d6aedff1358fc9e7aeeeefe289c0e31";
        let res = nostr::get_hex_prikey_by_bech32(prikey_bech32.to_string());
        // print!("res :{:?}", res);
        assert_eq!(&res, prikey_hex);
    }

    #[test]
    fn get_bech32_prikey_by_hex() {
        let prikey_bech32: &str = "nsec1qn4d6u3uxnyu05wrtepjsw2jpqxk4m0lzdv0e8n6amh0u2yupccsh28kvf";
        let prikey_hex: &str = "04eadd723c34c9c7d1c35e43283952080d6aedff1358fc9e7aeeeefe289c0e31";
        let res = nostr::get_bech32_prikey_by_hex(prikey_hex.to_string());
        // print!("res :{:?}", res);
        assert_eq!(&res, &prikey_bech32);
    }

    #[test]
    fn get_hex_pubkey_by_bech32() {
        let prikey_bech32: &str = "npub1aqvcvjd4qrvasvvw4d9hyur8x4w45p63twjd7ssnetfgnr7yplfslzt7z6";
        let prikey_hex: &str = "e8198649b500d9d8318eab4b727067355d5a07515ba4df4213cad2898fc40fd3";
        let res = nostr::get_hex_pubkey_by_bech32(prikey_bech32.to_string());
        assert_eq!(&res, prikey_hex);
    }

    #[test]
    fn import_key() {
        let prikey_hex: &str = "f342982efe443c41b56020ad590bfa4f10c43e42540247b40dccc3343128d602";
        let res = nostr::import_key(prikey_hex.to_string()).unwrap();
        print!("res :{:?}", res);
    }

    #[test]
    fn get_hex_pubkey_by_prikey() {
        let prikey_hex: &str = "f342982efe443c41b56020ad590bfa4f10c43e42540247b40dccc3343128d602";
        let pubkey_hex: &str = "29350dfb9315432584b1333ef13e7c06458bdd61994522333877eb15dfdf80f8";
        let res = nostr::get_hex_pubkey_by_prikey(prikey_hex.to_string()).unwrap();
        print!("res :{:?}", res);
        assert_eq!(&res, pubkey_hex);
    }

    #[test]
    fn get_bech32_by_hex() {
        let pubkey_hex: &str = "29350dfb9315432584b1333ef13e7c06458bdd61994522333877eb15dfdf80f8";
        let npub_pubkey: &str = "npub19y6sm7unz4pjtp93xvl0z0nuqezchhtpn9zjyvecwl43th7lsruq74x7c0";
        let res = nostr::get_bech32_pubkey_by_hex(pubkey_hex.to_string());
        assert_eq!(&res, npub_pubkey);
    }

    // #[test]
    // fn generate_mnemonic_and_private_key() {
    //     let phrase = nostr::generate_mnemonic();
    //     // let res = "snap view range apology comfort deputy silent pond potato detect tribe sight";
    //     // let phrase = "crop cash unable insane eight faith inflict route frame loud box vibrant";

    //     let keypair = nostr::generate_curve25519_keypair(phrase.to_string()).unwrap();

    //     println!("25519 signingkey :{:?}", keypair.0.as_bytes());
    //     println!("pubkey :{:?}", keypair.1.as_bytes());

    //     // secp256k1
    //     let secp = nostr::import_from_phrase(phrase).unwrap();
    //     println!("secp256k1 :{:?}", secp);
    //     assert_eq!(1, 1);
    // }

    #[test]
    fn mnemonic_test() {
        let m1 = Mnemonic::generate(12).unwrap();
        let m2: Mnemonic = m1.to_string().parse().unwrap();
        let m3: Mnemonic = Mnemonic::from_entropy(&m2.to_entropy()).unwrap();
        println!("phrase :{:?}", m1.to_string());
        println!("entropy :{:?}", m1.to_entropy());

        assert_eq!(m1.to_entropy(), m2.to_entropy(), "Entropy must be the same");
        assert_eq!(m1.to_entropy(), m3.to_entropy(), "Entropy must be the same");
        assert_eq!(m1.to_string(), m2.to_string(), "Phrase must be the same");
        assert_eq!(m1.to_string(), m3.to_string(), "Phrase must be the same");
    }

    #[test]
    fn generate_nostr_address_from_seed() {
        let seed = "[80, 196, 159, 146, 160, 164, 224, 233, 119, 250, 130, 50, 183, 127, 179, 163, 27, 117, 18, 64, 80, 99, 80, 255, 39, 172, 218, 131, 17, 65, 253, 64]-[5, 237, 169, 222, 100, 20, 59, 47, 213, 141, 184, 172, 141, 69, 254, 181, 63, 241, 90, 172, 222, 236, 118, 174, 137, 71, 29, 193, 117, 26, 31, 243, 68]".to_string();
        let result = nostr::generate_seed_from_ratchetkey_pair(seed);
        // println!("{}", result.unwrap());
        let seed2 = "[104, 89, 141, 47, 114, 129, 67, 58, 100, 39, 26, 27, 231, 16, 51, 120, 113, 148, 61, 34, 254, 113, 176, 92, 254, 20, 171, 39, 123, 160, 200, 83]-[5, 234, 35, 77, 143, 103, 193, 161, 172, 104, 213, 148, 240, 233, 62, 75, 155, 141, 194, 39, 128, 55, 138, 253, 142, 89, 207, 72, 99, 203, 146, 230, 22]".to_string();
        let result2 = nostr::generate_seed_from_ratchetkey_pair(seed2);
        // println!("{}", result2.unwrap());
        assert_eq!(result.unwrap(), result2.unwrap());
    }

    #[test]
    fn generate_msg_hash_from_seed() {
        let seed = "[170, 41, 77, 31, 221, 127, 111, 91, 158, 70, 196, 247, 123, 130, 168, 183, 116, 107, 229, 239, 28, 101, 180, 185, 180, 160, 156, 248, 136, 255, 56, 141]-[123, 118, 67, 226, 103, 95, 250, 159, 37, 188, 27, 63, 179, 54, 138, 232]".to_string();
        let result = nostr::generate_message_key_hash(seed);
        println!("{}", result.unwrap());
    }

    #[test]
    fn curve25519_verify() {
        let phrase = "crop cash unable insane eight faith inflict route frame loud box vibrant";

        let keypair = nostr::generate_curve25519_keypair(phrase.to_string(), None, None).unwrap();
        let private_key = keypair.0;
        let public_key = keypair.1;

        println!("25519 private_key :{:?}", private_key);

        println!("25519 public_key :{:?}", hex::encode(&public_key));
        let message: &[u8] = b"All I want is to pet all of the dogs.";

        let signed = nostr::curve25519_sign(private_key, message.to_vec()).unwrap();
        let sig = "0344aa9aeb7288bac1b2f329f7a4f776d335924e2903ba7db82d5242eadd3cd4f2cf8bd9f31f55e9adc8bdd624eb55a5d3a7fe61031b838098574fd846434d83";
        // assert_eq!(signed, sig);

        let verified = nostr::curve25519_verify(
            public_key.clone(),
            message.to_vec(),
            signed.to_ascii_uppercase(),
        );
        // println!("{}", verified.unwrap());
        assert!(verified.is_ok());

        let verified = nostr::curve25519_verify(public_key, message.to_vec(), sig.to_owned());
        assert!(verified.is_ok());
    }
    #[test]
    fn curve25519_prikey_pubkey() {
        let bob_identity_public =
            hex::decode("05f191f40dff0e56fe8833282f5512cf8f68e28794140f650324220f5ed3ee7e4d")
                .expect("valid hex");
        let bob_identity_private =
            hex::decode("38393385efdc31e5565c20610e665429430f6bfb9320adb4e5cbff680febae6e")
                .expect("valid hex");
        let prikey: String =
            "38393385efdc31e5565c20610e665429430f6bfb9320adb4e5cbff680febae6e".to_string();
        let res = nostr::curve25519_get_pubkey(prikey).unwrap();
        println!("{:}", res);
        assert_eq!(
            res,
            "05f191f40dff0e56fe8833282f5512cf8f68e28794140f650324220f5ed3ee7e4d"
        );
    }

    #[test]
    fn string_test() {
        let s = "secret seed";
        let bytes = b"secret seed";
        let bytes2 = s.as_bytes();
        let s2 = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(s, s2);
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn nip17() {
        let s = "secret seed 17";
        let sender_kp = nostr::generate_simple().unwrap();
        let receiver_kp = nostr::generate_simple().unwrap();

        let gift_json = nostr::create_gift_json(
            14,
            sender_kp.prikey,
            receiver_kp.pubkey,
            s.to_owned(),
            None,
            None,
            None,
        )
        .unwrap();

        // use nostr::nostr::util::JsonUtil;
        let gift = nostr::verify_event(gift_json).unwrap();
        let rumor = nostr::decrypt_gift(receiver_kp.prikey, gift.pubkey, gift.content).unwrap();
        assert_eq!(rumor.content, s);

        assert_ne!(rumor.created_at, gift.created_at);
    }

    #[test]
    fn nip17_without_timestamp_tweaked() {
        let s = "secret seed 17";
        let sender_kp = nostr::generate_simple().unwrap();
        let receiver_kp = nostr::generate_simple().unwrap();

        let gift_json = nostr::create_gift_json(
            14,
            sender_kp.prikey,
            receiver_kp.pubkey,
            s.to_owned(),
            None,
            None,
            Some(false),
        )
        .unwrap();

        // use nostr::nostr::util::JsonUtil;
        let gift = nostr::verify_event(gift_json).unwrap();
        let rumor = nostr::decrypt_gift(receiver_kp.prikey, gift.pubkey, gift.content).unwrap();
        assert_eq!(rumor.content, s);

        assert_eq!(rumor.created_at, gift.created_at);
    }

    #[test]
    fn test_sign_event_invalid_keys() {
      let sender_keys = "246ad4386c29680e5d9de9d3258708268d54c64a536c468b26b44b7dd921bc9a".to_string();
      let content = "Test content".to_string();
      let created_at = 1735021788;
      let kind = 4;
      let tags = vec![vec!["744bc6815ead8ae5db97a1f425ee8aead700a0ebd7ea9968704aee3e3f026f27".to_string()]];
  
      let result = nostr::sign_event(sender_keys, content, created_at, kind, tags);
      println!("result :{:?}", result);
      assert!(result.is_ok());
    }
  
}
