use crate::token::*;
use ic_cdk::api::{
    self,
    stable::{stable_bytes, StableWriter},
};
use ic_cdk_macros::*;
use std::{cell::RefCell};

thread_local! {
    pub static TOKEN: std::cell::RefCell<TokenBasic>  = RefCell::new(TokenBasic::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    TOKEN.with(|token| {
        let token = token.borrow();
        let token_bytes = candid::encode_one(&*token).unwrap();
        let bytes_array = token_bytes.as_slice();

        match StableWriter::default().write(bytes_array) {
            Ok(size) => {
                api::print(&format!("after pre_upgrade stable_write size{}", size));
            }
            Err(_) => {
                api::print("stable_write error");
            }
        }
    })
}

#[post_upgrade]
fn post_upgrade() {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let token_bytes = stable_bytes();
        *token = candid::decode_one(&*token_bytes).expect("decode_one failed");
    })
}