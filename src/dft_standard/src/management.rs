use crate::state::TOKEN;
use crate::token::TokenStandard;
use candid::candid_method;
use dft_types::*;
use ic_cdk::{api, export::Principal};
use ic_cdk_macros::*;
use std::{string::String};

#[update(name = "setOwner")]
#[candid_method(update, rename = "setOwner")]
fn set_owner(owner: Principal, nonce: Option<u64>) -> BooleanResult {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token
            .set_owner(&api::caller(), owner, nonce, api::time())
            .into()
    })
}

#[update(name = "setLogo")]
#[candid_method(update, rename = "setLogo")]
fn set_logo(logo: Option<Vec<u8>>, nonce: Option<u64>) -> BooleanResult {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.set_logo(&api::caller(), logo,nonce,api::time()).into()
    })
}

#[update(name = "setDesc")]
#[candid_method(update, rename = "setDesc")]
fn set_desc_info(desc_data: Vec<(String, String)>, nonce: Option<u64>) -> BooleanResult {
    // convert desc data to hashmap
    let mut desc_info = std::collections::HashMap::new();
    for (key, value) in desc_data {
        desc_info.insert(key, value);
    }
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.set_desc(&api::caller(), desc_info,nonce,api::time()).into()
    })
}

#[update(name = "setFee")]
#[candid_method(update, rename = "setFee")]
fn set_fee(fee: TokenFee, nonce: Option<u64>) -> BooleanResult {
    let caller = api::caller();
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.set_fee(&caller, fee, nonce, api::time()).into()
    })
}

#[update(name = "setFeeTo")]
#[candid_method(update, rename = "setFeeTo")]
fn set_fee_to(fee_to: String, nonce: Option<u64>) -> BooleanResult {
    match fee_to.parse::<TokenHolder>() {
        Ok(holder) => TOKEN.with(|token| {
            let mut token = token.borrow_mut();
            token
                .set_fee_to(&api::caller(), holder.clone(), nonce, api::time())
                .into()
        }),
        Err(_) => BooleanResult::Err(DFTError::InvalidArgFormatFeeTo.into()),
    }
}