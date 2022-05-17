use candid::{candid_method, Nat};
use dft_basic::{auto_scaling_storage::AutoScalingStorageService, service::basic_service};
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::*;
use std::string::String;

#[cfg_attr(coverage_nightly, no_coverage)]
#[update(name = "batchTransfer")]
#[candid_method(update, rename = "batchTransfer")]
async fn batch_transfer(
    from_sub_account: Option<Subaccount>,
    transfer_requests: Vec<(String, Nat)>,
    created_at: Option<u64>,
) -> Vec<OperationResult> {
    assert!(
        transfer_requests.len() <= 500,
        "batch transfer requests must be less than 500"
    );
    let now = api::time();
    let caller = api::caller();
    let token_id= api::id();
    let transfer_from = TokenHolder::new(caller, from_sub_account);

    let batch_res: Vec<OperationResult> = transfer_requests //
        .into_iter()
        .map(|req| {
            let receiver_parse_result = req.0.parse::<TokenReceiver>();
            match receiver_parse_result {
                Ok(receiver) => {
                    match basic_service::transfer(
                        &api::caller(),
                        &transfer_from,
                        &receiver,
                        req.1 .0,
                        created_at,
                        now,
                    ) {
                        Ok((block_height, _, tx_hash)) => OperationResult::Ok {
                            tx_id: hex::encode(tx_hash.as_ref()),
                            block_height: block_height.into(),
                        },
                        Err(e) => api::trap(e.to_string().as_ref()),
                    }
                }

                Err(_) => api::trap(DFTError::InvalidArgFormatTo.to_string().as_ref()),
            }
        })
        .collect();

    let auto_scaling_service = AutoScalingStorageService::new(token_id);
    auto_scaling_service.exec_auto_scaling_strategy().await;
    batch_res
}

#[cfg_attr(coverage_nightly, no_coverage)]
#[update(name = "batchTransferFrom")]
#[candid_method(update, rename = "batchTransferFrom")]
async fn batch_transfer_from(
    spender_sub_account: Option<Subaccount>,
    from: String,
    transfer_requests: Vec<(String, Nat)>,
    created_at: Option<u64>,
) -> Vec<OperationResult> {
    assert!(
        transfer_requests.len() <= 500,
        "batch mint requests must be less than 500"
    );
    let caller = api::caller();
    let token_id = api::id();
    let now = api::time();
    let spender = TokenHolder::new(caller, spender_sub_account);

    match from.parse::<TokenHolder>() {
        Ok(from_token_holder) => {
            let batch_res: Vec<OperationResult> = transfer_requests //
                .into_iter()
                .map(|req| {
                    let receiver_parse_result = req.0.parse::<TokenReceiver>();
                    match receiver_parse_result {
                        Ok(receiver) => {
                            match basic_service::transfer_from(
                                &api::caller(),
                                &from_token_holder,
                                &spender,
                                &receiver,
                                req.1 .0,
                                created_at,
                                now,
                            ) {
                                Ok((block_height, _, tx_hash)) => OperationResult::Ok {
                                    tx_id: hex::encode(tx_hash.as_ref()),
                                    block_height: block_height.into(),
                                },
                                Err(e) => api::trap(e.to_string().as_ref()),
                            }
                        }

                        Err(_) => api::trap(DFTError::InvalidArgFormatTo.to_string().as_ref()),
                    }
                })
                .collect();

            let auto_scaling_service = AutoScalingStorageService::new(token_id);
            auto_scaling_service.exec_auto_scaling_strategy().await;
            batch_res
        }
        _ => api::trap(DFTError::InvalidArgFormatFrom.to_string().as_ref()),
    }
}
