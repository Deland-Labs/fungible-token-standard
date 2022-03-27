use crate::token::TokenBasic;
use crate::token::TokenStandard;
use candid::Nat;
use candid::Principal;
use dft_types::constants::DEFAULT_FEE_RATE_DECIMALS;
use dft_types::*;
use rstest::*;
use std::collections::HashMap;
use std::io::Read;
use std::ops::Mul;

#[fixture]
fn test_logo() -> Vec<u8> {
    // read logo delandlabs.png as bytes
    let mut logo_bytes = Vec::new();
    std::fs::File::open("src/tests/deland-labs-old-logo.jpg")
        .unwrap()
        .read_to_end(&mut logo_bytes)
        .unwrap();
    logo_bytes
}

#[fixture]
fn new_logo() -> Vec<u8> {
    // read logo delandlabs.png as bytes
    let mut logo_bytes = Vec::new();
    std::fs::File::open("src/tests/deland-labs-new-logo.png")
        .unwrap()
        .read_to_end(&mut logo_bytes)
        .unwrap();
    logo_bytes
}

#[fixture]
fn test_owner() -> Principal {
    Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae").unwrap()
}

// other caller
#[fixture]
fn other_caller() -> Principal {
    Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe").unwrap()
}

// minter
#[fixture]
fn test_minter() -> Principal {
    Principal::from_text("o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae").unwrap()
}

// spender
#[fixture]
fn test_spender() -> Principal {
    Principal::from_text("7zap4-dnqjf-k2oei-jj2uj-sw6db-eksrj-kzc5h-nmki4-x5fcn-w53an-gae").unwrap()
}

#[fixture]
fn test_token_id() -> Principal {
    Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
}

#[fixture]
fn test_name() -> String {
    "Deland Labs Token".to_string()
}

#[fixture]
fn test_symbol() -> String {
    "DLT".to_string()
}

#[fixture]
fn test_decimals() -> u8 {
    18u8
}

#[fixture]
fn test_total_supply() -> u128 {
    100000000u128
}

// test fee 0 rate
#[fixture]
fn test_fee_0_rate() -> Fee {
    Fee {
        minimum: Nat::from(2u64),
        rate: Nat::from(0),
        rate_decimals: DEFAULT_FEE_RATE_DECIMALS,
    }
}

// test fee non 0 rate
#[fixture]
fn test_fee_non_0_rate() -> Fee {
    Fee {
        minimum: Nat::from(2u64),
        rate: Nat::from(1u64),
        rate_decimals: 2,
    }
}

#[fixture]
fn test_token_with_0_fee_rate() -> TokenBasic {
    let mut token = TokenBasic::default();
    let fee_to = TokenHolder::new(test_owner(), None);
    token.initialize(
        &test_owner(),
        test_token_id(),
        Some(test_logo()),
        test_name(),
        test_symbol(),
        test_decimals(),
        test_fee_0_rate(),
        fee_to,
    );
    token
}

#[fixture]
fn test_token_with_non_0_fee_rate() -> TokenBasic {
    let mut token = TokenBasic::default();
    let fee_to = TokenHolder::new(test_owner(), None);
    token.initialize(
        &test_owner(),
        test_token_id(),
        Some(test_logo()),
        test_name(),
        test_symbol(),
        test_decimals(),
        test_fee_non_0_rate(),
        fee_to,
    );
    token
}

#[fixture]
fn now() -> u64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    now as u64
}

// test default TokenBasic value
#[rstest]
fn test_token_basic_default_value() {
    //create default TokenBasic
    let token = TokenBasic::default();
    // check token id is Principal::anonymous()
    assert_eq!(token.id(), Principal::anonymous());
    // check owner is Principal::anonymous()
    assert_eq!(token.owner(), Principal::anonymous());
    // check token's name is empty
    assert_eq!(token.name(), "");
    // check token's symbol is empty
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

#[rstest]
#[should_panic]
fn test_token_basic_logo_invalid_image(
    test_owner: Principal,
    test_token_id: Principal,
    test_name: String,
    test_symbol: String,
    test_decimals: u8,
    test_fee_0_rate: Fee,
) {
    let mut token = TokenBasic::default();
    let fee_to = TokenHolder::new(test_owner.clone(), None);

    token.initialize(
        &test_owner,
        test_token_id,
        Some(vec![0u8; 20]),
        test_name.clone(),
        test_symbol,
        test_decimals,
        test_fee_0_rate,
        fee_to,
    );
    // will panic if logo is a unspported image type
    assert_eq!(token.name(), test_name);
}

#[rstest]
fn test_token_basic_initialize_all_parameters(test_token_with_0_fee_rate: TokenBasic) {
    let test_token = test_token_with_0_fee_rate;
    assert_eq!(test_token.id(), test_token_id());
    assert_eq!(test_token.owner(), test_owner());
    assert_eq!(test_token.name(), test_name());
    assert_eq!(test_token.symbol(), test_symbol());
    assert_eq!(test_token.decimals(), test_decimals());
    assert_eq!(test_token.total_supply(), 0);
    assert_eq!(test_token.logo(), test_logo());
    assert_eq!(test_token.fee(), test_fee_0_rate());
}

//test token set_fee
#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_set_fee(#[case] test_token: TokenBasic, test_owner: Principal, now: u64) {
    let mut token = test_token.clone();
    let new_fee = Fee {
        minimum: Nat::from(2),
        rate: Nat::from(0),
        rate_decimals: DEFAULT_FEE_RATE_DECIMALS,
    };
    let res = token.set_fee(&test_owner, new_fee.clone(), None, now);
    assert!(res.is_ok(), "set_fee should be ok");
    assert_eq!(token.fee(), new_fee);
}

//test token set_fee with invalid owner
#[rstest]
fn test_token_basic_set_fee_invalid_owner(
    test_token_with_0_fee_rate: TokenBasic,
    other_caller: Principal,
    now: u64,
) {
    let mut token = test_token_with_0_fee_rate.clone();
    let new_fee = Fee {
        minimum: Nat::from(2),
        rate: Nat::from(0),
        rate_decimals: DEFAULT_FEE_RATE_DECIMALS,
    };
    let res = token.set_fee(&other_caller, new_fee.clone(), None, now);
    assert!(res.is_err(), "set_fee should be err");
}

// test token set_fee_to
#[rstest]
fn test_update_token_basic_set_fee_to(
    test_token_with_0_fee_rate: TokenBasic,
    test_owner: Principal,
    other_caller: Principal,
    now: u64,
) {
    let mut token = test_token_with_0_fee_rate.clone();
    let new_fee_to = TokenHolder::new(other_caller.clone(), None);
    // set fee_to by other caller will failed
    let res = token.set_fee_to(&other_caller, new_fee_to.clone(), None, now.clone());
    assert!(res.is_err(), "set_fee_to should be err");
    // set fee_to by owner will ok
    let res = token.set_fee_to(&test_owner, new_fee_to.clone(), None, now);
    assert!(res.is_ok(), "set_fee_to should be ok");
    assert_eq!(token.token_info().fee_to, new_fee_to);
}

#[rstest]
fn test_token_basic_set_logo(
    test_token_with_0_fee_rate: TokenBasic,
    test_owner: Principal,
    new_logo: Vec<u8>,
    now: u64,
) {
    let mut token = test_token_with_0_fee_rate.clone();
    // set logo by other caller will failed
    let res = token.set_logo(&other_caller(), Some(new_logo.clone()), None, now.clone());
    assert!(res.is_err(), "set_logo should be err");
    // set logo by owner will ok
    let res = token.set_logo(&test_owner, Some(new_logo.clone()), None, now);
    assert!(res.is_ok(), "set_logo should be ok");
    assert_eq!(token.logo(), new_logo);
}

#[rstest]
fn test_token_basic_set_desc(test_token_with_0_fee_rate: TokenBasic, test_owner: Principal, now: u64) {
    let mut token = test_token_with_0_fee_rate.clone();
    let new_desc: HashMap<String, String> = vec![(
        "TWITTER".to_owned(),
        "https://twitter.com/DelandLabs".to_owned(),
    )]
        .into_iter()
        .collect();
    // set desc by other caller will failed
    let res = token.set_desc(&other_caller(), new_desc.clone(), None, now.clone());
    assert!(res.is_err(), "set_desc should be err");
    // set desc by owner will ok
    let res = token.set_desc(&test_owner, new_desc.clone(), None, now.clone());
    assert!(res.is_ok(), "set_desc should be ok");
    assert_eq!(token.desc(), new_desc);

    // try to add a new key in desc which is not exist in DESC_KEYS
    let new_desc1: HashMap<String, String> = vec![(
        "TWITTER1".to_owned(),
        "https://twitter.com/DelandLabs1".to_owned(),
    )]
        .into_iter()
        .collect();
    let res = token.set_desc(&test_owner, new_desc1.clone(), None, now);
    // the token's desc will not be changed
    assert!(res.is_ok(), "set_desc should be succeed");
    assert_eq!(token.desc(), new_desc);
}

#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_fee_calculation(
    #[case] test_token: TokenBasic,
    test_owner: Principal,
    test_minter: Principal,
    test_spender: Principal,
    other_caller: Principal,
    now: u64,
) {
    let mut token = test_token.clone();

    let minter_holder = TokenHolder::new(test_minter.clone(), None);
    let spender_holder = TokenHolder::new(test_spender.clone(), None);
    let to_holder = TokenHolder::new(other_caller.clone(), None);
    let fee_to = token.token_info().fee_to.clone();
    let fee = token.fee();
    // mint & approve
    let mint_val = Nat::from(10000);
    let approve_val = Nat::from(1010);
    let _ = token._mint(&test_owner, &minter_holder, mint_val.clone(), None, now.clone());
    let _ = token.approve(
        &test_minter,
        &minter_holder,
        &spender_holder,
        approve_val.clone(),
        None,
        now.clone(),
    );
    // check fee charge
    let approve_fee_charged = fee.minimum.clone();
    let fee_to_balance = token.balance_of(&fee_to);
    assert_eq!(approve_fee_charged, fee_to_balance);
    // check minter_holder balance
    let minter_holder_balance = token.balance_of(&minter_holder);
    assert_eq!(
        minter_holder_balance,
        mint_val.clone() - approve_fee_charged
    );

    // approve again
    let _ = token.approve(
        &test_minter,
        &minter_holder,
        &spender_holder,
        approve_val.clone(),
        None,
        now.clone(),
    );
    // check approve fee charge
    let approve_fee_charged: Nat = fee.clone().minimum.mul(2);
    let fee_to_balance = token.balance_of(&fee_to);
    assert_eq!(approve_fee_charged, fee_to_balance);

    // check minter_holder balance
    let minter_holder_balance = token.balance_of(&minter_holder);
    assert_eq!(
        minter_holder_balance,
        mint_val.clone() - fee.minimum.clone() * 2
    );

    // check spender_holder balance
    let spender_balance = token.balance_of(&spender_holder);
    assert_eq!(spender_balance, 0);

    // transfer from
    let transfer_val = Nat::from(1000);
    let transfer_from_res = token.transfer_from(
        &test_minter,
        &minter_holder,
        &spender_holder,
        &to_holder,
        transfer_val.clone(),
        None,
        now.clone(),
    );
    // check transfer_from_res is Ok
    assert!(
        transfer_from_res.is_ok(),
        "transfer_from should be Ok,{}",
        transfer_from_res.unwrap_err()
    );
    // check transfer_from fee charge
    let transfer_fee_charged = token.balance_of(&fee_to) - approve_fee_charged.clone();
    let rate_fee = transfer_val.clone() * fee.rate.clone() / 10u64.pow(fee.rate_decimals.into());
    let transfer_fee = if rate_fee > fee.minimum {
        rate_fee
    } else {
        fee.minimum.clone()
    };

    assert_eq!(transfer_fee_charged, transfer_fee);
    // check transfer_from result
    assert!(transfer_from_res.is_ok());
    // check from_holder balance
    let from_balance = token.balance_of(&minter_holder);
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
        &test_minter,
        &minter_holder,
        &to_holder,
        transfer_val2.clone(),
        None,
        now + 1,
    );
    // check transfer result
    assert!(transfer_res2.is_ok());
    // check transfer_to fee charged
    let transfer_fee_charged2 =
        token.balance_of(&fee_to) - transfer_fee_charged.clone() - approve_fee_charged.clone();
    let rate_fee2 = transfer_val2.clone() * fee.rate.clone() / 10u64.pow(fee.rate_decimals.into());
    let transfer_fee2 = if rate_fee2 > fee.minimum {
        rate_fee2
    } else {
        fee.minimum
    };
    assert_eq!(transfer_fee_charged2, transfer_fee2);
    // check from_holder balance
    let minter_holder_balance = token.balance_of(&minter_holder);
    assert_eq!(
        minter_holder_balance,
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
#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_approve(
    #[case] test_token: TokenBasic,
    test_owner: Principal,
    test_minter: Principal,
    test_spender: Principal,
) {
    let mut token = test_token.clone();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let now_u64 = now as u64;

    let minter_holder = TokenHolder::new(test_minter.clone(), None);
    let spender_holder = TokenHolder::new(test_spender.clone(), None);

    // mint token to owner_holder
    let mint_val = Nat::from(10000);
    let approve_val = Nat::from(1000);
    let mint_res = token._mint(&test_owner, &minter_holder, mint_val.clone(), None, now_u64.clone());
    // mint_res is ok
    assert!(mint_res.is_ok());
    // check owner_holder balance
    let owner_balance = token.balance_of(&minter_holder);
    assert_eq!(owner_balance, mint_val);
    // approve
    let approve_rs = token.approve(
        &test_minter,
        &minter_holder,
        &spender_holder,
        approve_val.clone(),
        None,
        now_u64.clone(),
    );
    // approve_rs is ok
    assert!(approve_rs.is_ok(), "{:?}", approve_rs.unwrap_err());
    // check allowance
    let allowance = token.allowance(&minter_holder, &spender_holder);
    assert_eq!(allowance, approve_val);
    // approve a new value to spender_holder
    let new_approve_val = Nat::from(2000);
    let new_approve_rs = token.approve(
        &test_minter,
        &minter_holder,
        &spender_holder,
        new_approve_val.clone(),
        None,
        now_u64,
    );
    // new_approve_rs is ok
    assert!(new_approve_rs.is_ok(), "{:?}", new_approve_rs.unwrap_err());
    // check allowance
    let new_allowance = token.allowance(&minter_holder, &spender_holder);
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

#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_transfer_from(
    #[case] test_token: TokenBasic,
    test_owner: Principal,
    test_minter: Principal,
    test_spender: Principal,
    other_caller: Principal,
    now: u64,
) {
    let mut token = test_token.clone();

    let minter_holder = TokenHolder::new(test_minter.clone(), None);
    let spender_holder = TokenHolder::new(test_spender.clone(), None);
    let to_holder = TokenHolder::new(other_caller.clone(), None);
    let fee_to = token.token_info().fee_to.clone();
    // mint & approve
    let mint_val = Nat::from(10000);
    let approve_val = Nat::from(1000);
    let _ = token._mint(&test_owner, &minter_holder, mint_val.clone(), None, now.clone());
    let _ = token.approve(
        &test_minter,
        &minter_holder,
        &spender_holder,
        approve_val.clone(),
        None,
        now.clone(),
    );

    // try transfer_from exceed allowance , should return err
    let transfer_from_val = Nat::from(1001);
    let result = token.transfer_from(
        &test_minter,
        &minter_holder,
        &spender_holder,
        &to_holder,
        transfer_from_val,
        None,
        now.clone() + 1,
    );
    assert!(result.is_err());
    // try transfer_from less than allowance , should return ok
    let transfer_from_val = Nat::from(500);
    let result = token.transfer_from(
        &test_minter,
        &minter_holder,
        &spender_holder,
        &to_holder,
        transfer_from_val.clone(),
        None,
        now + 1,
    );
    assert!(result.is_ok(), "{:?}", result.err().unwrap());
    // check allowance
    let allowance = token.allowance(&minter_holder, &spender_holder);
    let fee = token.fee();
    let approve_fee = fee.clone().minimum;
    let transfer_fee =
        transfer_from_val.clone() * fee.rate.clone() / 10u64.pow(fee.rate_decimals.into());
    let transfer_fee = if transfer_fee > fee.minimum.clone() {
        transfer_fee
    } else {
        fee.minimum.clone()
    };
    assert_eq!(
        allowance,
        approve_val - transfer_from_val.clone() - transfer_fee.clone()
    );
    // check minter_holder balance
    let from_balance = token.balance_of(&minter_holder);
    assert_eq!(
        from_balance,
        mint_val.clone() - transfer_from_val - approve_fee.clone() - transfer_fee.clone()
    );
    // check fee_to balance
    let fee_to_balance = token.balance_of(&fee_to);
    let total_fee = transfer_fee + approve_fee;
    assert_eq!(fee_to_balance, total_fee);
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
#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_transfer(
    #[case] test_token: TokenBasic,
    test_owner: Principal,
    test_minter: Principal,
    other_caller: Principal,
    now: u64,
) {
    let mut token = test_token.clone();
    let minter_holder = TokenHolder::new(test_minter.clone(), None);
    let to_holder = TokenHolder::new(other_caller.clone(), None);
    let fee = token.fee();
    // mint & approve
    let mint_val = Nat::from(10000);
    let _ = token._mint(&test_owner, &minter_holder, mint_val.clone(), None, now.clone());
    // transfer token from from_holder to to_holder
    let transfer_val = Nat::from(1000);
    let transfer_res = token.transfer(
        &test_owner,
        &minter_holder,
        &to_holder,
        transfer_val.clone(),
        None,
        now,
    );

    // transfer_res is ok
    assert!(transfer_res.is_ok(), "{:?}", transfer_res.unwrap_err());
    // check minter_holder balance
    let minter_holder_balance = token.balance_of(&minter_holder);
    let transfer_fee =
        transfer_val.clone() * fee.rate.clone() / 10u64.pow(fee.rate_decimals.into());
    let transfer_fee = if transfer_fee > fee.minimum.clone() {
        transfer_fee
    } else {
        fee.minimum.clone()
    };
    assert_eq!(
        minter_holder_balance,
        mint_val.clone() - transfer_val.clone() - transfer_fee
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
#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_mint_burn(
    #[case] test_token: TokenBasic,
    test_owner: Principal,
    test_minter: Principal,
    now: u64,
) {
    let mut token = test_token.clone();
    let minter_holder = TokenHolder::new(test_minter, None);

    // mint token to from_holder
    let mint_val = Nat::from(10000);
    // mint with wrong nonce should fail
    let _mint_res = token._mint(&test_owner, &minter_holder, mint_val.clone(), Some(2), now.clone());
    assert!(_mint_res.is_err());
    //check error message should be NonceNotMatch
    assert_eq!(
        _mint_res.err().unwrap().to_string(),
        DFTError::NonceNotMatch.to_string()
    );
    let _mint_res = token._mint(&test_owner, &minter_holder, mint_val.clone(), Some(1), now.clone());
    // check mint_res is ok, and check minter_holder balance
    assert!(_mint_res.is_ok(), "{:?}", _mint_res.unwrap_err());
    let owner_balance = token.balance_of(&minter_holder);
    assert_eq!(owner_balance, mint_val);
    // check test_owner nonce
    let nonce = token.nonce_of(&test_owner);
    assert_eq!(nonce, 1u64);

    // check total supply
    let total_supply = token.total_supply();
    assert_eq!(total_supply, mint_val);

    let burn_val = Nat::from(1000);
    let burn_res = token._burn(&test_owner, &minter_holder, &minter_holder, burn_val.clone(), 2, now.clone());

    // check burn_res is ok, and check minter_holder balance
    assert!(burn_res.is_ok(), "{:?}", burn_res.unwrap_err());
    let owner_balance = token.balance_of(&minter_holder);
    assert_eq!(owner_balance, mint_val.clone() - burn_val.clone());

    // check total supply
    let total_supply = token.total_supply();
    assert_eq!(total_supply, mint_val - burn_val);

    // burn value less than minimum fee will fail
    let burn_val = Nat::from(1);
    let burn_res = token._burn(&test_owner, &minter_holder, &minter_holder, burn_val.clone(), 3, now.clone());
    assert!(burn_res.is_err());
}

// test token approve/transfer_from/transfer anonymous call should fail
#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_approve_transfer_from_transfer(
    #[case] test_token: TokenBasic,
    test_minter: Principal,
    test_spender: Principal,
    other_caller: Principal,
    now: u64,
) {
    let mut token = test_token.clone();
    let minter_holder = TokenHolder::new(test_minter.clone(), None);
    let spender_holder = TokenHolder::new(test_spender.clone(), None);
    let to_holder = TokenHolder::new(other_caller.clone(), None);
    let anonymous_caller = Principal::anonymous();

    // approve with anonymous should fail
    let approve_val = Nat::from(1000);
    let approve_res = token.approve(
        &anonymous_caller,
        &minter_holder,
        &spender_holder,
        approve_val.clone(),
        None,
        now.clone(),
    );
    // check error message is DFTError::Unauthorized
    assert_eq!(
        approve_res.unwrap_err().to_string(),
        DFTError::NotAllowAnonymous.to_string()
    );

    // transfer_from with anonymous should fail
    let transfer_from_val = Nat::from(1000);
    let transfer_from_res = token.transfer_from(
        &anonymous_caller,
        &minter_holder,
        &spender_holder,
        &to_holder,
        transfer_from_val.clone(),
        None,
        now.clone(),
    );
    assert_eq!(
        transfer_from_res.unwrap_err().to_string(),
        DFTError::NotAllowAnonymous.to_string()
    );
    // transfer with anonymous should fail
    let transfer_val = Nat::from(1000);
    let transfer_res = token.transfer(
        &anonymous_caller,
        &minter_holder,
        &spender_holder,
        transfer_val.clone(),
        None,
        now,
    );
    assert_eq!(
        transfer_res.unwrap_err().to_string(),
        DFTError::NotAllowAnonymous.to_string()
    );
}

// test token approve/transfer_from/transfer with wrong nonce will fail
#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_approve_transfer_from_transfer_with_wrong_nonce(
    #[case] test_token: TokenBasic,
    test_owner: Principal,
    test_minter: Principal,
    test_spender: Principal,
    other_caller: Principal,
    now: u64,
) {
    let mut token = test_token.clone();
    let minter_holder = TokenHolder::new(test_minter.clone(), None);
    let spender_holder = TokenHolder::new(test_spender.clone(), None);
    let to_holder = TokenHolder::new(other_caller.clone(), None);

    // approve with wrong nonce should fail
    let approve_val = Nat::from(1000);
    let approve_res = token.approve(
        &test_owner,
        &minter_holder,
        &spender_holder,
        approve_val.clone(),
        Some(2),
        now.clone(),
    );
    assert_eq!(
        approve_res.unwrap_err().to_string(),
        DFTError::NonceNotMatch.to_string()
    );

    // transfer_from with wrong nonce should fail
    let transfer_from_val = Nat::from(1000);
    let transfer_from_res = token.transfer_from(
        &test_owner,
        &minter_holder,
        &spender_holder,
        &to_holder,
        transfer_from_val.clone(),
        Some(2),
        now.clone(),
    );
    assert_eq!(
        transfer_from_res.unwrap_err().to_string(),
        DFTError::NonceNotMatch.to_string()
    );
    // transfer with wrong nonce should fail
    let transfer_val = Nat::from(1000);
    let transfer_res = token.transfer(
        &test_owner,
        &minter_holder,
        &spender_holder,
        transfer_val.clone(),
        Some(2),
        now,
    );
    assert_eq!(
        transfer_res.unwrap_err().to_string(),
        DFTError::NonceNotMatch.to_string()
    );
}
