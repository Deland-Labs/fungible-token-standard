use crate::token::MintableExtension;
use candid::candid_method;
use dft_standard::auto_scaling_storage::exec_auto_scaling_strategy;
use dft_standard::state::TOKEN;
use dft_types::*;
use ic_cdk::{api, export::candid::Nat};
use ic_cdk_macros::*;
use std::string::String;

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(to: String, value: Nat, nonce: Option<u64>) -> OperationResult {
    let holder_parse_res = to.parse::<TokenHolder>();

    match holder_parse_res {
        Ok(holder) => {
            match TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.mint(&api::caller(), &holder, value.clone(), nonce, api::time())
            }) {
                Ok((block_height, _, tx_hash)) => OperationResult::Ok {
                    tx_id: hex::encode(tx_hash.as_ref()),
                    block_height,
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
