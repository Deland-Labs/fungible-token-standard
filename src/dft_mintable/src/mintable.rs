use candid::{candid_method, Nat, Principal};
use dft_standard::{
    auto_scaling_storage::exec_auto_scaling_strategy, state::TOKEN, token::TokenStandard,
};
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::*;
use std::string::String;

#[query(name = "minters")]
#[candid_method(query, rename = "minters")]
fn minters() -> Vec<Principal> {
    TOKEN.with(|token| {
        let token = token.borrow();
        token.minters().to_vec()
    })
}

#[update(name = "addMinter")]
#[candid_method(update, rename = "addMinter")]
fn add_minter(minter: Principal, created_at: Option<u64>) -> BooleanResult {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token
            .add_minter(&api::caller(), minter, created_at, api::time())
            .into()
    })
}

#[update(name = "removeMinter")]
#[candid_method(update, rename = "removeMinter")]
fn remove_minter(minter: Principal, created_at: Option<u64>) -> BooleanResult {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token
            .remove_minter(&api::caller(), minter, created_at, api::time())
            .into()
    })
}

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(to: String, value: Nat, created_at: Option<u64>) -> OperationResult {
    let holder_parse_res = to.parse::<TokenHolder>();

    match holder_parse_res {
        Ok(holder) => {
            match TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.mint(&api::caller(), &holder, value.0, created_at, api::time())
            }) {
                Ok((block_height, _, tx_hash)) => OperationResult::Ok {
                    tx_id: hex::encode(tx_hash.as_ref()),
                    block_height: block_height.into(),
                    error: match exec_auto_scaling_strategy().await {
                        Ok(_) => None,
                        Err(e) => Some(e.into()),
                    },
                },
                Err(e) => OperationResult::Err(e.into()),
            }
        }

        Err(_) => OperationResult::Err(DFTError::InvalidArgFormatTo.into()),
    }
}
