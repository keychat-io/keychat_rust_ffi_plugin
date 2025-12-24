#[macro_use]
extern crate tracing;

use rust::api_cashu::{self as api, MnemonicInfo};

const DB_PATH_OLD: &str = "rustest-old.db";
const DB_PATH: &str = "rustest-new.db";
const DB_PATH_V2: &str = "ecash_v2.db";
const MINT_URL: &str = "https://8333.space:3338/";
const MINT_URL_MINIBITS: &str = "https://mint.minibits.cash/Bitcoin";

fn main() {
    let words = &MnemonicInfo::generate_words(12).unwrap();
    println!("{}", words);
    let words = "broom only exhibit sand air primary bamboo income sphere climb worth rapid";
    // test_request_mint(words);
    // test_mint_state(words);
    // test_check_transaction(words);
    // test_get_txs(words);
    // test_mint_token(words);
    // test_melt(words);
    // test_check_melt_quote_id(words);
    // test_check_proofs(&words);
    // test_prepare_proofs(words);
    // test_send_all(words);
    // test_merge_proofs(words);
    // test_send_stmap(words);
    // test_load_v2(words);
    // test_send(words);
    // test_v1_receive(words);
    //// test_cashu_v1_init_proofs(words);
    // test_init_v1_and_get_poorfs_to_v2(words);
    // test_get_balance(words);
    // test_split_32(words);
    // test_receive(words);
    // test_restore(words);
    // test_v1_counters(words);
}

fn test_v1_receive(words: &str) {
    // let words = MnemonicInfo::generate_words(12).unwrap();
    println!("generate_words is {:?}", words);
    let tokens = "cashuBo2Ftd2h0dHBzOi8vODMzMy5zcGFjZTozMzM4YXVjc2F0YXSBomFpSADUzeNPraP9YXCCpGFhBGFzeEBiMTNmYWZjZGIzZjQ5ZGI2MDU3YzkwNGEwOTFkNTQ1MjAzNWYwYjdmMjdkNDcxMTVhNDRkZTAwYjNmMTY0MjliYWNYIQIebPW9UT61eo89nffr-TlsON2h_eXVRGFYTXKwGQzmIGFko2FlWCAd6W9rr76AtxlL6TxiYBrD9N1ZZHilcJFN54o6f8aW0WFzWCDV3bpDa4vkw2c4iaj9rsjrvb1cBhsBkR55fVjPNGpixmFyWCB06n1JlhhGHxI44KYSZ7ITza9BNPIcCXCuqOgNpd3EEqRhYQFhc3hANjVjMjVmOTQ1OGFlZjYwNmU5YTRhNDI4MzE5YmU3MWUyNDQ5OWNjMmZiZTlmZDRmOWRhNDFhMjBjYWI0ODFjNGFjWCEDvz8bwqKrbeqIwWFK_V40zlkRhsP8nHFy4xBN4YtpUM5hZKNhZVggtD_-iOulgov4-lxU6-iTATonbZ9TbrYYKJ9Wa_WtN61hc1ggwDBuh8HlPZH6cH5p_FkDJmh228uGAsZqNEJvkW4VXx1hclgg53BqogKFktsO6xF1kmJ4qlHJKdfyY7_ggtc1SWT8UjU=".to_string();
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

    // let mints = api::get_mints();
    // println!("get_mints {:?}", mints);

    // let re = api::remove_mint(MINT_URL.to_string());
    // println!("remove_mint {:?}", re);

    // let mints = api::get_mints();
    // println!("get_mints {:?}", mints);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());
}

fn test_get_txs(words: &str) {
    println!("generate_words is {:?}", words);
    let _init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let txs = api::get_all_transactions();
    for tx in txs.unwrap() {
        println!("tx {:?}", tx);
    }
}

//check_melt_quote_id
fn test_check_melt_quote_id(words: &str) {
    println!("generate_words is {:?}", words);
    let _init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    let quote_id = "VU3jzP1nVnqeaq_a_3HBQSKIQ2jFRTPG3NXtZ05C".to_string();

    // test fot check_mint_quote_id
    let _tx = api::check_melt_quote_id(quote_id, MINT_URL_MINIBITS.to_string());
}

fn test_check_single(words: &str) {
    println!("generate_words is {:?}", words);
    let _init_db = api::init_db(DB_PATH_V2.to_string(), words.to_owned(), false);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);
    // let mints = api::get_mints();
    // println!("get_mints {:?}", mints);

    let txs = api::get_ln_pending_transactions();
    println!("get_ln_pending_transactions before {:?}", txs);

    let _ = api::check_single_pending(
        "55d32fba65cdf1ecf9c12672f55a3c8736d0ac03980015fc892625eac2274896".to_string(),
        "https://8333.space:3338".to_string(),
    );

    // let _ = api::check_proofs_test();
    // let _ = api::check_pending_test();

    // test for print proofs
    // let _ = api::print_proofs(MINT_URL.to_string());
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

fn test_restore(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // api::check_pending_test();

    let restore = api::restore(MINT_URL.to_string(), Some(words.to_string()));
    println!("restore {:?}", restore);

    let b1 = api::get_balances();
    println!("get_balances after {:?}", b1);

    // let txs = api::get_all_transactions();
    // for tx in txs.unwrap() {
    //     println!("tx {:?}", tx);
    // }
}

fn test_check_proofs(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // api::check_pending_test();

    let check = api::check_proofs();
    println!("check_proofs {:?}", check);

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

    let send = api::send(2, MINT_URL_MINIBITS.to_string(), None);
    println!("send token is {:?}", send);

    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);
}

fn test_merge_proofs(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);
    // test for print proofs
    let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());

    let send_all = api::merge_proofs(10);
    println!("send_all is {:?}", send_all);
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);
    // test for print proofs
    let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());
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

    let send_all = api::send_all(MINT_URL.to_string());
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
        let start = std::time::Instant::now();
        let stamp = api::send_stamp(1, vec![MINT_URL.to_string()], None);
        let elapsed = start.elapsed();
        println!(
            "send_stamp took {} ms, result: {:?}",
            elapsed.as_millis(),
            stamp
        );
        stamps.push(stamp.unwrap().tx.token);
    }
    println!("all stamps {:?}", stamps);

    // test for multi receive stamps
    let _ = api::multi_receive(stamps);

    // let stamp = api::send_stamp(1, vec![MINT_URL_MINIBITS.to_string()], None);
    // println!("send_stamp {:?}", stamp);
    // let is_need_split = stamp.unwrap().is_need_split;
    // if is_need_split {
    //     println!("need split proofs first");
    //     let pp = api::prepare_one_proofs(MINT_URL_MINIBITS.to_string());
    //     println!("send_stamp after split {:?}", pp);
    // }

    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);
}

fn test_split_32(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    println!("need split proofs first");
    let pp = api::prepare_one_proofs(MINT_URL_MINIBITS.to_string());
    println!("send_stamp after split {:?}", pp);
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
    let encoded_token: &str = "cashuBo2FteCJodHRwczovL21pbnQubWluaWJpdHMuY2FzaC9CaXRjb2luYXVjc2F0YXSBomFpSABQBVDwSUFGYXCJpGFhGQQAYXN4QDZiYjg0NGM5ZGQzYTk0Nzk3YWRhMzFlMjRjNjNiNzQ3NjA1MGM1NTBmOTYwOGE0ZTQwZjk2ZTMxNzFiY2RiZGRhY1ghA0FT_yivUUttKOoKqNvocRd1jRvkP_lGU9d5exAyrDp5YWSjYWVYIN75q5INMvgModFxthkBhCPH-JJvR46WcPoajk1GTI50YXNYIBvV8QHAgLGnqu2VI-cFWAgZK1RTDMXsYCEEomXL8e_yYXJYIICAsAfxPWQBpdIdnAdc0ompGzANVgfpdcutddRkalTkpGFhGQQAYXN4QDAyYmNmOGI3OGU0NTBlMGQ0MTM4YjJkYWRmMDBhNTI2ODRkZDM5ZmQ3N2E0OTNmMzZmMTA4NzBkYzg0MzE0YWZhY1ghAihGx5xYastfgfl-dA3tuLSx1ouS5iXZ9vp2MlY_FEmSYWSjYWVYIF9kflukwkx_uxxwgWAvQnyQqFXuq--7o75q9-SgdEO3YXNYIH3xgFoO7fHgyXkd5oJxLf3Nr8R6ZPgndwVrzFFTRXA1YXJYIDMuwbst-Jb-teQLhVbfsmMPtZeaB4t-viHe2caUkw-lpGFhGEBhc3hAZDE1NWRjNjdmZjM0NmZiMjdkNDEyNTQ1NzUyMDhjODgwNjk0Y2YyMWI1NmVlYjhiY2Q4YWUxMWIwNDY2NjMzZmFjWCEC1ijFFN9zJDHboB2MM0Ka6vdFlpEHMmzA7NP5I2xmKaFhZKNhZVggs_svdxCJ7ws71XyEvh-XRAqUsJC89V5VcgtFaMlbwbphc1gg9NPteX5J5qgqMl0MKCBYKA7Ip0asC519zOM1XF0iIo5hclggQra3uRxzLiY-jNXX6QJiOUhR4ZtuzGLAwNJsewohGtykYWEYQGFzeEAyYWI4NTZmZGFlMTNhNDk4MjdhNDE0OTJlOTVlMzE1OWM2OWMzNWEwZjY1MGQ3ZjY3NDhiMzk3MGZiZmFmZDY5YWNYIQJR4w1HnlGlE7LKwIkdihPrEJwTFVDLBWJOPlHHSHwxPWFko2FlWCBVOfeMqVHoWKPBBipc599gGBc4uNYFIZM3p1U_REJCR2FzWCCruzKQIKIXud5gh51b2S89d6qn8mM9_HfdltgHqxJp1WFyWCBsl9FKBIn1s9zNiFNPKW-4mWV1peTZlRijicc8swhVoqRhYRBhc3hAMTUzMGUwYjUzNDFhNzhlOTExNDhkNTA4YzQxNThhMzk5M2FiYmQ2OTQ1NzM4YzQzOWNiMTYwMDg0ODNmZjUwOWFjWCED_SxrVFDX9ESuCrKgFs1qa5-DTTQIa74boFg4E8uQrGlhZKNhZVgguOmc5g3cAFAaStaJWXPf98D7dNZeRIQRw8rDYef7kaNhc1ggW9pBGhDwRTF7dJgjLftuiDjAoG8lPzjlrSHbDcFhC21hclgguoZLMJgqg44WYpDLDYMCuomdq0Dy4GFtov3J2LTn8R2kYWECYXN4QDU5YWE1MGY4NGRkMGU2NTY0YWMwZjdkYTRkY2MzMjMzNmZhODkyYjExOGY4MDg2MTExNzMyNWVkYmJiZmU3YmFhY1ghA41wyKVzK8v8N0DPjTVq3JZbmg6PzHqBkm_pZnfHBOmOYWSjYWVYIDA5HJSYhqEd67xvP0cAAeUYcN9DYa0VG_KpCbglp_PSYXNYIMTVKv4giPG1Dv2eVedqmaUyWgRhLH6XN6XyYUvK5ajVYXJYIEov85qLfCBdykjIlGn5B8FHIyMHiQEukmf7XVR_xapIpGFhAmFzeEA5NzE3MzY4ZTIyYWVkMDJiNTc2M2NlNzlmZDQ5M2VlYjc3NGVlMTExMzU4YmFhZDY2ZGJlNDMxZjdmOTFhOTE3YWNYIQLUIJAbsh8MC3XYv2LNZLGcoXAovvp1z8CoicV3qSeeQGFko2FlWCApKVyS3ntluZ32PzBc-_SDcrprTk85Ld-acw63AC5sKWFzWCDCQyVbnMT_wnCIllDiJYUHZQuSm16LvG5KPrRr7msmJWFyWCAtAzNg_tVUsgdB2lghPf0CBuaPtVYTrpG9e-ZnR5o-8KRhYQJhc3hANzI3MGRiZmUzMGM2MjA1MDVlNzdjNzZkMmEyNWEzZmNiMDRkMDQ0NmE3ZWNiZDllMmY5OGNmMWZmNmQyZTE2MWFjWCEDqZizqYJ3GdxfXU6Ar1dhxUJS42CeX2pyWTRKVjFoJcRhZKNhZVggHIkc4nwgE6LkM-PLDbiBiz8UJ0OiW9hRMKpxniUYkpdhc1ggNZOa_nD01UKde1OgkRRxYBEiGaFpxzdWlzZg4bp10RBhclgghWnXNmgi1LMvwLS31fYMSrk8mO8fgB7riy4v3lhg9qSkYWEBYXN4QDM3YTZjZWE4NzdiMGFmMWY3YjcxNjhkNTFiNDgwZmRjODc3ZmI1MzU4NTM0MmQ2ZGE2ZGZmZWQ0ZDQ4MGVkZDlhY1ghAqrdDZcCYCVq7F38LSjZRsVIlYdsOoehyAHAEsMeKXXVYWSjYWVYIJuJOLb4GNrgSQV472liiRlBQ2Kd6VJ3J8bMbDbOUTGtYXNYILVHFRWIZPqf5QWMLLZhkRXwTx7shqlWJ_iMwKckBGaRYXJYII6oR59NxKW4rlGqxE-oC-ZDZbxKv3RJq_uSQUU3juAW".trim();

    let re = api::receive_token(encoded_token.to_string());
    println!("receive token is {:?}", re);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());
}

fn test_request_mint(words: &str) {
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
    let invoice = api::request_mint(2200, MINT_URL.to_string());
    println!("request_mint invoice is {:?}", invoice);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL.to_string());
}

fn test_mint_state(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    // test for receive token
    let amount = api::check_all_mint_quotes();
    println!("test_mint_state amount is {:?}", amount);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());
}

fn test_check_transaction(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    // test for check transaction
    let tx_id = "505acabc42d0cffa3874f824a17a93477f79eca13d1a46a9a403eb408662290b".to_string();
    let amount = api::check_transaction(tx_id);
    println!("test_check_mint amount is {:?}", amount);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL.to_string());
}

fn test_mint_token(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    // test for check transaction
    let quote_id = "B7rKkwCcMMcYv7wXEIJbkScIZtOaptIGqq4mKy7c".to_string();
    let amount = api::mint_token(10, quote_id, MINT_URL_MINIBITS.to_string());
    println!("test_mint_token amount is {:?}", amount);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());
}

fn test_melt(words: &str) {
    println!("generate_words is {:?}", words);
    let init_db = api::init_db(DB_PATH.to_string(), words.to_owned(), false);
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    let invoice = "lnbc21u1p5n5ljdpp5fxg5p8jyvqvd8vuywrrpgrpzsyxc8yr3gtvun6yyfp50znkl08fqdqqcqzpuxqrwzqsp5rjqen5c4q567sagagd4y32pn4g9xaq0s5k7gyv3v6rl3ve3w4adq9qxpqysgqn6q8jzf60s6lxvem652k9e28w35fqnses3fsqts2vp92zsjf8y7ya0kcw3lgsmxds5k88ammtn6msv0pek3cxt3tak459qnyvpevpkgqqep6cr".to_string();

    // test for receive token
    let invoice = api::melt(invoice, MINT_URL.to_string(), None);
    println!("melt invoice is {:?}", invoice);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL.to_string());
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
