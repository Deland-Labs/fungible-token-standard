use candid::{candid_method, Nat};
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::*;
use std::string::String;

#[update(name = "onTokenReceived")]
#[candid_method(update, rename = "onTokenReceived")]
async fn on_token_received(from: TokenHolder, value: Nat) -> bool {
    api::print(format!("DFT is {:?}", api::caller()));
    api::print(format!("from is {:?}", from));
    api::print(format!("value is {}", value));
    true
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}
