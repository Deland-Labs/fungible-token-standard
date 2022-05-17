use candid::{candid_method, Nat};
use dft_basic::auto_scaling_storage::AutoScalingStorageService;
use dft_types::*;
use ic_cdk::api;
use ic_cdk_macros::*;
use std::string::String;

#[cfg_attr(coverage_nightly, no_coverage)]
#[update(name = "batchMint")]
#[candid_method(update, rename = "batchMint")]
async fn batch_mint(
    mint_requests: Vec<(String, Nat)>,
    created_at: Option<u64>,
) -> Vec<OperationResult> {
    assert!(
        mint_requests.len() <= 500,
        "batch mint requests must be less than 500"
    );
    let token_id = api::id();
    let batch_res: Vec<OperationResult> = mint_requests //
        .into_iter()
        .map(|req| {
            let holder_parse_res = req.0.parse::<TokenHolder>();
            match holder_parse_res {
                Ok(holder) => {
                    match dft_mintable::mint(
                        &api::caller(),
                        &holder,
                        req.1 .0,
                        created_at,
                        api::time(),
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
