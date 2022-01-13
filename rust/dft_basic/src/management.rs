extern crate dft_types;
extern crate dft_utils;
use crate::state::TOKEN;
use candid::candid_method;
use dft_standard::token::TokenStandard;
use dft_types::*;
use ic_cdk::{api, export::Principal};
use ic_cdk_macros::*;
use std::{collections::HashMap, string::String};

#[update(name = "setOwner")]
#[candid_method(update, rename = "setOwner")]
fn set_owner(owner: Principal) -> ActorResult<bool> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.set_owner(&api::caller(), owner)?;
        Ok(true)
    })
}

#[update(name = "setLogo")]
#[candid_method(update, rename = "setLogo")]
fn set_logo(logo: Vec<u8>) -> ActorResult<bool> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        to_actor_result(token.set_logo(&api::caller(), logo))
    })
}

#[update(name = "setDesc")]
#[candid_method(update, rename = "setDesc")]
fn set_desc_info(desc_data: Vec<(String, String)>) -> ActorResult<bool> {
    // convert desc data to hashmap
    let mut desc_info = HashMap::new();
    for (key, value) in desc_data {
        desc_info.insert(key, value);
    }
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        to_actor_result(token.set_desc(&api::caller(), desc_info))
    })
}

#[update(name = "setFee")]
#[candid_method(update, rename = "setFee")]
fn set_fee(fee: Fee) -> ActorResult<bool> {
    let caller = api::caller();
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        to_actor_result(token.set_fee(&caller, fee))
    })
}

#[query(name = "setFeeTo")]
#[candid_method(update, rename = "setFeeTo")]
fn set_fee_to(fee_to: String) -> ActorResult<bool> {
    match fee_to.parse::<TokenReceiver>() {
        Ok(holder) => TOKEN.with(|token| {
            let mut token = token.borrow_mut();
            to_actor_result(token.set_fee_to(&api::caller(), holder))
        }),
        Err(_) => Err(DFTError::InvalidArgFormatFeeTo.into()),
    }
}
