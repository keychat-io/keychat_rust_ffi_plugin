#[macro_use]
extern crate tracing;

use anyhow::bail;
use rust::api_cashu_new::{self as api, MnemonicInfo};

use api::*;

const DB_PATH: &str = "rustest.db";
const MINT_URL: &str = "https://8333.space:3338/";
const MINT_URL_MINIBITS: &str = "https://mint.minibits.cash/Bitcoin";

fn main() {
    let migen_words = MnemonicInfo::generate_words(12).unwrap();

    let r1 = api::init_db(DB_PATH.to_string(), Some(migen_words.to_owned()), false);
    info!("init_db {}: {:?}", DB_PATH, r1);

    let _r2 = api::init_cashu(32);
    let encoded_token_8338: &str = "
    cashuBo2Ftd2h0dHBzOi8vODMzMy5zcGFjZTozMzM4YXVjc2F0YXSBomFpSADUzeNPraP9YXCCpGFhCGFzeEAwZTA0NWYzZjRjZDY3ZWNlNDcyYTM0ZjJjOWYzODljMTlmY2NjNWY5NzEzMzY3Nzg4NmIwZTA5NDZmYTFkMDQyYWNYIQOU_Y7FHmOAXINPW4AmwGQ07hZ9-AcL7rDtCp_iyuCAs2Fko2FlWCBx_pE41VXLU5445IlneDMCVOk5-Gr3S7Z10XDUSU7lomFzWCBcqHU9tKGS-BKdIWbWScrB0R4Ff4caY96iSEcviupC-mFyWCBsChqz1nwyJ1YLzNYvWJANwNyFNrPa8DDqbEr4JO33DqRhYQJhc3hAOGYxYTJmOWYyMDdkNDU0YzI4OTY3NjE3YzUwYTcyMjIxNWQ4Zjg3NmNjYTY2NzI4N2Y2OTkyZjMwYjZjN2U5NGFjWCEDUmct9nolmJ0fcSJ9995513qa2sV3mlK6VRhddxrTwMBhZKNhZVgg5dIjHJCuFye97zVpagG5aKpaypADIK7xkKjDlgxQ7dRhc1ggYSJcunjG3ECL7BOSqO5L4D-hfo1od94T3R1SzwjAMgZhclgguoeIZ3vHYzsnXmQCTvejXEMxD6DUimGQeQeJWwM0cNc-WbJVbC_9kYzZUHiBQj6VtLLTjKT6GFko2FlWCDTi92qog-b05IHVfWdRj6pBfRNcCKgLAcq7HzhenNRdmFzWCDfN1m9KVrVslmmLHAAxxg6HgEwhk9ecPkIXs029ZmdOGFyWCBy6EHafPw7hNZ7IYosOsp91vdHLucSODhSIuzLe4oUdqRhYQJhc3hAYTFmMjNmMTdiNzZlNjJkM2U3MTlmZmVjNTM0NWVlNzllYzExOGQ3ZTgzYjJkODM3MjhjYzVhNjU0MTliMjJlNmFjWCECSvj_wpaMyyB_e4Xk5w4xs4ekEW7xNeygRl1xr2zq7hphZKNhZVggwvhhZyT8UnlPZhFtpOuTyTwU3_2FYx6QJohdq3PzoIBhc1gglSSV-QAyN2zftL7QZutcubTLFSaM5cU0GE6OASdzreRhclggxQQrITja2YTUj1DZU1rBdxLj1f2An1fEu8sSBSkH7RCkYWECYXN4QGNhMTcxNDUzYjlhM2Y2NzM3NmYxMDcyYTBmZmU2ZGJmMmNmNTc5MjBiM2EzYzc0NGQ4MGYzNTlkMjNlYzdkZmRhY1ghAmkGlUujfErQLzC-9B7g-epclM9u4Ztlt-1TeEcDk_QUYWSjYWVYIFiWJy2YzO0FN4ZToXvX5RDBZhP4jZundh3aNN7zzsNkYXNYINg9LkzWKBsBGQg9f7CszLpEqeivDcfqiTiR4LEHu-FgYXJYIGSc-ovNetnigHeR7QMprgUmqxvwVc44Aao7BUMLwiCspGFhAmFzeEBlZmQxOWRiMzdiMDBhYTlmN2Y4ZjU5MzYwODliOTMwMWJiNDRlMWFkMDJkNTU2ODliYjllMjFkMzlkMmI0NjRiYWNYIQNdkcPsboZph-8dFR9_rSKaptpBhTw0F0053FeJItKQymFko2FlWCBFj-I-cycT-a_kzJAAT7fYcqoe-3l8I1-fep1l1Aj_oGFzWCDBxTNyhQZjGiayEADiYEsP2WxA82f8jReH5FGIo-bJWGFyWCDd7F6JtMrjMDgipSKp0e-lf1icUsV7GX61RlKcRhZ92qRhYQJhc3hAZjFmNjNkZDE4ZDE0YjZkODRlMDZkOWMzMzg4M2ViYzllNmI3Njc4ZGVhYTM4MDgyOTBhNDQzNGI5YmZiYjM1NGFjWCEDzX687ZVHQBg47GzqHHPq865f0Cguc0fVPPA21P5GyyhhZKNhZVggcBs3BJF4Oo7i1IUc9SjVyXc5Q-N247W4cXmtbgTfKP5hc1ggtgMBXavn8gX1K2SpzASmmkY3s4-qsT1tRHqdY6W6xLxhclgg6trDvxVs4QS-78bSObKI_Z7Jbs6x4XnmdPfOkVy0afc
    ".trim();
    let _r6 = api::receive_token(encoded_token_8338.to_string());

    let try_get_wallet = api::prepare_one_proofs(9, MINT_URL.to_string());
    println!("try_get_wallet: {:?}", try_get_wallet);
   
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
