use candid::{candid_method, Nat};
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::*;
use std::sync::Once;
use dft_utils::ic_logger::ICLogger;
use log::debug;
use std::cell::RefCell;

thread_local! {
    pub static NOTIFICATIONS_RECEIVED: RefCell<u64>  = RefCell::new(0u64);
}

static INIT: Once = Once::new();

#[cfg_attr(coverage_nightly, no_coverage)]
pub(crate) fn canister_module_init() {
    INIT.call_once(|| {
        ICLogger::init();
    });
}

#[cfg_attr(coverage_nightly, no_coverage)]
#[init]
#[candid_method(init)]
async fn canister_init() {
    canister_module_init();
}

#[cfg_attr(coverage_nightly, no_coverage)]
#[update(name = "onTokenReceived")]
#[candid_method(update, rename = "onTokenReceived")]
async fn on_token_received(block_height: Nat, from: TokenHolder, value: Nat) {
    debug!("on_token_received in");
    NOTIFICATIONS_RECEIVED.with(|cell| {
        *cell.borrow_mut() += 1u64;
    });
    debug!("Token(caller) is {:?},block height is {},from is {:?},value is {}", api::caller().to_text(), block_height,from.to_hex(),value);
}


#[cfg_attr(coverage_nightly, no_coverage)]
#[query(name = "notificationCount")]
#[candid_method(query, rename = "notificationCount")]
async fn get_notification_count() -> u64 {
    NOTIFICATIONS_RECEIVED.with(|cell| {
        let count = cell.borrow();
        count.clone()
    })
}
