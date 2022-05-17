use candid::{Principal};
use dft_basic::service::{basic_service, management_service};
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

// spender
#[fixture]
fn test_fee_to() -> Principal {
    Principal::from_text("7b6mv-nyoey-gkj2b-2r6mp-fa2rr-6ktwc-qrx7e-l3eax-32jd7-ahwnj-3qe").unwrap()
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
fn test_fee_0_rate() -> TokenFee {
    TokenFee {
        minimum: 2u32.into(),
        rate: 0,
        rate_decimals: DEFAULT_FEE_RATE_DECIMALS,
    }
}

// test fee non 0 rate
#[fixture]
fn test_fee_non_0_rate() -> TokenFee {
    TokenFee {
        minimum: 2u32.into(),
        rate: 1,
        rate_decimals: 2,
    }
}

#[fixture]
fn test_token_with_0_fee_rate() {
    let fee_to = TokenHolder::new(test_fee_to(), None);
    basic_service::token_initialize(
        &test_owner(),
        test_token_id(),
        Some(test_logo()),
        test_name(),
        test_symbol(),
        test_decimals(),
        test_fee_0_rate(),
        fee_to,
        None,
    );
}

#[fixture]
fn test_token_with_non_0_fee_rate() {
    let fee_to = TokenHolder::new(test_fee_to(), None);
    basic_service::token_initialize(
        &test_owner(),
        test_token_id(),
        Some(test_logo()),
        test_name(),
        test_symbol(),
        test_decimals(),
        test_fee_non_0_rate(),
        fee_to,
        None,
    );
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
    // check token id is Principal::anonymous()
    assert_eq!(basic_service::token_id(), Principal::anonymous());
    // check owner is Principal::anonymous()
    assert_eq!(basic_service::owner(), Principal::anonymous());
    // check token's name is empty
    assert_eq!(basic_service::name(), "");
    // check token's symbol is empty
    assert_eq!(basic_service::metadata().symbol(), "");
    // check token's decimals is 0
    assert_eq!(basic_service::metadata().decimals(), &0);
    // check token's total supply is 0
    assert_eq!(basic_service::total_supply(), TokenAmount::default());
    // check token's logo is empty
    let null_logo: Vec<u8> = vec![];
    assert_eq!(basic_service::logo().clone().unwrap_or(vec![]), null_logo);
    // check token's fee is 0
    let fee = basic_service::fee();
    assert_eq!(fee.minimum, TokenAmount::default());
    assert_eq!(fee.rate, 0);
    // check desc is empty
    let empty_map: HashMap<String, String> = HashMap::new();
    assert_eq!(basic_service::desc(), empty_map);
}

#[rstest]
#[should_panic]
fn test_token_basic_logo_invalid_image(
    test_owner: Principal,
    test_token_id: Principal,
    test_name: String,
    test_symbol: String,
    test_decimals: u8,
    test_fee_0_rate: TokenFee,
) {
    let fee_to = TokenHolder::new(test_owner.clone(), None);

    basic_service::token_initialize(
        &test_owner,
        test_token_id,
        Some(vec![0u8; 20]),
        test_name.clone(),
        test_symbol,
        test_decimals,
        test_fee_0_rate,
        fee_to,
        None,
    );
    // will panic if logo is a unsupported image type
    assert_eq!(basic_service::name(), test_name);
}

#[rstest]
fn test_token_basic_initialize_all_parameters() {
    test_token_with_0_fee_rate();
    assert_eq!(basic_service::token_id(), test_token_id());
    assert_eq!(basic_service::owner(), test_owner());
    assert_eq!(basic_service::name(), test_name());
    assert_eq!(basic_service::symbol(), test_symbol());
    assert_eq!(basic_service::decimals(), test_decimals());
    assert_eq!(basic_service::total_supply(), TokenAmount::default());
    assert_eq!(basic_service::logo().clone().unwrap_or(vec![]), test_logo());
    assert_eq!(basic_service::fee(), test_fee_0_rate());
}

//test token set_fee
#[rstest]
fn test_token_basic_set_fee(test_owner: Principal, now: u64) {
    test_token_with_0_fee_rate();
    let new_fee = TokenFee {
        minimum: TokenAmount::from(2u32),
        rate: 0,
        rate_decimals: DEFAULT_FEE_RATE_DECIMALS,
    };
    let res = management_service::set_fee(&test_owner, new_fee.clone(), None, now);
    assert!(res.is_ok(), "set_fee should be ok");
    assert_eq!(basic_service::fee(), new_fee);
}

//test token set_fee with invalid owner
#[rstest]
fn test_token_basic_set_fee_invalid_owner(other_caller: Principal, now: u64) {
    test_token_with_0_fee_rate();
    let new_fee = TokenFee {
        minimum: TokenAmount::from(2u32),
        rate: 0,
        rate_decimals: DEFAULT_FEE_RATE_DECIMALS,
    };
    let res = management_service::set_fee(&other_caller, new_fee.clone(), None, now);
    assert!(res.is_err(), "set_fee should be err");
}

// test token set_fee_to
#[rstest]
fn test_update_token_basic_set_fee_to(test_owner: Principal, other_caller: Principal, now: u64) {
    test_token_with_0_fee_rate();
    let new_fee_to = TokenHolder::new(other_caller.clone(), None);
    // set fee_to by other caller will failed
    let res = management_service::set_fee_to(&other_caller, new_fee_to.clone(), None, now.clone());
    assert!(res.is_err(), "set_fee_to should be err");
    // set fee_to by owner will ok
    let res = management_service::set_fee_to(&test_owner, new_fee_to.clone(), None, now);
    assert!(res.is_ok(), "set_fee_to should be ok");
    assert_eq!(basic_service::token_info().fee_to, new_fee_to);
}

#[rstest]
fn test_token_basic_set_logo(test_owner: Principal, new_logo: Vec<u8>) {
    test_token_with_0_fee_rate();
    // set logo by other caller will failed
    let res = management_service::set_logo(&other_caller(), Some(new_logo.clone()));
    assert!(res.is_err(), "set_logo should be err");
    // set logo by owner will ok
    let res = management_service::set_logo(&test_owner, Some(new_logo.clone()));
    assert!(res.is_ok(), "set_logo should be ok");
    assert_eq!(basic_service::logo().clone().unwrap_or(vec![]), new_logo);
}

#[rstest]
fn test_token_basic_set_desc(test_owner: Principal) {
    test_token_with_0_fee_rate();
    let new_desc: HashMap<String, String> = vec![(
        "TWITTER".to_owned(),
        "https://twitter.com/DelandLabs".to_owned(),
    )]
        .into_iter()
        .collect();
    // set desc by other caller will failed
    let res = management_service::set_desc(&other_caller(), new_desc.clone());
    assert!(res.is_err(), "set_desc should be err");
    // set desc by owner will ok
    let res = management_service::set_desc(&test_owner, new_desc.clone());
    assert!(res.is_ok(), "set_desc should be ok");
    assert_eq!(basic_service::desc(), new_desc);

    // try to add a new key in desc which is not exist in DESC_KEYS
    let new_desc1: HashMap<String, String> = vec![(
        "TWITTER1".to_owned(),
        "https://twitter.com/DelandLabs1".to_owned(),
    )]
        .into_iter()
        .collect();
    let res = management_service::set_desc(&test_owner, new_desc1.clone());
    // the token's desc will not be changed
    assert!(res.is_ok(), "set_desc should be succeed");
    assert_eq!(basic_service::desc(), new_desc);
}

#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_fee_calculation(
    #[case] test_token: (),
    test_owner: Principal,
    test_minter: Principal,
    test_spender: Principal,
    other_caller: Principal,
    now: u64,
) {
    let minter_holder = TokenHolder::new(test_minter.clone(), None);
    let spender_holder = TokenHolder::new(test_spender.clone(), None);
    let to_holder = TokenHolder::new(other_caller.clone(), None);
    let fee_to = basic_service::token_info().fee_to.clone();
    let fee = basic_service::metadata().fee().clone();
    // mint & approve
    let mint_val = TokenAmount::from(10000u32);
    let approve_val = TokenAmount::from(1010u32);
    let _ = dft_mintable::add_minter(&test_owner, test_owner, None, now);
    let _ = dft_mintable::mint(
        &test_owner,
        &minter_holder,
        mint_val.clone(),
        None,
        now.clone(),
    );
    let _ = basic_service::approve(
        &test_minter,
        &minter_holder,
        &spender_holder,
        approve_val.clone(),
        None,
        now.clone(),
    );
    // check fee charge
    let approve_fee_charged = fee.minimum.clone();
    let fee_to_balance = basic_service::balance_of(&fee_to);
    assert_eq!(approve_fee_charged, fee_to_balance);
    // check minter_holder balance
    let minter_holder_balance = basic_service::balance_of(&minter_holder);
    assert_eq!(
        minter_holder_balance,
        mint_val.clone() - approve_fee_charged
    );

    let approve_val = TokenAmount::from(1020u32);
    // approve again
    let _ = basic_service::approve(
        &test_minter,
        &minter_holder,
        &spender_holder,
        approve_val.clone(),
        None,
        now.clone(),
    );
    // check approve fee charge
    let approve_fee_charged: TokenAmount = fee.clone().minimum.mul(2u32);
    let fee_to_balance = basic_service::balance_of(&fee_to);
    assert_eq!(approve_fee_charged, fee_to_balance);

    // check minter_holder balance
    let minter_holder_balance = basic_service::balance_of(&minter_holder);
    assert_eq!(
        minter_holder_balance,
        mint_val.clone() - fee.minimum.clone() * 2u32
    );

    // check spender_holder balance
    let spender_balance = basic_service::balance_of(&spender_holder);
    assert_eq!(spender_balance, TokenAmount::from(0u32));

    // transfer from
    let transfer_val = TokenAmount::from(1000u32);
    let transfer_from_res = basic_service::transfer_from(
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
    let transfer_fee_charged = basic_service::balance_of(&fee_to) - approve_fee_charged.clone();
    let rate_fee = transfer_val.clone() * fee.rate.clone() / 10u128.pow(fee.rate_decimals.into());
    let transfer_fee = if rate_fee > fee.minimum {
        rate_fee
    } else {
        fee.minimum.clone()
    };

    assert_eq!(transfer_fee_charged, transfer_fee);
    // check transfer_from result
    assert!(transfer_from_res.is_ok());
    // check from_holder balance
    let from_balance = basic_service::balance_of(&minter_holder);
    assert_eq!(
        from_balance,
        mint_val.clone()
            - approve_fee_charged.clone()
            - transfer_val.clone()
            - transfer_fee.clone()
    );
    // check spender_holder balance
    let spender_balance = basic_service::balance_of(&spender_holder);
    assert_eq!(spender_balance, TokenAmount::from(0u32));

    // check to_holder balance
    let to_balance = basic_service::balance_of(&to_holder);
    assert_eq!(to_balance, transfer_val);

    // transfer
    let transfer_val2 = TokenAmount::from(2000u32);
    let transfer_res2 = basic_service::transfer(
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
    let transfer_fee_charged2 = basic_service::balance_of(&fee_to)
        - transfer_fee_charged.clone()
        - approve_fee_charged.clone();
    let rate_fee2 = transfer_val2.clone() * fee.rate.clone() / 10u128.pow(fee.rate_decimals.into());
    let transfer_fee2 = if rate_fee2 > fee.minimum {
        rate_fee2
    } else {
        fee.minimum
    };
    assert_eq!(transfer_fee_charged2, transfer_fee2);
    // check from_holder balance
    let minter_holder_balance = basic_service::balance_of(&minter_holder);
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
    let total_supply = basic_service::total_supply();
    assert_eq!(total_supply, mint_val.clone());
}

//test token approve
#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_approve(
    #[case] test_token: (),
    test_owner: Principal,
    test_minter: Principal,
    test_spender: Principal,
    other_caller: Principal,
    now: u64,
) {
    let minter_holder = TokenHolder::new(test_minter.clone(), None);
    let spender_holder = TokenHolder::new(test_spender.clone(), None);
    let other_holder = TokenHolder::new(other_caller.clone(), None);

    // mint token to owner_holder
    let mint_val = TokenAmount::from(10000u32);
    let approve_val = TokenAmount::from(1000u32);
    let _ = dft_mintable::add_minter(&test_owner, test_owner, None, now);
    let mint_res = dft_mintable::mint(&test_owner, &minter_holder, mint_val.clone(), None, now);
    // mint_res is ok
    assert!(mint_res.is_ok());
    // check owner_holder balance
    let owner_balance = basic_service::balance_of(&minter_holder);
    assert_eq!(owner_balance, mint_val);
    // approve
    let approve_rs = basic_service::approve(
        &test_minter,
        &minter_holder,
        &spender_holder,
        approve_val.clone(),
        None,
        now,
    );
    // approve_rs is ok
    assert!(approve_rs.is_ok(), "{:?}", approve_rs.unwrap_err());
    // check allowance
    let allowance = basic_service::allowance(&minter_holder, &spender_holder);
    assert_eq!(allowance, approve_val);
    // approve a new value to spender_holder
    let new_approve_val = TokenAmount::from(2000u32);
    let new_approve_rs = basic_service::approve(
        &test_minter,
        &minter_holder,
        &spender_holder,
        new_approve_val.clone(),
        None,
        now,
    );
    // new_approve_rs is ok
    assert!(new_approve_rs.is_ok(), "{:?}", new_approve_rs.unwrap_err());
    // check allowance
    let new_allowance = basic_service::allowance(&minter_holder, &spender_holder);
    let allowance_size = basic_service::token_info().allowance_size;
    assert_eq!(
        new_allowance, new_approve_val,
        "allowance size: {}",
        allowance_size
    );
    assert_eq!(
        basic_service::token_info().block_height,
        candid::Nat::from(4u32)
    );
    // check total supply
    let total_supply = basic_service::total_supply();
    assert_eq!(total_supply, mint_val.clone());

    // approve
    let approve_rs = basic_service::approve(
        &test_minter,
        &minter_holder,
        &other_holder,
        approve_val.clone(),
        None,
        now,
    );

    let allowances = basic_service::allowances_of(&minter_holder);
    assert_eq!(allowances.len(), 2);

    for allowance in allowances.clone() {
        if allowance.0.to_hex() == spender_holder.clone().to_hex() {
            assert_eq!(allowance.1, new_approve_val, "{:?}", allowances.clone());
        } else {
            assert_eq!(allowance.1, approve_val, "{:?}", allowances.clone());
        }
    }
}

#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_transfer_from(
    #[case] test_token: (),
    test_owner: Principal,
    test_minter: Principal,
    test_spender: Principal,
    other_caller: Principal,
    now: u64,
) {
    let minter_holder = TokenHolder::new(test_minter.clone(), None);
    let spender_holder = TokenHolder::new(test_spender.clone(), None);
    let to_holder = TokenHolder::new(other_caller.clone(), None);
    let fee_to = basic_service::token_info().fee_to.clone();
    // mint & approve
    let mint_val = TokenAmount::from(10000u32);
    let approve_val = TokenAmount::from(1000u32);
    let _ = dft_mintable::add_minter(&test_owner, test_owner, None, now);
    let _ = dft_mintable::mint(
        &test_owner,
        &minter_holder,
        mint_val.clone(),
        None,
        now.clone(),
    );
    let _ = basic_service::approve(
        &test_minter,
        &minter_holder,
        &spender_holder,
        approve_val.clone(),
        None,
        now.clone(),
    );

    // try transfer_from exceed allowance , should return err
    let transfer_from_val = TokenAmount::from(1001u32);
    let result = basic_service::transfer_from(
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
    let transfer_from_val = TokenAmount::from(500u32);
    let result = basic_service::transfer_from(
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
    let allowance = basic_service::allowance(&minter_holder, &spender_holder);
    let fee = basic_service::fee();
    let approve_fee = fee.clone().minimum;
    let transfer_fee =
        transfer_from_val.clone() * fee.rate.clone() / 10u128.pow(fee.rate_decimals.into());
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
    let from_balance = basic_service::balance_of(&minter_holder);
    assert_eq!(
        from_balance,
        mint_val.clone() - transfer_from_val - approve_fee.clone() - transfer_fee.clone()
    );
    // check fee_to balance
    let fee_to_balance = basic_service::balance_of(&fee_to);
    let total_fee = transfer_fee + approve_fee;
    assert_eq!(fee_to_balance, total_fee);
    assert_eq!(
        basic_service::token_info().block_height,
        candid::Nat::from(4u32)
    );
    // check total supply
    let total_supply = basic_service::total_supply();
    assert_eq!(total_supply, mint_val);
}

// test token transfer
#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_transfer(
    #[case] test_token: (),
    test_owner: Principal,
    test_minter: Principal,
    other_caller: Principal,
    now: u64,
) {
    let minter_holder = TokenHolder::new(test_minter.clone(), None);
    let to_holder = TokenHolder::new(other_caller.clone(), None);
    let fee = basic_service::metadata().fee().clone();
    // mint & approve
    let mint_val = TokenAmount::from(10000u32);
    let _ = dft_mintable::add_minter(&test_owner, test_owner, None, now);
    let _ = dft_mintable::mint(
        &test_owner,
        &minter_holder,
        mint_val.clone(),
        None,
        now.clone(),
    );
    // transfer token from from_holder to to_holder
    let transfer_val = TokenAmount::from(1000u32);
    let transfer_res = basic_service::transfer(
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
    let minter_holder_balance = basic_service::balance_of(&minter_holder);
    let transfer_fee =
        transfer_val.clone() * fee.rate.clone() / 10u128.pow(fee.rate_decimals.into());
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
    let to_balance = basic_service::balance_of(&to_holder);
    assert_eq!(to_balance, transfer_val);
    assert_eq!(
        basic_service::token_info().block_height,
        candid::Nat::from(3u32)
    );
    // check total supply
    let total_supply = basic_service::total_supply();
    assert_eq!(total_supply, mint_val);
}

// test token _mint/_burn
#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_mint_burn(
    #[case] test_token: (),
    test_owner: Principal,
    test_minter: Principal,
    other_caller: Principal,
    now: u64,
) {
    let minter_holder = TokenHolder::new(test_minter, None);

    // mint token to from_holder
    let mint_val = TokenAmount::from(20000u32);
    // mint with not minter will fail
    let _mint_res = dft_mintable::mint(
        &test_owner,
        &minter_holder,
        mint_val.clone(),
        Some(2),
        now.clone(),
    );
    let _ = dft_mintable::add_minter(&test_owner, test_owner, None, now);
    // mint with wrong created at should fail
    let _mint_res = dft_mintable::mint(
        &test_owner,
        &minter_holder,
        mint_val.clone(),
        Some(2),
        now.clone(),
    );
    assert!(_mint_res.is_err());
    //check error message should be TxTooOld
    assert_eq!(
        _mint_res.err().unwrap().to_string(),
        DFTError::TxTooOld.to_string()
    );
    let _mint_res = dft_mintable::mint(
        &test_owner,
        &minter_holder,
        mint_val.clone(),
        Some(now),
        now.clone(),
    );
    // check mint_res is ok, and check minter_holder balance
    assert!(_mint_res.is_ok(), "{:?}", _mint_res.unwrap_err());
    let owner_balance = basic_service::balance_of(&minter_holder);
    assert_eq!(owner_balance, mint_val);

    // check total supply
    let total_supply = basic_service::total_supply();
    assert_eq!(total_supply, mint_val);

    let burn_val = TokenAmount::from(1000u32);
    let burn_res = dft_burnable::burn(
        &test_minter,
        &minter_holder,
        burn_val.clone(),
        None,
        now.clone(),
    );

    // check burn_res is ok, and check minter_holder balance
    assert!(burn_res.is_ok(), "{:?}", burn_res.unwrap_err());
    let owner_balance = basic_service::balance_of(&minter_holder);
    assert_eq!(owner_balance, mint_val.clone() - burn_val.clone());

    // burn from
    let burn_from_val = TokenAmount::from(1000u32);
    let spender = TokenHolder::new(other_caller, None);
    let approve_res = basic_service::approve(
        &test_minter,
        &minter_holder,
        &spender,
        burn_from_val.clone() * 2u32,
        None,
        now.clone(),
    );

    assert_eq!(approve_res.is_ok(), true);

    let burn_res = dft_burnable::burn_from(
        &other_caller,
        &minter_holder,
        &spender,
        burn_from_val.clone(),
        None,
        now.clone(),
    );

    assert_eq!(burn_res.is_ok(), true);
    // check total supply
    let total_supply = basic_service::total_supply();
    assert_eq!(total_supply, mint_val - burn_val - burn_from_val.clone());


    let burn_res = dft_burnable::burn_from(
        &other_caller,
        &minter_holder,
        &spender,
        1u32.into(),
        None,
        now.clone() + 1u64,
    );

    assert_eq!(burn_res, Err(DFTError::BurnValueTooSmall));

    // burn value less than minimum fee will fail
    let burn_val = TokenAmount::from(1u32);
    let burn_res = dft_burnable::burn(
        &test_minter,
        &minter_holder,
        burn_val.clone(),
        None,
        now.clone(),
    );
    assert_eq!(burn_res, Err(DFTError::BurnValueTooSmall));

    let token_metrics = basic_service::token_metrics();

    assert_eq!(token_metrics.allowance_size, 1);
    assert_eq!(token_metrics.total_block_count, 5);
    assert_eq!(token_metrics.local_block_count, 5);
    assert_eq!(token_metrics.holders, 2);
}

// test token approve/transfer_from/transfer anonymous call should fail
#[rstest]
#[case(test_token_with_0_fee_rate())]
#[case(test_token_with_non_0_fee_rate())]
fn test_token_basic_approve_transfer_from_transfer(
    #[case] test_token: (),
    test_minter: Principal,
    test_spender: Principal,
    other_caller: Principal,
    now: u64,
) {
    let minter_holder = TokenHolder::new(test_minter.clone(), None);
    let spender_holder = TokenHolder::new(test_spender.clone(), None);
    let to_holder = TokenHolder::new(other_caller.clone(), None);
    let anonymous_caller = Principal::anonymous();

    // approve with anonymous should fail
    let approve_val = TokenAmount::from(1000u32);
    let approve_res = basic_service::approve(
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
    let transfer_from_val = TokenAmount::from(1000u32);
    let transfer_from_res = basic_service::transfer_from(
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
    let transfer_val = TokenAmount::from(1000u32);
    let transfer_res = basic_service::transfer(
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

#[rstest]
fn test_token_basic_minters_add_remove(test_owner: Principal, test_minter: Principal, now: u64) {
    test_token_with_0_fee_rate();
    let res = dft_mintable::add_minter(&test_owner, test_owner, None, now);
    assert!(res.is_ok());
    let minters = dft_mintable::minters();
    assert_eq!(minters.len(), 1);
    assert_eq!(minters[0], test_owner);
    let res = dft_mintable::add_minter(&test_owner, test_minter, None, now);
    assert!(res.is_ok());
    let minters = dft_mintable::minters();
    assert_eq!(minters.len(), 2);
    assert_eq!(minters[0], test_owner);
    assert_eq!(minters[1], test_minter);
    let res = dft_mintable::remove_minter(&test_owner, test_minter, None, now);
    assert!(res.is_ok());
    let minters = dft_mintable::minters();

    assert_eq!(minters.len(), 1);
    assert_eq!(minters[0], test_owner);
    let res = dft_mintable::remove_minter(&test_owner, test_owner, None, now);
    assert!(res.is_ok());
    let minters = dft_mintable::minters();
    assert_eq!(minters.len(), 0);
}