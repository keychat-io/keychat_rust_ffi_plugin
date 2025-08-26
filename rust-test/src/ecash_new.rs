#[macro_use]
extern crate tracing;

use rust::api_cashu_v2::{self as api, MnemonicInfo};

const DB_PATH: &str = "rustest-new.db";
const MINT_URL: &str = "https://8333.space:3338/";
const MINT_URL_MINIBITS: &str = "https://mint.minibits.cash/Bitcoin";

fn main() {
    let migen_words = MnemonicInfo::generate_words(12).unwrap();
    // let migen_words = "mushroom venture grab fatigue excite solve onion include minute joy trade anxiety";
    println!("migen_words is {:?}", migen_words);

    let init_db = api::init_db(DB_PATH.to_string(), migen_words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);

    // let mi = api::get_mnemonic_info();
    // println!("get_mnemonic_info is {:?}", mi);

    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    // test for receive token
    // let encoded_token_8338: &str = "cashuBo2Ftd2h0dHBzOi8vODMzMy5zcGFjZTozMzM4YXVjc2F0YXSBomFpSADUzeNPraP9YXCEpGFhAmFzeEA1OTBmNTlmNWFiMTc1OTQ5NGQ3N2JmNjcxNzdiMjhjYjQyMThhZWVkOTBmYjRhZWVjNTJlNTU0NDIxZmJiZTMzYWNYIQOmyIp1UTfH59EBNFwdhkBSkRNtZSHiIKnfmpyeswr6uWFko2FlWCC4r_Cz2yBd99aXO-DR3l-DBr8-LT0useNY7o6A4Ojr2mFzWCA1KFfS7n1SfGd7EtDMOoyJu_vESUAFnDD0Aalac8z8VGFyWCDNmi0CbpiKxGjKACZkXjw2flhXiTpln1kUvJvMtdMQhqRhYQFhc3hANzlhMGE5MzhhNGQ5NTYzMzE3ZTYwMmY0OWMyOGVmNzNjYzhmNDM4YzlhY2IzY2NlODE1MmYwNDljMjIyOTJjNmFjWCECDiJhQtaOOentTBUjfZzAvQZUJd112uzHPy87bFbi6sBhZKNhZVggsakHGGwMFJUKR-T4aCbmz6KEQHyCoJJF08nqMozxhMlhc1gg5z5eujKe4rYLYBKNbUsTDkoAoQyVJb9zZpAMOV_RS4FhclgghPmgGU5DuTl07zZl72FnWCkwZJCTvNKt1V8EJvJBf4akYWEBYXN4QDk0Y2MyNDcyMjRkMGRmNjYxYTEzMzFkZjE5ZDQ0MTYxM2I5ODM2ZTkyNDViYmNmOTVmOTcyOTUyMDBiNjViODFhY1ghAmf_y_-w8Sjl8P4lTWUtokiFgmt7M_t9KcNV0vKNjjNdYWSjYWVYIH0Y__fgvIuyuen6zNqG65PTfQufGAoMVaTJhKnBCJ2qYXNYIFzx9MdA4pF8wInMtevWGC42aARCz7vsb26vuLQQANuvYXJYIL_NT5hfGYrM9FQpi9n4DhB8WZ8k6mzgmW3qgJEbPkAIpGFhAWFzeEBiYWMwNGJmOTk3ODIzYWM5OTkxZTczMThiYjI3MzA3NTAwNjEzMWU4YWM0NWFlNjk0ZTRlZjAyMGE1ODBmODg3YWNYIQOIH0ONlXr8qLn-qKJvAfB6tHlEzw-Xo2RA12_dGpwK2WFko2FlWCB4uKn3VqpVuDY1ZPsnvA3bJ62ekceOiILO2l7nPMsUC2FzWCBuobQP4840Pj_y375F2FONypi1eLg4NIQ50AWgxZ3IPWFyWCDycqGHfoT52DqNGRxDil3hnOFPo1t7Ip423-O_LwRrxQ".trim();
    // let re = api::receive_token(encoded_token_8338.to_string());
    // println!("receive token is {:?}", re);

    // test for get mints
    // let mints = api::get_mints();
    // println!("get_mints is {:?}", mints);

    // test for send token
    let send = api::send_all(MINT_URL.to_string());
    println!("send token is {:?}", send);

    // let send = api::send_all(MINT_URL_MINIBITS.to_string());
    // println!("send token is {:?}", send);

    // test for request mint
    // let request_mint = api::request_mint(10, MINT_URL.to_string());
    // println!("request_mint is {:?}", request_mint);
    // lnbc20n1p58vdq6pp5w46w0ve9rq40skwzlvk08774mtm44xymnrc3w5znymuj62zj6cssdqqcqzpuxqrwzqsp5hy7ltttz0fwgu8mpgnvn9re0638vye4ug97dr6zyxeppzukh784s9qxpqysgqxz8zzps2fyefmx8d9mq92m8xej9dy7s6kpg0x0lnxtlct30ml65rlkhwdw80ugefmeyl2jr484x4l255rc9f8nxche0lnnf6nhjwxucp4yq5my

    // test for check quote
    let amounts = api::check_all_mint_quotes();
    println!("check_all_mint_quotes {:?}", amounts);

    // test for melt
    // let invoice = "lnbc30n1p52csftpp5jdjqw63kjlgkupl97awn23fj8hsqkm4cvckrrvg7ejxzu9zv66zqdqqcqzpuxqrwzqsp5sn07ddv5ggu37y7mgredgy6aucjs26jgd67gmw8fpvftug33vxts9qxpqysgqua8aj7u3yxm3atw5t7jjk4lnh3k7zvc3d04gs7y9mh4grk5ks7t3sj6z7srck3du5ezaec93a8q24cvghwhqffr9dcp7k8j6h5rljagpw7sq08".to_string();
    // let _melt = api::melt(invoice, MINT_URL.to_string(), None);

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
    let ln_txs = api::get_ln_transactions_with_offset(0, 100);
    for tx in ln_txs.unwrap() {
        println!("ln {:?}", tx);
    }

    // test for print proofs
    let _ = api::print_proofs(MINT_URL.to_string());
    // let _ = api::test_print_proofs(MINT_URL_MINIBITS.to_string());

    // let prepare_one_proofs = api::prepare_one_proofs(32, MINT_URL.to_string());
    // println!("prepare_one_proofs: {:?}", prepare_one_proofs);

    // // test fot get balances
    // let b2 = api::get_balances();
    // println!("get_balances after {:?}", b2);

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
