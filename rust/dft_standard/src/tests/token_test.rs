#[cfg(test)]
use crate::token::Token;
use crate::token::TokenBasic;
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
        fee,
        fee_to,
    );
    assert_eq!(token.id(), token_id);
    assert_eq!(token.owner(), owner);
    assert_eq!(token.name(), "test_token");
    assert_eq!(token.symbol(), "TEST");
    assert_eq!(token.decimals(), 18);
    assert_eq!(token.total_supply(), 0);
    assert_eq!(token.logo(), logo);
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
    let owner_balance = token.balance_of(&from_holder);
    assert_eq!(
        owner_balance,
        mint_val - transfer_from_val - fee.minimum * 2
    );
}
