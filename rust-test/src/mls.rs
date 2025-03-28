use anyhow::Result;
use rust::api_mls::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WelcomeMessage {
    queued_msg: Vec<u8>,
    welcome: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct EncryptMsg {
    encrypt_msg: Vec<u8>,
    ratchet_key: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DecryptMsg {
    decrypt_msg: Vec<u8>,
    sender: String,
    ratchet_key: Option<Vec<u8>>,
}

fn main() {
    // let _ = test_basic();
    let _ = test_extension();
    // let _ = test_secret_key();
    // let _ = test_self_decrypt();
    // let _ = test_diff_groups();
    // let _ = test_exist_group();
    // let _ = test_diff_db2();
    // let _ = test_replay_delay();
    // let _ = test_remove_then_add_group();
}

fn test_diff_db1() -> Result<()> {
    println!("start -------------- start");

    let group_id = "G1";

    let a = "A";

    let db_mls_base = "./mls-base.sqlite";
    init_mls_db(db_mls_base.to_string(), a.to_string())?;

    // a create group
    let group_join_config = create_mls_group(
        a.to_string(),
        group_id.to_string(),
        "new group".to_string(),
        "new group".to_string(),
        ["admin".to_string()].to_vec(),
        ["relay.keychat.io".to_string()].to_vec(),
        "alive".to_string(),
    )?;
    println!("The group_join_config is: {:?}", group_join_config);

    let b_pk = [
        0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 164, 211, 109, 112, 177, 156, 218, 244,
        51, 130, 85, 53, 30, 239, 78, 113, 102, 33, 98, 61, 191, 96, 161, 69, 208, 208, 101, 131,
        23, 114, 88, 32, 0, 0, 0, 0, 0, 0, 0, 171, 147, 177, 201, 63, 70, 198, 109, 30, 186, 205,
        179, 177, 22, 122, 232, 128, 22, 72, 103, 156, 66, 42, 34, 10, 135, 153, 108, 237, 219,
        158, 18, 32, 0, 0, 0, 0, 0, 0, 0, 213, 167, 151, 33, 182, 209, 24, 144, 72, 246, 81, 163,
        190, 185, 52, 65, 239, 164, 246, 163, 215, 57, 126, 117, 185, 26, 64, 159, 139, 80, 174,
        101, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 66, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 2, 0, 3, 0, 77, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2,
        0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0,
        0, 3, 0, 0, 0, 4, 0, 0, 0, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153,
        122, 45, 103, 0, 0, 0, 0, 169, 70, 156, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0,
        0, 0, 0, 0, 0, 69, 220, 45, 144, 212, 86, 208, 58, 147, 229, 122, 176, 193, 113, 131, 75,
        79, 89, 211, 84, 212, 14, 167, 152, 181, 49, 232, 84, 107, 218, 176, 134, 47, 104, 63, 43,
        1, 68, 232, 66, 13, 213, 10, 202, 36, 18, 198, 103, 78, 51, 167, 104, 72, 50, 238, 131,
        152, 167, 209, 14, 142, 94, 161, 14, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 186,
        169, 255, 221, 2, 214, 70, 117, 175, 28, 239, 31, 79, 228, 182, 208, 202, 1, 94, 86, 243,
        24, 119, 196, 204, 89, 228, 25, 189, 209, 214, 41, 77, 136, 8, 148, 198, 51, 51, 159, 233,
        226, 108, 178, 57, 100, 172, 174, 51, 159, 202, 159, 126, 10, 20, 156, 107, 210, 253, 186,
        216, 116, 124, 13,
    ]
    .to_vec();

    // A add B
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk].to_vec())?;
    println!("The welcome is: {:?}", welcome);
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;
    // A send msg to B
    let msg = create_message(a.to_string(), group_id.to_string(), "hello, B".to_string())?;

    println!("A send msg to B ,the result is {:?}", msg);
    Ok(())
}

fn test_diff_db2() -> Result<()> {
    println!("start -------------- start");

    let group_id = "G1";

    let b = "B";
    let db_mls_base = "./mls-base.sqlite";

    init_mls_db(db_mls_base.to_string(), b.to_string())?;

    let b_pk = create_key_package(b.to_string())?;
    println!("The b_pk is: {:?}", b_pk);

    // a create group
    let group_join_config = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 5, 0, 0, 0, 232, 3, 0, 0,
    ]
    .to_vec();

    // A add B
    let welcome = [
        0, 1, 0, 3, 0, 1, 64, 152, 32, 177, 99, 98, 128, 139, 162, 232, 218, 114, 90, 128, 153, 66,
        124, 187, 8, 12, 74, 76, 13, 82, 177, 57, 167, 198, 116, 182, 176, 167, 40, 23, 243, 32,
        195, 241, 7, 94, 227, 136, 197, 24, 130, 23, 78, 111, 233, 252, 39, 68, 244, 54, 229, 184,
        139, 100, 76, 46, 178, 130, 61, 50, 236, 81, 45, 63, 64, 84, 24, 242, 236, 135, 187, 239,
        2, 130, 194, 220, 138, 113, 97, 134, 114, 79, 100, 229, 77, 239, 75, 100, 59, 35, 67, 252,
        11, 30, 70, 189, 235, 112, 84, 210, 24, 9, 136, 144, 253, 39, 213, 36, 174, 72, 44, 1, 100,
        254, 109, 19, 73, 186, 69, 88, 140, 246, 36, 132, 23, 223, 237, 166, 213, 11, 103, 152, 2,
        119, 152, 183, 147, 153, 138, 26, 197, 118, 131, 65, 222, 225, 25, 23, 204, 161, 66, 177,
        21, 102, 57, 212, 108, 118, 213, 185, 230, 145, 50, 143, 164, 225, 221, 65, 119, 2, 90,
        152, 244, 72, 214, 247, 72, 26, 217, 22, 1, 233, 14, 222, 198, 211, 243, 140, 18, 40, 75,
        147, 167, 24, 179, 125, 251, 194, 112, 167, 145, 204, 121, 0, 192, 70, 219, 123, 189, 131,
        35, 12, 62, 65, 182, 115, 174, 4, 42, 62, 31, 180, 29, 87, 51, 197, 209, 126, 85, 52, 237,
        163, 111, 21, 92, 35, 165, 196, 15, 14, 219, 88, 227, 145, 216, 51, 45, 66, 86, 221, 122,
        76, 177, 128, 132, 177, 50, 226, 91, 98, 55, 164, 91, 109, 247, 179, 198, 136, 193, 107,
        188, 77, 21, 0, 106, 108, 123, 33, 247, 217, 223, 48, 24, 86, 113, 227, 216, 114, 224, 253,
        89, 150, 157, 187, 10, 110, 87, 202, 78, 245, 22, 12, 245, 15, 137, 118, 185, 48, 209, 51,
        43, 150, 12, 148, 207, 253, 115, 74, 167, 199, 198, 65, 227, 239, 230, 236, 252, 249, 202,
        72, 2, 116, 128, 24, 217, 80, 26, 6, 35, 9, 184, 190, 170, 113, 230, 73, 206, 20, 80, 79,
        61, 86, 3, 244, 143, 90, 233, 219, 146, 226, 166, 240, 51, 70, 52, 177, 46, 196, 73, 126,
        197, 60, 191, 182, 85, 55, 64, 37, 140, 253, 80, 111, 147, 133, 38, 23, 144, 118, 106, 225,
        116, 54, 47, 93, 3, 70, 58, 197, 177, 166, 68, 120, 223, 188, 210, 75, 103, 193, 163, 233,
        200, 0, 229, 52, 245, 126, 253, 199, 214, 67, 74, 7, 244, 114, 116, 185, 252, 248, 92, 191,
        197, 110, 115, 151, 91, 110, 111, 158, 167, 71, 115, 182, 88, 38, 22, 218, 65, 13, 255,
        228, 132, 195, 105, 88, 11, 120, 90, 255, 46, 123, 248, 10, 59, 11, 248, 24, 37, 215, 147,
        249, 51, 43, 93, 135, 133, 63, 82, 156, 47, 164, 121, 166, 232, 87, 67, 221, 207, 27, 82,
        95, 219, 85, 154, 19, 155, 37, 183, 28, 195, 180, 42, 124, 185, 223, 2, 44, 176, 205, 20,
        32, 207, 16, 51, 134, 52, 201, 130, 4, 234, 144, 221, 215, 61, 73, 195, 56, 201, 184, 225,
        35, 69, 142, 22, 98, 253, 131, 170, 106, 61, 43, 186, 27, 205, 172, 108, 116, 101, 243,
        245, 200, 183, 237, 88, 140, 141, 252, 217, 111, 222, 99, 118, 122, 17, 222, 168, 227, 104,
        255, 133, 232, 58, 197, 163, 247, 115, 154, 157, 148, 211, 91, 55, 208, 234, 196, 183, 130,
        97, 78, 90, 98, 84, 189, 171, 73, 87, 46, 25, 44, 25, 27, 127, 150, 59, 231, 100, 130, 133,
        179, 25, 234, 231, 174, 194, 143, 131, 150, 152, 207, 84, 219, 126, 183, 48, 229, 46, 3,
        185, 154, 154, 74, 29, 187, 117, 142, 79, 206, 230, 111, 105, 232, 126, 129, 47, 111, 225,
        237, 94, 108, 236, 251, 196, 18, 127, 174, 210, 6, 245, 183, 56, 166, 100, 220, 247, 32,
        176, 103, 79, 21, 198, 95, 37, 154, 214, 8, 125, 81, 193, 251, 81, 52, 181, 220, 92, 153,
        71, 23, 120, 42, 142, 22, 71, 165, 43, 134, 64, 3, 15, 219, 236, 85, 112, 177, 61, 5, 118,
        174, 225, 115, 230, 130, 156, 210, 177, 113, 235, 54, 66, 253, 199, 206, 146, 13, 65, 124,
        53, 179, 209, 100, 23, 101, 157, 192, 33, 11, 135, 248, 245, 225, 218, 223, 91, 37, 150,
        138, 178, 216, 94, 134, 5, 247, 222, 31, 55, 163, 65, 1, 33, 85, 225, 153, 242, 210, 42,
        163, 67, 149, 168, 6, 143, 38, 67, 138, 239, 151, 11, 54, 48, 241, 18, 133, 201, 4, 235,
        229, 172, 166, 46, 37, 114, 222, 120, 85, 48, 186, 145, 150, 250, 154, 193, 248, 2, 213,
        162, 105, 246, 6, 43, 143, 15, 239, 148, 8, 226, 166, 142, 248, 217, 144, 7, 237, 64, 141,
        51, 212, 16, 172, 221, 195, 41, 239, 111, 21, 43, 181, 151, 29, 72, 113, 18, 204, 167,
    ]
    .to_vec();

    // b join in the group
    join_mls_group(b.to_string(), group_id.to_string(), welcome)?;
    let msg = [
        0, 1, 0, 2, 2, 71, 49, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 28, 231, 68, 57, 222, 238, 153, 36,
        202, 193, 244, 141, 146, 123, 207, 174, 216, 253, 62, 44, 89, 18, 85, 255, 179, 37, 190, 6,
        104, 64, 91, 179, 190, 1, 202, 39, 176, 193, 98, 121, 231, 51, 241, 0, 73, 115, 5, 242, 32,
        249, 135, 86, 182, 200, 10, 231, 62, 226, 88, 27, 85, 247, 116, 112, 133, 253, 94, 26, 155,
        76, 4, 89, 86, 237, 114, 154, 159, 0, 20, 219, 211, 187, 105, 101, 200, 142, 95, 182, 27,
        195, 169, 155, 232, 249, 135, 240, 183, 155, 216, 121, 209, 23, 236, 63, 38, 224, 211, 199,
        233, 108, 94, 177, 232, 125, 2, 34, 166, 66, 41, 249, 115, 246,
    ]
    .to_vec();
    // B decrypt A's msg
    let text = decrypt_message(b.to_string(), group_id.to_string(), msg)?;

    println!("A send msg to B ,the result is {:?}", text);
    Ok(())
}

fn test_secret_key() -> Result<()> {
    println!("start -------------- start");

    let group_id = "G1";

    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";

    let db_mls_base = "./mls-base.sqlite";
    init_mls_db(db_mls_base.to_string(), a.to_string())?;
    init_mls_db(db_mls_base.to_string(), b.to_string())?;
    init_mls_db(db_mls_base.to_string(), c.to_string())?;
    init_mls_db(db_mls_base.to_string(), d.to_string())?;
    init_mls_db(db_mls_base.to_string(), e.to_string())?;

    let b0_pk = create_key_package(b.to_string())?;
    let b_pk = create_key_package(b.to_string())?;
    let c_pk = create_key_package(c.to_string())?;
    let d0_pk = create_key_package(d.to_string())?;
    let d_pk = create_key_package(d.to_string())?;
    let e_pk = create_key_package(e.to_string())?;

    // a create group
    let group_join_config = create_mls_group(
        a.to_string(),
        group_id.to_string(),
        "new group".to_string(),
        "new group".to_string(),
        ["admin".to_string()].to_vec(),
        ["relay.keychat.io".to_string()].to_vec(),
        "alive".to_string(),
    )?;

    // A add B
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // b join in the group
    join_mls_group(b.to_string(), group_id.to_string(), welcome.welcome)?;

    // A send msg to B
    let msg = create_message(a.to_string(), group_id.to_string(), "hello, B".to_string())?;
    println!("{:?}", msg);
    // B decrypt A's msg
    let text = decrypt_message(
        b.to_string(),
        group_id.to_string(),
        msg.encrypt_msg.as_bytes().to_vec(),
    )?;
    // println!("A send msg to B ,the result is {:?}", text);
    println!("{:?}", text);

    // B send msg to A
    // let msg2 = create_message(b.to_string(), group_id.to_string(), "hello, A".to_string())?;
    // A decrypt B's msg
    // let text2 = decrypt_msg(a.to_string(), group_id.to_string(), msg2.0)?;
    // println!("B send msg to A ,the result is {:?}", text2);

    // A send msg to B
    let msg3 = create_message(a.to_string(), group_id.to_string(), "hello, B2".to_string())?;
    println!("{:?}", msg3);
    // B decrypt A's msg
    let text3 = decrypt_message(
        b.to_string(),
        group_id.to_string(),
        msg3.encrypt_msg.as_bytes().to_vec(),
    )?;
    // println!("A send msg to B2 ,the result is {:?}", text3);
    println!("{:?}", text3);

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group tree hash {:?}",
        get_tree_hash(b.to_string(), group_id.to_string()).unwrap()
    );
    Ok(())
}

fn test_exist_group() -> Result<()> {
    println!("start -------------- start");

    let group_id = "G1";
    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";
    let f = "F";
    let g = "G";

    let db_mls_base = "./mls-base.sqlite";
    init_mls_db(db_mls_base.to_string(), a.to_string())?;
    init_mls_db(db_mls_base.to_string(), b.to_string())?;
    init_mls_db(db_mls_base.to_string(), c.to_string())?;
    init_mls_db(db_mls_base.to_string(), d.to_string())?;
    init_mls_db(db_mls_base.to_string(), e.to_string())?;
    init_mls_db(db_mls_base.to_string(), f.to_string())?;
    init_mls_db(db_mls_base.to_string(), g.to_string())?;

    let group_join_config = get_group_config(a.to_string(), group_id.to_string())?;

    let f_pk = create_key_package(f.to_string())?;
    let g_pk = create_key_package(g.to_string())?;

    // A add G
    let welcome = add_members(a.to_string(), group_id.to_string(), [f_pk, g_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // F join in the group
    join_mls_group(f.to_string(), group_id.to_string(), welcome.welcome.clone())?;

    // G join in the group
    join_mls_group(g.to_string(), group_id.to_string(), welcome.welcome.clone())?;

    // D commit
    let _ = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        welcome.queued_msg.clone().as_bytes().to_vec(),
    )?;

    // E commit
    let _ = others_commit_normal(
        e.to_string(),
        group_id.to_string(),
        welcome.queued_msg.clone().as_bytes().to_vec(),
    )?;

    // A send msg to G
    let msg = create_message(a.to_string(), group_id.to_string(), "hello, G".to_string())?;
    // F decrypt A's msg
    let text = decrypt_message(
        f.to_string(),
        group_id.to_string(),
        msg.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to F ,the result is {:?}", text);

    // E decrypt A's msg
    let text = decrypt_message(
        e.to_string(),
        group_id.to_string(),
        msg.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to E ,the result is {:?}", text);

    // G decrypt A's msg
    let text = decrypt_message(
        g.to_string(),
        group_id.to_string(),
        msg.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to G ,the result is {:?}", text);

    // D decrypt A's msg
    let text = decrypt_message(
        d.to_string(),
        group_id.to_string(),
        msg.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to D ,the result is {:?}", text);

    Ok(())
}

fn test_remove_then_add_group() -> Result<()> {
    println!("start -------------- start");

    let group_id = "G1";
    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";

    let db_mls_base = "./mls-base.sqlite";
    init_mls_db(db_mls_base.to_string(), a.to_string())?;
    init_mls_db(db_mls_base.to_string(), b.to_string())?;
    init_mls_db(db_mls_base.to_string(), c.to_string())?;
    init_mls_db(db_mls_base.to_string(), d.to_string())?;

    let group_join_config = create_mls_group(
        a.to_string(),
        group_id.to_string(),
        "new group".to_string(),
        "new group".to_string(),
        ["admin".to_string()].to_vec(),
        ["relay.keychat.io".to_string()].to_vec(),
        "alive".to_string(),
    )?;

    let b_pk = create_key_package(b.to_string())?;
    let b_pk2 = create_key_package(b.to_string())?;
    let c_pk = create_key_package(c.to_string())?;

    // A add B C
    let welcome = add_members(
        a.to_string(),
        group_id.to_string(),
        [b_pk.clone(), c_pk].to_vec(),
    )?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // B join in the group
    join_mls_group(b.to_string(), group_id.to_string(), welcome.welcome.clone())?;

    // C join in the group
    join_mls_group(c.to_string(), group_id.to_string(), welcome.welcome.clone())?;
    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    let b_leaf_node = get_lead_node_index(a.to_string(), b.to_string(), group_id.to_string())?;
    // A remove B
    let queued_msg = remove_members(a.to_string(), group_id.to_string(), [b_leaf_node].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // B commit
    let _ = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    let _ = delete_group(b.to_string(), group_id.to_string())?;

    // C commit
    let _ = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    // then A add B again
    // A add B
    let welcome2 = add_members(
        a.to_string(),
        group_id.to_string(),
        [b_pk2.clone()].to_vec(),
    )?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // B join in the group
    join_mls_group(
        b.to_string(),
        group_id.to_string(),
        welcome2.welcome.clone(),
    )?;
    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    Ok(())
}

fn test_diff_groups() -> Result<()> {
    println!("start -------------- start");

    let group_id = "G1";
    let group_id2 = "G2";

    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";

    let db_mls_base = "./mls-base.sqlite";

    init_mls_db(db_mls_base.to_string(), a.to_string())?;
    init_mls_db(db_mls_base.to_string(), b.to_string())?;
    init_mls_db(db_mls_base.to_string(), c.to_string())?;
    init_mls_db(db_mls_base.to_string(), d.to_string())?;
    init_mls_db(db_mls_base.to_string(), e.to_string())?;

    let b_pk = create_key_package(b.to_string())?;
    let c_pk = create_key_package(c.to_string())?;
    let d_pk = create_key_package(d.to_string())?;
    let e_pk = create_key_package(e.to_string())?;

    // a create group
    let group_join_config = create_mls_group(
        a.to_string(),
        group_id.to_string(),
        "new group".to_string(),
        "new group".to_string(),
        ["admin".to_string()].to_vec(),
        ["relay.keychat.io".to_string()].to_vec(),
        "alive".to_string(),
    )?;

    // A add B
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk.clone()].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // b join in the group
    join_mls_group(b.to_string(), group_id.to_string(), welcome.welcome)?;

    // A send msg
    let msg = create_message(
        a.to_string(),
        group_id.to_string(),
        "hello, A to B".to_string(),
    )?;

    // B decrypt A's msg
    let text = decrypt_message(
        b.to_string(),
        group_id.to_string(),
        msg.encrypt_msg.as_bytes().to_vec(),
    )?;
    println!("A send msg to B ,the result is {:?}", text);

    // create second group use the same keypackage
    // c create group
    let group_join_config2 = create_mls_group(
        c.to_string(),
        group_id2.to_string(),
        "new group".to_string(),
        "new group".to_string(),
        ["admin".to_string()].to_vec(),
        ["relay.keychat.io".to_string()].to_vec(),
        "alive".to_string(),
    )?;

    // C add B
    let welcome2 = add_members(
        c.to_string(),
        group_id2.to_string(),
        [b_pk.clone()].to_vec(),
    )?;
    // A commit
    self_commit(c.to_string(), group_id2.to_string())?;

    // b join in the group
    join_mls_group(b.to_string(), group_id2.to_string(), welcome2.welcome)?;
    println!("join_mls_group");

    // C send msg
    let msg = create_message(
        c.to_string(),
        group_id2.to_string(),
        "hello, C to B".to_string(),
    )?;

    // B decrypt A's msg
    let text = decrypt_message(
        b.to_string(),
        group_id2.to_string(),
        msg.encrypt_msg.as_bytes().to_vec(),
    )?;
    println!("C send msg to B ,the result is {:?}", text);

    Ok(())
}

// could not decrypt self msg? YES
fn test_self_decrypt() -> Result<()> {
    println!("start -------------- start");

    let group_id = "G11";

    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";

    let db_mls_base = "./mls-base.sqlite";

    init_mls_db(db_mls_base.to_string(), a.to_string())?;
    init_mls_db(db_mls_base.to_string(), b.to_string())?;
    init_mls_db(db_mls_base.to_string(), c.to_string())?;
    init_mls_db(db_mls_base.to_string(), d.to_string())?;
    init_mls_db(db_mls_base.to_string(), e.to_string())?;

    let b0_pk = create_key_package(b.to_string())?;
    let b1_pk = create_key_package(b.to_string())?;
    let b2_pk = create_key_package(b.to_string())?;
    let b_pk = create_key_package(b.to_string())?;

    // a create group
    let group_join_config = create_mls_group(
        a.to_string(),
        group_id.to_string(),
        "new group".to_string(),
        "new group".to_string(),
        ["admin".to_string()].to_vec(),
        ["relay.keychat.io".to_string()].to_vec(),
        "alive".to_string(),
    )?;

    // A add B
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // b join in the group
    join_mls_group(b.to_string(), group_id.to_string(), welcome.welcome)?;

    // A send msg
    let msg = create_message(a.to_string(), group_id.to_string(), "hello, B".to_string())?;
    // B decrypt A's msg
    let text = decrypt_message(
        b.to_string(),
        group_id.to_string(),
        msg.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to B ,the result is {:?}", text);
    // A can not decrypt self msg
    // // A decrypt A's msg
    // let text = decrypt_msg(a.to_string(), group_id.to_string(), msg.0.clone())?;
    // println!("A send msg to A ,the result is {:?}", text);
    Ok(())
}

// create add create_message decrypt_msg remove leave
fn test_basic() -> Result<()> {
    println!("start -------------- start");

    let group_id = "G1";

    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";

    let db_mls_base = "./mls-base.sqlite";

    // let users = vec![a, b, c, d, e];
    // let base_db_path = "./mls-lite";
    // for user in users {
    //     let db_path = format!("{}-{}.sqlite", base_db_path, user);
    //     init_mls_db(db_path.to_string(), user.to_string())?;
    // }

    init_mls_db(db_mls_base.to_string(), a.to_string())?;
    init_mls_db(db_mls_base.to_string(), b.to_string())?;
    init_mls_db(db_mls_base.to_string(), c.to_string())?;
    init_mls_db(db_mls_base.to_string(), d.to_string())?;
    init_mls_db(db_mls_base.to_string(), e.to_string())?;

    let b0_pk = create_key_package(b.to_string())?;
    let b_pk = create_key_package(b.to_string())?;

    // test delete keypackage
    // let _ = delete_key_package( b.to_string(), b0_pk);

    let c_pk = create_key_package(c.to_string())?;
    let d0_pk = create_key_package(d.to_string())?;
    let d_pk = create_key_package(d.to_string())?;
    let e_pk = create_key_package(e.to_string())?;

    let description: String = "123456".to_string();
    let admin_pubkeys_hex: Vec<String> = ["abc".to_string()].to_vec();
    let group_relays: Vec<String> = ["wss://relay.keychat.io".to_string()].to_vec();

    // a create group
    create_mls_group(
        a.to_string(),
        group_id.to_string(),
        group_id.to_string(),
        description,
        admin_pubkeys_hex,
        group_relays,
        "alive".to_string(),
    )?;

    // A add B
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    let welcome = welcome.welcome;

    // let extension = parse_welcome_message(b.to_string(), welcome.clone())?;
    // println!("b parse welcome message {:?}", extension);

    // b join in the group
    join_mls_group(b.to_string(), group_id.to_string(), welcome.clone())?;

    let members = get_group_members(a.to_string(), group_id.to_string())?;
    println!("group members of a is {:?}", members);

    let extension = get_group_extension(a.to_string(), group_id.to_string())?;
    println!("group extension of a is {:?}", extension);

    let members = get_group_members(b.to_string(), group_id.to_string())?;
    println!("group members of b is {:?}", members);

    let extension = get_group_extension(b.to_string(), group_id.to_string())?;
    println!("group extension of b is {:?}", extension);

    // A send msg to B
    let msg = create_message(a.to_string(), group_id.to_string(), "hello, B".to_string())?;

    // B decrypt A's msg
    let text = decrypt_message(
        b.to_string(),
        group_id.to_string(),
        msg.encrypt_msg.as_bytes().to_vec(),
    )?;

    println!("A send msg to B ,the result is {:?}", text);

    // B send msg to A
    let msg2 = create_message(b.to_string(), group_id.to_string(), "hello, A".to_string())?;

    // A decrypt B's msg
    let text2 = decrypt_message(
        a.to_string(),
        group_id.to_string(),
        msg2.encrypt_msg.as_bytes().to_vec(),
    )?;
    println!("B send msg to A ,the result is {:?}", text2);

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group tree hash {:?}",
        get_tree_hash(b.to_string(), group_id.to_string()).unwrap()
    );

    println!("--B add C --------------");

    // B add C
    let welcome2 = add_members(b.to_string(), group_id.to_string(), [c_pk].to_vec())?;
    // B commit
    self_commit(b.to_string(), group_id.to_string())?;

    // c join the group
    join_mls_group(c.to_string(), group_id.to_string(), welcome2.welcome)?;
    // A commit
    let _ = others_commit_normal(
        a.to_string(),
        group_id.to_string(),
        welcome2.queued_msg.as_bytes().to_vec(),
    )?;

    // B send msg
    let msg3 = create_message(
        b.to_string(),
        group_id.to_string(),
        "hello, A, C".to_string(),
    )?;

    // C decrypt B's msg
    let text3 = decrypt_message(
        c.to_string(),
        group_id.to_string(),
        msg3.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B send msg to C ,the result is {:?}", text3);

    // A decrypt B's msg
    let text4 = decrypt_message(
        a.to_string(),
        group_id.to_string(),
        msg3.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B send msg to A ,the result is {:?}", text4);

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group tree hash {:?}",
        get_tree_hash(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group tree hash {:?}",
        get_tree_hash(c.to_string(), group_id.to_string()).unwrap()
    );

    println!("--A add D --------------");

    // A add D
    let welcome3 = add_members(a.to_string(), group_id.to_string(), [d_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // d join the group
    join_mls_group(d.to_string(), group_id.to_string(), welcome3.welcome)?;

    // B commit
    let _ = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;

    // C commit
    let _ = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;

    // A send msg
    let msg4 = create_message(
        a.to_string(),
        group_id.to_string(),
        "hello, ABC".to_string(),
    )?;

    // B decrypt A's msg
    let text5 = decrypt_message(
        b.to_string(),
        group_id.to_string(),
        msg4.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to B ,the result is {:?}", text5);
    // C decrypt A's msg
    let text6 = decrypt_message(
        c.to_string(),
        group_id.to_string(),
        msg4.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to C ,the result is {:?}", text6);
    // D decrypt B's msg
    let text7 = decrypt_message(
        d.to_string(),
        group_id.to_string(),
        msg4.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to D ,the result is {:?}", text7);

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    let b_leaf_node = get_lead_node_index(a.to_string(), b.to_string(), group_id.to_string())?;

    // A remove B
    let queued_msg = remove_members(a.to_string(), group_id.to_string(), [b_leaf_node].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // B commit
    let _ = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;

    // C commit
    let _ = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;

    // D commit
    let _ = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;

    println!("--A remove B --------------");

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    // A add E
    let welcome4 = add_members(a.to_string(), group_id.to_string(), [e_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // E join the group
    join_mls_group(e.to_string(), group_id.to_string(), welcome4.welcome)?;

    // C commit
    let _ = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        welcome4.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // D commit
    let _ = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        welcome4.queued_msg.clone().as_bytes().to_vec(),
    )?;

    println!("--A add E --------------");

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        get_export_secret(e.to_string(), group_id.to_string()).unwrap()
    );

    println!("--C leave --------------");

    let queued_msg = self_leave(c.to_string(), group_id.to_string())?;

    // when C proposal that can not send msg again.
    // C send msg
    // let msg5 = create_message(
    //     c.to_string(),
    //     group_id.to_string(),
    //     "C hello, ADE".to_string(),
    // )?;

    // // A decrypt C's msg
    // let text5 = decrypt_msg(a.to_string(), group_id.to_string(), msg5.0.clone())?;
    // println!("C send msg to A ,the result is {:?}", text5);

    // A proposal
    let _ = others_proposal_leave(a.to_string(), group_id.to_string(), queued_msg.clone())?;
    // D proposal
    let _ = others_proposal_leave(d.to_string(), group_id.to_string(), queued_msg.clone())?;
    // E proposal
    let _ = others_proposal_leave(e.to_string(), group_id.to_string(), queued_msg.clone())?;

    // admin proposal and commit
    let queued_msg = admin_proposal_leave(a.to_string(), group_id.to_string())?;
    println!(
        "c self_leave a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    // when A proposal that can not send msg again.
    // A send msg
    // let msg5 = create_message(
    //     a.to_string(),
    //     group_id.to_string(),
    //     "A hello, DCE".to_string(),
    // )?;

    // // A decrypt C's msg
    // let text5 = decrypt_msg(d.to_string(), group_id.to_string(), msg5.0.clone())?;
    // println!("A send msg to D ,the result is {:?}", text5);

    let _ = admin_commit_leave(a.to_string(), group_id.to_string())?;

    // D commit
    let _ = normal_member_commit_leave(d.to_string(), group_id.to_string(), queued_msg.clone())?;
    // E commit
    let _ = normal_member_commit_leave(e.to_string(), group_id.to_string(), queued_msg.clone())?;
    // C commit
    let _ = normal_member_commit_leave(c.to_string(), group_id.to_string(), queued_msg.clone())?;

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    // println!(
    //     "c_mls_group export secret {:?}",
    //     get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    // );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        get_export_secret(e.to_string(), group_id.to_string()).unwrap()
    );

    println!("--A UPDATE --------------");

    // admin update
    let queued_msg = self_update(a.to_string(), group_id.to_string(), "".as_bytes().to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // E commit
    let _ = others_commit_normal(
        e.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // D commit
    let _ = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        get_export_secret(e.to_string(), group_id.to_string()).unwrap()
    );

    println!("end --------------end");
    Ok(())
}

// create add create_message decrypt_msg remove leave
fn test_extension() -> Result<()> {
    println!("start -------------- start");

    let group_id = "G1";

    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";

    let db_mls_base = "./mls-base.sqlite";

    init_mls_db(db_mls_base.to_string(), a.to_string())?;
    init_mls_db(db_mls_base.to_string(), b.to_string())?;
    init_mls_db(db_mls_base.to_string(), c.to_string())?;
    init_mls_db(db_mls_base.to_string(), d.to_string())?;
    init_mls_db(db_mls_base.to_string(), e.to_string())?;

    // let a_pk = create_key_package(a.to_string())?;
    let b_pk = create_key_package(b.to_string())?;
    let c_pk = create_key_package(c.to_string())?;
    let d_pk = create_key_package(d.to_string())?;
    let e_pk = create_key_package(e.to_string())?;

    let description: String = "this a group test".to_string();
    let admin_pubkeys_hex: Vec<String> = ["A".to_string()].to_vec();
    let group_relays: Vec<String> = ["wss://relay.keychat.io".to_string()].to_vec();

    // a create group
    create_mls_group(
        a.to_string(),
        group_id.to_string(),
        group_id.to_string(),
        description,
        admin_pubkeys_hex.clone(),
        group_relays.clone(),
        "alive".to_string(),
    )?;

    // A add B
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // let extension = parse_welcome_message(b.to_string(), welcome.clone())?;
    // println!("b parse welcome message {:?}", extension);

    // b join in the group
    join_mls_group(b.to_string(), group_id.to_string(), welcome.welcome)?;

    let members = get_group_members(a.to_string(), group_id.to_string())?;
    println!("group members of a is {:?}", members);

    let extension = get_group_extension(a.to_string(), group_id.to_string())?;
    println!("group extension of a is {:?}", extension);

    let members = get_group_members(b.to_string(), group_id.to_string())?;
    println!("group members of b is {:?}", members);

    let extension = get_group_extension(b.to_string(), group_id.to_string())?;
    println!("group extension of b is {:?}", extension);

    println!("--B add C E--------------");

    // B add C E
    let welcome2 = add_members(b.to_string(), group_id.to_string(), [c_pk, e_pk].to_vec())?;
    // B commit
    self_commit(b.to_string(), group_id.to_string())?;

    // c join the group
    join_mls_group(c.to_string(), group_id.to_string(), welcome2.welcome)?;
    // // e join the group
    // join_mls_group(e.to_string(), group_id.to_string(), welcome2.welcome)?;
    // A commit
    let a_commit = others_commit_normal(
        a.to_string(),
        group_id.to_string(),
        welcome2.queued_msg.as_bytes().to_vec(),
    )?;
    println!("a_commit is {:?}", a_commit);

    println!("--A add D --------------");

    // A add D
    let welcome3 = add_members(a.to_string(), group_id.to_string(), [d_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // d join the group
    join_mls_group(d.to_string(), group_id.to_string(), welcome3.welcome)?;

    // B commit
    let b_commit = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("b_commit is {:?}", b_commit);

    // C commit
    let c_commit = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("c_commit is {:?}", c_commit);

    println!("--A UPDATE --------------");

    let a_update_extension = "A update the extension".as_bytes().to_vec();
    // A update
    let queued_msg = self_update(a.to_string(), group_id.to_string(), a_update_extension)?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    let leaf_nodes_extension = get_member_extension(a.to_string(), group_id.to_string())?;
    println!("leaf_nodes_extension is {:?}", leaf_nodes_extension);

    let sender = get_sender(
        b.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("sender is {:?}", sender);

    let sender = is_admin(
        b.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("is admin is {:?}", sender);

    // // B commit
    let b_commit = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B commit is {:?}", b_commit);
    // C commit
    let c_commit = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("C commit is {:?}", c_commit);
    // D commit
    let d_commit = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("D commit is {:?}", d_commit);

    println!("--B UPDATE --------------");

    let b_update_extension = "B update the extension".as_bytes().to_vec();
    // admin update
    let queued_msg = self_update(b.to_string(), group_id.to_string(), b_update_extension)?;
    // B commit
    self_commit(b.to_string(), group_id.to_string())?;

    let leaf_nodes_extension = get_member_extension(b.to_string(), group_id.to_string())?;
    println!("leaf_nodes_extension is {:?}", leaf_nodes_extension);

    let sender = get_sender(
        a.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("sender is {:?}", sender);

    let sender = is_admin(
        a.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("is admin is {:?}", sender);
    // // A commit
    let a_commit = others_commit_normal(
        a.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A commit is {:?}", a_commit);
    // C commit
    let c_commit = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("C commit is {:?}", c_commit);
    // D commit
    let d_commit = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("D commit is {:?}", d_commit);

    println!("--C UPDATE --------------");

    let c_update_extension = "C update the extension".as_bytes().to_vec();
    // A update
    let queued_msg = self_update(c.to_string(), group_id.to_string(), c_update_extension)?;
    // A commit
    self_commit(c.to_string(), group_id.to_string())?;

    let leaf_nodes_extension = get_member_extension(c.to_string(), group_id.to_string())?;
    println!("leaf_nodes_extension is {:?}", leaf_nodes_extension);

    let sender = get_sender(
        a.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("sender is {:?}", sender);

    let sender = is_admin(
        a.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("is admin is {:?}", sender);

    // A commit
    let _ = others_commit_normal(
        a.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A commit");
    // B commit
    let _ = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B commit");
    // D commit
    let _ = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("D commit");

    println!("--D UPDATE --------------");

    let d_update_extension = "D update the extension".as_bytes().to_vec();
    // D update
    let queued_msg = self_update(d.to_string(), group_id.to_string(), d_update_extension)?;
    // D commit
    self_commit(d.to_string(), group_id.to_string())?;

    let leaf_nodes_extension = get_member_extension(d.to_string(), group_id.to_string())?;
    println!("leaf_nodes_extension is {:?}", leaf_nodes_extension);

    // A commit
    let _ = others_commit_normal(
        a.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A commit");
    // B commit
    let _ = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B commit");
    // C commit
    let _ = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("C commit");

    println!("A update_group_context_extensions");
    let update_description: String = "update group test".to_string();
    let update_state: String = "dissolve".to_string();
    let update_result = update_group_context_extensions(
        a.to_string(),
        group_id.to_string(),
        Some("test test".to_string()),
        Some(update_description),
        None,
        None,
        Some(update_state),
    )?;

    self_commit(a.to_string(), group_id.to_string())?;

    let a_extension = get_group_extension(a.to_string(), group_id.to_string())?;
    println!("a_extension is {:?}", a_extension);

    // B commit
    let b_commit = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        update_result.clone().as_bytes().to_vec(),
    )?;
    println!("B commit is {:?}", b_commit);
    // C commit
    let c_commit = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        update_result.clone().as_bytes().to_vec(),
    )?;
    println!("C commit is {:?}", c_commit);
    // D commit
    let d_commit = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        update_result.clone().as_bytes().to_vec(),
    )?;
    println!("D commit is {:?}", d_commit);

    let b_extension = get_group_extension(b.to_string(), group_id.to_string())?;
    println!("b_extension is {:?}", b_extension);

    let c_extension = get_group_extension(c.to_string(), group_id.to_string())?;
    println!("c_extension is {:?}", c_extension);

    let d_extension = get_group_extension(d.to_string(), group_id.to_string())?;
    println!("d_extension is {:?}", d_extension);

    // B send msg
    let msg3 = create_message(
        b.to_string(),
        group_id.to_string(),
        "hello, A, C".to_string(),
    )?;
    // C decrypt B's msg
    let text3 = decrypt_message(
        c.to_string(),
        group_id.to_string(),
        msg3.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B send msg to C ,the result is {:?}", text3);

    // A decrypt B's msg
    let text4 = decrypt_message(
        a.to_string(),
        group_id.to_string(),
        msg3.encrypt_msg.as_bytes().to_vec(),
    )?;
    println!("B send msg to A ,the result is {:?}", text4);

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group tree hash {:?}",
        get_tree_hash(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group tree hash {:?}",
        get_tree_hash(c.to_string(), group_id.to_string()).unwrap()
    );

    println!("--A remove B C --------------");

    let b_leaf_node = get_lead_node_index(a.to_string(), b.to_string(), group_id.to_string())?;
    let c_leaf_node = get_lead_node_index(a.to_string(), c.to_string(), group_id.to_string())?;

    // A remove B C
    let queued_msg = remove_members(
        a.to_string(),
        group_id.to_string(),
        [b_leaf_node, c_leaf_node].to_vec(),
    )?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // B commit
    let b_commit = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B commit is {:?}", b_commit);

    // C commit
    let c_commit = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("C commit is {:?}", c_commit);

    // D commit
    let d_commit = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    println!("D commit is {:?}", d_commit);

    // delte group
    delete_group(a.to_string(), group_id.to_string())?;

    println!("end --------------end");
    Ok(())
}

fn test_normal() -> Result<()> {
    println!("start --------------start");
    let group_id = "G1";
    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";
    let f = "F";
    let g = "G";

    let db_mls_base = "./mls-base.sqlite";

    init_mls_db(db_mls_base.to_string(), a.to_string())?;
    init_mls_db(db_mls_base.to_string(), b.to_string())?;
    init_mls_db(db_mls_base.to_string(), c.to_string())?;
    init_mls_db(db_mls_base.to_string(), d.to_string())?;
    init_mls_db(db_mls_base.to_string(), e.to_string())?;
    init_mls_db(db_mls_base.to_string(), f.to_string())?;
    init_mls_db(db_mls_base.to_string(), g.to_string())?;

    let b_pk = create_key_package(b.to_string())?;
    let c_pk = create_key_package(c.to_string())?;
    let d_pk = create_key_package(d.to_string())?;
    let e_pk = create_key_package(e.to_string())?;
    let f_pk = create_key_package(f.to_string())?;
    let g_pk = create_key_package(g.to_string())?;

    // a create group
    let group_join_config = create_mls_group(
        a.to_string(),
        group_id.to_string(),
        group_id.to_string(),
        "new group".to_string(),
        ["admin".to_string()].to_vec(),
        ["relay.keychat.io".to_string()].to_vec(),
        "alive".to_string(),
    )?;

    // A add B F
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk, f_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // b join in the group
    join_mls_group(b.to_string(), group_id.to_string(), welcome.welcome.clone())?;

    // f join in the group
    join_mls_group(f.to_string(), group_id.to_string(), welcome.welcome.clone())?;

    // A send msg to B F
    let msg = create_message(
        a.to_string(),
        group_id.to_string(),
        "hello, B F".to_string(),
    )?;

    // B decrypt A's msg
    let text = decrypt_message(
        b.to_string(),
        group_id.to_string(),
        msg.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to B ,the result is {:?}", text);
    // F decrypt A's msg
    let text = decrypt_message(
        f.to_string(),
        group_id.to_string(),
        msg.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to F ,the result is {:?}", text);

    // B send msg to A
    let msg2 = create_message(b.to_string(), group_id.to_string(), "hello, A".to_string())?;

    // A decrypt B's msg
    let text2 = decrypt_message(
        a.to_string(),
        group_id.to_string(),
        msg2.encrypt_msg.as_bytes().to_vec(),
    )?;
    println!("B send msg to A ,the result is {:?}", text2);

    // F send msg to A
    let msg2_1 = create_message(f.to_string(), group_id.to_string(), "hello, F".to_string())?;

    // A decrypt F's msg
    let text2_1 = decrypt_message(
        a.to_string(),
        group_id.to_string(),
        msg2_1.encrypt_msg.as_bytes().to_vec(),
    )?;
    println!("F send msg to A ,the result is {:?}", text2_1);

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );
    println!(
        "f_mls_group export secret {:?}",
        get_export_secret(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group tree hash {:?}",
        get_tree_hash(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group tree hash {:?}",
        get_tree_hash(f.to_string(), group_id.to_string()).unwrap()
    );

    println!("--B add C G --------------");

    // B add C and G
    let welcome2 = add_members(b.to_string(), group_id.to_string(), [c_pk, g_pk].to_vec())?;
    // B commit
    self_commit(b.to_string(), group_id.to_string())?;
    // c join the group
    join_mls_group(
        c.to_string(),
        group_id.to_string(),
        welcome2.welcome.clone(),
    )?;

    // g join the group
    join_mls_group(
        g.to_string(),
        group_id.to_string(),
        welcome2.welcome.clone(),
    )?;

    // A commit
    let _ = others_commit_normal(
        a.to_string(),
        group_id.to_string(),
        welcome2.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // F commit
    let _ = others_commit_normal(
        f.to_string(),
        group_id.to_string(),
        welcome2.queued_msg.clone().as_bytes().to_vec(),
    )?;

    // B send msg
    let msg3 = create_message(
        b.to_string(),
        group_id.to_string(),
        "hello, A, C, F, G".to_string(),
    )?;

    // A decrypt B's msg
    let text3 = decrypt_message(
        a.to_string(),
        group_id.to_string(),
        msg3.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B send msg to A ,the result is {:?}", text3);
    // C decrypt B's msg
    let text3 = decrypt_message(
        c.to_string(),
        group_id.to_string(),
        msg3.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B send msg to C ,the result is {:?}", text3);
    // F decrypt B's msg
    let text3 = decrypt_message(
        f.to_string(),
        group_id.to_string(),
        msg3.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B send msg to F ,the result is {:?}", text3);
    // G decrypt B's msg
    let text3 = decrypt_message(
        g.to_string(),
        group_id.to_string(),
        msg3.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B send msg to G ,the result is {:?}", text3);

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group export secret {:?}",
        get_export_secret(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        get_export_secret(g.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group tree hash {:?}",
        get_tree_hash(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group tree hash {:?}",
        get_tree_hash(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group tree hash {:?}",
        get_tree_hash(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group tree hash {:?}",
        get_tree_hash(g.to_string(), group_id.to_string()).unwrap()
    );

    println!("--A add D --------------");

    // A add D
    let welcome3 = add_members(a.to_string(), group_id.to_string(), [d_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // d join the group
    join_mls_group(d.to_string(), group_id.to_string(), welcome3.welcome)?;

    // B commit
    let _ = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // C commit
    let _ = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // F commit
    let _ = others_commit_normal(
        f.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // G commit
    let _ = others_commit_normal(
        g.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;

    // A send msg
    let msg4 = create_message(
        a.to_string(),
        group_id.to_string(),
        "hello, ABCDFG".to_string(),
    )?;

    // B decrypt A's msg
    let text4 = decrypt_message(
        b.to_string(),
        group_id.to_string(),
        msg4.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to B ,the result is {:?}", text4);
    // C decrypt A's msg
    let text4 = decrypt_message(
        c.to_string(),
        group_id.to_string(),
        msg4.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to C ,the result is {:?}", text4);
    // D decrypt B's msg
    let text4 = decrypt_message(
        d.to_string(),
        group_id.to_string(),
        msg4.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to D ,the result is {:?}", text4);
    // F decrypt B's msg
    let text4 = decrypt_message(
        f.to_string(),
        group_id.to_string(),
        msg4.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to F ,the result is {:?}", text4);
    // G decrypt B's msg
    let text4 = decrypt_message(
        g.to_string(),
        group_id.to_string(),
        msg4.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to G ,the result is {:?}", text4);

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group export secret {:?}",
        get_export_secret(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        get_export_secret(g.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group tree hash {:?}",
        get_tree_hash(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group tree hash {:?}",
        get_tree_hash(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group tree hash {:?}",
        get_tree_hash(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group tree hash {:?}",
        get_tree_hash(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group tree hash {:?}",
        get_tree_hash(g.to_string(), group_id.to_string()).unwrap()
    );

    println!("--A remove B --------------");

    let b_leaf_node = get_lead_node_index(a.to_string(), b.to_string(), group_id.to_string())?;

    // A remove B
    let queued_msg = remove_members(a.to_string(), group_id.to_string(), [b_leaf_node].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // B commit
    let _ = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // C commit
    let _ = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // D commit
    let _ = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // F commit
    let _ = others_commit_normal(
        f.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // G commit
    let _ = others_commit_normal(
        g.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group export secret {:?}",
        get_export_secret(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        get_export_secret(g.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group tree hash {:?}",
        get_tree_hash(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group tree hash {:?}",
        get_tree_hash(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group tree hash {:?}",
        get_tree_hash(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group tree hash {:?}",
        get_tree_hash(g.to_string(), group_id.to_string()).unwrap()
    );

    println!("--A add E --------------");
    // A add E
    let welcome4 = add_members(a.to_string(), group_id.to_string(), [e_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // E join the group
    join_mls_group(e.to_string(), group_id.to_string(), welcome4.welcome)?;
    // C commit
    let _ = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        welcome4.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // D commit
    let _ = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        welcome4.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // F commit
    let _ = others_commit_normal(
        f.to_string(),
        group_id.to_string(),
        welcome4.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // G commit
    let _ = others_commit_normal(
        g.to_string(),
        group_id.to_string(),
        welcome4.queued_msg.clone().as_bytes().to_vec(),
    )?;

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        get_export_secret(e.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group export secret {:?}",
        get_export_secret(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        get_export_secret(g.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group tree hash {:?}",
        get_tree_hash(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group tree hash {:?}",
        get_tree_hash(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "e_mls_group tree hash {:?}",
        get_tree_hash(e.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group tree hash {:?}",
        get_tree_hash(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group tree hash {:?}",
        get_tree_hash(g.to_string(), group_id.to_string()).unwrap()
    );

    println!("--C leave --------------");
    let queued_msg = self_leave(c.to_string(), group_id.to_string())?;
    // A proposal
    let _ = others_proposal_leave(a.to_string(), group_id.to_string(), queued_msg.clone())?;
    // D proposal
    let _ = others_proposal_leave(d.to_string(), group_id.to_string(), queued_msg.clone())?;
    // E proposal
    let _ = others_proposal_leave(e.to_string(), group_id.to_string(), queued_msg.clone())?;
    // F proposal
    let _ = others_proposal_leave(f.to_string(), group_id.to_string(), queued_msg.clone())?;
    // G proposal
    let _ = others_proposal_leave(g.to_string(), group_id.to_string(), queued_msg.clone())?;

    // admin proposal and commit
    let queued_msg = admin_proposal_leave(a.to_string(), group_id.to_string())?;
    let _ = admin_commit_leave(a.to_string(), group_id.to_string())?;

    // C commit
    let _ = normal_member_commit_leave(c.to_string(), group_id.to_string(), queued_msg.clone())?;
    // D commit
    let _ = normal_member_commit_leave(d.to_string(), group_id.to_string(), queued_msg.clone())?;
    // E commit
    let _ = normal_member_commit_leave(e.to_string(), group_id.to_string(), queued_msg.clone())?;
    // F commit
    let _ = normal_member_commit_leave(f.to_string(), group_id.to_string(), queued_msg.clone())?;
    // G commit
    let _ = normal_member_commit_leave(g.to_string(), group_id.to_string(), queued_msg.clone())?;

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        get_export_secret(e.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group export secret {:?}",
        get_export_secret(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        get_export_secret(g.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group tree hash {:?}",
        get_tree_hash(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "e_mls_group tree hash {:?}",
        get_tree_hash(e.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group tree hash {:?}",
        get_tree_hash(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group tree hash {:?}",
        get_tree_hash(g.to_string(), group_id.to_string()).unwrap()
    );

    println!("--A UPDATE --------------");
    // admin update
    let queued_msg = self_update(a.to_string(), group_id.to_string(), "".as_bytes().to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // D commit
    let _ = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // E commit
    let _ = others_commit_normal(
        e.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // F commit
    let _ = others_commit_normal(
        f.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // G commit
    let _ = others_commit_normal(
        g.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        get_export_secret(e.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group export secret {:?}",
        get_export_secret(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        get_export_secret(g.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group tree hash {:?}",
        get_tree_hash(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "e_mls_group tree hash {:?}",
        get_tree_hash(e.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group tree hash {:?}",
        get_tree_hash(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group tree hash {:?}",
        get_tree_hash(g.to_string(), group_id.to_string()).unwrap()
    );

    // A send msg
    let msg5 = create_message(
        a.to_string(),
        group_id.to_string(),
        "hello, DEFG".to_string(),
    )?;
    // D decrypt A's msg
    let text5 = decrypt_message(
        d.to_string(),
        group_id.to_string(),
        msg5.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to D, the result is {:?}", text5);
    // E decrypt A's msg
    let text5 = decrypt_message(
        e.to_string(),
        group_id.to_string(),
        msg5.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to E, the result is {:?}", text5);
    // F decrypt A's msg
    let text5 = decrypt_message(
        f.to_string(),
        group_id.to_string(),
        msg5.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to F, the result is {:?}", text5);
    // G decrypt A's msg
    let text5 = decrypt_message(
        g.to_string(),
        group_id.to_string(),
        msg5.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("A send msg to G, the result is {:?}", text5);

    // G send msg
    let msg6 = create_message(
        g.to_string(),
        group_id.to_string(),
        "hello, ADE".to_string(),
    )?;

    // D decrypt G's msg
    let text6 = decrypt_message(
        d.to_string(),
        group_id.to_string(),
        msg6.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("G send msg to D, the result is {:?}", text6);
    // E decrypt G's msg
    let text6 = decrypt_message(
        e.to_string(),
        group_id.to_string(),
        msg6.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("G send msg to E, the result is {:?}", text6);
    // F decrypt G's msg
    let text6 = decrypt_message(
        f.to_string(),
        group_id.to_string(),
        msg6.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("G send msg to F, the result is {:?}", text6);
    // A decrypt G's msg
    let text6 = decrypt_message(
        a.to_string(),
        group_id.to_string(),
        msg6.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("G send msg to A, the result is {:?}", text6);

    println!("end --------------end");
    Ok(())
}

fn test_basic2() -> Result<()> {
    println!("start -------------- start");

    let group_id = "G1";

    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";

    let db_mls_base = "./mls-base.sqlite";

    init_mls_db(db_mls_base.to_string(), a.to_string())?;
    init_mls_db(db_mls_base.to_string(), b.to_string())?;
    init_mls_db(db_mls_base.to_string(), c.to_string())?;
    init_mls_db(db_mls_base.to_string(), d.to_string())?;
    init_mls_db(db_mls_base.to_string(), e.to_string())?;

    let b_pk = create_key_package(b.to_string())?;
    let c_pk = create_key_package(c.to_string())?;
    let d_pk = create_key_package(d.to_string())?;
    let e_pk = create_key_package(e.to_string())?;

    // a create group
    let group_join_config = create_mls_group(
        a.to_string(),
        group_id.to_string(),
        group_id.to_string(),
        "new group".to_string(),
        ["admin".to_string()].to_vec(),
        ["relay.keychat.io".to_string()].to_vec(),
        "alive".to_string(),
    )?;

    // A add B
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // b join in the group
    join_mls_group(b.to_string(), group_id.to_string(), welcome.welcome)?;

    // A send msg to B
    let msg = create_message(a.to_string(), group_id.to_string(), "hello, B".to_string())?;

    // B decrypt A's msg
    let text = decrypt_message(
        b.to_string(),
        group_id.to_string(),
        msg.encrypt_msg.as_bytes().to_vec(),
    )?;

    println!("A send msg to B ,the result is {:?}", text);

    // B send msg to A
    let msg2 = create_message(b.to_string(), group_id.to_string(), "hello, A".to_string())?;
    // A decrypt B's msg
    let text2 = decrypt_message(
        a.to_string(),
        group_id.to_string(),
        msg2.encrypt_msg.as_bytes().to_vec(),
    )?;
    println!("B send msg to A ,the result is {:?}", text2);

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "a_mls_group tree hash {:?}",
        get_tree_hash(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group tree hash {:?}",
        get_tree_hash(b.to_string(), group_id.to_string()).unwrap()
    );

    println!("--B add C --------------");

    // B add C
    let welcome2 = add_members(b.to_string(), group_id.to_string(), [c_pk].to_vec())?;
    // B send msg
    let msg3 = create_message(
        b.to_string(),
        group_id.to_string(),
        "notice A, you need to update, C will join in.".to_string(),
    )?;

    // A decrypt B's msg
    let text3 = decrypt_message(
        a.to_string(),
        group_id.to_string(),
        msg3.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B send msg to A ,the result is {:?}", text3);

    // c join the group
    join_mls_group(c.to_string(), group_id.to_string(), welcome2.welcome)?;

    // B commit add
    self_commit(b.to_string(), group_id.to_string())?;
    // A commit
    let _ = others_commit_normal(
        a.to_string(),
        group_id.to_string(),
        welcome2.queued_msg.as_bytes().to_vec(),
    )?;

    // B send msg
    let msg4 = create_message(
        b.to_string(),
        group_id.to_string(),
        "hello, A, C.".to_string(),
    )?;

    // C decrypt B's msg
    let text4 = decrypt_message(
        c.to_string(),
        group_id.to_string(),
        msg4.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B send msg to C ,the result is {:?}", text4);

    // A decrypt B's msg
    let text4 = decrypt_message(
        a.to_string(),
        group_id.to_string(),
        msg4.encrypt_msg.clone().as_bytes().to_vec(),
    )?;
    println!("B send msg to A ,the result is {:?}", text4);

    println!("--------------");
    Ok(())
}
// if add some members, for example due to F reply delay and lack of one commit, this will lead to F tree is diff from others.
// So if dely, but every operation F should receive it, and process it in order by time, if not it will be error.
fn test_replay_delay() -> Result<()> {
    println!("start --------------start");
    let group_id = "G2";
    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";
    let f = "F";
    let g = "G";

    let db_mls_base = "./mls-base.sqlite";

    init_mls_db(db_mls_base.to_string(), a.to_string())?;
    init_mls_db(db_mls_base.to_string(), b.to_string())?;
    init_mls_db(db_mls_base.to_string(), c.to_string())?;
    init_mls_db(db_mls_base.to_string(), d.to_string())?;
    init_mls_db(db_mls_base.to_string(), e.to_string())?;
    init_mls_db(db_mls_base.to_string(), f.to_string())?;
    init_mls_db(db_mls_base.to_string(), g.to_string())?;

    let b_pk = create_key_package(b.to_string())?;
    let c_pk = create_key_package(c.to_string())?;
    let d_pk = create_key_package(d.to_string())?;
    let e_pk = create_key_package(e.to_string())?;
    let f_pk = create_key_package(f.to_string())?;
    let g_pk = create_key_package(g.to_string())?;
    // a create group
    let group_join_config = create_mls_group(
        a.to_string(),
        group_id.to_string(),
        "new group".to_string(),
        "new group".to_string(),
        ["admin".to_string()].to_vec(),
        ["relay.keychat.io".to_string()].to_vec(),
        "alive".to_string(),
    )?;

    // A add B F, but F not reply right now
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk, f_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;
    // b join in the group
    join_mls_group(b.to_string(), group_id.to_string(), welcome.welcome.clone())?;

    // // f join in the group
    // join_mls_group(
    //     f.to_string(),
    //     group_id.to_string(),
    //     welcome.1.clone(),
    //
    // )?;

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );

    println!("--B add C G --------------");

    // B add C and G
    let welcome2 = add_members(b.to_string(), group_id.to_string(), [c_pk, g_pk].to_vec())?;
    // A commit
    self_commit(b.to_string(), group_id.to_string())?;

    // c join the group
    join_mls_group(
        c.to_string(),
        group_id.to_string(),
        welcome2.welcome.clone(),
    )?;

    // g join the group
    join_mls_group(
        g.to_string(),
        group_id.to_string(),
        welcome2.welcome.clone(),
    )?;

    // f join in the group
    join_mls_group(f.to_string(), group_id.to_string(), welcome.welcome.clone())?;

    // A commit
    let _ = others_commit_normal(
        a.to_string(),
        group_id.to_string(),
        welcome2.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // // F commit
    // let _ = others_commit_normal(f.to_string(), group_id.to_string(), welcome2.0.clone())?;

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        get_export_secret(g.to_string(), group_id.to_string()).unwrap()
    );

    println!("--A add D --------------");

    // A add D
    let welcome3 = add_members(a.to_string(), group_id.to_string(), [d_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // d join the group
    join_mls_group(d.to_string(), group_id.to_string(), welcome3.welcome)?;

    // B commit
    let _ = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // C commit
    let _ = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // F commit, add some members, due to F reply dely, lead to F lack ont commit
    let _ = others_commit_normal(
        f.to_string(),
        group_id.to_string(),
        welcome2.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // F commit
    let _ = others_commit_normal(
        f.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // G commit
    let _ = others_commit_normal(
        g.to_string(),
        group_id.to_string(),
        welcome3.queued_msg.clone().as_bytes().to_vec(),
    )?;

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        get_export_secret(b.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "f_mls_group export secret {:?}",
        get_export_secret(f.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        get_export_secret(g.to_string(), group_id.to_string()).unwrap()
    );

    println!("--A remove B F --------------");

    let b_leaf_node = get_lead_node_index(a.to_string(), b.to_string(), group_id.to_string())?;
    let f_leaf_node = get_lead_node_index(a.to_string(), f.to_string(), group_id.to_string())?;

    // A remove B F
    let queued_msg = remove_members(
        a.to_string(),
        group_id.to_string(),
        [b_leaf_node, f_leaf_node].to_vec(),
    )?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // B commit
    let _ = others_commit_normal(
        b.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // C commit
    let _ = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // D commit
    let _ = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // F commit
    let _ = others_commit_normal(
        f.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;
    // G commit
    let _ = others_commit_normal(
        g.to_string(),
        group_id.to_string(),
        queued_msg.clone().as_bytes().to_vec(),
    )?;

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        get_export_secret(g.to_string(), group_id.to_string()).unwrap()
    );

    println!("--A add E --------------");
    // A add E
    let welcome4 = add_members(a.to_string(), group_id.to_string(), [e_pk].to_vec())?;
    // A commit
    self_commit(a.to_string(), group_id.to_string())?;

    // E join the group
    join_mls_group(e.to_string(), group_id.to_string(), welcome4.welcome)?;
    // C commit
    let _ = others_commit_normal(
        c.to_string(),
        group_id.to_string(),
        welcome4.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // D commit
    let _ = others_commit_normal(
        d.to_string(),
        group_id.to_string(),
        welcome4.queued_msg.clone().as_bytes().to_vec(),
    )?;
    // G commit
    let _ = others_commit_normal(
        g.to_string(),
        group_id.to_string(),
        welcome4.queued_msg.clone().as_bytes().to_vec(),
    )?;

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        get_export_secret(c.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        get_export_secret(e.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        get_export_secret(g.to_string(), group_id.to_string()).unwrap()
    );

    println!("--C leave --------------");
    let queued_msg = self_leave(c.to_string(), group_id.to_string())?;
    // A proposal
    let _ = others_proposal_leave(a.to_string(), group_id.to_string(), queued_msg.clone())?;
    // D proposal
    let _ = others_proposal_leave(d.to_string(), group_id.to_string(), queued_msg.clone())?;
    // E proposal
    let _ = others_proposal_leave(e.to_string(), group_id.to_string(), queued_msg.clone())?;
    // G proposal
    let _ = others_proposal_leave(g.to_string(), group_id.to_string(), queued_msg.clone())?;

    // admin proposal and commit
    let queued_msg = admin_proposal_leave(a.to_string(), group_id.to_string())?;
    let _ = admin_commit_leave(a.to_string(), group_id.to_string())?;

    // C commit
    let _ = normal_member_commit_leave(c.to_string(), group_id.to_string(), queued_msg.clone())?;
    // D commit
    let _ = normal_member_commit_leave(d.to_string(), group_id.to_string(), queued_msg.clone())?;
    // E commit
    let _ = normal_member_commit_leave(e.to_string(), group_id.to_string(), queued_msg.clone())?;
    // G commit
    let _ = normal_member_commit_leave(g.to_string(), group_id.to_string(), queued_msg.clone())?;

    println!(
        "a_mls_group export secret {:?}",
        get_export_secret(a.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        get_export_secret(d.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        get_export_secret(e.to_string(), group_id.to_string()).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        get_export_secret(g.to_string(), group_id.to_string()).unwrap()
    );

    println!("end --------------end");
    Ok(())
}
