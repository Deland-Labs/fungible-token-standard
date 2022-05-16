use candid::{candid_method, Nat};
use dft_basic::auto_scaling_storage::AutoScalingStorageService;
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::*;
use std::string::String;

#[update(name = "burnFrom")]
#[candid_method(update, rename = "burnFrom")]
async fn burn_from(
    from_sub_account: Option<Subaccount>,
    owner: String,
    value: Nat,
    created_at: Option<u64>,
) -> OperationResult {
    let caller = api::caller();
    let token_id = api::id();
    let spender = TokenHolder::new(caller, from_sub_account);
    let owner_parse_res = owner.parse::<TokenHolder>();
    match owner_parse_res {
        Ok(owner_holder) => {
            match dft_burnable::burn_from(
                &caller,
                &owner_holder,
                &spender,
                value.0,
                created_at,
                api::time(),
            ) {
                Ok((block_height, _, tx_hash)) => {
<<<<<<< HEAD
<<<<<<< HEAD
                    let auto_scaling_service = AutoScalingStorageService::new(token_id);
=======
                    let auto_scaling_service = AutoScalingStorageService::new();
>>>>>>> 626ad9f (Fix: auto scaling)
=======
                    let auto_scaling_service = AutoScalingStorageService::new(token_id);
>>>>>>> 202560d (Unit Test: auto scaling storage)
                    auto_scaling_service.exec_auto_scaling_strategy().await;
                    OperationResult::Ok {
                        tx_id: hex::encode(tx_hash.as_ref()),
                        block_height: block_height.into(),
                    }
                }
                Err(e) => OperationResult::Err(e.into()),
            }
        }

        Err(_) => OperationResult::Err(DFTError::InvalidSpender.into()),
    }
}

#[update(name = "burn")]
#[candid_method(update, rename = "burn")]
async fn burn(
    from_sub_account: Option<Subaccount>,
    value: Nat,
    created_at: Option<u64>,
) -> OperationResult {
    let caller = api::caller();
    let token_id = api::id();
    let transfer_from = TokenHolder::new(caller, from_sub_account);
    match dft_burnable::burn(&caller, &transfer_from, value.0, created_at, api::time()) {
        Ok((block_height, _, tx_hash)) => {
<<<<<<< HEAD
<<<<<<< HEAD
            let auto_scaling_service = AutoScalingStorageService::new(token_id);
=======
            let auto_scaling_service = AutoScalingStorageService::new();
>>>>>>> 626ad9f (Fix: auto scaling)
=======
            let auto_scaling_service = AutoScalingStorageService::new(token_id);
>>>>>>> 202560d (Unit Test: auto scaling storage)
            auto_scaling_service.exec_auto_scaling_strategy().await;
            OperationResult::Ok {
                tx_id: hex::encode(tx_hash.as_ref()),
                block_height: block_height.into(),
            }
        }
        Err(e) => OperationResult::Err(e.into()),
    }
}
