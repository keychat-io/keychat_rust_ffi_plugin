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
    // let encoded_token_8338: &str = "cashuBo2FteCJodHRwczovL21pbnQubWluaWJpdHMuY2FzaC9CaXRjb2luYXVjc2F0YXSBomFpSABQBVDwSUFGYXCCpGFhBGFzeEAxNmNiN2I4ZDBlYmEyN2RlZjM3MWU3ZDc5ZTFkOTI0MzA1ZGI2NmY1NWJkMGQyY2ZjNDQ0ZDhjMWUwZWU1ZjE2YWNYIQIE02vnQHZdc_R9ICwZeevzwLT60xC7fGTaOkTB4eF4b2Fko2FlWCBfLo0MSQYvgF_txSwQ3Z2pK2gEzriFowPn0gp8n9GuqWFzWCCSc_MnxZFodM57ueEWwhC3N2i3n347KzfEb-L1qa9y_mFyWCBVQT9VL_dEH9FKo6eMkUXBQew6-yMqkTheDj5K6klcPqRhYQFhc3hAM2U2N2E0ZTE2OTVlNDdhYWZiZjg5MjE5MTg4Y2E1MGMwYTFmZjE4NDI1YTkzNDFkYTFmZDhjODYwNmFhNzA1ZWFjWCECC9LTuz4_MOuitMLCxIuCqLsMN4-6hQivlSs7Phwjf3NhZKNhZVggSZ7BSlXLJOZyWcR8Zs7M1w1xN18HqRZcpLFD3mHUdS9hc1gg_x-Z5RwOE6Cgm_k3SvyW-ug5dhs6qBRs3XIZdnylQ_hhclggedxBShp_9snXpd9xzoJHDHyGBdc3RHjwjCoW5uaXf1U".trim();
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

    // let _remove = api::remove_transactions(1756795836, api::TransactionStatus::Pending);

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
