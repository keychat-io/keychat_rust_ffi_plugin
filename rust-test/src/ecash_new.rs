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
    // let encoded_token_8338: &str = "cashuBo2Ftd2h0dHBzOi8vODMzMy5zcGFjZTozMzM4YXVjc2F0YXSBomFpSADUzeNPraP9YXCEpGFhGEBhc3hAYThiMmE2ZjU0Mjk4NmZjYjFmN2RjNTBiODkxZWJmZmU5YTBhOTJiMDRmOGZhYWQ0YmZkYzkzNmI3MmZkYjI0OWFjWCECo2bjL4GERzUBKzFWX7TGmyXJkCq6FgR3E3EcM4IPX9hhZKNhZVggIwFsaEyV4rQlKyTH3GOnbF661BDj7KfmABSryae0zpNhc1gg4winszCgYLFVAlMLlYg_J_4sZgJWaLg0xUpe12obTGBhclggixLz2djfzSPuG2qrpIzy3d3JRh8f8otmE8keC-aL7xmkYWEQYXN4QDViNDM5MDllZjAzZGI5ZDU5ZWJiYjFlMmRiODZiZDNhM2U0ODhlZDIzMmNlOTM4NTBjNjNiNGVhZDU0NDIyZDdhY1ghAwd9afgU8iGdfr8nMA26LDvNJMgsWwnSrdLTW8fi2OxmYWSjYWVYIECkGzlVFUJV0NxQ7zprchul6PnO-Ej8yIX_ruUk-94pYXNYIJ05wkjrWHi8Ge0EH3EHlhPpm3XHiHg9UZNSowpP3HVLYXJYIJlqoZZPKfo9w0pe0gSIFTGPp-qYIP4SLQNbjVPhPbr_pGFhCGFzeEAxMDc0MGYxMTJlMzIyYWU0ZDk2MzdjYTE3MmRkZjZjMzdkYjIxNzRhYjE1NGQ0YTcxNzdjYjhiZmQyOGM4NWYxYWNYIQO8VhMEli4T8oomjCteHoeVMWSJBYHHttHE9omfVIthCmFko2FlWCDJPJk6PHLWiYitMzjRP51iJRZBjVpMLHecw2qxb_Bso2FzWCAR4hJP0E1tonDB8EupIO357D8v_lGKe5uv1y67Q-bxBWFyWCB_rrAktyIqsrDp8iNRnKLl3_jfbDz21G_BrkB0MSnC86RhYQJhc3hAYWRlZTUwZTk5ZDM5NDk4NDc3MmU0YzkwZjI3MzY3MzQ0NTFhMjExODA5ZDQzOGE0ZGZlNDFhNGU1ZGY2ODcxN2FjWCED6B26LLsnZ4170MkkrasiFnxZlyTaLXLjHDA_4N_vqJRhZKNhZVggr1vYL91h01QzO3Qxnm1gzMu1FcVrObHuDP2F57ThD9Vhc1ggGVZUJkHQJPKOPNu5TBxshN0ELY9PCIbyjImRcMA_uMxhclgg-SnfBCtsxUxoUL6vhjXdGAVDmgKo2ZWi35u7KnvKPBo".trim();
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
    // let mut stamps = vec![];
    // for _i in 0..3 {
    //     let stamp = api::send_stamp(1, vec![MINT_URL.to_string()], None).unwrap();
    //     println!("send_stamp {:?}", stamp);

    //     stamps.push(stamp.0);
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
    let _ = api::print_proofs(MINT_URL.to_string());
    // let _ = api::test_print_proofs(MINT_URL_MINIBITS.to_string());

    let prepare_one_proofs = api::prepare_one_proofs(32, MINT_URL.to_string());
    println!("prepare_one_proofs: {:?}", prepare_one_proofs);

    // test fot get balances
    let b2 = api::get_balances();
    println!("get_balances after {:?}", b2);

    // test for print proofs
    let _ = api::print_proofs(MINT_URL.to_string());
    // let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());

    // merge some proofs
    let _ = api::merge_proofs(20);


    // // send all
    // let send_all = api::send_all(MINT_URL.to_string());
    // println!("send_all token is {:?}", send_all);

    // test fot get balances
    let b3 = api::get_balances();
    println!("get_balances after {:?}", b3);
    // test for print proofs
    let _ = api::print_proofs(MINT_URL.to_string());
    // let _ = api::print_proofs(MINT_URL_MINIBITS.to_string());
}
