// #![allow(dead_code, unused_imports, unused_variables)]

// use rust::api_cashu_v1 as api;

// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[allow(dead_code)]
// fn bad_add(a: i32, b: i32) -> i32 {
//     a - b
// }

// pub fn setup() {
//     const DB_PATH: &str = "cashu.db";
//     let r1 = api::init_db(DB_PATH.to_string(), None, false);
//     println!("db open :{:?}", r1);
//     let r2 = api::init_cashu(0);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_get_balance() {
//         setup();
//         let mut r5 = api::get_balances();
//         println!("r5 :{:?}", r5);
//         assert_eq!(add(1, 2), 3);
//     }

//     #[test]
//     fn test_get_txs() {
//         setup();
//         let mut r5 = api::get_transactions();
//         println!("r5 :{:?}", r5);
//         assert_eq!(add(1, 2), 3);
//     }

//     #[test]
//     fn test_send_cashu() {
//         setup();
//         let mut r5 = api::send(1, "https://8333.space:3338/".to_string(), None);
//         //    let mut r6 = api::send(1, "https://8333.space:3338/".to_string());
//         //  let mut r7 = api::send(1, "https://8333.space:3338/".to_string());
//         println!("r5 :{:?}", r5.unwrap().content());
//         //println!("r6 :{:?}", r6);
//         //println!("r6 :{:?}", r7);
//         assert_eq!(add(1, 2), 3);
//     }

//     #[test]
//     fn test_receive_cashu() {
//         setup();
//         let encoded_token: &str = "cashuAeyJ0b2tlbiI6W3sibWludCI6Imh0dHBzOi8vODMzMy5zcGFjZTozMzM4IiwicHJvb2ZzIjpbeyJhbW91bnQiOjEsInNlY3JldCI6Ijlabk9oVUMxSnlHOGNlQlN3cjBBY1dhYm5tV2N5L3VyQTkzZDZoZ0JWNTA9IiwiQyI6IjAyMjI4MjU1MzM1MTAwMThjNTAzMmE1MjFhNjU3Y2I4NDhkMmQwODAxNmJjYjgwNGRlZWEzNTNkMzc5ZjM1ZGFkMyIsImlkIjoiSTJ5TitpUllma3pUIn1dfV0sIm1lbW8iOm51bGx9";
//         let r6 = api::receive_token(encoded_token.to_string());
//         println!("r6 :{:?}", r6);
//         assert_eq!(add(1, 2), 3);
//     }

//     #[test]
//     fn check_pending() {
//         setup();
//         let r6 = api::check_pending();
//         println!("r6 :{:?}", r6);
//         assert_eq!(add(1, 2), 3);
//     }

//     #[test]
//     fn get_pending_transactions() {
//         setup();
//         let r6 = api::get_pending_transactions();
//         println!("r6 :{:?}", r6);
//         assert_eq!(add(1, 2), 3);
//     }
// }
