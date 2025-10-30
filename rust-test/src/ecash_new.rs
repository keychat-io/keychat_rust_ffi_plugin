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
    let words = "today talk cheap laptop better donate forward train beauty subway enjoy meat";
    // test_prepare_proofs(words);
    // test_send_all(words);
    // test_merge_proofs(words);
    test_send_stmap(words);
    // test_load_v2(words);
    // test_send(words);
    // test_v1_receive(words);
    //// test_cashu_v1_init_proofs(words);
    // test_init_v1_and_get_poorfs_to_v2(words);
    // test_get_balance(words);
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
    let mints = api::get_mints();
    println!("get_mints {:?}", mints);

    // test fot get balances
    let b1 = api::get_balances();
    println!("get_balances before {:?}", b1);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL.to_string());
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
    println!("init_db {}: {:?}", DB_PATH, init_db);
    let init_cashu = api::init_cashu(32);
    println!("init_cashu is {:?}", init_cashu);

    let restore = api::restore(MINT_URL.to_string(), Some(words.to_string()));
    println!("restore {:?}", restore);

    // let restore = api::restore(MINT_URL.to_string(), Some(words.to_string()));
    // println!("restore {:?}", restore);

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
    let encoded_token: &str = "cashuBo2FteCJodHRwczovL21pbnQubWluaWJpdHMuY2FzaC9CaXRjb2luYXVjc2F0YXSBomFpSABQBVDwSUFGYXCMpGFhAWFzeEBhMWVhYTZmMmE5ZGY3YmYwNTM1Zjc4MWFjNTAzNTc5OTYwZjFiYWI1NmU4MmNkYjA1NjE2NDcxNGNiMjE0NjRhYWNYIQORASNLWTYGjcUM2gKG6Ha_nDbTw-ykH7ZcD14JaS600WFko2FlWCDW1UdZrJU2FP2huhMp91LH2beXwBOaJfvAAoX5HtjNGmFzWCDCaZbm2VGTkWun8IoBWFxGvdLtc9H8eWZEPuKD_8k9uGFyWCCtIYf37yl3P9Ynk4-ExFT_CN_UE9I7pHDMpweg-N_FUKRhYQFhc3hANzE5YTcwOWVjZjg0ZDJmY2U2YzFiNWQxZjhkZWFjMjBmY2NkOTQ4Njg1YTVlNzY4YzhlNTNiY2IwYjIwYTk0ZmFjWCEDZMOsv35CR3nweOLam0wfPnSg72dw7V01DnAvnPb5BXVhZKNhZVggTaLZ0yIRqlFyRQeOqsmlSdTzLiqjAeBjZgAwrZ6edmNhc1ggqnT_FdXXcpLdGXTm-EvmK5Ze_4WjQjxd77vN5w125othclggVMXaXCfK559yBkJ-fymLp0HfYKErlhPbbt1zHavf-B2kYWEEYXN4QDljZmQxOTg0MTE2MjhmM2E2ODZmZTA5NWEzOGVlMjA0NGQ5ZTZmNWM4M2EyZDBjMWZkZDRjMzRiYTJkODViNDlhY1ghAky7po6uDjRYcp-i6vbWIZfMImsK0GTFH9Vhnl8zes8XYWSjYWVYIBM5pVQ6ngpCkf2XVIql-buRBHKG4vXlKWpCoSBoLeTMYXNYIKLFn8bA8HxH-MsM8YM6ihb1pwBW1w30QBB3kymsAUpTYXJYIH6gAnidskhepTJRqCcu9ZKUsnr3stIszaraxcNLtA2OpGFhBGFzeEAwMDY1OTU4NWRlNWQ2ZmFmODE1YTBlNDFlNGQ0ZjRhZGU4YjVmNGY0ZGU1MTI0NmU0YjAxMWRjNzRlYmYwNmYyYWNYIQLKfklpnXffzy2xZ32Oo6g4Pqjb0jCirEFjoV7Hwuvz8GFko2FlWCAxNFLkFyoFsWYnscqozILuG4NXjNdwc4W4zcE0p5Zb_2FzWCAqNQ4dvvnXm3Fo1jbntQWb9mGPCO6G5vR5YVm5DowpAGFyWCBFpc0aiA3yyjA8ZJaV1dPIFu2IXV05EJ10yHa3-p6n7aRhYQFhc3hAMWRmYTMwNzg2ODgzNjMyYjM0MGVmOWIzZGQ5OGYzOWViODE3YmU5Yjk1YmI4NTcxZWMyYjVkZGQ4NzdkNWQ3MGFjWCECG25LyabCKmY6ykp_w4GZ86VGpIAHAEtulr9mdNh6LtRhZKNhZVggXd0PFoEDkj_ELaeOfy4W1X-eIWvSS9V4aNW5Td1WRtlhc1ggL89ZPJZJNi200v6QD9PJJIXkW_9nfWPZvNJzgCmZ61VhclggQAP8UZSrMQLdWmjL8linsNtwzp8c9pu4amSbZEKLBXakYWEIYXN4QDVjMDExM2E2MjYzNWQzYjY3YjMwYmQ1ZjZmNTg0ODgwMmYzYzU1YTMxZTgxMDQwNGI2YTViZWY0MGY0ZmVjYWZhY1ghAmY-lB4tOLRfOY5sMzdfgAS9RD_7Ad65_xAF64uZ63bVYWSjYWVYILs-sexg8jvVhiKOTHSaq7Hxpghh32KdSF6KPUTB4VCHYXNYIBd4XkMAordaM13ax3GAQVsONAI9EQLRKLNeoG-xI20JYXJYIOBOvrC-dAlDoAQqHMOfr1jLwYz325IYtIW-yWaoNCwJpGFhBGFzeEBlZjk2OGI5MzA0OGJiMWQ0N2I4MDQ0MGI5YTcwYzA0ZjU4Y2IxMmRiYjUwOWQyNzk4ZTI2MDRjMWI5ZWM5YmYzYWNYIQNrEkbvJxPwQLXhYV-SfxTWCiNnOtT5tz4OkTZdV4Z_hWFko2FlWCCxDijTwdR6qKB7zpdCA3hwLlPDZ9u_I3ibU6A77b5bp2FzWCD3yD_82LyzJN4xTiUb5CVvFKRLUnVfEXo921deHXeBrWFyWCCwiQFjq_DReHDcOClFZPvq5ZXbL1zCbtOKWX8Gyq0Qx6RhYQFhc3hANWJiODJlZGFiNmE5M2E3ZjYzMWNhODZhODZhZTI3ZWZmMzNkYmVhM2YxY2ExNTY0OTIwNjA2MTA4NTU2MzZiMGFjWCEDB2wjZmcGQ8nDo9eeWKewzFzWp01R5IA-CqIMBsp7W6ZhZKNhZVggmxHw2eV-ikxr9otT_nocMnDtbXSfa5WTSGM_MzJ0DNFhc1gg4H-GEoOFx5fJHYpyeJtubSY11uAkQOBXjfwjg-jZewhhclggLBXtGG6b7UEhHsqF0gBrQUOUl3KPsqIxy_jgDs5Ie2CkYWECYXN4QDE0MDY4OGIwZTU4MzYwY2JmYzYyMGMxNWY1OGI1MDI5ZjkwODhkODE3YTE5OGRjMjEwYWQ0MjZlYzViNGUzY2RhY1ghAjswtCZUfnIYN2IoaUjPCQO8qvHXHVVM9jFwoozZaNoSYWSjYWVYIB8Op_IbGlli1f5gPbXSu7OWXZqEhgSoqA6SHpsfdLKLYXNYIOS-xI71JKNbtViC9VUe3ubA8iTTbdBehMXzP8D3sWfOYXJYIPDoUZod5ooqE4AYsAySR5ENGNTc1jb5s0NxfAMKZqjLpGFhBGFzeEAxOGFiN2QwZDU4YjE2OWJmMGQ5ZjU4ZDIyOGU4NDQ3ODMzODA5NzJjZTlkODg0OWI4NDEyODcyOTM4M2ZlNzI3YWNYIQP18oyuGbv_yTM75HowZwVL_-sPYsvau3GIgOlypRl4tmFko2FlWCBuZpyK2u82ynryfahlFk4ZOom5tTKRnLjBZ-iTOu_tr2FzWCBENL4pfywiGixnS-vsk3R7eL3w-eEDFpLoprDPAf3-8GFyWCDnFd8nQbkuX_qjBVtOGFqHSBZFz3tfNeRGxlpcUEiQrKRhYQJhc3hAZjg2N2IwMzZlMWM5ZDNjOGM4NGYyY2MxZmVkODMwZjI4MmMwN2VkYjY0MmM2NGM2M2NhZGNiYWU1MzE5M2IxZmFjWCEDk06jBeU9CxF9ttf7xy5AbzXm9YvRnS1H1iSKamWn5TdhZKNhZVggWq9oaricwDSFlqTwTTl84i1u3OBiuEX5Hszuj-If4PJhc1ggyK5lpWLMmWEN5L0bFUInvLWdi2mWxT_uoTR5gCntCSFhclggaV7rzRPLrUhdi8NC_79lt5XMkEvMAPp7hDrcLLQ3HWWkYWECYXN4QDM3MjIwZGE2Y2I2OWM1NDQ1ZjQyYTZlNWYxOWFlYjQzMjdjNWUzNzU3Y2NiZDllNGRkMWI4OTY4NTNhNjdhNjlhY1ghA6M_WONaJ4uW1maWtkQO5F5lrOVTFd6PcA7Klg_AL2PSYWSjYWVYIEFEGkOs_0xgfRGdSwnuzYpYaWfxnvV1TLQ017U6nqrDYXNYIGXWM97yHn-wAW52Pzt-db5pklW2sV_MXamWeTVGGyOxYXJYIBHnpb4kjZTd0LRq1iTkusikIfeyBAGaseSZriYN1Pgw".trim();

    let re = api::receive_token(encoded_token.to_string());
    println!("receive token is {:?}", re);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());
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
