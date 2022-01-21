extern crate dft_types;
extern crate dft_utils;

use crate::storage::*;
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::cell::RefCell;

thread_local! {
    pub static STORAGE: std::cell::RefCell<AutoScalingStorage>  = RefCell::new(AutoScalingStorage::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    STORAGE.with(|data| {
        let data = data.borrow();
        storage::stable_save((data.clone(), )).unwrap();
    })
}

#[post_upgrade]
fn post_upgrade() {
    // There can only be one value in stable memory, currently. otherwise, lifetime error.
    // https://docs.rs/ic-cdk/0.3.0/ic_cdk/storage/fn.stable_restore.html
    let (payload, ): (AutoScalingStorage, ) = storage::stable_restore().unwrap();
    STORAGE.with(|data| {
        let mut data = data.borrow_mut();
        data.restore(payload);
    })
}
