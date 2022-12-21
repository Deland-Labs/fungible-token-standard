use async_trait::async_trait;
use candid::Principal;
use dft_types::{BooleanResult, CommonResult, DFTError, EncodedBlock};
use ic_cdk::api;
use ic_cdk::api::call::RejectionCode;
use log::{debug, error};
use std::collections::VecDeque;

#[async_trait]
pub trait IDFTTxStorageAPI {
    async fn batch_append(
        &self,
        storage_canister_id: Principal,
        blocks: VecDeque<EncodedBlock>,
    ) -> CommonResult<()>;
}
#[derive(Default)]
pub struct DFTTxStorageAPI;

#[cfg_attr(coverage_nightly, no_coverage)]
#[async_trait]
impl IDFTTxStorageAPI for DFTTxStorageAPI {
    async fn batch_append(
        &self,
        storage_canister_id: Principal,
        blocks: VecDeque<EncodedBlock>,
    ) -> CommonResult<()> {
        //save the txs to auto-scaling storage
        let res: Result<(BooleanResult,), (RejectionCode, String)> =
            api::call::call(storage_canister_id, "batchAppend", (blocks,)).await;
        match res {
            Ok((res,)) => match res {
                BooleanResult::Ok(success) => {
                    if success {
                        debug!("batchAppend success");
                        Ok(())
                    } else {
                        error!("batchAppend failed");
                        Err(DFTError::MoveTxToScalingStorageFailed)
                    }
                }
                BooleanResult::Err(err) => Err(err.into()),
            },
            Err((_, msg)) => {
                error!("batchAppend: save to auto-scaling storage failed,{0}", msg);
                Err(DFTError::MoveTxToScalingStorageFailed)
            }
        }
    }
}
