use crate::token::*;
use ic_cdk::api::stable::{stable64_write, stable_bytes};
use ic_cdk_macros::*;
use std::cell::RefCell;

thread_local! {
    pub static TOKEN: std::cell::RefCell<TokenBasic>  = RefCell::new(TokenBasic::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    TOKEN.with(|token| {
        let token = token.borrow();
        let token_bytes = candid::encode_one(&*token).unwrap();
        stable64_write(0, &token_bytes);
    })
}

#[post_upgrade]
fn post_upgrade() {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let token_bytes = stable_bytes();
        *token = candid::decode_one(&token_bytes).unwrap();
    })
}
