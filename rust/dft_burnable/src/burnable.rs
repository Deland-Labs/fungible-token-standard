extern crate dft_types;
extern crate dft_utils;

use candid::candid_method;
use dft_standard::auto_scaling_storage::exec_auto_scaling_strategy;
use dft_standard::state::TOKEN;
use dft_types::*;
use dft_utils::*;
use ic_cdk::{api, export::candid::Nat};
use ic_cdk_macros::*;
use std::string::String;

use crate::token::BurnableExtension;

#[update(name = "burnFrom")]
#[candid_method(update, rename = "burnFrom")]
async fn burn_from(
    from_sub_account: Option<Subaccount>,
    owner: String,
    value: Nat,
    nonce: Option<u64>,
) -> ActorResult<TransactionResponse> {
    let caller = api::caller();
    let spender = TokenHolder::new(caller, from_sub_account);
    let owner_parse_res = owner.parse::<TokenHolder>();
    match owner_parse_res {
        Ok(owner_holder) => {
            // call token burn_from
            let tx_index = TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.burn_from(
                    &caller,
                    &owner_holder,
                    &spender,
                    value.clone(),
                    nonce,
                    api::time(),
                )
            })?;

            let result = Ok(TransactionResponse {
                tx_id: encode_tx_id(api::id(), tx_index),
                error: match exec_auto_scaling_strategy().await {
                    Ok(_) => None,
                    Err(e) => Some(e),
                },
            });
            result
        }

        Err(_) => Err(DFTError::InvalidSpender.into()),
    }
}

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(
    from_sub_account: Option<Subaccount>,
    value: Nat,
    nonce: Option<u64>,
) -> ActorResult<TransactionResponse> {
    let caller = api::caller();
    let transfer_from = TokenHolder::new(caller, from_sub_account);
    let tx_index = TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.burn(&caller, &transfer_from, value.clone(), nonce, api::time())
    })?;
    //exec auto-scaling storage strategy
    Ok(TransactionResponse {
        tx_id: encode_tx_id(api::id(), tx_index),
        error: match exec_auto_scaling_strategy().await {
            Ok(_) => None,
            Err(e) => Some(e),
        },
    })
}
