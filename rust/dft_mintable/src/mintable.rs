extern crate dft_types;
extern crate dft_utils;

use crate::token::MintableExtension;
use candid::candid_method;
use dft_standard::auto_scaling_storage::exec_auto_scaling_strategy;
use dft_standard::state::TOKEN;
use dft_types::*;
use dft_utils::*;
use ic_cdk::{api, export::candid::Nat};
use ic_cdk_macros::*;
use std::string::String;

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(to: String, value: Nat, nonce: Option<u64>) -> ActorResult<TransactionResponse> {
    let holder_parse_res = to.parse::<TokenHolder>();

    match holder_parse_res {
        Ok(holder) => {
            let tx_index = TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.mint(&api::caller(), &holder, value.clone(), nonce, api::time())
            })?;
            // exec auto scaling strategy
            Ok(TransactionResponse {
                tx_id: encode_tx_id(api::id(), tx_index),
                error: match exec_auto_scaling_strategy().await {
                    Err(e) => Some(e),
                    _ => None,
                },
            })
        }

        Err(_) => Err(DFTError::InvalidArgFormatTo.into()),
    }
}
