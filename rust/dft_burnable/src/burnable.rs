extern crate dft_types;
extern crate dft_utils;
use candid::candid_method;
use dft_types::*;
use dft_utils::*;
use ic_cdk::{api, export::candid::Nat};
use ic_cdk_macros::*;
use std::string::String;

use crate::{
    auto_scaling_storage::exec_auto_scaling_strategy, state::TOKEN, token::BurnableExtension,
};

#[update(name = "burnFrom")]
#[candid_method(update, rename = "burnFrom")]
async fn burn_from(
    from_sub_account: Option<Subaccount>,
    spender: String,
    value: Nat,
) -> ActorResult<TransactionResponse> {
    let caller = api::caller();
    let token_holder_owner = TokenHolder::new(caller, from_sub_account);
    let spender_parse_res = spender.parse::<TokenHolder>();
    match spender_parse_res {
        Ok(holder) => {
            // call token burn_from
            let tx_index = TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.burn_from(
                    &caller,
                    &token_holder_owner,
                    &holder,
                    value.clone(),
                    api::time(),
                )
            })?;

            let mut errors: Vec<ActorError> = vec![];
            //exec auto-scaling storage strategy
            match exec_auto_scaling_strategy().await {
                Ok(_) => (),
                Err(e) => {
                    errors.push(e);
                }
            };

            Ok(TransactionResponse {
                tx_id: encode_tx_id(api::id(), tx_index),
                error: if errors.len() > 0 { Some(errors) } else { None },
            })
        }

        Err(_) => Err(DFTError::InvalidSpender.into())
    }
}

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(from_sub_account: Option<Subaccount>, value: Nat) -> ActorResult<TransactionResponse> {
    let caller = api::caller();
    let transfer_from = TokenHolder::new(caller, from_sub_account);
    let tx_index = TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.burn(
            &caller,
            &transfer_from,
            value.clone(),
            api::time(),
        )
    })?;
    let mut errors: Vec<ActorError> = vec![];
    //exec auto-scaling storage strategy
    match exec_auto_scaling_strategy().await {
        Ok(_) => (),
        Err(e) => {
            errors.push(e);
        }
    };

    Ok(TransactionResponse {
        tx_id: encode_tx_id(api::id(), tx_index),
        error: if errors.len() > 0 { Some(errors) } else { None },
    })
}