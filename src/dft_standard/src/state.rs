use crate::token::*;
use ic_cdk::api::stable::{stable_bytes, StableWriter};
use ic_cdk_macros::*;
use log::{error, info};
use std::cell::RefCell;

thread_local! {
    pub static TOKEN: std::cell::RefCell<TokenBasic>  = RefCell::new(TokenBasic::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    TOKEN.with(|token| {
        let token = token.borrow();
        let token_bytes = bincode::serialize(&*token).unwrap();

        match StableWriter::default().write(token_bytes.as_slice()) {
            Ok(size) => {
                info!("after pre_upgrade stable_write size{}", size);
            }
            Err(_) => {
                error!("{}", "stable_write error");
            }
        }
    })
}

#[post_upgrade]
fn post_upgrade() {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let token_bytes = stable_bytes();
        *token = bincode::deserialize(&*token_bytes).expect("deserialize failed");
    })
}
