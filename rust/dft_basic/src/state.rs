extern crate dft_types;
extern crate dft_utils;
use dft_standard::token::TokenBasic;
use dft_types::*;
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::cell::RefCell;

thread_local! {
    pub static TOKEN: std::cell::RefCell<TokenBasic>  = RefCell::new(TokenBasic::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    TOKEN.with(|token| {
        let token = token.borrow();
        storage::stable_save((token.to_token_payload(),)).unwrap();
    })
}

#[post_upgrade]
fn post_upgrade() {
    // There can only be one value in stable memory, currently. otherwise, lifetime error.
    // https://docs.rs/ic-cdk/0.3.0/ic_cdk/storage/fn.stable_restore.html
    let (payload,): (TokenPayload,) = storage::stable_restore().unwrap();
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.load_from_token_payload(payload);
    })
}
