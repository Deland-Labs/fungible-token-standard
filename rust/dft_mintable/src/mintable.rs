extern crate dft_types;
extern crate dft_utils;
use candid::candid_method;
use dft_types::*;
use dft_utils::*;
use ic_cdk::{api, export::candid::Nat};
use ic_cdk_macros::*;
use std::string::String;

use crate::{
    auto_scaling_storage::exec_auto_scaling_strategy, state::TOKEN, token::MintableExtension,
};

#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(to: String, value: Nat) -> ActorResult<TransactionResponse> {
    let holder_parse_res = to.parse::<TokenHolder>();

    match holder_parse_res {
        Ok(holder) => {
            let tx_index = TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.mint(&api::caller(), &holder, value.clone(), api::time())
            })?;
            let mut errors: Vec<ActorError> = Vec::new();
            // exec auto scaling strategy
            match exec_auto_scaling_strategy().await {
                Err(e) => errors.push(e),
                _ => {}
            };
            Ok(TransactionResponse {
                tx_id: encode_tx_id(api::id(), tx_index),
                error: if errors.len() > 0 { Some(errors) } else { None },
            })
        }

        Err(_) => Err(DFTError::InvalidArgFormatTo.into()),
    }
}
