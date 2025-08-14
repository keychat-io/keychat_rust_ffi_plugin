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

    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    // test for receive token
    // let encoded_token_8338: &str = "cashuBo2Ftd2h0dHBzOi8vODMzMy5zcGFjZTozMzM4YXVjc2F0YXSBomFpSADUzeNPraP9YXCDo2FhCGFzeEAyMzM1ZmFhY2VkZDUyNDdjMzJjMjFiZDJjZGUzNDE4YjFlMzE2NWFkYjVmMDRjOWE0MTQ5MGQ3NjdiMGJjODc4YWNYIQPHCF808VtYjwLrkCseYet1pWN2u0JTOaxnvvIMfNG116NhYRBhc3hAMDdlNmVjMmRjNzU4MTIzZjliMDNjYjQ4MWQ3YzY3Y2IxMGE5OGZlNTA0ZTYyMmVmM2JjMDUxZDczMWJlMGQxNGFjWCECk5P_E3gPuhepSfXGm63zGLbKWrH54oKAHd-8gzcETfijYWEYQGFzeEBiZmM5OWU4YzFkYTE5NjVmOWI1NzY3ODY5Y2YwZmRjMjk2NTU1OWMwOGZmMTExOGQxMTc3YTA2NjU3NjEzYTBjYWNYIQJwKve6PwFYETgFMycakZ388PywnpofjQvxwBwwrdcHdg".trim();
    // let re = api::receive_token(encoded_token_8338.to_string());
    // println!("receive token is {:?}", re);

    // test for get mints
    // let mints = api::get_mints();
    // println!("get_mints is {:?}", mints);

    // test for send token
    // let send = api::send(18, MINT_URL.to_string(), None);
    // println!("send token is {:?}", send);

    // test for request mint
    // let request_mint = api::request_mint(10, MINT_URL.to_string());
    // println!("request_mint is {:?}", request_mint);
    // lnbc20n1p58vdq6pp5w46w0ve9rq40skwzlvk08774mtm44xymnrc3w5znymuj62zj6cssdqqcqzpuxqrwzqsp5hy7ltttz0fwgu8mpgnvn9re0638vye4ug97dr6zyxeppzukh784s9qxpqysgqxz8zzps2fyefmx8d9mq92m8xej9dy7s6kpg0x0lnxtlct30ml65rlkhwdw80ugefmeyl2jr484x4l255rc9f8nxche0lnnf6nhjwxucp4yq5my

    // test for check quote
    let amounts = api::check_all_mint_quotes();
    println!("check_all_mint_quotes {:?}", amounts);

    // test for melt
    // let invoice = "lnbc90n1p5fmypfpp50uh0sqwwxrsvddhafgagnxpja6g3ldey7uz9uynz0lg2h9nx934qdqqcqzpuxqrwzqsp5vwr46r7h2tcx5g5v8erk7zwqqsj28865jq9r5m3tvdu5p9nasd8s9qxpqysgqjrdcamhzpc8ap2xk4899jg06elnrd58au0tu2nkult0fhsa777usfn9xvnfqd7afskg54e0lsd6vh8f6avsnjjeuvvf8u9q2yvry9ssq3y2l26".to_string();
    // let _melt = api::melt(invoice, MINT_URL.to_string(), None);

    // test for send stamp
    // let stamp = api::send_stamp(1,  vec![MINT_URL_MINIBITS.to_string()], None);
    // println!("send_stamp {:?}", stamp);

    // test for multi send stamp
    let mut stamps = vec![];
    for _i in 0..3 {
        let stamp = api::send_stamp(1, vec![MINT_URL.to_string()], None).unwrap();
        println!("send_stamp {:?}", stamp);

        stamps.push(stamp.0);
    }
    std::thread::sleep(std::time::Duration::from_secs(15));

    // // // test for check quote
    // // let amounts = api::check_all_mint_quotes();
    // // println!("check_all_mint_quotes {:?}", amounts);
    // // let amount = api::check_all_pending_proofs(MINT_URL_MINIBITS.to_string());
    // // println!("check_all_mint_quotes {:?}", amount);

    // test for multi receive stamps
    let _ = api::multi_receive(stamps);

    // let restore = api::restore(MINT_URL_MINIBITS.to_string(), None).unwrap();
    // println!("restore {:?}", restore);

    // let txs = api::get_all_transactions();
    // for tx in txs.unwrap() {
    //     println!("tx {:?}", tx);
    // }

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
