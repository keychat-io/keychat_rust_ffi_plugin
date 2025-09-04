#[macro_use]
extern crate tracing;

use rust::api_cashu::{self as api, MnemonicInfo};

const DB_PATH: &str = "rustest-new.db";
const MINT_URL: &str = "https://8333.space:3338/";
const MINT_URL_MINIBITS: &str = "https://mint.minibits.cash/Bitcoin";

fn main() {
    let words = MnemonicInfo::generate_words(12).unwrap();
    // let words = "unaware awesome finish calm deputy doll roast own dumb liar afraid differ";

    // // let migen_words = "mushroom venture grab fatigue excite solve onion include minute joy trade anxiety";
    // println!("migen_words is {:?}", words);

    // let tokens =
    //     api::cashu_v1_init_send_all("ecash.db".to_string(), Some(words.to_owned()));
    // println!("send all tokens {:?}", tokens);

    // let words = "bunker feel away slide trip girl amazing resource veteran direct cotton blanket";

    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);

    // let mi = api::get_mnemonic_info();
    // println!("get_mnemonic_info is {:?}", mi);

    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // if tokens.is_ok() {
    //     for token in tokens.unwrap() {
    //         let re = api::receive_token(token);
    //         println!("receive token is {:?}", re);
    //     }
    // }
    

    // test for receive token
    // let encoded_token_8338: &str = "cashuBo2Ftd2h0dHBzOi8vODMzMy5zcGFjZTozMzM4YXVjc2F0YXSBomFpSADUzeNPraP9YXCCpGFhBGFzeEA2YWM4MGExNTI0ZDQ4Y2RkOTBhYjIzMTkzNWI0YzgxZWNmZWEzZTg0NmI1MDVlNzM0MjMzZDhhNWM5NTdlOTI2YWNYIQJs59_-_747aI3JZzDXT-Ct5pmLM-hUm0kaGR7paTPSGGFko2FlWCARFE6PVaRqnGqQhJU1W7LlaKUJJxkVTzqWf5dAgYMb6mFzWCCqQ9aXX0Nt-DtMBQCKaSH_VCZvJhiDggaQ7qM53M7U5WFyWCDnz1i31hf0nrYneMwQjSUvsMK3G1VCEwYA1n3GE8vTnaRhYQFhc3hAZjFkNDA0ZTVlM2ZmOWE3MTY1ODgyNDM1NmVhMmJkNTY5OWY2OWNjZDkxOGU3YTczMDFmNWJiNmI2M2FjOGIzM2FjWCEC0so6IWiARRb-6Fk2SOuTKKQiG9hnBKyWgXZcwY7Zuy5hZKNhZVggRU4V2IfuDmpv_ulZ0AhCLa6A6lGsqKT6X3NlXQKBuyxhc1ggvMaH6mN8GE5FnNAYbaVJnlOfo07CBFQJsOWxA4HAqIhhclggCTbgnLg3SbEC8XRuccxZ83XawwlWhZAH-DH_pnRIFIg".trim();
    // let re = api::receive_token(encoded_token_8338.to_string());
    // println!("receive token is {:?}", re);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);
    // return;

    let words = MnemonicInfo::generate_words(12).unwrap();
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);

    // test for get mints
    // let mints = api::get_mints();
    // println!("get_mints is {:?}", mints);

    // test for send token
    // let send = api::send_all(MINT_URL_MINIBITS.to_string());
    // println!("send token is {:?}", send);

    // let send = api::send_all(MINT_URL.to_string());
    // println!("send token is {:?}", send);

    // let send = api::send(1, MINT_URL_MINIBITS.to_string(), None);
    // println!("send token is {:?}", send);

    // let send = api::send(1, MINT_URL_MINIBITS.to_string(), None);
    // println!("send token is {:?}", send);

    let txs = api::get_ln_pending_transactions();
    for tx in txs.unwrap() {
        println!("ln tx pending {:?}", tx);
    }

    let txs = api::get_cashu_pending_transactions();
    for tx in txs.unwrap() {
        println!("cashu tx pending {:?}", tx);
    }

    let check_tx = api::check_pending();
    println!("check_pending is {:?}", check_tx);

    // let check_proofs_tx: Result<(u64, u64, u64), anyhow::Error> = api::check_proofs();
    // println!("check_proofs is {:?}", check_proofs_tx);

    // test for request mint
    // let request_mint = api::request_mint(10, MINT_URL_MINIBITS.to_string());
    // println!("request_mint is {:?}", request_mint);
    // lnbc20n1p58vdq6pp5w46w0ve9rq40skwzlvk08774mtm44xymnrc3w5znymuj62zj6cssdqqcqzpuxqrwzqsp5hy7ltttz0fwgu8mpgnvn9re0638vye4ug97dr6zyxeppzukh784s9qxpqysgqxz8zzps2fyefmx8d9mq92m8xej9dy7s6kpg0x0lnxtlct30ml65rlkhwdw80ugefmeyl2jr484x4l255rc9f8nxche0lnnf6nhjwxucp4yq5my

    // let txs = api::get_all_transactions();
    // for tx in txs.unwrap() {
    //     println!("tx {:?}", tx);
    // }

    // let _remove = api::remove_transactions(1756981707, api::TransactionStatus::Pending);

    // let tx = api::check_transaction(
    //     "b6b48cef5f376115eabc3e629273f3dc94a8e9f4b99666da5fe3ccb4336a92e9".to_string(),
    // );
    // println!("check_transaction is {:?}", tx);

    let txs = api::get_ln_pending_transactions();
    for tx in txs.unwrap() {
        println!("ln tx pending {:?}", tx);
    }

    let txs = api::get_cashu_pending_transactions();
    for tx in txs.unwrap() {
        println!("cashu tx pending {:?}", tx);
    }

    // test for check quote
    // let amounts = api::check_all_mint_quotes();
    // println!("check_all_mint_quotes {:?}", amounts);

    // test for melt
    let invoice = "lnbc50n1p5tju7dpp5qu6xmp4kufa5dqfgqppttawctny2trnfth080yxyyq5rl5rys9esdqqcqzpuxqrwzqsp5thzfau3t5my8l8ncvkfscet9qyxqzadhxdaruuvzruvw5w4jlxxq9qxpqysgqd2hp8uj9l0l3g7lu8np555rgqwgykzkf3wsl6rartuk6mpc59089h82qd3r66zuph7yv5wthpucdmaqjwl96cfcy4e06p6u4czplq4qqc7tser".to_string();
    let _melt = api::melt(invoice, MINT_URL.to_string(), None);

    // test for send stamp
    // let stamp = api::send_stamp(1,  vec![MINT_URL_MINIBITS.to_string()], None);
    // println!("send_stamp {:?}", stamp);

    // test for multi send stamp
    // let mut stamps = vec![];
    // for _i in 0..4 {
    //     let stamp = api::send_stamp(1, vec![MINT_URL.to_string()], None).unwrap();
    //     println!("send_stamp {:?}", stamp);

    //     stamps.push(stamp.token);
    // }
    // std::thread::sleep(std::time::Duration::from_secs(15));

    // // // test for check quote
    // // let amounts = api::check_all_mint_quotes();
    // // println!("check_all_mint_quotes {:?}", amounts);
    // // let amount = api::check_all_pending_proofs(MINT_URL_MINIBITS.to_string());
    // // println!("check_all_mint_quotes {:?}", amount);

    // test for multi receive stamps
    // let _ = api::multi_receive(stamps);

    // let restore = api::restore(MINT_URL_MINIBITS.to_string(), None).unwrap();
    // println!("restore {:?}", restore);

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
    // let _ = api::print_proofs(MINT_URL.to_string());
    // let _ = api::test_print_proofs(MINT_URL_MINIBITS.to_string());

    // let prepare_one_proofs = api::prepare_one_proofs(32, MINT_URL.to_string());
    // println!("prepare_one_proofs: {:?}", prepare_one_proofs);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // // test for print proofs
    // let _ = api::print_proofs(MINT_URL.to_string());
    // // let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());

    // // merge some proofs
    // let _ = api::merge_proofs(20);

    // // // send all
    // // let send_all = api::send_all(MINT_URL.to_string());
    // // println!("send_all token is {:?}", send_all);

    // // test fot get balances
    // let b3 = api::get_balances();
    // println!("get_balances after {:?}", b3);
    // // test for print proofs
    // let _ = api::print_proofs(MINT_URL.to_string());
    // // let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());
}
