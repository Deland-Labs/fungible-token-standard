#![cfg_attr(coverage_nightly, feature(no_coverage))]
use candid::{candid_method, Nat};
use dft_types::*;
use ic_cdk_macros::*;
use std::string::String;
pub mod receiver;

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}
