#[macro_use]
extern crate tracing;

use anyhow::bail;
use rust::api_cashu::{self as api, cashu_wallet::wallet::MnemonicInfo};

use api::*;

const DB_PATH: &str = "rustest.db";
const MINT_URL: &str = "https://8333.space:3338/";

fn main() {
    if std::env::var("RUST_LOG").is_ok() {
        tracing_subscriber::fmt::init();
    } else {
        let max = tracing::Level::INFO;
        tracing_subscriber::fmt().with_max_level(max).init()
    }

    let migen = MnemonicInfo::generate(12).unwrap();
    println!("{}: {}", migen.pubkey(), migen.mnemonic().to_string());
    let words = "sample lock neither measure violin animal upgrade shrimp wash crazy guilt issue";
    let mi = MnemonicInfo::with_words(words).unwrap();
    println!("{}: {}", mi.pubkey(), words);

    let r1 = api::init_db(DB_PATH.to_string(), Some(words.to_owned()));
    info!("init_db {}: {:?}", DB_PATH, r1);

    let start = std::time::Instant::now();
    let r2 = api::init_cashu(32);
    info!("init_cashu {:?}: {:?}", start.elapsed(), r2);

    if r2.is_ok()
        && api::get_mints()
            .map(|mints| mints.iter().any(|m| m.url == MINT_URL))
            .unwrap_or(false)
    {
        let try_get_wallet = api::prepare_one_proofs(0, MINT_URL.to_string());
        info!(
            "try_get_wallet.prepare_one_proofs {}: {:?}\n",
            0, try_get_wallet
        );
        assert!(try_get_wallet.is_ok(), "{:?}", try_get_wallet);
    }

    info!("add_mint: {:?}", api::add_mint(MINT_URL.to_string()));

    let r4 = api::get_mints();
    info!("get_mints: {:?}", r4);

    let r5 = api::get_balances();
    info!("get_balances: {:?}", r5);

    // receive
    let encoded_token: &str = "
    cashuAeyJ0b2tlbiI6W3sibWludCI6Imh0dHBzOi8vODMzMy5zcGFjZTozMzM4IiwicHJvb2ZzIjpbeyJhbW91bnQiOjEsInNlY3JldCI6ImxtUEhlem5aaVVPOU81VGVaNHpsbGdJK0hwSnRtNWdxRGhBZnNGekdrYlU9IiwiQyI6IjAyMTcwNjA3MDNlMzZkNGM4NTFkODI5ZTJlN2M0ZTRiNDFmMWExNTZmNzk5MjEyNzZjMWQ1MTRiNzE3M2E0N2Y3YSIsImlkIjoiSTJ5TitpUllma3pUIn1dfV0sIm1lbW8iOm51bGx9
    ".trim();
    let r6 = api::receive_token(encoded_token.to_string());
    info!("receive_token: {:?}", r6);

    info!("check_pending: {:?}", api::check_pending());

    info!("check_proofs: {:?}", api::check_proofs());

    let r5 = api::get_balances();
    info!("get_balances: {:?}", r5);

    let amount = 0;
    let s = api::prepare_one_proofs(amount, MINT_URL.to_string());
    info!("prepare_one_proofs {}:{:?}\n", amount, s);

    // let amount = 1;
    // let s = api::send(amount, MINT_URL.to_string(), None).err();
    // info!("send {}:{:?}\n", amount, s);

    // loop {
    //     let s = api::prepare_one_proofs(0, MINT_URL.to_string());
    //     info!("prepare_one_proofs 0:{:?}\n", s);
    //     let s = api::send(1, MINT_URL.to_string(), None).err();
    //     info!("send 1:{:?}\n", s,);
    //     if s.is_some() {
    //         break;
    //     }
    // }

    // let amount = 9000;
    // let s = api::request_mint(amount, MINT_URL.to_string());
    // info!("request_mint {}:{:?}\n", amount, s);

    // let hash = "wvyuGV8pgfREUegTjB12OoEFaA7ocOKILqg-oBvJ";
    // let s = api::mint_token(amount, hash.to_owned(), MINT_URL.to_string());
    // info!("mint {}:{:?}\n", amount, s);

    // let pr = "lnbc500n1pjnpg9lsp50gumnh4ntmpche8wtycj6xc0f04rpavuqg0q4u6t334tqc8r3e2qpp5zhl334fm3fwlu623d0kq5k9nx8vuqaqrfsnkpgjgrxn23t65uvzsdq4gdshx6r4ypjx2ur0wd5hgxqzjccqpjrzjq2m8arjumv7cphpcwdhvpa3h3dsen2vaw6dwmmfvx3s02jx73q3djzhamyqqf6gqqyqqqqlgqqqqqzsqyg9qxpqysgqcdmk2ussa3fcu80uwp6rxnxrsyujxl24tnuc3ad6jjq02fv5z7mjqss2c87vm5xldt80gptzl88h27lm47dhnkp0mgwk9r4ywlnlswsq0mcy8t";
    // let inv: api::Invoice = pr.parse().unwrap();
    // info!("pr.ts: {:?}", inv.timestamp());
    // info!("pr.to: {:?}", inv.expiry_time());
    // info!(
    //     "ps.amount: {:?} -> {:?}",
    //     inv.amount_milli_satoshis(),
    //     inv.amount_milli_satoshis().map(|a| a / 1000)
    // );

    // let s = api::melt(pr.to_owned(), MINT_URL.to_string(), None);
    // info!("melt: {:?}\n", s);

    // let r5 = api::get_balances();
    // info!("get_balances: {:?}", r5);

    info!(
        "get_pending_transactions_count: {:?}\n",
        api::get_pending_transactions_count()
    );

    // get_txs(3);
    info!(
        "remove_transactions:{:?}\n",
        api::remove_transactions(1696928517890, TransactionStatus::Success)
    );
    get_txs(15);

    // info!("get_transactions_with_offset: {:?}", api::get_transactions_with_offset(0, 0));

    // test update restore
    // #[rustfmt::skip]
    // info!("{} restore: {:?}", mi.pubkey(), api::restore(MINT_URL.to_string(), Some(mi.mnemonic().to_string()),1));
    // #[rustfmt::skip]
    // info!("{} restore: {:?}", migen.pubkey(), api::restore(MINT_URL.to_string(), Some(migen.mnemonic().to_string()),1));

    // test update mnemonic
    #[rustfmt::skip]
    let pubkey = api::get_mnemonic_info().unwrap().unwrap();
    #[rustfmt::skip]
    let _r2 = api::set_mnemonic(Some(migen.mnemonic().to_string().to_owned())).unwrap();
    #[rustfmt::skip]
    let pubkey2 =api::get_mnemonic_info().unwrap().unwrap();
    assert_eq!(pubkey, mi.pubkey());
    assert_eq!(pubkey2, migen.pubkey());
}

fn get_txs(page_limit: usize) {
    let mut txs = vec![];
    let mut pendings = vec![];

    if page_limit > 0 {
        let limit = page_limit;
        let mut offset = 0;
        loop {
            info!("offset {}, limit {}", offset, limit);
            match api::get_transactions_with_offset(offset, limit) {
                Err(e) => return info!("get_transactions_with_offset failed: {:?}", e),
                Ok(t) => {
                    info!(
                        "get_transactions_with_offset({}, {}) ok.len: {:?}",
                        offset,
                        limit,
                        t.len()
                    );

                    let got = t.len();
                    txs.extend(t);

                    if got < limit {
                        break;
                    }
                    offset += got;
                }
            }
        }
    } else {
        match api::get_transactions() {
            Err(e) => return info!("get_all_transactions failed: {:?}", e),
            Ok(mut t) => {
                t.sort_by_key(|a| a.time());

                info!("get_all_transactions ok.len: {:?}", t.len());
                txs = t;
            }
        }
    }

    for (idx, tx) in txs.into_iter().enumerate() {
        let dt = timestamp_millis_to_dt(tx.time() as _).unwrap();
        println!(
            "{:>2} {} {}: {:>3} {:>7} {} {}",
            idx,
            tx.time(),
            dt,
            tx.direction().as_ref(),
            tx.status().as_ref(),
            tx.amount(),
            tx.id()
        );

        if tx.is_pending() {
            pendings.push(tx.content().to_owned());
        }
    }

    for (i, tx) in pendings.into_iter().enumerate() {
        println!("{:>2}: {}", i, tx)
    }
}

fn timestamp_millis_to_dt(ts: i64) -> anyhow::Result<String> {
    use chrono::LocalResult;
    use chrono::TimeZone;

    let dt = chrono::Local::now().offset().clone();
    let dt = match dt.timestamp_millis_opt(ts) {
        LocalResult::Single(dt) => dt,
        e => {
            bail!("Incorrect timestamp_millis: {:?}", e)
        }
    };

    Ok(dt.to_string())
}