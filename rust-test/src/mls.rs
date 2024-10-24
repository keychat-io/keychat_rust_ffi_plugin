use anyhow::Result;
use rust::api_mls::*;

fn main() {
    // let _ = test_basic();
    // let _ = test_exist_group();
    let _ = test_normal();
    // let _ = test_replay_delay();
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

    let db_path = "./mls-lite.sqlite";
    // every user show init this
    init_mls_db(db_path.to_string(), a.to_string())?;
    init_mls_db(db_path.to_string(), b.to_string())?;
    init_mls_db(db_path.to_string(), c.to_string())?;
    init_mls_db(db_path.to_string(), d.to_string())?;
    init_mls_db(db_path.to_string(), e.to_string())?;
    init_mls_db(db_path.to_string(), f.to_string())?;
    init_mls_db(db_path.to_string(), g.to_string())?;

    let group_join_config = get_group_config(a.to_string(), group_id.to_string())?;

    let f_pk = create_key_package(f.to_string())?;
    let g_pk = create_key_package(g.to_string())?;

    // A add G
    let welcome = add_members(a.to_string(), group_id.to_string(), [f_pk, g_pk].to_vec())?;

    // F join in the group
    join_mls_group(
        f.to_string(),
        group_id.to_string(),
        welcome.1.clone(),
        group_join_config.clone(),
    )?;

    // G join in the group
    join_mls_group(
        g.to_string(),
        group_id.to_string(),
        welcome.1.clone(),
        group_join_config.clone(),
    )?;

    // D commit
    let _ = others_commit_normal(d.to_string(), group_id.to_string(), welcome.0.clone())?;

    // E commit
    let _ = others_commit_normal(e.to_string(), group_id.to_string(), welcome.0.clone())?;

    // A send msg to G
    let msg = send_msg(a.to_string(), group_id.to_string(), "hello, G".to_string())?;
    // F decrypt A's msg
    let text = decrypt_msg(f.to_string(), group_id.to_string(), msg.0.clone())?;
    println!("A send msg to F ,the result is {:?}", text);

    // E decrypt A's msg
    let text = decrypt_msg(e.to_string(), group_id.to_string(), msg.0.clone())?;
    println!("A send msg to E ,the result is {:?}", text);

    // G decrypt A's msg
    let text = decrypt_msg(g.to_string(), group_id.to_string(), msg.0.clone())?;
    println!("A send msg to G ,the result is {:?}", text);

    // D decrypt A's msg
    let text = decrypt_msg(d.to_string(), group_id.to_string(), msg.0.clone())?;
    println!("A send msg to D ,the result is {:?}", text);

    Ok(())
}
// create add send_msg decrypt_msg remove leave
fn test_basic() -> Result<()> {
    println!("start -------------- start");

    let group_id = "G1";

    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";

    let db_path = "./mls-lite.sqlite";

    init_mls_db(db_path.to_string(), a.to_string())?;
    init_mls_db(db_path.to_string(), b.to_string())?;
    init_mls_db(db_path.to_string(), c.to_string())?;
    init_mls_db(db_path.to_string(), d.to_string())?;
    init_mls_db(db_path.to_string(), e.to_string())?;

    let b_pk = create_key_package(b.to_string())?;
    let c_pk = create_key_package(c.to_string())?;
    let d_pk = create_key_package(d.to_string())?;
    let e_pk = create_key_package(e.to_string())?;

    // a create group
    let group_join_config = create_mls_group(a.to_string(), group_id.to_string())?;

    // A add B
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk].to_vec())?;

    // b join in the group
    join_mls_group(
        b.to_string(),
        group_id.to_string(),
        welcome.1,
        group_join_config.clone(),
    )?;

    // A send msg to B
    let msg = send_msg(a.to_string(), group_id.to_string(), "hello, B".to_string())?;
    // B decrypt A's msg
    let text = decrypt_msg(b.to_string(), group_id.to_string(), msg.0)?;

    println!("A send msg to B ,the result is {:?}", text);

    // B send msg to A
    let msg2 = send_msg(b.to_string(), group_id.to_string(), "hello, A".to_string())?;
    // A decrypt B's msg
    let text2 = decrypt_msg(a.to_string(), group_id.to_string(), msg2.0)?;
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

    // c join the group
    join_mls_group(
        c.to_string(),
        group_id.to_string(),
        welcome2.1,
        group_join_config.clone(),
    )?;
    // A commit
    let _ = others_commit_normal(a.to_string(), group_id.to_string(), welcome2.0)?;

    // B send msg
    let msg3 = send_msg(
        b.to_string(),
        group_id.to_string(),
        "hello, A, C".to_string(),
    )?;
    // C decrypt B's msg
    let text3 = decrypt_msg(c.to_string(), group_id.to_string(), msg3.0.clone())?;
    println!("B send msg to C ,the result is {:?}", text3);

    // A decrypt B's msg
    let text4 = decrypt_msg(a.to_string(), group_id.to_string(), msg3.0.clone())?;
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

    // d join the group
    join_mls_group(
        d.to_string(),
        group_id.to_string(),
        welcome3.1,
        group_join_config.clone(),
    )?;

    // B commit
    let _ = others_commit_normal(b.to_string(), group_id.to_string(), welcome3.0.clone())?;

    // C commit
    let _ = others_commit_normal(c.to_string(), group_id.to_string(), welcome3.0.clone())?;

    // A send msg
    let msg4 = send_msg(
        a.to_string(),
        group_id.to_string(),
        "hello, ABC".to_string(),
    )?;

    // B decrypt A's msg
    let text5 = decrypt_msg(b.to_string(), group_id.to_string(), msg4.0.clone())?;
    println!("A send msg to B ,the result is {:?}", text5);
    // C decrypt A's msg
    let text6 = decrypt_msg(c.to_string(), group_id.to_string(), msg4.0.clone())?;
    println!("A send msg to C ,the result is {:?}", text6);
    // D decrypt B's msg
    let text7 = decrypt_msg(d.to_string(), group_id.to_string(), msg4.0.clone())?;
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

    let b_leaf_node = get_lead_node_index(b.to_string(), group_id.to_string())?;

    // A remove B
    let queued_msg = remove_members(a.to_string(), group_id.to_string(), [b_leaf_node].to_vec())?;

    // B commit
    let _ = others_commit_normal(b.to_string(), group_id.to_string(), queued_msg.clone())?;

    // C commit
    let _ = others_commit_normal(c.to_string(), group_id.to_string(), queued_msg.clone())?;

    // D commit
    let _ = others_commit_normal(d.to_string(), group_id.to_string(), queued_msg.clone())?;

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

    // E join the group
    join_mls_group(
        e.to_string(),
        group_id.to_string(),
        welcome4.1,
        group_join_config.clone(),
    )?;

    // C commit
    let _ = others_commit_normal(c.to_string(), group_id.to_string(), welcome4.0.clone())?;
    // D commit
    let _ = others_commit_normal(d.to_string(), group_id.to_string(), welcome4.0.clone())?;

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
    // A proposal
    let _ = others_proposal_leave(a.to_string(), group_id.to_string(), queued_msg.clone())?;
    // D proposal
    let _ = others_proposal_leave(d.to_string(), group_id.to_string(), queued_msg.clone())?;
    // E proposal
    let _ = others_proposal_leave(e.to_string(), group_id.to_string(), queued_msg.clone())?;

    // admin commit
    let queued_msg = admin_commit_leave(a.to_string(), group_id.to_string())?;

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
    let queued_msg = self_update(a.to_string(), group_id.to_string())?;

    // E commit
    let _ = others_commit_normal(e.to_string(), group_id.to_string(), queued_msg.clone())?;
    // D commit
    let _ = others_commit_normal(d.to_string(), group_id.to_string(), queued_msg.clone())?;

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

    let db_path = "./mls-lite.sqlite";
    init_mls_db(db_path.to_string(), a.to_string())?;
    init_mls_db(db_path.to_string(), b.to_string())?;
    init_mls_db(db_path.to_string(), c.to_string())?;
    init_mls_db(db_path.to_string(), d.to_string())?;
    init_mls_db(db_path.to_string(), e.to_string())?;
    init_mls_db(db_path.to_string(), f.to_string())?;
    init_mls_db(db_path.to_string(), g.to_string())?;

    let b_pk = create_key_package(b.to_string())?;
    let c_pk = create_key_package(c.to_string())?;
    let d_pk = create_key_package(d.to_string())?;
    let e_pk = create_key_package(e.to_string())?;
    let f_pk = create_key_package(f.to_string())?;
    let g_pk = create_key_package(g.to_string())?;

    // a create group
    let group_join_config = create_mls_group(a.to_string(), group_id.to_string())?;

    // A add B F
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk, f_pk].to_vec())?;

    // b join in the group
    join_mls_group(
        b.to_string(),
        group_id.to_string(),
        welcome.1.clone(),
        group_join_config.clone(),
    )?;

    // f join in the group
    join_mls_group(
        f.to_string(),
        group_id.to_string(),
        welcome.1.clone(),
        group_join_config.clone(),
    )?;

    // A send msg to B F
    let msg = send_msg(
        a.to_string(),
        group_id.to_string(),
        "hello, B F".to_string(),
    )?;
    // B decrypt A's msg
    let text = decrypt_msg(b.to_string(), group_id.to_string(), msg.0.clone())?;
    println!("A send msg to B ,the result is {:?}", text);
    // F decrypt A's msg
    let text = decrypt_msg(f.to_string(), group_id.to_string(), msg.0.clone())?;
    println!("A send msg to F ,the result is {:?}", text);

    // B send msg to A
    let msg2 = send_msg(b.to_string(), group_id.to_string(), "hello, A".to_string())?;

    // A decrypt B's msg
    let text2 = decrypt_msg(a.to_string(), group_id.to_string(), msg2.0)?;
    println!("B send msg to A ,the result is {:?}", text2);

    // F send msg to A
    let msg2_1 = send_msg(f.to_string(), group_id.to_string(), "hello, F".to_string())?;

    // A decrypt F's msg
    let text2_1 = decrypt_msg(a.to_string(), group_id.to_string(), msg2_1.0)?;
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

    // c join the group
    join_mls_group(
        c.to_string(),
        group_id.to_string(),
        welcome2.1.clone(),
        group_join_config.clone(),
    )?;

    // g join the group
    join_mls_group(
        g.to_string(),
        group_id.to_string(),
        welcome2.1.clone(),
        group_join_config.clone(),
    )?;

    // A commit
    let _ = others_commit_normal(a.to_string(), group_id.to_string(), welcome2.0.clone())?;
    // F commit
    let _ = others_commit_normal(f.to_string(), group_id.to_string(), welcome2.0.clone())?;

    // B send msg
    let msg3 = send_msg(
        b.to_string(),
        group_id.to_string(),
        "hello, A, C, F, G".to_string(),
    )?;
    // A decrypt B's msg
    let text3 = decrypt_msg(a.to_string(), group_id.to_string(), msg3.0.clone())?;
    println!("B send msg to A ,the result is {:?}", text3);
    // C decrypt B's msg
    let text3 = decrypt_msg(c.to_string(), group_id.to_string(), msg3.0.clone())?;
    println!("B send msg to C ,the result is {:?}", text3);
    // F decrypt B's msg
    let text3 = decrypt_msg(f.to_string(), group_id.to_string(), msg3.0.clone())?;
    println!("B send msg to F ,the result is {:?}", text3);
    // G decrypt B's msg
    let text3 = decrypt_msg(g.to_string(), group_id.to_string(), msg3.0.clone())?;
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

    // d join the group
    join_mls_group(
        d.to_string(),
        group_id.to_string(),
        welcome3.1,
        group_join_config.clone(),
    )?;

    // B commit
    let _ = others_commit_normal(b.to_string(), group_id.to_string(), welcome3.0.clone())?;
    // C commit
    let _ = others_commit_normal(c.to_string(), group_id.to_string(), welcome3.0.clone())?;
    // F commit
    let _ = others_commit_normal(f.to_string(), group_id.to_string(), welcome3.0.clone())?;
    // G commit
    let _ = others_commit_normal(g.to_string(), group_id.to_string(), welcome3.0.clone())?;

    // A send msg
    let msg4 = send_msg(
        a.to_string(),
        group_id.to_string(),
        "hello, ABCDFG".to_string(),
    )?;

    // B decrypt A's msg
    let text4 = decrypt_msg(b.to_string(), group_id.to_string(), msg4.0.clone())?;
    println!("A send msg to B ,the result is {:?}", text4);
    // C decrypt A's msg
    let text4 = decrypt_msg(c.to_string(), group_id.to_string(), msg4.0.clone())?;
    println!("A send msg to C ,the result is {:?}", text4);
    // D decrypt B's msg
    let text4 = decrypt_msg(d.to_string(), group_id.to_string(), msg4.0.clone())?;
    println!("A send msg to D ,the result is {:?}", text4);
    // F decrypt B's msg
    let text4 = decrypt_msg(f.to_string(), group_id.to_string(), msg4.0.clone())?;
    println!("A send msg to F ,the result is {:?}", text4);
    // G decrypt B's msg
    let text4 = decrypt_msg(g.to_string(), group_id.to_string(), msg4.0.clone())?;
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

    let b_leaf_node = get_lead_node_index(b.to_string(), group_id.to_string())?;

    // A remove B
    let queued_msg = remove_members(a.to_string(), group_id.to_string(), [b_leaf_node].to_vec())?;

    // B commit
    let _ = others_commit_normal(b.to_string(), group_id.to_string(), queued_msg.clone())?;
    // C commit
    let _ = others_commit_normal(c.to_string(), group_id.to_string(), queued_msg.clone())?;
    // D commit
    let _ = others_commit_normal(d.to_string(), group_id.to_string(), queued_msg.clone())?;
    // F commit
    let _ = others_commit_normal(f.to_string(), group_id.to_string(), queued_msg.clone())?;
    // G commit
    let _ = others_commit_normal(g.to_string(), group_id.to_string(), queued_msg.clone())?;

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

    // E join the group
    join_mls_group(
        e.to_string(),
        group_id.to_string(),
        welcome4.1,
        group_join_config.clone(),
    )?;
    // C commit
    let _ = others_commit_normal(c.to_string(), group_id.to_string(), welcome4.0.clone())?;
    // D commit
    let _ = others_commit_normal(d.to_string(), group_id.to_string(), welcome4.0.clone())?;
    // F commit
    let _ = others_commit_normal(f.to_string(), group_id.to_string(), welcome4.0.clone())?;
    // G commit
    let _ = others_commit_normal(g.to_string(), group_id.to_string(), welcome4.0.clone())?;

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

    // admin commit
    let queued_msg = admin_commit_leave(a.to_string(), group_id.to_string())?;

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
    let queued_msg = self_update(a.to_string(), group_id.to_string())?;

    // D commit
    let _ = others_commit_normal(d.to_string(), group_id.to_string(), queued_msg.clone())?;
    // E commit
    let _ = others_commit_normal(e.to_string(), group_id.to_string(), queued_msg.clone())?;
    // F commit
    let _ = others_commit_normal(f.to_string(), group_id.to_string(), queued_msg.clone())?;
    // G commit
    let _ = others_commit_normal(g.to_string(), group_id.to_string(), queued_msg.clone())?;

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
    let msg5 = send_msg(
        a.to_string(),
        group_id.to_string(),
        "hello, DEFG".to_string(),
    )?;
    // D decrypt A's msg
    let text5 = decrypt_msg(d.to_string(), group_id.to_string(), msg5.0.clone())?;
    println!("A send msg to D, the result is {:?}", text5);
    // E decrypt A's msg
    let text5 = decrypt_msg(e.to_string(), group_id.to_string(), msg5.0.clone())?;
    println!("A send msg to E, the result is {:?}", text5);
    // F decrypt A's msg
    let text5 = decrypt_msg(f.to_string(), group_id.to_string(), msg5.0.clone())?;
    println!("A send msg to F, the result is {:?}", text5);
    // G decrypt A's msg
    let text5 = decrypt_msg(g.to_string(), group_id.to_string(), msg5.0.clone())?;
    println!("A send msg to G, the result is {:?}", text5);

    // G send msg
    let msg6 = send_msg(
        g.to_string(),
        group_id.to_string(),
        "hello, ADE".to_string(),
    )?;
    // D decrypt G's msg
    let text6 = decrypt_msg(d.to_string(), group_id.to_string(), msg6.0.clone())?;
    println!("G send msg to D, the result is {:?}", text6);
    // E decrypt G's msg
    let text6 = decrypt_msg(e.to_string(), group_id.to_string(), msg6.0.clone())?;
    println!("G send msg to E, the result is {:?}", text6);
    // F decrypt G's msg
    let text6 = decrypt_msg(f.to_string(), group_id.to_string(), msg6.0.clone())?;
    println!("G send msg to F, the result is {:?}", text6);
    // A decrypt G's msg
    let text6 = decrypt_msg(a.to_string(), group_id.to_string(), msg6.0.clone())?;
    println!("G send msg to A, the result is {:?}", text6);

    println!("end --------------end");
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

    let db_path = "./mls-lite.sqlite";
    init_mls_db(db_path.to_string(), a.to_string())?;
    init_mls_db(db_path.to_string(), b.to_string())?;
    init_mls_db(db_path.to_string(), c.to_string())?;
    init_mls_db(db_path.to_string(), d.to_string())?;
    init_mls_db(db_path.to_string(), e.to_string())?;
    init_mls_db(db_path.to_string(), f.to_string())?;
    init_mls_db(db_path.to_string(), g.to_string())?;

    let b_pk = create_key_package(b.to_string())?;
    let c_pk = create_key_package(c.to_string())?;
    let d_pk = create_key_package(d.to_string())?;
    let e_pk = create_key_package(e.to_string())?;
    let f_pk = create_key_package(f.to_string())?;
    let g_pk = create_key_package(g.to_string())?;
    // a create group
    let group_join_config = create_mls_group(a.to_string(), group_id.to_string())?;

    // A add B F, but F not reply right now
    let welcome = add_members(a.to_string(), group_id.to_string(), [b_pk, f_pk].to_vec())?;

    // b join in the group
    join_mls_group(
        b.to_string(),
        group_id.to_string(),
        welcome.1.clone(),
        group_join_config.clone(),
    )?;

    // // f join in the group
    // join_mls_group(
    //     f.to_string(),
    //     group_id.to_string(),
    //     welcome.1.clone(),
    //     group_join_config.clone(),
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

    // c join the group
    join_mls_group(
        c.to_string(),
        group_id.to_string(),
        welcome2.1.clone(),
        group_join_config.clone(),
    )?;

    // g join the group
    join_mls_group(
        g.to_string(),
        group_id.to_string(),
        welcome2.1.clone(),
        group_join_config.clone(),
    )?;

    // f join in the group
    join_mls_group(
        f.to_string(),
        group_id.to_string(),
        welcome.1.clone(),
        group_join_config.clone(),
    )?;

    // A commit
    let _ = others_commit_normal(a.to_string(), group_id.to_string(), welcome2.0.clone())?;
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

    // d join the group
    join_mls_group(
        d.to_string(),
        group_id.to_string(),
        welcome3.1,
        group_join_config.clone(),
    )?;

    // B commit
    let _ = others_commit_normal(b.to_string(), group_id.to_string(), welcome3.0.clone())?;
    // C commit
    let _ = others_commit_normal(c.to_string(), group_id.to_string(), welcome3.0.clone())?;
    // F commit, add some members, due to F reply dely, lead to F lack ont commit
    let _ = others_commit_normal(f.to_string(), group_id.to_string(), welcome2.0.clone())?;
    // F commit
    let _ = others_commit_normal(f.to_string(), group_id.to_string(), welcome3.0.clone())?;
    // G commit
    let _ = others_commit_normal(g.to_string(), group_id.to_string(), welcome3.0.clone())?;

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

    let b_leaf_node = get_lead_node_index(b.to_string(), group_id.to_string())?;
    let f_leaf_node = get_lead_node_index(f.to_string(), group_id.to_string())?;

    // A remove B F
    let queued_msg = remove_members(
        a.to_string(),
        group_id.to_string(),
        [b_leaf_node, f_leaf_node].to_vec(),
    )?;

    // B commit
    let _ = others_commit_normal(b.to_string(), group_id.to_string(), queued_msg.clone())?;
    // C commit
    let _ = others_commit_normal(c.to_string(), group_id.to_string(), queued_msg.clone())?;
    // D commit
    let _ = others_commit_normal(d.to_string(), group_id.to_string(), queued_msg.clone())?;
    // F commit
    let _ = others_commit_normal(f.to_string(), group_id.to_string(), queued_msg.clone())?;
    // G commit
    let _ = others_commit_normal(g.to_string(), group_id.to_string(), queued_msg.clone())?;

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

    // E join the group
    join_mls_group(
        e.to_string(),
        group_id.to_string(),
        welcome4.1,
        group_join_config.clone(),
    )?;
    // C commit
    let _ = others_commit_normal(c.to_string(), group_id.to_string(), welcome4.0.clone())?;
    // D commit
    let _ = others_commit_normal(d.to_string(), group_id.to_string(), welcome4.0.clone())?;
    // G commit
    let _ = others_commit_normal(g.to_string(), group_id.to_string(), welcome4.0.clone())?;

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

    // admin commit
    let queued_msg = admin_commit_leave(a.to_string(), group_id.to_string())?;

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
