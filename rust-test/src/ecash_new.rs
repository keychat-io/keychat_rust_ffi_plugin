#[macro_use]
extern crate tracing;

use rust::api_cashu_new::{self as api, MnemonicInfo};

const DB_PATH: &str = "rustest-new.db";
const MINT_URL: &str = "https://8333.space:3338/";
const MINT_URL_MINIBITS: &str = "https://mint.minibits.cash/Bitcoin";

fn main() {
    let migen_words = MnemonicInfo::generate_words(12).unwrap();
    // let migen_words = "mushroom venture grab fatigue excite solve onion include minute joy trade anxiety";
    println!("migen_words is {:?}", migen_words);

    let init_db = api::init_db(DB_PATH.to_string(), Some(migen_words.to_owned()), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);

    // let mi = api::get_mnemonic_info();
    // println!("get_mnemonic_info is {:?}", mi);

    // let init_cashu = api::init_cashu(32);
    // println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    // test for receive token
    // let encoded_token_8338: &str = "cashuBo2FteCJodHRwczovL21pbnQubWluaWJpdHMuY2FzaC9CaXRjb2luYXVjc2F0YXSBomFpSABQBVDwSUFGYXCEpGFhEGFzeEAxOWQ5YmJjMGUxZjVlODU1ZDdmMTA4NWUwYTViZmFiZDRmYzNhMzU3M2FmZDExNWI3NTZlNzkzNWVhYzliNzkzYWNYIQO5_ShacCDkyFeSiwSS-2E566aRLBampc2pr1aqX3F_72Fko2FlWCAQGR8cA1CBGFV9EG2DZxPpz4asM9pF68gyVvM0U3EqX2FzWCCQFiW4LUSop3pd-jtwp4gVzCN8w4kpoiQJS6UWNc_MUGFyWCA5-YdB5LDCIB7aii5htT5Iml2X2JoBAxbfSTGr0oArmKRhYQhhc3hANDAxZDcxMTIyNDNlNDA4YWQ5MzI5NmI5NTE2M2RmZGVhNzg5MjhkMzFkNjlmZDIwMWVkZTllZTNlMjQ1YWIyMWFjWCEDLp5pTutYmgiGhrTDXFz4iVJskf_nSsprne7fOmJJQxhhZKNhZVggqwn9cXEYjbNED6njpBVhDm6u3_Ls-o-X0kG0r14C79Fhc1ggjQZBUO-wIIuGQKjYBwEGksOlahehkv4vHqa28TVkA5FhclggAOceN9TcTDCjdihD2KR4wYuRUV-Oc0bJWXbq3DyvR8akYWEEYXN4QDVjMzExM2UzM2IwYTIwMTI1NWE0ZDNjNGExNzU4Nzg4ZTU4ODMzYWZhMTM5MWU1ZjE5OWQ3MDM3ZmZiZTE5YjNhY1ghA1GbWbgq1LBdyT8US6gWgwqRZwXa_nbSdAccjWBnsiLOYWSjYWVYIAHuS5R4X8tZjCiIEByXKAtODrZGrATAP-Xa2K0WHM4oYXNYIErxgOlw8iDeOMejkFX_ryWhpNpP0IQsFG75BT9-a-IDYXJYIGbBv1zlCXVLT-ASOnkbIM9KPZMFBW_dCzBT3W5ItoI3pGFhAmFzeEAzNTc2M2RlNGFhMjI2MDBkYTZlN2I1YWI2MTM0N2VmMDNkY2MzMzY1ODk3NjQwZTM2NTU1ZWFkYTMzMDM3ZTYxYWNYIQIad4ST6DYAlwmOGKaacQQUxoFxjuYJ-UUv0PB24aHVO2Fko2FlWCAm2ILze7djcR_MxFQE_9Y21LgWaj_WJxbG28hDvaGJyGFzWCBWTMLgb3Un0rbvbqminq6iJK9XcG4gHf2-k1zDKcSEZmFyWCAGAGrQVBNdEuSRoC9hYO48TeCD70x9aUe0PYpJIcQMAw".trim();
    // let re = api::receive_token(encoded_token_8338.to_string());
    // println!("receive token is {:?}", re);

    // test for get mints
    // let mints = api::get_mints();
    // println!("get_mints is {:?}", mints);

    // test for send token
    // let send = api::send(20, MINT_URL.to_string(), None);
    // println!("send token is {:?}", send);

    // test for request mint
    // let request_mint = api::request_mint(12, MINT_URL.to_string());
    // println!("request_mint is {:?}", request_mint);
    // lnbc20n1p58vdq6pp5w46w0ve9rq40skwzlvk08774mtm44xymnrc3w5znymuj62zj6cssdqqcqzpuxqrwzqsp5hy7ltttz0fwgu8mpgnvn9re0638vye4ug97dr6zyxeppzukh784s9qxpqysgqxz8zzps2fyefmx8d9mq92m8xej9dy7s6kpg0x0lnxtlct30ml65rlkhwdw80ugefmeyl2jr484x4l255rc9f8nxche0lnnf6nhjwxucp4yq5my

    // test for check quote
    let amounts = api::check_all_mint_quotes();
    println!("check_all_mint_quotes {:?}", amounts);

    // test for melt
    // let invoice = "lnbc200n1p5fqne5pp5xr9ydqd3z7prmp5exw85xqadj8qujpjvtatq9z6ku9053rqe9muqdqqcqzpuxqrwzqsp5mzqpmu0u8haphqm4hd5lxvq45lhaqtu42psr2z9mrx6mgzekrthq9qxpqysgqhlm8kzuzryy8c357m8z78pvxg6rvtqeksntjjjneaxfavv2zwmqhd9euu4j3w4p3ty8fz5sscn6rygw6rz873ghmxuxe8z7r5m36fyqp7py35u".to_string();
    // let _melt = api::melt(invoice, MINT_URL.to_string(), None);

    // test for send stamp
    // let stamp = api::send_stamp(1,  vec![MINT_URL_MINIBITS.to_string()], None);
    // println!("send_stamp {:?}", stamp);

    // test for multi send stamp
    let mut stamps = vec![];
    for _i in 0..3 {
        let stamp = api::send_stamp(1, vec![MINT_URL_MINIBITS.to_string()], None).unwrap();
        println!("send_stamp {:?}", stamp);

        stamps.push(stamp);
    }
    std::thread::sleep(std::time::Duration::from_secs(15));

    // // test for check quote
    // let amounts = api::check_all_mint_quotes();
    // println!("check_all_mint_quotes {:?}", amounts);
    // let amount = api::check_all_pending_proofs(MINT_URL_MINIBITS.to_string());
    // println!("check_all_mint_quotes {:?}", amount);

    // test for multi receive stamps
    let _ = api::multi_receive(stamps);


    let restore = api::restore(MINT_URL_MINIBITS.to_string(), None).unwrap();
    println!("restore {:?}", restore);

    let txs = api::get_all_transactions();
    for tx in txs.unwrap() {
        println!("tx {:?}", tx);
    }

    // // test for get_cashu_transactions_with_offset
    // let cashu_txs = api::get_cashu_transactions_with_offset(0, 100);
    // for tx in cashu_txs.unwrap() {
    //     println!("cashu {:?}", tx);
    // }

    // // test for get_ln_transactions_with_offset
    // let ln_txs = api::get_ln_transactions_with_offset(0, 100);
    // for tx in ln_txs.unwrap() {
    //     println!("ln {:?}", tx);
    // }

    // test for print proofs
    // let _ = api::test_print_proofs(MINT_URL.to_string());
    // let _ = api::test_print_proofs(MINT_URL_MINIBITS.to_string());

    // let prepare_one_proofs = api::prepare_one_proofs(32, MINT_URL.to_string());
    // println!("prepare_one_proofs: {:?}", prepare_one_proofs);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL.to_string());
    // let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());
}