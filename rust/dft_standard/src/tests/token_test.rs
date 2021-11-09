use crate::token::TokenBasic;
use crate::token::TokenStandard;
use candid::Nat;
use candid::Principal;
use dft_types::*;
use std::collections::HashMap;
// test default TokenBasic value
#[test]
fn test_token_basic_default_value() {
    //create default TokenBasic
    let token = TokenBasic::default();
    // check token id is Principal::anonymous()
    assert_eq!(token.id(), Principal::anonymous());
    // check owner is Principal::anonymous()
    assert_eq!(token.owner(), Principal::anonymous());
    // check token's name is empty
    assert_eq!(token.name(), "");
    // check token's symblo is empty
    assert_eq!(token.symbol(), "");
    // check token's decimals is 0
    assert_eq!(token.decimals(), 0);
    // check token's total supply is 0
    assert_eq!(token.total_supply(), 0);
    // check token's owner is Principal::anonymous()
    assert_eq!(token.owner(), Principal::anonymous());
    // check token's logo is empty
    let null_logo: Vec<u8> = vec![];
    assert_eq!(token.logo(), null_logo);
    // check token's fee is 0
    let fee = token.fee();
    assert_eq!(fee.minimum, 0);
    assert_eq!(fee.rate, 0);
    // check desc is empty
    let empty_map: HashMap<String, String> = HashMap::new();
    assert_eq!(token.desc(), empty_map);
}

//test token initialize with all parameters
#[test]
fn test_token_basic_initialize_all_parameters() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let owner =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let fee_to = TokenHolder::new(owner, None);
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &owner,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18,
        fee.clone(),
        fee_to,
    );
    assert_eq!(token.id(), token_id);
    assert_eq!(token.owner(), owner);
    assert_eq!(token.name(), "test_token");
    assert_eq!(token.symbol(), "TEST");
    assert_eq!(token.decimals(), 18);
    assert_eq!(token.total_supply(), 0);
    assert_eq!(token.logo(), logo);
    assert_eq!(token.fee(), fee);
}
//test token set_fee
#[test]
fn test_token_basic_set_fee() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let owner =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let fee_to = TokenHolder::new(owner, None);
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &owner,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18,
        fee.clone(),
        fee_to,
    );
    let fee = Fee {
        minimum: Nat::from(2),
        rate: Nat::from(0),
    };
    let res = token.set_fee(&owner, fee.clone());
    assert!(res.is_ok(), "set_fee should be ok");
    assert_eq!(token.fee(), fee);
}
//test token set_fee with invalid owner
#[test]
fn test_token_basic_set_fee_invalid_owner() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let owner =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let fee_to = TokenHolder::new(owner, None);
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &owner,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18,
        fee.clone(),
        fee_to,
    );
    let new_fee = Fee {
        minimum: Nat::from(2),
        rate: Nat::from(0),
    };
    let res = token.set_fee(&Principal::anonymous(), new_fee.clone());
    // check result is Err
    assert!(res.is_err());
    // check fee is not changed
    assert_eq!(token.fee(), fee);
}

// test token set_fee_to
#[test]
fn test_token_basic_set_fee_to() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let owner =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let fee_to = TokenHolder::new(owner, None);
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &owner,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18u8,
        fee.clone(),
        fee_to,
    );
    let new_fee_to = TokenHolder::new(
        Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe")
            .unwrap(),
        None,
    );
    let res = token.set_fee_to(&owner, new_fee_to.clone());
    assert!(res.is_ok(), "set_fee_to should be ok");
    assert_eq!(token.token_info().fee_to, new_fee_to);
}

// test token set_fee_to with invalid owner
#[test]
fn test_token_basic_set_fee_to_invalid_owner() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let owner =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let fee_to = TokenHolder::new(owner, None);
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &owner,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18u8,
        fee.clone(),
        fee_to.clone(),
    );
    let new_fee_to = TokenHolder::new(
        Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe")
            .unwrap(),
        None,
    );
    let res = token.set_fee_to(&Principal::anonymous(), new_fee_to.clone());
    // check result is Err
    assert!(res.is_err(), "set_fee_to should be Err");
    // check fee_to is not changed
    // let crt_fee_to = token.token_info().fee_to;
    // assert_eq!(crt_fee_to, new_fee_to);
}

// test token set_logo
#[test]
fn test_token_basic_set_logo() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let owner =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let fee_to = TokenHolder::new(owner, None);
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &owner,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18u8,
        fee.clone(),
        fee_to,
    );
    let new_logo = vec![0x00, 0x01, 0x02, 0x04];
    let res = token.set_logo(&owner, new_logo.clone());
    assert!(res.is_ok(), "set_logo should be ok");
    assert_eq!(token.logo(), new_logo);
}

// test token set_logo with invalid owner
#[test]
fn test_token_basic_set_logo_invalid_owner() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let owner =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let fee_to = TokenHolder::new(owner, None);
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &owner,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18u8,
        fee.clone(),
        fee_to,
    );
    let new_logo = vec![0x00, 0x01, 0x02, 0x04];
    let res = token.set_logo(&Principal::anonymous(), new_logo.clone());
    // check result is Err
    assert!(res.is_err(), "set_logo should be Err");
    // check logo is not changed
    assert_eq!(token.logo(), logo);
}

// test token set_desc
#[test]
fn test_token_basic_set_desc() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let owner =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let fee_to = TokenHolder::new(owner, None);
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &owner,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18u8,
        fee.clone(),
        fee_to,
    );

    let new_desc: HashMap<String, String> = vec![(
        "TWITTER".to_owned(),
        "https://twitter.com/DelandLabs".to_owned(),
    )]
    .into_iter()
    .collect();
    let res = token.set_desc(&owner, new_desc.clone());
    assert!(res.is_ok(), "set_desc should be ok");
    assert_eq!(token.desc(), new_desc);

    // try to add a new key in desc which is not exist in EXTEND_KEYS
    let new_desc1: HashMap<String, String> = vec![(
        "TWITTER1".to_owned(),
        "https://twitter.com/DelandLabs1".to_owned(),
    )]
    .into_iter()
    .collect();
    let res = token.set_desc(&owner, new_desc1.clone());
    // the token's desc will not be changed
    assert!(res.is_ok(), "set_desc should be succeed");
    assert_eq!(token.desc(), new_desc);
}

// test token set_desc with invalid owner
#[test]
fn test_token_basic_set_desc_invalid_owner() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let owner =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let fee_to = TokenHolder::new(owner, None);
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &owner,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18u8,
        fee.clone(),
        fee_to,
    );
    let new_desc: HashMap<String, String> = vec![(
        "TWITTER".to_owned(),
        "https://twitter.com/DelandLabs".to_owned(),
    )]
    .into_iter()
    .collect();
    let res = token.set_desc(&Principal::anonymous(), new_desc.clone());
    // check result is Err
    assert!(res.is_err(), "set_desc should be Err");
    // check logo is not changed
    assert_eq!(token.desc(), HashMap::new());
}

//test token fee calculation
#[test]
fn test_token_basic_fee_calculation() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let caller_principal =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let from_principal =
        Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe")
            .unwrap();
    let spender_principal =
        Principal::from_text("7zap4-dnqjf-k2oei-jj2uj-sw6db-eksrj-kzc5h-nmki4-x5fcn-w53an-gae")
            .unwrap();
    let to_principal =
        Principal::from_text("b2zme-qgvk4-rgln6-mycul-7bgye-dbpyl-7rrvm-i2rzy-ybw5r-ncnd6-xqe")
            .unwrap();
    let from_holder = TokenHolder::new(from_principal, None);
    let spender_holder = TokenHolder::new(spender_principal, None);
    let to_holder = TokenHolder::new(to_principal, None);
    let fee_to = TokenHolder::new(caller_principal, None);
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(1000000), //1%
    };
    token.initialize(
        &caller_principal,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18,
        fee.clone(),
        fee_to.clone(),
    );

    //now
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let now_u64 = now as u64;

    // mint token to from_holder
    let mint_val = Nat::from(10000);
    let approve_val = Nat::from(1011);
    let mint_res = token._mint(&caller_principal, &from_holder, mint_val.clone(), now_u64);
    assert!(mint_res.is_ok());
    // check owner_holder balance
    let owner_balance = token.balance_of(&from_holder);
    assert_eq!(owner_balance, mint_val.clone());
    // approve
    let approve_rs = token.approve(
        &caller_principal,
        &from_holder,
        &spender_holder,
        approve_val.clone(),
        now_u64,
    );
    // check approve result is ok
    assert!(approve_rs.is_ok());
    // check approve fee charge
    let approve_fee_charged = token.balance_of(&fee_to);
    assert_eq!(approve_fee_charged, fee.minimum.clone());
    // check from_holder balance
    let from_balance = token.balance_of(&from_holder);
    assert_eq!(from_balance, mint_val.clone() - fee.minimum.clone());

    // approve again
    let approve_rs = token.approve(
        &caller_principal,
        &from_holder,
        &spender_holder,
        approve_val.clone(),
        now_u64,
    );
    // check approve is ok
    assert!(approve_rs.is_ok());
    // check approve fee charge
    let approve_fee_charged = token.balance_of(&fee_to);
    assert_eq!(approve_fee_charged.clone(), fee.minimum.clone() * 2);

    // check from_holder balance
    let from_balance = token.balance_of(&from_holder);
    assert_eq!(from_balance, mint_val.clone() - fee.minimum.clone() * 2);

    // check spender_holder balance
    let spender_balance = token.balance_of(&spender_holder);
    assert_eq!(spender_balance, 0);

    // transfer from
    let transfer_val = Nat::from(1000);
    let transfer_from_res = token.transfer_from(
        &caller_principal,
        &from_holder,
        &spender_holder,
        &to_holder,
        transfer_val.clone(),
        now_u64,
    );
    // check transfer_from_res is Ok
    assert!(
        transfer_from_res.is_ok(),
        "transfer_from_res is Err,{:?}",
        transfer_from_res.unwrap_err()
    );
    // check transfer_from fee charge
    let transfer_fee_charged = token.balance_of(&fee_to) - approve_fee_charged.clone();
    let transfer_fee: Nat = transfer_val.clone() * fee.rate.clone() / 100000000;
    assert_eq!(transfer_fee_charged, transfer_fee);
    // check transfer_from result
    assert!(transfer_from_res.is_ok());
    // check from_holder balance
    let from_balance = token.balance_of(&from_holder);
    assert_eq!(
        from_balance,
        mint_val.clone()
            - approve_fee_charged.clone()
            - transfer_val.clone()
            - transfer_fee.clone()
    );
    // check spender_holder balance
    let spender_balance = token.balance_of(&spender_holder);
    assert_eq!(spender_balance, 0);

    // check to_holder balance
    let to_balance = token.balance_of(&to_holder);
    assert_eq!(to_balance, transfer_val);

    // transfer
    let transfer_val2 = Nat::from(2000);
    let transfer_res2 = token.transfer(
        &caller_principal,
        &from_holder,
        &to_holder,
        transfer_val2.clone(),
        now_u64,
    );
    // check transfer result
    assert!(transfer_res2.is_ok());
    // check transfer_to fee charged
    let transfer_fee_charged2 =
        token.balance_of(&fee_to) - transfer_fee_charged.clone() - approve_fee_charged.clone();
    let transfer_fee2 = transfer_val2.clone() * fee.rate.clone() / 100000000;
    assert_eq!(transfer_fee_charged2, transfer_fee2);
    // check from_holder balance
    let from_balance = token.balance_of(&from_holder);
    assert_eq!(
        from_balance,
        mint_val.clone()
            - approve_fee_charged
            - transfer_val.clone()
            - transfer_fee
            - transfer_val2.clone()
            - transfer_fee2
    );

    // check total supply
    let total_supply = token.total_supply();
    assert_eq!(total_supply, mint_val.clone());
}

//test token approve
#[test]
fn test_token_basic_approve() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let caller_principal =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let spender_principal =
        Principal::from_text("7zap4-dnqjf-k2oei-jj2uj-sw6db-eksrj-kzc5h-nmki4-x5fcn-w53an-gae")
            .unwrap();
    let owner_holder = TokenHolder::new(caller_principal, None);
    let spender_holder = TokenHolder::new(spender_principal, None);
    let fee_to = owner_holder.clone();
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &caller_principal,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18,
        fee.clone(),
        fee_to,
    );
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let now_u64 = now as u64;

    // mint token to owner_holder
    let mint_val = Nat::from(10000);
    let approve_val = Nat::from(1000);
    let mint_res = token._mint(&caller_principal, &owner_holder, mint_val.clone(), now_u64);
    // mint_res is ok
    assert!(mint_res.is_ok());
    // check owner_holder balance
    let owner_balance = token.balance_of(&owner_holder);
    assert_eq!(owner_balance, mint_val);
    // approve
    let approve_rs = token.approve(
        &caller_principal,
        &owner_holder,
        &spender_holder,
        approve_val.clone(),
        now_u64,
    );
    // approve_rs is ok
    assert!(approve_rs.is_ok(), "{:?}", approve_rs.unwrap_err());
    // check allowance
    let allowance = token.allowance(&owner_holder, &spender_holder);
    assert_eq!(allowance, approve_val);
    // approve a new value to spender_holder
    let new_approve_val = Nat::from(2000);
    let new_approve_rs = token.approve(
        &caller_principal,
        &owner_holder,
        &spender_holder,
        new_approve_val.clone(),
        now_u64,
    );
    // new_approve_rs is ok
    assert!(new_approve_rs.is_ok(), "{:?}", new_approve_rs.unwrap_err());
    // check allowance
    let new_allowance = token.allowance(&owner_holder, &spender_holder);
    assert_eq!(new_allowance, new_approve_val);
    // check token's txs have 3 records
    let token_payload = token.to_token_payload();
    let token_txs = token_payload.txs_inner;
    assert_eq!(token_txs.len(), 3);
    assert_eq!(token_payload.tx_index_cursor, 3);
    // check total supply
    let total_supply = token.total_supply();
    assert_eq!(total_supply, mint_val.clone());
}

// test token approve/transfer_from
#[test]
fn test_token_basic_transfer_from() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let caller_principal =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();

    let from_principal =
        Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe")
            .unwrap();
    let spender_principal =
        Principal::from_text("7zap4-dnqjf-k2oei-jj2uj-sw6db-eksrj-kzc5h-nmki4-x5fcn-w53an-gae")
            .unwrap();
    let to_principal =
        Principal::from_text("b2zme-qgvk4-rgln6-mycul-7bgye-dbpyl-7rrvm-i2rzy-ybw5r-ncnd6-xqe")
            .unwrap();
    let owner_holder = TokenHolder::new(caller_principal, None);
    let from_holder = TokenHolder::new(from_principal, None);
    let spender_holder = TokenHolder::new(spender_principal, None);
    let to_holder = TokenHolder::new(to_principal, None);
    let fee_to = owner_holder.clone();
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &caller_principal,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18,
        fee.clone(),
        fee_to,
    );
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let now_u64 = now as u64;

    // mint token to from_holder
    let mint_val = Nat::from(10000);
    let approve_val = Nat::from(1000);
    let mint_res = token._mint(&caller_principal, &from_holder, mint_val.clone(), now_u64);
    // mint_res is ok
    assert!(mint_res.is_ok());
    // check from_holder balance
    let owner_balance = token.balance_of(&from_holder);
    assert_eq!(owner_balance, mint_val);
    // approve
    let approve_rs = token.approve(
        &caller_principal,
        &from_holder,
        &spender_holder,
        approve_val.clone(),
        now_u64,
    );
    // approve_rs is ok
    assert!(approve_rs.is_ok(), "{:?}", approve_rs.unwrap_err());
    // check allowance
    let allowance = token.allowance(&from_holder, &spender_holder);
    assert_eq!(allowance, approve_val);
    // check from_holder balance
    let owner_balance = token.balance_of(&from_holder);
    assert_eq!(owner_balance, mint_val.clone() - fee.clone().minimum);

    // try transfer_from exceed allowance , should return err
    let transfer_from_val = Nat::from(1001);
    let result = token.transfer_from(
        &spender_principal,
        &from_holder,
        &spender_holder,
        &to_holder,
        transfer_from_val,
        now_u64 + 1,
    );
    assert!(result.is_err());
    // try transfer_from less than allowance , should return ok
    let transfer_from_val = Nat::from(500);
    let result = token.transfer_from(
        &spender_principal,
        &from_holder,
        &spender_holder,
        &to_holder,
        transfer_from_val.clone(),
        now_u64 + 1,
    );
    assert!(result.is_ok(), "{:?}", result.err().unwrap());
    // check allowance
    let allowance = token.allowance(&from_holder, &spender_holder);
    assert_eq!(
        allowance,
        approve_val - transfer_from_val.clone() - fee.clone().minimum
    );
    // check from_holder balance
    let from_balance = token.balance_of(&from_holder);
    assert_eq!(
        from_balance,
        mint_val.clone() - transfer_from_val - fee.clone().minimum * 2
    );
    // check owner_holder balance
    let owner_balance = token.balance_of(&owner_holder);
    assert_eq!(owner_balance, fee.clone().minimum * 2);
    // check token's txs have 3 records
    let token_payload = token.to_token_payload();
    let token_txs = token_payload.txs_inner;
    assert_eq!(token_txs.len(), 3);
    assert_eq!(token_payload.tx_index_cursor, 3);
    // check total supply
    let total_supply = token.total_supply();
    assert_eq!(total_supply, mint_val);
}

// test token transfer
#[test]
fn test_token_basic_transfer() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let caller_principal =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();

    let from_principal =
        Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe")
            .unwrap();
    let to_principal =
        Principal::from_text("b2zme-qgvk4-rgln6-mycul-7bgye-dbpyl-7rrvm-i2rzy-ybw5r-ncnd6-xqe")
            .unwrap();
    let owner_holder = TokenHolder::new(caller_principal, None);
    let from_holder = TokenHolder::new(from_principal, None);
    let to_holder = TokenHolder::new(to_principal, None);
    let fee_to = owner_holder.clone();
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &caller_principal,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18,
        fee.clone(),
        fee_to,
    );
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let now_u64 = now as u64;
    // mint token to from_holder
    let mint_val = Nat::from(10000);
    let _mint_res = token._mint(&caller_principal, &from_holder, mint_val.clone(), now_u64);

    // transfer token from from_holder to to_holder
    let transfer_val = Nat::from(1000);
    let transfer_res = token.transfer(
        &caller_principal,
        &from_holder,
        &to_holder,
        transfer_val.clone(),
        now_u64,
    );

    // transfer_res is ok
    assert!(transfer_res.is_ok(), "{:?}", transfer_res.unwrap_err());
    // check from_holder balance
    let from_balance = token.balance_of(&from_holder);
    assert_eq!(
        from_balance,
        mint_val.clone() - transfer_val.clone() - fee.clone().minimum
    );
    // check to_holder balance
    let to_balance = token.balance_of(&to_holder);
    assert_eq!(to_balance, transfer_val);
    // check token's txs have 2 records
    let token_payload = token.to_token_payload();
    let token_txs = token_payload.txs_inner;
    assert_eq!(token_txs.len(), 2);
    assert_eq!(token_payload.tx_index_cursor, 2);
    // check total supply
    let total_supply = token.total_supply();
    assert_eq!(total_supply, mint_val);
}

// test token _mint/_burn
#[test]
fn test_token_basic_mint_burn() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let caller_principal =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();

    let owner_holder = TokenHolder::new(caller_principal, None);
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &caller_principal,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18,
        fee.clone(),
        owner_holder.clone(),
    );
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let now_u64 = now as u64;
    // mint token to from_holder
    let mint_val = Nat::from(10000);
    let _mint_res = token._mint(&caller_principal, &owner_holder, mint_val.clone(), now_u64);
    // check mint_res is ok, and check owner_holder balance
    assert!(_mint_res.is_ok(), "{:?}", _mint_res.unwrap_err());
    let owner_balance = token.balance_of(&owner_holder);
    assert_eq!(owner_balance, mint_val);

    // check total supply
    let total_supply = token.total_supply();
    assert_eq!(total_supply, mint_val);

    // transfer token from owner_holder to to_holder
    let burn_val = Nat::from(1000);
    let burn_res = token._burn(&caller_principal, &owner_holder, burn_val.clone(), now_u64);

    // check burn_res is ok, and check owner_holder balance
    assert!(burn_res.is_ok(), "{:?}", burn_res.unwrap_err());
    let owner_balance = token.balance_of(&owner_holder);
    assert_eq!(owner_balance, mint_val.clone() - burn_val.clone());

    // check total supply
    let total_supply = token.total_supply();
    assert_eq!(total_supply, mint_val - burn_val);
}

// test token approve/transfer_from/transfer anonymous call should fail
#[test]
fn test_token_basic_approve_transfer_from_transfer() {
    //create TokenBasic with all parameters
    let mut token = TokenBasic::default();
    let caller_principal =
        Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae")
            .unwrap();
    let from_principal =
        Principal::from_text("b2zme-qgvk4-rgln6-mycul-7bgye-dbpyl-7rrvm-i2rzy-ybw5r-ncnd6-xqe")
            .unwrap();
    let to_principal =
        Principal::from_text("b2zme-qgvk4-rgln6-mycul-7bgye-dbpyl-7rrvm-i2rzy-ybw5r-ncnd6-xqe")
            .unwrap();
    let owner_holder = TokenHolder::new(caller_principal, None);
    let from_holder = TokenHolder::new(from_principal, None);
    let to_holder = TokenHolder::new(to_principal, None);
    let fee_to = owner_holder.clone();
    let token_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let logo = vec![0x00, 0x01, 0x02, 0x03];
    let fee = Fee {
        minimum: Nat::from(1),
        rate: Nat::from(0),
    };
    token.initialize(
        &caller_principal,
        token_id,
        Some(logo.clone()),
        "test_token".to_owned(),
        "TEST".to_owned(),
        18,
        fee.clone(),
        fee_to.clone(),
    );
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let now_u64 = now as u64;
    // apporve with anonymous should fail
    let approve_val = Nat::from(1000);
    let approve_res = token.approve(
        &caller_principal,
        &from_holder,
        &to_holder,
        approve_val.clone(),
        now_u64,
    );
    assert!(approve_res.is_err());
    // transfer_from with anonymous should fail
    let transfer_from_val = Nat::from(1000);
    let transfer_from_res = token.transfer_from(
        &caller_principal,
        &owner_holder,
        &from_holder,
        &to_holder,
        transfer_from_val.clone(),
        now_u64,
    );
    assert!(transfer_from_res.is_err());
    // transfer with anonymous should fail
    let transfer_val = Nat::from(1000);
    let transfer_res = token.transfer(
        &caller_principal,
        &from_holder,
        &to_holder,
        transfer_val.clone(),
        now_u64,
    );
    assert!(transfer_res.is_err());
}
