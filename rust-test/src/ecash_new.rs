#[macro_use]
extern crate tracing;

use rust::api_cashu::{self as api, MnemonicInfo};

const DB_PATH_OLD: &str = "rustest-old.db";
const DB_PATH: &str = "rustest-new.db";
const DB_PATH_V2: &str = "ecash_v2.db";
const MINT_URL: &str = "https://8333.space:3338/";
const MINT_URL_MINIBITS: &str = "https://mint.minibits.cash/Bitcoin";

fn main() {
    // let words = &MnemonicInfo::generate_words(12).unwrap();
    let words = "whale install write age truth belt feed before devote trip lunch remember";
    // test_prepare_proofs(words);
    // test_send_all(words);
    test_send_stmap(words);
    // test_load_v2(words);
    // test_send(words);
    // test_v1_receive(words);
    // test_cashu_v1_init_proofs(words);
    // test_init_v1_and_get_poorfs_to_v2(words);
    // test_get_balance(words);
    // test_receive(words);
    // test_restore(words);
    // test_v1_counters(words);
}

fn test_v1_receive(words: &str) {
    // let words = MnemonicInfo::generate_words(12).unwrap();
    println!("generate_words is {:?}", words);
    let tokens = "cashuBo2Ftd2h0dHBzOi8vODMzMy5zcGFjZTozMzM4YXVjc2F0YXSBomFpSADUzeNPraP9YXCCpGFhAmFzeEA1NmEyNmY3NGJkZDEwMWUzOTdiZTg0MTdmNDkxMjQxZDQzOTU2YTcxYTRkMmVjYjMzYjZlMzY5MGIyNzg0MTU0YWNYIQNpoElFt-GBsEqkoFvraDh4CX_KduiqHJS7bApYTK3xYWFko2FlWCAd8yAufDDERTqtlhlvNyCM7jBeSbEyda0os95sOZHSGGFzWCA1cHfbI8JtmrfEM7lkHENYT7WHaibElHzfAl4ofBn18GFyWCCquZHnwmPhrMsDmXkFJ5m7Rfw7QMcQLaHFV-cHcToR9aRhYQFhc3hAYjMyYmQ2OWJkYWYyNjY4ZDMwMDBlYmNhYWZjMmI0Y2VjYTY5MzMzOWZkZGMwYTIzZmI2ZjAzNzZhZWRhMzk1NWFjWCECJQd5IKA7IJnlSXl9RuSIUEMRM_DVZ2se5cqeFHcWFBphZKNhZVggctmXT2d6XpkfKgMPa4PJwkYbBBhEM0UILM7vciVYP6thc1gg7x287V1Ur8frEvlHhKA_kwSX9B2mD0gJ0cYsdsAylQVhclggTDQs8W0CMLaZZPG9T7tU_RwwYygZgl56OifKn6iyEQQ=".to_string();
    let re = api::cashu_v1_init_test(DB_PATH_OLD.to_string(), Some(words.to_string()), tokens);
}

fn test_cashu_v1_init_proofs(words: &str) {
    println!("generate_words is {:?}", words);
    let re = api::cashu_v1_init_proofs(DB_PATH_OLD.to_string(), Some(words.to_string()));
    println!("test_cashu_v1_init_proofs: {:?}", re);
}

fn test_init_v1_and_get_poorfs_to_v2(words: &str) {
    println!("generate_words is {:?}", words);
    let re = api::init_v1_and_get_poorfs_to_v2(
        DB_PATH_OLD.to_string(),
        DB_PATH.to_string(),
        words.to_string(),
    );
    println!("test_init_v1_and_get_poorfs_to_v2: {:?}", re);
    // let mints = api::get_mints();
    // println!("get_mints {:?}", mints);
    // // need to init db again!
    // let b = api::get_balances();
    // println!("get_balances before {:?}", b);
}

fn test_get_balance(words: &str) {
    println!("generate_words is {:?}", words);
    let _init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);
    let mints = api::get_mints();
    println!("get_mints {:?}", mints);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);
}

fn test_load_v2(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH_V2.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH_V2, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // let restore = api::restore(MINT_URL_MINIBITS.to_string(), Some(words.to_string()));
    // println!("restore {:?}", restore);

    let b1 = api::get_balances();
    println!("get_balances after {:?}", b1);

    let txs = api::get_all_transactions();
    for tx in txs.unwrap() {
        println!("tx {:?}", tx);
    }
}

// this is my ios, v1 to v2 restore will have some errors
fn test_restore(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH_V2, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    let restore = api::restore(MINT_URL_MINIBITS.to_string(), Some(words.to_string()));
    println!("restore {:?}", restore);

    let restore = api::restore(MINT_URL.to_string(), Some(words.to_string()));
    println!("restore {:?}", restore);

    let b1 = api::get_balances();
    println!("get_balances after {:?}", b1);

    // let txs = api::get_all_transactions();
    // for tx in txs.unwrap() {
    //     println!("tx {:?}", tx);
    // }
}

fn test_send(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    let send = api::send(6, MINT_URL_MINIBITS.to_string(), None);
    println!("send token is {:?}", send);

    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);
}

fn test_send_all(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    let send_all = api::send_all(MINT_URL_MINIBITS.to_string());
    println!("send_all is {:?}", send_all);
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);
}

fn test_send_stmap(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

   

    let mut stamps = vec![];
    for _i in 0..4 {
        let stamp = api::send_stamp(1, vec![MINT_URL_MINIBITS.to_string()], None);
        println!("send_stamp {:?}", stamp);
        stamps.push(stamp.unwrap().unwrap().token);
    }

    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);
}

fn test_receive(words: &str) {
    // let words = MnemonicInfo::generate_words(12).unwrap();
    // let words = "harsh city pave response hotel jelly midnight venue borrow loan act gun";
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    // test for receive token
    let encoded_token: &str = "cashuBo2FteCJodHRwczovL21pbnQubWluaWJpdHMuY2FzaC9CaXRjb2luYXVjc2F0YXSBomFpSABQBVDwSUFGYXCCpGFhBGFzeEAyYWMyNzg1MjY2MDdkYmNkZDcwYjc3NTExOWZhYzI1MmM5YjViYjRlNmFjZWIxNDUzYWM0OWYxNDk0NjI1NzA2YWNYIQMG8S24lyQEzZfA5KefKD7BGcSQbPrSFTRWVrUJy6YrMWFko2FlWCAidO0ht4RSU3r1DxV0TlQD9zhhuyYc07wl0kkhTB4p_GFzWCA_HHsfmmELLjYis25fQSNTEirn5MNz9jQHfZcz92aDoGFyWCBsmrDpqN-Semn8V7_y7Gl1Tigr2A60RDgnihxt1Ou2IqRhYQFhc3hAMTI4MjYyYTYxODI1YjkxMzQwYWQwZjhjNWFmOWVlYTQyYzkyZGRmMWIwY2ZhMGMyNWRhNGE4ZTY3ZTE4ODU4ZmFjWCEC7xGxUPe7u84Q02rQL8F52FxLXrKFswmYxSG5qvZIoQRhZKNhZVggZ4YQ6dGvDeLYMUOosqfYbgGocPquWl8bdAzQaTC4qyNhc1ggymXPXw3AbgnwIKwyAzfrcIrhXrW_EwzKPds6IXR9Nf5hclgglxSVhEr2oZ-QAfLxBhLOXNALbUCsyH81KAN_O04ez2E".trim();

    let re = api::receive_token(encoded_token.to_string());
    println!("receive token is {:?}", re);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // test for print proofs
    // let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());
}

fn test_prepare_proofs(words: &str) {
    // let words = "stairs slim wasp turn poem supply any suggest stove unknown flat enrich";
    // let words = MnemonicInfo::generate_words(12).unwrap();
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test for receive token
    let encoded_token: &str = "cashuBo2Ftd2h0dHBzOi8vODMzMy5zcGFjZTozMzM4YXVjc2F0YXSBomFpSADUzeNPraP9YXCDo2FhAmFzeEBmZmNmNzMzNjJiYmJlMGJiNzhlMmQ2OWViNTQyZDI1NDNmMGQ1ODBiYTE2MjdjMWNjNTBjNzlhZTM0NDFmZTE5YWNYIQIb24-O5cxyy8W7ajbSAR7u-YKRQeTj7f7tpXDvERJIFqNhYRBhc3hAZTU5OWQ1MmJkMDg1OTNmODg0MTg0ZTZlYmMwMmFkYzI5MDMyY2I4NGU5Yjg2ZWNhMzk1YzIyNWI0ZjZhMzg4N2FjWCEDtiQmeq_JArsInkvh2ZiMYqSs1fiDdKNN_3tq5Nr-eOujYWEYIGFzeEA0OTlmYTM0NzIyYjJlMDkxNWU4NzU4YjFmNzlhNmQyOTY5ZjU5OGIwZWMzYzgzZDg4Mjc5ODM3MjIxYTQzZTY2YWNYIQJkT2EG5zFWS4o6U9MY7qykLAahnqNYeawbSLs1hgvLmA".trim();

    let re = api::receive_token(encoded_token.to_string());
    println!("receive token is {:?}", re);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL.to_string());

    let stamp = api::send_stamp(1, vec![MINT_URL.to_string()], None);
    println!("send_stamp {:?}", stamp);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL.to_string());

    let txs = api::get_all_transactions();
    for tx in txs.unwrap() {
        println!("get tx {:?}", tx);
    }
}

fn test_v1_counters(words: &str) {
    // let words = MnemonicInfo::generate_words(12).unwrap();
    println!("generate_words is {:?}", words);
    let tokens = api::cashu_v1_init_send_all("rustest.db".to_string(), Some(words.to_owned()));
    println!("get_all_counters: {:?}", tokens);
    // let words = "void vintage diamond alcohol parrot second old magnet decade chalk october motor";
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);
    let unwrapped_tokens = tokens.unwrap();
    let _ = api::add_counters(unwrapped_tokens.counters);
    let receive = api::receive_token(unwrapped_tokens.tokens[0].clone());
    println!("receive_token: {:?}", receive);
}

// fn main() {
//     // let words = MnemonicInfo::generate_words(12).unwrap();
//     // let words = "vacant chest hungry choice host ginger castle cancel cloud turkey leisure kite";
//     // let words = "culture ignore divide assault cost deposit exercise drill flee deer office sun";
//     let words = "unaware awesome finish calm deputy doll roast own dumb liar afraid differ";

//     // // let migen_words = "mushroom venture grab fatigue excite solve onion include minute joy trade anxiety";
//     // println!("migen_words is {:?}", words);

//     // let tokens =
//     //     api::cashu_v1_init_send_all("ecash.db".to_string(), Some(words.to_owned()));
//     // println!("send all tokens {:?}", tokens);

//     // let words = "bunker feel away slide trip girl amazing resource veteran direct cotton blanket";

//     let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
//     println!("init_db {}: {:?}", DB_PATH, init_db);

//     // let mi = api::get_mnemonic_info();
//     // println!("get_mnemonic_info is {:?}", mi);

//     let init_cashu = api::init_cashu(32);
//     println!("init_cashu is {:?}", init_cashu);

//     // if tokens.is_ok() {
//     //     for token in tokens.unwrap() {
//     //         let re = api::receive_token(token);
//     //         println!("receive token is {:?}", re);
//     //     }
//     // }

//     // test for receive token
//     // let encoded_token_8338: &str = "cashuBo2Ftd2h0dHBzOi8vODMzMy5zcGFjZTozMzM4YXVjc2F0YXSBomFpSADUzeNPraP9YXCCo2FhBGFzeEAxNGZiMTkxNjZiZGQ1ZGI2YjU0ZGZlZTE0ZTRmMjUzNTI0ZjE5MGFiNjMxOWYyNzZlNDA1ZDZlNDljOTg5ZmE0YWNYIQOhsesMrYhgunuYkkWyoi7jkTbNJ2rQOo55RVcqbQTQkqNhYQFhc3hAZDE2OTlkZDFjMzQzZDRjNjllMzRiYTlhNzQ4ODMyZGViNTRmODQ1NDFmNzlmMjI1MDM1ZjFmMDY4ZGU4NjFkNGFjWCEDtSfpj3NBCk48Yhezk0eK13t4LdU6j1HwnhRoAWVrpVo".trim();
//     // let re = api::receive_token(encoded_token_8338.to_string());
//     // println!("receive token is {:?}", re);

//     // test fot get balances
//     let b1 = api::get_balances();
//     println!("get_balances before {:?}", b1);

//     // let words = MnemonicInfo::generate_words(12).unwrap();
//     // let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);

//     // test for get mints
//     // let mints = api::get_mints();
//     // println!("get_mints is {:?}", mints);

//     // test for send token
//     // let send = api::send_all(MINT_URL_MINIBITS.to_string());
//     // println!("send token is {:?}", send);

//     // let send = api::send_all(MINT_URL.to_string());
//     // println!("send token is {:?}", send);

//     // let send = api::send(2, MINT_URL_MINIBITS.to_string(), None);
//     // println!("send token is {:?}", send);

//     // let send = api::send(1, MINT_URL_MINIBITS.to_string(), None);
//     // println!("send token is {:?}", send);

//     // let txs = api::get_ln_pending_transactions();
//     // for tx in txs.unwrap() {
//     //     println!("ln tx pending {:?}", tx);
//     // }

//     // let txs = api::get_cashu_pending_transactions();
//     // for tx in txs.unwrap() {
//     //     println!("cashu tx pending {:?}", tx);
//     // }

//     // let check_tx = api::check_pending();
//     // println!("check_pending is {:?}", check_tx);

//     let restore = api::restore(MINT_URL.to_string(), Some(words.to_string()));
//     println!("restore {:?}", restore);

//     let restore = api::restore(MINT_URL_MINIBITS.to_string(), Some(words.to_string()));
//     println!("restore {:?}", restore);

//     // let check_proofs_tx: Result<(u64, u64, u64), anyhow::Error> = api::check_proofs();
//     // println!("check_proofs is {:?}", check_proofs_tx);

//     // test for request mint
//     // let request_mint = api::request_mint(10, MINT_URL_MINIBITS.to_string());
//     // println!("request_mint is {:?}", request_mint);
//     // lnbc20n1p58vdq6pp5w46w0ve9rq40skwzlvk08774mtm44xymnrc3w5znymuj62zj6cssdqqcqzpuxqrwzqsp5hy7ltttz0fwgu8mpgnvn9re0638vye4ug97dr6zyxeppzukh784s9qxpqysgqxz8zzps2fyefmx8d9mq92m8xej9dy7s6kpg0x0lnxtlct30ml65rlkhwdw80ugefmeyl2jr484x4l255rc9f8nxche0lnnf6nhjwxucp4yq5my

//     // let txs = api::get_all_transactions();
//     // for tx in txs.unwrap() {
//     //     println!("tx {:?}", tx);
//     // }

//     // let _remove = api::remove_transactions(1756981707, api::TransactionStatus::Pending);

//     // let tx = api::check_transaction(
//     //     "b6b48cef5f376115eabc3e629273f3dc94a8e9f4b99666da5fe3ccb4336a92e9".to_string(),
//     // );
//     // println!("check_transaction is {:?}", tx);

//     // let txs = api::get_ln_pending_transactions();
//     // for tx in txs.unwrap() {
//     //     println!("ln tx pending {:?}", tx);
//     // }

//     // let txs = api::get_cashu_pending_transactions();
//     // for tx in txs.unwrap() {
//     //     println!("cashu tx pending {:?}", tx);
//     // }

//     // test for check quote
//     // let amounts = api::check_all_mint_quotes();
//     // println!("check_all_mint_quotes {:?}", amounts);

//     // test for melt
//     // let invoice = "lnbc50n1p5tju7dpp5qu6xmp4kufa5dqfgqppttawctny2trnfth080yxyyq5rl5rys9esdqqcqzpuxqrwzqsp5thzfau3t5my8l8ncvkfscet9qyxqzadhxdaruuvzruvw5w4jlxxq9qxpqysgqd2hp8uj9l0l3g7lu8np555rgqwgykzkf3wsl6rartuk6mpc59089h82qd3r66zuph7yv5wthpucdmaqjwl96cfcy4e06p6u4czplq4qqc7tser".to_string();
//     // let melt = api::melt(invoice, MINT_URL_MINIBITS.to_string(), None);
//     // println!("melt is {:?}", melt);

//     // test for send stamp
//     // let stamp = api::send_stamp(1,  vec![MINT_URL_MINIBITS.to_string()], None);
//     // println!("send_stamp {:?}", stamp);

//     // test for multi send stamp
//     // let mut stamps = vec![];
//     // for _i in 0..4 {
//     //     let stamp = api::send_stamp(1, vec![MINT_URL.to_string()], None).unwrap();
//     //     println!("send_stamp {:?}", stamp);

//     //     stamps.push(stamp.token);
//     // }
//     // std::thread::sleep(std::time::Duration::from_secs(15));

//     // // // test for check quote
//     // // let amounts = api::check_all_mint_quotes();
//     // // println!("check_all_mint_quotes {:?}", amounts);
//     // // let amount = api::check_all_pending_proofs(MINT_URL_MINIBITS.to_string());
//     // // println!("check_all_mint_quotes {:?}", amount);

//     // test for multi receive stamps
//     // let _ = api::multi_receive(stamps);

//     // let restore = api::restore(MINT_URL_MINIBITS.to_string(), None).unwrap();
//     // println!("restore {:?}", restore);

//     // let txs = api::get_all_transactions();
//     // for tx in txs.unwrap() {
//     //     println!("tx {:?}", tx);
//     // }

//     // // test for get_cashu_transactions_with_offset
//     // let cashu_txs = api::get_cashu_transactions_with_offset(0, 100);
//     // for tx in cashu_txs.unwrap() {
//     //     println!("cashu {:?}", tx);
//     // }

//     // // test for get_ln_transactions_with_offset
//     // let ln_txs = api::get_ln_transactions_with_offset(0, 100);
//     // for tx in ln_txs.unwrap() {
//     //     println!("ln {:?}", tx);
//     // }

//     // test for print proofs
//     // let _ = api::print_proofs(MINT_URL.to_string());
//     // let _ = api::test_print_proofs(MINT_URL_MINIBITS.to_string());

//     // let prepare_one_proofs = api::prepare_one_proofs(32, MINT_URL.to_string());
//     // println!("prepare_one_proofs: {:?}", prepare_one_proofs);

//     // test fot get balances
//     let b2 = api::get_balances();
//     println!("get_balances after {:?}", b2);

//     // // test for print proofs
//     // let _ = api::print_proofs(MINT_URL.to_string());
//     // // let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());

//     // // merge some proofs
//     // let _ = api::merge_proofs(20);

//     // // // send all
//     // // let send_all = api::send_all(MINT_URL.to_string());
//     // // println!("send_all token is {:?}", send_all);

//     // // test fot get balances
//     // let b3 = api::get_balances();
//     // println!("get_balances after {:?}", b3);
//     // // test for print proofs
//     // let _ = api::print_proofs(MINT_URL.to_string());
//     // // let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());
// }
