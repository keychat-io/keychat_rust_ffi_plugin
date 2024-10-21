use anyhow::Result;
use rust::api_mls::*;

fn main() {
    // let _ = test_basic();
    // let _ = test_complex();
    let _ = test_complex2();
}

// create add send_msg decrypt_msg remove leave
fn test_basic() -> Result<()> {
    println!("start --------------start");
    let group_id = "G1".to_string();
    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";

    let db_path = "./mls-lite.sqlite".to_string();
    init_mls_db(db_path)?;

    let a_provider = create_provider(a.to_string())?;
    let b_provider = create_provider(b.to_string())?;
    let c_provider = create_provider(c.to_string())?;
    let d_provider = create_provider(d.to_string())?;
    let e_provider = create_provider(e.to_string())?;

    let group_create_config = group_create_config()?;

    let a_identity = create_identity(a.to_string(), &a_provider)?;
    let mut b_identity = create_identity(b.to_string(), &b_provider)?;
    let mut c_identity = create_identity(c.to_string(), &c_provider)?;
    let mut d_identity = create_identity(d.to_string(), &d_provider)?;
    let mut e_identity = create_identity(e.to_string(), &e_provider)?;

    let b_pk = create_key_package(&b_provider, &mut b_identity)?;
    let c_pk = create_key_package(&c_provider, &mut c_identity)?;
    let d_pk = create_key_package(&d_provider, &mut d_identity)?;
    let e_pk = create_key_package(&e_provider, &mut e_identity)?;

    let mut a_mls_group =
        create_mls_group(group_id, &group_create_config, &a_provider, &a_identity)?;

    // A add B
    let welcome = add_member(&mut a_mls_group, &a_provider, &a_identity, b_pk)?;

    let mut b_mls_group = bob_join_mls_group(welcome.1, &b_provider, &group_create_config)?;

    // A send msg to B
    let msg = send_msg(
        &mut a_mls_group,
        &a_provider,
        &a_identity,
        "hello, B".to_string(),
    )?;
    // B decrypt A's msg
    let text = decrypt_msg(&mut b_mls_group, &b_provider, msg)?;
    println!("A send msg to B ,the result is {:?}", text);

    // B send msg to A
    let msg2 = send_msg(
        &mut b_mls_group,
        &b_provider,
        &b_identity,
        "hello, A".to_string(),
    )?;
    // A decrypt B's msg
    let text2 = decrypt_msg(&mut a_mls_group, &a_provider, msg2)?;
    println!("B send msg to A ,the result is {:?}", text2);

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        b_mls_group.export_secret(&b_provider, "", &[], 32).unwrap()
    );

    println!("--B add C --------------");

    // B add C
    let welcome2 = add_member(&mut b_mls_group, &b_provider, &b_identity, c_pk)?;

    let mut c_mls_group = bob_join_mls_group(welcome2.1, &c_provider, &group_create_config)?;
    // A commit
    let _ = others_commit_add_member(&mut a_mls_group, welcome2.0, &a_provider)?;

    // B send msg
    let msg3 = send_msg(
        &mut b_mls_group,
        &b_provider,
        &b_identity,
        "hello, A, C".to_string(),
    )?;
    // C decrypt B's msg
    let text3 = decrypt_msg(&mut c_mls_group, &c_provider, msg3.clone())?;
    println!("B send msg to C ,the result is {:?}", text3);

    // A decrypt B's msg
    let text4 = decrypt_msg(&mut a_mls_group, &a_provider, msg3.clone())?;
    println!("B send msg to A ,the result is {:?}", text4);

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        b_mls_group.export_secret(&b_provider, "", &[], 32).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );

    println!("--A add D --------------");

    // A add D
    let welcome3 = add_member(&mut a_mls_group, &a_provider, &a_identity, d_pk)?;

    let mut d_mls_group = bob_join_mls_group(welcome3.1, &d_provider, &group_create_config)?;

    // B commit
    let _ = others_commit_add_member(&mut b_mls_group, welcome3.0.clone(), &b_provider)?;

    // C commit
    let _ = others_commit_add_member(&mut c_mls_group, welcome3.0.clone(), &c_provider)?;

    // A send msg
    let msg4 = send_msg(
        &mut a_mls_group,
        &a_provider,
        &a_identity,
        "hello, ABC".to_string(),
    )?;
    // B decrypt A's msg
    let text5 = decrypt_msg(&mut b_mls_group, &b_provider, msg4.clone())?;
    println!("A send msg to B ,the result is {:?}", text5);
    // C decrypt A's msg
    let text6 = decrypt_msg(&mut c_mls_group, &c_provider, msg4.clone())?;
    println!("A send msg to C ,the result is {:?}", text6);
    // D decrypt B's msg
    let text7 = decrypt_msg(&mut d_mls_group, &d_provider, msg4.clone())?;
    println!("A send msg to D ,the result is {:?}", text7);

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        b_mls_group.export_secret(&b_provider, "", &[], 32).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );

    // A remove B
    let queued_msg = remove_member(&mut a_mls_group, &mut b_mls_group, &a_identity, &a_provider)?;

    // B commit
    let _ = others_commit_remove_member(&mut b_mls_group, queued_msg.clone(), &b_provider)?;

    // C commit
    let _ = others_commit_remove_member(&mut c_mls_group, queued_msg.clone(), &c_provider)?;

    // D commit
    let _ = others_commit_remove_member(&mut d_mls_group, queued_msg.clone(), &d_provider)?;

    println!("--A remove B --------------");

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );

    // A add E
    let welcome4 = add_member(&mut a_mls_group, &a_provider, &a_identity, e_pk)?;

    let mut e_mls_group = bob_join_mls_group(welcome4.1, &e_provider, &group_create_config)?;

    // C commit
    let _ = others_commit_add_member(&mut c_mls_group, welcome4.0.clone(), &c_provider)?;
    // D commit
    let _ = others_commit_add_member(&mut d_mls_group, welcome4.0.clone(), &d_provider)?;

    println!("--A add E --------------");

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        e_mls_group.export_secret(&e_provider, "", &[], 32).unwrap()
    );

    println!("--C leave --------------");

    let queued_msg = self_leave(&mut c_mls_group, &c_identity, &c_provider)?;
    // A proposal
    let _ = others_proposal_leave(&mut a_mls_group, queued_msg.clone(), &a_provider)?;
    // D proposal
    let _ = others_proposal_leave(&mut d_mls_group, queued_msg.clone(), &d_provider)?;
    // E proposal
    let _ = others_proposal_leave(&mut e_mls_group, queued_msg.clone(), &e_provider)?;

    // admin commit
    let queued_msg = admin_commit_leave(&mut a_mls_group, &a_identity, &a_provider)?;

    // D commit
    let _ = normal_member_commit_leave(&mut d_mls_group, queued_msg.clone(), &d_provider)?;
    // E commit
    let _ = normal_member_commit_leave(&mut e_mls_group, queued_msg.clone(), &e_provider)?;
    // C commit
    let _ = normal_member_commit_leave(&mut c_mls_group, queued_msg.clone(), &c_provider)?;

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        e_mls_group.export_secret(&e_provider, "", &[], 32).unwrap()
    );

    println!("end --------------end");
    Ok(())
}

fn test_complex() -> Result<()> {
    println!("start --------------start");
    let group_id = "G1".to_string();
    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";
    let f = "F";
    let g = "G";

    let db_path = "./mls-lite.sqlite".to_string();
    init_mls_db(db_path)?;

    let a_provider = create_provider(a.to_string())?;
    let b_provider = create_provider(b.to_string())?;
    let c_provider = create_provider(c.to_string())?;
    let d_provider = create_provider(d.to_string())?;
    let e_provider = create_provider(e.to_string())?;
    let f_provider = create_provider(f.to_string())?;
    let g_provider = create_provider(g.to_string())?;

    let group_create_config = group_create_config()?;

    let a_identity = create_identity(a.to_string(), &a_provider)?;
    let mut b_identity = create_identity(b.to_string(), &b_provider)?;
    let mut c_identity = create_identity(c.to_string(), &c_provider)?;
    let mut d_identity = create_identity(d.to_string(), &d_provider)?;
    let mut e_identity = create_identity(e.to_string(), &e_provider)?;
    let mut f_identity = create_identity(f.to_string(), &f_provider)?;
    let mut g_identity = create_identity(g.to_string(), &g_provider)?;

    let b_pk = create_key_package(&b_provider, &mut b_identity)?;
    let c_pk = create_key_package(&c_provider, &mut c_identity)?;
    let d_pk = create_key_package(&d_provider, &mut d_identity)?;
    let e_pk = create_key_package(&e_provider, &mut e_identity)?;
    let f_pk = create_key_package(&f_provider, &mut f_identity)?;
    let g_pk = create_key_package(&g_provider, &mut g_identity)?;

    let mut a_mls_group =
        create_mls_group(group_id, &group_create_config, &a_provider, &a_identity)?;

    // A add B
    let welcome = add_members(&mut a_mls_group, &a_provider, &a_identity, &[b_pk, f_pk])?;
    let mut b_mls_group = bob_join_mls_group(welcome.1.clone(), &b_provider, &group_create_config)?;

    let mut f_mls_group = bob_join_mls_group(welcome.1.clone(), &f_provider, &group_create_config)?;

    // A send msg to B
    let msg = send_msg(
        &mut a_mls_group,
        &a_provider,
        &a_identity,
        "hello, B".to_string(),
    )?;
    // B decrypt A's msg
    let text = decrypt_msg(&mut b_mls_group, &b_provider, msg)?;
    println!("A send msg to B ,the result is {:?}", text);

    // A send msg to F
    let msg_f = send_msg(
        &mut a_mls_group,
        &a_provider,
        &a_identity,
        "hello, F".to_string(),
    )?;
    // F decrypt A's msg
    let text_f = decrypt_msg(&mut f_mls_group, &f_provider, msg_f)?;
    println!("A send msg to F ,the result is {:?}", text_f);

    // B send msg to A
    let msg2 = send_msg(
        &mut b_mls_group,
        &b_provider,
        &b_identity,
        "hello, A".to_string(),
    )?;
    // A decrypt B's msg
    let text2 = decrypt_msg(&mut a_mls_group, &a_provider, msg2)?;
    println!("B send msg to A ,the result is {:?}", text2);

    // F send msg to A
    let msg2_1 = send_msg(
        &mut f_mls_group,
        &f_provider,
        &f_identity,
        "hello, A".to_string(),
    )?;
    // A decrypt F's msg
    let text2_1 = decrypt_msg(&mut a_mls_group, &a_provider, msg2_1)?;
    println!("F send msg to A ,the result is {:?}", text2_1);

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        b_mls_group.export_secret(&b_provider, "", &[], 32).unwrap()
    );

    println!(
        "f_mls_group export secret {:?}",
        f_mls_group.export_secret(&f_provider, "", &[], 32).unwrap()
    );

    println!("--B add C F G --------------");

    // B add C and G
    let welcome2 = add_members(&mut b_mls_group, &b_provider, &b_identity, &[c_pk, g_pk])?;

    let mut c_mls_group =
        bob_join_mls_group(welcome2.1.clone(), &c_provider, &group_create_config)?;

    let mut g_mls_group =
        bob_join_mls_group(welcome2.1.clone(), &g_provider, &group_create_config)?;

    // A commit
    let _ = others_commit_add_member(&mut a_mls_group, welcome2.0.clone(), &a_provider)?;
    // F commit
    let _ = others_commit_add_member(&mut f_mls_group, welcome2.0.clone(), &f_provider)?;

    // B send msg
    let msg3 = send_msg(
        &mut b_mls_group,
        &b_provider,
        &b_identity,
        "hello, A, C, F, G".to_string(),
    )?;
    // A decrypt B's msg
    let text4 = decrypt_msg(&mut a_mls_group, &a_provider, msg3.clone())?;
    println!("B send msg to A ,the result is {:?}", text4);
    // C decrypt B's msg
    let text3 = decrypt_msg(&mut c_mls_group, &c_provider, msg3.clone())?;
    println!("B send msg to C ,the result is {:?}", text3);
    // F decrypt B's msg
    let text4_1 = decrypt_msg(&mut f_mls_group, &f_provider, msg3.clone())?;
    println!("B send msg to F ,the result is {:?}", text4_1);
    // G decrypt B's msg
    let text4_2 = decrypt_msg(&mut g_mls_group, &g_provider, msg3.clone())?;
    println!("B send msg to G ,the result is {:?}", text4_2);

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        b_mls_group.export_secret(&b_provider, "", &[], 32).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );

    println!(
        "f_mls_group export secret {:?}",
        f_mls_group.export_secret(&f_provider, "", &[], 32).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        g_mls_group.export_secret(&g_provider, "", &[], 32).unwrap()
    );

    println!("--A add D --------------");

    // A add D
    let welcome3 = add_member(&mut a_mls_group, &a_provider, &a_identity, d_pk)?;

    let mut d_mls_group = bob_join_mls_group(welcome3.1, &d_provider, &group_create_config)?;

    // B commit
    let _ = others_commit_add_member(&mut b_mls_group, welcome3.0.clone(), &b_provider)?;
    // C commit
    let _ = others_commit_add_member(&mut c_mls_group, welcome3.0.clone(), &c_provider)?;
    // F commit
    let _ = others_commit_add_member(&mut f_mls_group, welcome3.0.clone(), &f_provider)?;
    // F commit
    let _ = others_commit_add_member(&mut g_mls_group, welcome3.0.clone(), &g_provider)?;

    // A send msg
    let msg4 = send_msg(
        &mut a_mls_group,
        &a_provider,
        &a_identity,
        "hello, ABCDGF".to_string(),
    )?;
    // B decrypt A's msg
    let text5 = decrypt_msg(&mut b_mls_group, &b_provider, msg4.clone())?;
    println!("A send msg to B ,the result is {:?}", text5);
    // C decrypt A's msg
    let text6 = decrypt_msg(&mut c_mls_group, &c_provider, msg4.clone())?;
    println!("A send msg to C ,the result is {:?}", text6);
    // D decrypt B's msg
    let text7 = decrypt_msg(&mut d_mls_group, &d_provider, msg4.clone())?;
    println!("A send msg to D ,the result is {:?}", text7);
    // F decrypt B's msg
    let text7_1 = decrypt_msg(&mut f_mls_group, &f_provider, msg4.clone())?;
    println!("A send msg to F ,the result is {:?}", text7_1);
    // G decrypt B's msg
    let text7_2 = decrypt_msg(&mut g_mls_group, &g_provider, msg4.clone())?;
    println!("A send msg to G ,the result is {:?}", text7_2);

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );
    println!(
        "b_mls_group export secret {:?}",
        b_mls_group.export_secret(&b_provider, "", &[], 32).unwrap()
    );
    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );
    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );
    println!(
        "f_mls_group export secret {:?}",
        f_mls_group.export_secret(&f_provider, "", &[], 32).unwrap()
    );
    println!(
        "g_mls_group export secret {:?}",
        g_mls_group.export_secret(&g_provider, "", &[], 32).unwrap()
    );

    println!("--A remove B F --------------");

    // A remove B F
    let queued_msg = remove_members(
        &mut a_mls_group,
        &[b_mls_group.own_leaf_index(), f_mls_group.own_leaf_index()],
        &a_identity,
        &a_provider,
    )?;

    // B commit
    let _ = others_commit_remove_member(&mut b_mls_group, queued_msg.clone(), &b_provider)?;
    // C commit
    let _ = others_commit_remove_member(&mut c_mls_group, queued_msg.clone(), &c_provider)?;
    // D commit
    let _ = others_commit_remove_member(&mut d_mls_group, queued_msg.clone(), &d_provider)?;
    // F commit
    let _ = others_commit_remove_member(&mut f_mls_group, queued_msg.clone(), &f_provider)?;
    // G commit
    let _ = others_commit_remove_member(&mut g_mls_group, queued_msg.clone(), &g_provider)?;

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );
    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );
    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );
    println!(
        "g_mls_group export secret {:?}",
        g_mls_group.export_secret(&g_provider, "", &[], 32).unwrap()
    );

    // A add E
    let welcome4 = add_member(&mut a_mls_group, &a_provider, &a_identity, e_pk)?;

    let mut e_mls_group = bob_join_mls_group(welcome4.1, &e_provider, &group_create_config)?;

    // C commit
    let _ = others_commit_add_member(&mut c_mls_group, welcome4.0.clone(), &c_provider)?;
    // D commit
    let _ = others_commit_add_member(&mut d_mls_group, welcome4.0.clone(), &d_provider)?;
    // G commit
    let _ = others_commit_add_member(&mut g_mls_group, welcome4.0.clone(), &g_provider)?;

    println!("--A add E --------------");
    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );
    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );
    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );
    println!(
        "e_mls_group export secret {:?}",
        e_mls_group.export_secret(&e_provider, "", &[], 32).unwrap()
    );
    println!(
        "g_mls_group export secret {:?}",
        g_mls_group.export_secret(&g_provider, "", &[], 32).unwrap()
    );

    println!("--C leave --------------");

    let queued_msg = self_leave(&mut c_mls_group, &c_identity, &c_provider)?;
    // A proposal
    let _ = others_proposal_leave(&mut a_mls_group, queued_msg.clone(), &a_provider)?;
    // D proposal
    let _ = others_proposal_leave(&mut d_mls_group, queued_msg.clone(), &d_provider)?;
    // E proposal
    let _ = others_proposal_leave(&mut e_mls_group, queued_msg.clone(), &e_provider)?;
    // G proposal
    let _ = others_proposal_leave(&mut g_mls_group, queued_msg.clone(), &g_provider)?;

    // admin commit
    let queued_msg = admin_commit_leave(&mut a_mls_group, &a_identity, &a_provider)?;

    // C commit
    let _ = normal_member_commit_leave(&mut c_mls_group, queued_msg.clone(), &c_provider)?;
    // D commit
    let _ = normal_member_commit_leave(&mut d_mls_group, queued_msg.clone(), &d_provider)?;
    // E commit
    let _ = normal_member_commit_leave(&mut e_mls_group, queued_msg.clone(), &e_provider)?;
    // G commit
    let _ = normal_member_commit_leave(&mut g_mls_group, queued_msg.clone(), &g_provider)?;

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        e_mls_group.export_secret(&e_provider, "", &[], 32).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        g_mls_group.export_secret(&g_provider, "", &[], 32).unwrap()
    );

    // A send msg
    let msg5 = send_msg(
        &mut a_mls_group,
        &a_provider,
        &a_identity,
        "hello, DEG".to_string(),
    )?;
    // D decrypt A's msg
    let text8 = decrypt_msg(&mut d_mls_group, &d_provider, msg5.clone())?;
    println!("A send msg to D ,the result is {:?}", text8);
    // E decrypt A's msg
    let text9 = decrypt_msg(&mut e_mls_group, &e_provider, msg5.clone())?;
    println!("A send msg to E ,the result is {:?}", text9);
    // G decrypt A's msg
    let text10 = decrypt_msg(&mut g_mls_group, &g_provider, msg5.clone())?;
    println!("A send msg to G ,the result is {:?}", text10);

    // G send msg
    let msg6 = send_msg(
        &mut g_mls_group,
        &g_provider,
        &g_identity,
        "hello, ADE".to_string(),
    )?;
    // D decrypt G's msg
    let text6_1 = decrypt_msg(&mut d_mls_group, &d_provider, msg6.clone())?;
    println!("G send msg to D ,the result is {:?}", text6_1);
    // E decrypt G's msg
    let text6_2 = decrypt_msg(&mut e_mls_group, &e_provider, msg6.clone())?;
    println!("G send msg to E ,the result is {:?}", text6_2);
    // A decrypt G's msg
    let text6_3 = decrypt_msg(&mut a_mls_group, &a_provider, msg6.clone())?;
    println!("G send msg to A ,the result is {:?}", text6_3);

    println!("end --------------end");
    Ok(())
}

// if add some members, for example due to F reply dely and lack of one commit, this will lead to F tree is diff from others.
// So if dely, but every operation F should receive it, and process it in order by time, if not it will be error.
fn test_complex2() -> Result<()> {
    println!("start --------------start");
    let group_id = "G2".to_string();
    let a = "A";
    let b = "B";
    let c = "C";
    let d = "D";
    let e = "E";
    let f = "F";
    let g = "G";

    let db_path = "./mls-lite.sqlite".to_string();
    init_mls_db(db_path)?;

    let a_provider = create_provider(a.to_string())?;
    let b_provider = create_provider(b.to_string())?;
    let c_provider = create_provider(c.to_string())?;
    let d_provider = create_provider(d.to_string())?;
    let e_provider = create_provider(e.to_string())?;
    let f_provider = create_provider(f.to_string())?;
    let g_provider = create_provider(g.to_string())?;

    let a_identity = create_identity(a.to_string(), &a_provider)?;
    let mut b_identity = create_identity(b.to_string(), &b_provider)?;
    let mut c_identity = create_identity(c.to_string(), &c_provider)?;
    let mut d_identity = create_identity(d.to_string(), &d_provider)?;
    let mut e_identity = create_identity(e.to_string(), &e_provider)?;
    let mut f_identity = create_identity(f.to_string(), &f_provider)?;
    let mut g_identity = create_identity(g.to_string(), &g_provider)?;

    let b_pk = create_key_package(&b_provider, &mut b_identity)?;
    let c_pk = create_key_package(&c_provider, &mut c_identity)?;
    let d_pk = create_key_package(&d_provider, &mut d_identity)?;
    let e_pk = create_key_package(&e_provider, &mut e_identity)?;
    let f_pk = create_key_package(&f_provider, &mut f_identity)?;
    let g_pk = create_key_package(&g_provider, &mut g_identity)?;

    let group_create_config = group_create_config()?;

    let mut a_mls_group =
        create_mls_group(group_id, &group_create_config, &a_provider, &a_identity)?;

    // A add B F, but F not reply right now
    let welcome = add_members(
        &mut a_mls_group,
        &a_provider,
        &a_identity,
        &[b_pk, f_pk.clone()],
    )?;
    let mut b_mls_group = bob_join_mls_group(welcome.1.clone(), &b_provider, &group_create_config)?;
    // let mut f_mls_group = bob_join_mls_group(welcome.1.clone(), &f_provider, &group_create_config)?;

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        b_mls_group.export_secret(&b_provider, "", &[], 32).unwrap()
    );

    println!("--B add C G --------------");

    // B add C and G
    let welcome2 = add_members(&mut b_mls_group, &b_provider, &b_identity, &[c_pk, g_pk])?;

    // let mut f_mls_group = bob_join_mls_group(welcome.1.clone(), &f_provider, &group_create_config)?;

    let mut c_mls_group =
        bob_join_mls_group(welcome2.1.clone(), &c_provider, &group_create_config)?;

    let mut g_mls_group =
        bob_join_mls_group(welcome2.1.clone(), &g_provider, &group_create_config)?;

    // A commit
    let _ = others_commit_add_member(&mut a_mls_group, welcome2.0.clone(), &a_provider)?;
    // // F commit
    // let _ = others_commit_add_member(&mut f_mls_group, welcome2.0.clone(), &f_provider)?;

    println!("--------------");

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "b_mls_group export secret {:?}",
        b_mls_group.export_secret(&b_provider, "", &[], 32).unwrap()
    );

    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        g_mls_group.export_secret(&g_provider, "", &[], 32).unwrap()
    );

    let members = a_mls_group.members().collect::<Vec<_>>();
    println!("{}", members.len());
    let credential0 = members[0].credential.serialized_content();
    let credential1 = members[1].credential.serialized_content();
    let credential2 = members[2].credential.serialized_content();
    let credential3 = members[3].credential.serialized_content();
    let credential4 = members[4].credential.serialized_content();
    println!(
        " a_mls_group members is {:?}, {:?}, {:?}, {:?}, {:?}",
        String::from_utf8(credential0.to_vec()).unwrap(),
        String::from_utf8(credential1.to_vec()).unwrap(),
        String::from_utf8(credential2.to_vec()).unwrap(),
        String::from_utf8(credential3.to_vec()).unwrap(),
        String::from_utf8(credential4.to_vec()).unwrap()
    );

    let members = b_mls_group.members().collect::<Vec<_>>();
    println!("{}", members.len());
    let credential0 = members[0].credential.serialized_content();
    let credential1 = members[1].credential.serialized_content();
    let credential2 = members[2].credential.serialized_content();
    let credential3 = members[3].credential.serialized_content();
    let credential4 = members[4].credential.serialized_content();
    println!(
        " b_mls_group members is {:?}, {:?}, {:?}, {:?}, {:?}",
        String::from_utf8(credential0.to_vec()).unwrap(),
        String::from_utf8(credential1.to_vec()).unwrap(),
        String::from_utf8(credential2.to_vec()).unwrap(),
        String::from_utf8(credential3.to_vec()).unwrap(),
        String::from_utf8(credential4.to_vec()).unwrap()
    );

    let members = c_mls_group.members().collect::<Vec<_>>();
    println!("{}", members.len());
    let credential0 = members[0].credential.serialized_content();
    let credential1 = members[1].credential.serialized_content();
    let credential2 = members[2].credential.serialized_content();
    let credential3 = members[3].credential.serialized_content();
    let credential4 = members[4].credential.serialized_content();
    println!(
        " c_mls_group members is {:?}, {:?}, {:?}, {:?}, {:?}",
        String::from_utf8(credential0.to_vec()).unwrap(),
        String::from_utf8(credential1.to_vec()).unwrap(),
        String::from_utf8(credential2.to_vec()).unwrap(),
        String::from_utf8(credential3.to_vec()).unwrap(),
        String::from_utf8(credential4.to_vec()).unwrap()
    );

    let members = g_mls_group.members().collect::<Vec<_>>();
    println!("{}", members.len());
    let credential0 = members[0].credential.serialized_content();
    let credential1 = members[1].credential.serialized_content();
    let credential2 = members[2].credential.serialized_content();
    let credential3 = members[3].credential.serialized_content();
    let credential4 = members[4].credential.serialized_content();
    println!(
        " g_mls_group members is {:?}, {:?}, {:?}, {:?}, {:?}",
        String::from_utf8(credential0.to_vec()).unwrap(),
        String::from_utf8(credential1.to_vec()).unwrap(),
        String::from_utf8(credential2.to_vec()).unwrap(),
        String::from_utf8(credential3.to_vec()).unwrap(),
        String::from_utf8(credential4.to_vec()).unwrap()
    );

    println!("--A add D --------------");

    // A add D
    let welcome3 = add_member(&mut a_mls_group, &a_provider, &a_identity, d_pk)?;
    let mut d_mls_group =
        bob_join_mls_group(welcome3.1.clone(), &d_provider, &group_create_config)?;

    // // A add D F
    // let welcome3 = add_members(&mut a_mls_group, &a_provider, &a_identity, &[d_pk, f_pk.clone()])?;

    println!("--f_mls_group --------------");
    let mut f_mls_group = bob_join_mls_group(welcome.1.clone(), &f_provider, &group_create_config)?;
    let _ = others_commit_add_member(&mut f_mls_group, welcome2.0.clone(), &f_provider)?;

    // B commit
    let _ = others_commit_add_member(&mut b_mls_group, welcome3.0.clone(), &b_provider)?;
    // C commit
    let _ = others_commit_add_member(&mut c_mls_group, welcome3.0.clone(), &c_provider)?;
    println!("--F commit --------------");
    // F commit, add some members, due to F reply dely, lead to F lack ont commit
    let _ = others_commit_add_member(&mut f_mls_group, welcome3.0.clone(), &f_provider)?;
    // G commit
    let _ = others_commit_add_member(&mut g_mls_group, welcome3.0.clone(), &g_provider)?;

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );
    println!(
        "b_mls_group export secret {:?}",
        b_mls_group.export_secret(&b_provider, "", &[], 32).unwrap()
    );
    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );
    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );
    println!(
        "f_mls_group export secret {:?}",
        f_mls_group.export_secret(&f_provider, "", &[], 32).unwrap()
    );
    println!(
        "g_mls_group export secret {:?}",
        g_mls_group.export_secret(&g_provider, "", &[], 32).unwrap()
    );

    println!("--A remove B F --------------");

    // A remove B F
    let queued_msg = remove_members(
        &mut a_mls_group,
        &[b_mls_group.own_leaf_index(), f_mls_group.own_leaf_index()],
        &a_identity,
        &a_provider,
    )?;

    // B commit
    let _ = others_commit_remove_member(&mut b_mls_group, queued_msg.clone(), &b_provider)?;
    // C commit
    let _ = others_commit_remove_member(&mut c_mls_group, queued_msg.clone(), &c_provider)?;
    // D commit
    let _ = others_commit_remove_member(&mut d_mls_group, queued_msg.clone(), &d_provider)?;
    // F commit
    let _ = others_commit_remove_member(&mut f_mls_group, queued_msg.clone(), &f_provider)?;
    // G commit
    let _ = others_commit_remove_member(&mut g_mls_group, queued_msg.clone(), &g_provider)?;

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );
    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );
    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );
    println!(
        "g_mls_group export secret {:?}",
        g_mls_group.export_secret(&g_provider, "", &[], 32).unwrap()
    );

    // A add E
    let welcome4 = add_member(&mut a_mls_group, &a_provider, &a_identity, e_pk)?;

    let mut e_mls_group = bob_join_mls_group(welcome4.1, &e_provider, &group_create_config)?;

    // C commit
    let _ = others_commit_add_member(&mut c_mls_group, welcome4.0.clone(), &c_provider)?;
    // D commit
    let _ = others_commit_add_member(&mut d_mls_group, welcome4.0.clone(), &d_provider)?;
    // G commit
    let _ = others_commit_add_member(&mut g_mls_group, welcome4.0.clone(), &g_provider)?;

    println!("--A add E --------------");
    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );
    println!(
        "c_mls_group export secret {:?}",
        c_mls_group.export_secret(&c_provider, "", &[], 32).unwrap()
    );
    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );
    println!(
        "e_mls_group export secret {:?}",
        e_mls_group.export_secret(&e_provider, "", &[], 32).unwrap()
    );
    println!(
        "g_mls_group export secret {:?}",
        g_mls_group.export_secret(&g_provider, "", &[], 32).unwrap()
    );

    println!("--C leave --------------");

    let queued_msg = self_leave(&mut c_mls_group, &c_identity, &c_provider)?;
    // A proposal
    let _ = others_proposal_leave(&mut a_mls_group, queued_msg.clone(), &a_provider)?;
    // D proposal
    let _ = others_proposal_leave(&mut d_mls_group, queued_msg.clone(), &d_provider)?;
    // E proposal
    let _ = others_proposal_leave(&mut e_mls_group, queued_msg.clone(), &e_provider)?;
    // G proposal
    let _ = others_proposal_leave(&mut g_mls_group, queued_msg.clone(), &g_provider)?;

    // admin commit
    let queued_msg = admin_commit_leave(&mut a_mls_group, &a_identity, &a_provider)?;

    // C commit
    let _ = normal_member_commit_leave(&mut c_mls_group, queued_msg.clone(), &c_provider)?;
    // D commit
    let _ = normal_member_commit_leave(&mut d_mls_group, queued_msg.clone(), &d_provider)?;
    // E commit
    let _ = normal_member_commit_leave(&mut e_mls_group, queued_msg.clone(), &e_provider)?;
    // G commit
    let _ = normal_member_commit_leave(&mut g_mls_group, queued_msg.clone(), &g_provider)?;

    println!(
        "a_mls_group export secret {:?}",
        a_mls_group.export_secret(&a_provider, "", &[], 32).unwrap()
    );

    println!(
        "d_mls_group export secret {:?}",
        d_mls_group.export_secret(&d_provider, "", &[], 32).unwrap()
    );

    println!(
        "e_mls_group export secret {:?}",
        e_mls_group.export_secret(&e_provider, "", &[], 32).unwrap()
    );

    println!(
        "g_mls_group export secret {:?}",
        g_mls_group.export_secret(&g_provider, "", &[], 32).unwrap()
    );

    println!("end --------------end");
    Ok(())
}
