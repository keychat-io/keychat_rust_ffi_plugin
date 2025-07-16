#[macro_use]
extern crate tracing;

use anyhow::bail;
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
    // let encoded_token_8338: &str = "cashuBo2Ftd2h0dHBzOi8vODMzMy5zcGFjZTozMzM4YXVjc2F0YXSBomFpSADUzeNPraP9YXCBpGFhCGFzeEA2MWViMTc5NjM2M2NhMDI2Njg1MTQ0NWE2YWVhNDhhMTJjZDBlYjQ2NzM3NjkzZDMwZmIwNTY1NTRlZTdlY2Y1YWNYIQMb9jewzX6rQvTbFjXiw7e68XXexs97ZYUBetjWBX91nGFko2FlWCDtNwZfO-9e8i3_uIKczjUG5O7-vvqto-PiWv9xWrXa-WFzWCBZRQtJ41ejEvkR10uO4w6bI1Y-BNIWEx0SoJ9BFfgyI2FyWCCks2KB82PCxZoQ0aKxuGBzHTDW1a0-b9U4HCBrBkYztA".trim();
    // let re = api::receive_token(encoded_token_8338.to_string());
    // println!("receive token is {:?}", re);

    // test for get mints
    // let mints = api::get_mints();
    // println!("get_mints is {:?}", mints);

    // test for send token
    // let send = api::send(2, MINT_URL.to_string(), None);
    // println!("send token is {:?}", send);

    // test for request mint
    // let request_mint = api::request_mint(2, MINT_URL.to_string());
    // println!("request_mint is {:?}", request_mint);
    // lnbc20n1p58vdq6pp5w46w0ve9rq40skwzlvk08774mtm44xymnrc3w5znymuj62zj6cssdqqcqzpuxqrwzqsp5hy7ltttz0fwgu8mpgnvn9re0638vye4ug97dr6zyxeppzukh784s9qxpqysgqxz8zzps2fyefmx8d9mq92m8xej9dy7s6kpg0x0lnxtlct30ml65rlkhwdw80ugefmeyl2jr484x4l255rc9f8nxche0lnnf6nhjwxucp4yq5my

    // test for check quote
    let amounts = api::check_all_mint_quotes();
    println!("check_all_mint_quotes {:?}", amounts);

    // test for melt
    // let invoice = "lnbc20n1p58w9l4pp5gdpv54fafwf9g8l88pjgchn07xgqwgw6srs04w4s4wptqx0qu40sdqqcqzpuxqrwzqsp50pe0jklwtcj74snhp7dw47pvsy57u2dxddffn8c65dqkhpkrekxq9qxpqysgqrdlsqcmp9wq0a56gf8r6yaxhclqn4afj2qpm8k7ff06cma5ayjrjs693jfn0x243whqd7r3a5mfak0ar3awrluydsgcr5z8dtjcgrqcqytrmul".to_string();
    // let _melt = api::melt(invoice, MINT_URL.to_string(), None);


    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);
    
    // let txs = api::get_all_transactions();
    // println!("txs {:?}", txs);
    // for tx in txs.unwrap() {
    //     println!("tx {:?}", tx);
    // }

    // test for get_cashu_transactions_with_offset
    let cashu_txs = api::get_cashu_transactions_with_offset(0,100);
    for tx in cashu_txs.unwrap() {
        println!("cashu {:?}", tx);
    }

    // test for get_ln_transactions_with_offset
    let ln_txs = api::get_ln_transactions_with_offset(0,100);
    for tx in ln_txs.unwrap() {
        println!("ln {:?}", tx);
    }


    // let try_get_wallet = api::prepare_one_proofs(9, MINT_URL.to_string());
    // println!("try_get_wallet: {:?}", try_get_wallet);
   
}

// fn main() {
//     let words = "eight reunion dish major flash artwork average usual vocal minute entire believe";
//     let sleepms_after_check_a_batch = 10;
//     let mi = MnemonicInfo::with_words(words).unwrap();
//     println!("{}: {}", mi.pubkey(), words);

//     let r1 = api::init_db(DB_PATH.to_string(), Some(words.to_owned()), false);
//     info!("init_db {}: {:?}", DB_PATH, r1);

//     let _r2 = api::init_cashu(32);
//     let start = std::time::Instant::now();
//     let re = restore(
//         MINT_URL.to_string(),
//         Some(words.to_string()),
//         sleepms_after_check_a_batch,
//     )
//     .unwrap();
//     let end = std::time::Instant::now();
//     println!("the ts gap is {:?}, result is {:?}", end - start, re);
// }

// fn main() {
//     let logger = tracing_subscriber::fmt::fmt().with_line_number(true);
//     if std::env::var("RUST_LOG").is_ok() {
//         logger.init();
//     } else {
//         let max = tracing::Level::INFO;
//         logger.with_max_level(max).init();
//     }

//     let migen_words = MnemonicInfo::generate_words(12).unwrap();
//     let migen = MnemonicInfo::with_words(&migen_words).unwrap();
//     println!("{}: {}", migen.pubkey(), migen_words);
//     let words = "sample lock neither measure violin animal upgrade shrimp wash crazy guilt issue";
//     let mi = MnemonicInfo::with_words(words).unwrap();
//     println!("{}: {}", mi.pubkey(), words);

//     let r1 = api::init_db(DB_PATH.to_string(), Some(words.to_owned()), false);
//     info!("init_db {}: {:?}", DB_PATH, r1);

//     let start = std::time::Instant::now();
//     let r2 = api::init_cashu(32);
//     info!("init_cashu {:?}: {:?}", start.elapsed(), r2);

//     if r2.is_ok()
//         && api::get_mints()
//             .map(|mints| mints.iter().any(|m| m.url == MINT_URL))
//             .unwrap_or(false)
//     {
//         let try_get_wallet = api::prepare_one_proofs(0, MINT_URL.to_string());
//         info!(
//             "try_get_wallet.prepare_one_proofs {}: {:?}\n",
//             0, try_get_wallet
//         );
//         assert!(try_get_wallet.is_ok(), "{:?}", try_get_wallet);
//     }

//     // info!("add_mint: {:?}", api::add_mint(MINT_URL.to_string()));

//     // test update restore
//     // #[rustfmt::skip]
//     // info!("{} restore: {:?}", mi.pubkey(), api::restore(MINT_URL.to_string(), Some(mi.mnemonic().to_string()),1));
//     // #[rustfmt::skip]
//     // info!("{} restore: {:?}", migen.pubkey(), api::restore(MINT_URL.to_string(), Some(migen.mnemonic().to_string()),1));

//     // test update mnemonic
//     #[rustfmt::skip]
//     let pubkey = api::get_mnemonic_info().unwrap().unwrap();
//     #[rustfmt::skip]
//     let _r2 = api::set_mnemonic(Some(migen_words.clone())).unwrap();
//     #[rustfmt::skip]
//     let pubkey2 =api::get_mnemonic_info().unwrap().unwrap();
//     assert_eq!(pubkey, mi.pubkey());
//     assert_eq!(pubkey2, migen.pubkey());
//     api::set_mnemonic(None).unwrap();

//     let r4 = api::get_mints();
//     info!("get_mints: {:?}", r4);

//     let r5 = api::get_balances();
//     info!("get_balances: {:?}", r5);

//     // receive
//     let encoded_token_8338: &str = "
//     cashuAeyJ0b2tlbiI6W3sibWludCI6Imh0dHBzOi8vODMzMy5zcGFjZTozMzM4IiwicHJvb2ZzIjpbeyJhbW91bnQiOjEsInNlY3JldCI6ImxtUEhlem5aaVVPOU81VGVaNHpsbGdJK0hwSnRtNWdxRGhBZnNGekdrYlU9IiwiQyI6IjAyMTcwNjA3MDNlMzZkNGM4NTFkODI5ZTJlN2M0ZTRiNDFmMWExNTZmNzk5MjEyNzZjMWQ1MTRiNzE3M2E0N2Y3YSIsImlkIjoiSTJ5TitpUllma3pUIn1dfV0sIm1lbW8iOm51bGx9
//     ".trim();
//     let r6 = api::receive_token(encoded_token_8338.to_string());
//     info!("receive_token_8338: {:?}", r6);
//     let encoded_token_minibits: &str = "
//     cashuBo2Ftd2h0dHBzOi8vODMzMy5zcGFjZTozMzM4YXVjc2F0YXSBomFpSAB1nj-LBrNvYXCBo2FhAWFzeEA3NGM2Mzk4MDBlYTc1MmI1NWQ3MjFkMDM4NGY0NGU3NDcxZjkzMTk4MGJkYTljZWU5MDk0YzQyYmZkNGRlMmViYWNYIQIwlROa-xDBc1uOzVUBeWN3hzkr-yk0VyYAd096_PmbQg
//     ".trim();
//     let r6 = api::receive_token(encoded_token_minibits.to_string());
//     info!("receive_token-minibits: {:?}", r6);

//     info!("check_pending: {:?}", api::check_pending());

//     info!("check_proofs: {:?}", api::check_proofs());

//     let r5 = api::get_balances();
//     info!("get_balances: {:?}", r5);

//     let amount = 0;
//     let s = api::prepare_one_proofs(amount, MINT_URL.to_string());
//     info!("prepare_one_proofs {}:{:?}\n", amount, s);

//     let s = api::get_cashu_pending_transactions();
//     info!("get_cashu_pending_transactions {}:{:?}\n", amount, s);
//     let s = api::get_ln_pending_transactions();
//     info!("get_ln_pending_transactions {}:{:?}\n", amount, s);
//     // let amount = 1;
//     // let s = api::send(amount, MINT_URL.to_string(), None).err();
//     // info!("send {}:{:?}\n", amount, s);

//     // let amount = 1;
//     // let s = api::send_stamp(amount, vec![MINT_URL_MINIBITS.to_owned(), MINT_URL.to_owned()], None).err();
//     // info!("send_stamp {}:{:?}\n", amount, s);

//     // loop {
//     //     let s = api::prepare_one_proofs(0, MINT_URL.to_string());
//     //     info!("prepare_one_proofs 0:{:?}\n", s);
//     //     let s = api::send(1, MINT_URL.to_string(), None).err();
//     //     info!("send 1:{:?}\n", s,);
//     //     if s.is_some() {
//     //         break;
//     //     }
//     // }

//     // let amount = 10;
//     // let s = api::request_mint(amount, MINT_URL.to_string());
//     // info!("request_mint {}:{:?}\n", amount, s);

//     // api::set_mnemonic(None).unwrap();
//     // let amount = 10;
//     // let tx = api::request_mint(amount, MINT_URL.to_string()).unwrap();
//     // info!("request_mint {}:{:?}\n", amount, tx);

//     // loop {
//     //     std::thread::sleep_ms(10000);
//     //     let tx = api::check_transaction(tx.id().to_owned()).unwrap();
//     //     info!("mint {}:{:?}\n", amount, tx.status());
//     //     if tx.status().is_success() {
//     //         break;
//     //     }
//     // }

//     // let hash = "wvyuGV8pgfREUegTjB12OoEFaA7ocOKILqg-oBvJ";
//     // let s = api::mint_token(amount, hash.to_owned(), MINT_URL.to_string());
//     // info!("mint {}:{:?}\n", amount, s);

//     // let pr = "lnbc500n1pjnpg9lsp50gumnh4ntmpche8wtycj6xc0f04rpavuqg0q4u6t334tqc8r3e2qpp5zhl334fm3fwlu623d0kq5k9nx8vuqaqrfsnkpgjgrxn23t65uvzsdq4gdshx6r4ypjx2ur0wd5hgxqzjccqpjrzjq2m8arjumv7cphpcwdhvpa3h3dsen2vaw6dwmmfvx3s02jx73q3djzhamyqqf6gqqyqqqqlgqqqqqzsqyg9qxpqysgqcdmk2ussa3fcu80uwp6rxnxrsyujxl24tnuc3ad6jjq02fv5z7mjqss2c87vm5xldt80gptzl88h27lm47dhnkp0mgwk9r4ywlnlswsq0mcy8t";
//     // let inv: api::Invoice = pr.parse().unwrap();
//     // info!("pr.ts: {:?}", inv.timestamp());
//     // info!("pr.to: {:?}", inv.expiry_time());
//     // info!(
//     //     "ps.amount: {:?} -> {:?}",
//     //     inv.amount_milli_satoshis(),
//     //     inv.amount_milli_satoshis().map(|a| a / 1000)
//     // );

//     // let s = api::melt(pr.to_owned(), MINT_URL.to_string(), None);
//     // info!("melt: {:?}\n", s);

//     // let r5 = api::get_balances();
//     // info!("get_balances: {:?}", r5);

//     info!(
//         "get_pending_transactions_count: {:?}\n",
//         api::get_pending_transactions_count()
//     );

//     // get_txs(3);
//     info!(
//         "remove_transactions:{:?}\n",
//         api::remove_transactions(1696928517890, TransactionStatus::Success)
//     );
//     get_txs(15);

//     // info!("get_transactions_with_offset: {:?}", api::get_transactions_with_offset(0, 0));
// }

// fn get_txs(page_limit: usize) {
//     let mut txs = vec![];
//     let mut pendings = vec![];

//     if page_limit > 0 {
//         let limit = page_limit;
//         let mut offset = 0;
//         loop {
//             info!("offset {}, limit {}", offset, limit);
//             match api::get_transactions_with_offset(offset, limit) {
//                 Err(e) => return info!("get_transactions_with_offset failed: {:?}", e),
//                 Ok(t) => {
//                     info!(
//                         "get_transactions_with_offset({}, {}) ok.len: {:?}",
//                         offset,
//                         limit,
//                         t.len()
//                     );

//                     let got = t.len();
//                     txs.extend(t);

//                     if got < limit {
//                         break;
//                     }
//                     offset += got;
//                 }
//             }
//         }
//     } else {
//         match api::get_transactions() {
//             Err(e) => return info!("get_all_transactions failed: {:?}", e),
//             Ok(mut t) => {
//                 t.sort_by_key(|a| a.time());

//                 info!("get_all_transactions ok.len: {:?}", t.len());
//                 txs = t;
//             }
//         }
//     }

//     for (idx, tx) in txs.into_iter().enumerate() {
//         let dt = timestamp_millis_to_dt(tx.time() as _).unwrap();
//         println!(
//             "{:>2} {} {}: {:>3} {:>7} {} {}",
//             idx,
//             tx.time(),
//             dt,
//             tx.direction().as_ref(),
//             tx.status().as_ref(),
//             tx.amount(),
//             tx.id()
//         );

//         if tx.is_pending() {
//             pendings.push(tx.content().to_owned());
//         }
//     }

//     for (i, tx) in pendings.into_iter().enumerate() {
//         println!("{:>2}: {}", i, tx)
//     }
// }

// fn timestamp_millis_to_dt(ts: i64) -> anyhow::Result<String> {
//     use chrono::LocalResult;
//     use chrono::TimeZone;

//     let dt = chrono::Local::now().offset().clone();
//     let dt = match dt.timestamp_millis_opt(ts) {
//         LocalResult::Single(dt) => dt,
//         e => {
//             bail!("Incorrect timestamp_millis: {:?}", e)
//         }
//     };

//     Ok(dt.to_string())
// }
