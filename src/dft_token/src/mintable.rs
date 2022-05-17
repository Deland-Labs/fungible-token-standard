use candid::{candid_method, Nat, Principal};
use dft_basic::auto_scaling_storage::AutoScalingStorageService;
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::*;
use std::string::String;

#[cfg_attr(coverage_nightly, no_coverage)]
#[query(name = "minters")]
#[candid_method(query, rename = "minters")]
fn minters() -> Vec<Principal> {
    dft_mintable::minters()
}

#[cfg_attr(coverage_nightly, no_coverage)]
#[update(name = "addMinter")]
#[candid_method(update, rename = "addMinter")]
fn add_minter(minter: Principal, created_at: Option<u64>) -> BooleanResult {
    dft_mintable::add_minter(&api::caller(), minter, created_at, api::time()).into()
}

#[cfg_attr(coverage_nightly, no_coverage)]
#[update(name = "removeMinter")]
#[candid_method(update, rename = "removeMinter")]
fn remove_minter(minter: Principal, created_at: Option<u64>) -> BooleanResult {
    dft_mintable::remove_minter(&api::caller(), minter, created_at, api::time()).into()
}

#[cfg_attr(coverage_nightly, no_coverage)]
#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
async fn mint(to: String, value: Nat, created_at: Option<u64>) -> OperationResult {
    let token_id = api::id();
    let holder_parse_res = to.parse::<TokenHolder>();

    match holder_parse_res {
        Ok(holder) => {
            match dft_mintable::mint(&api::caller(), &holder, value.0, created_at, api::time()) {
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

        Err(_) => OperationResult::Err(DFTError::InvalidArgFormatTo.into()),
    }
}
