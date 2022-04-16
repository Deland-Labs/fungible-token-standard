use candid::{candid_method, Nat};
use dft_types::*;
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use std::string::String;

#[path = "../../dft_basic/src/basic.rs"]
mod basic;
#[path = "../../dft_basic/src/http.rs"]
mod http;
#[path = "../../dft_basic/src/management.rs"]
mod management;
mod mintable;
mod token;

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}
