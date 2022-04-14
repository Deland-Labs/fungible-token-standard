use candid::candid_method;
use dft_types::*;
use ic_cdk::{
    export::{candid::Nat, Principal},
};
use ic_cdk_macros::*;
use std::string::String;

mod basic;

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}
