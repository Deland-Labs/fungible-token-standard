use std::collections::VecDeque;
use async_trait::async_trait;
use candid::Principal;
use dft_types::{BooleanResult, CommonResult, DFTError, EncodedBlock};
use ic_cdk::api;
use ic_cdk::api::call::RejectionCode;
use log::{debug, error};

#[async_trait]
pub trait IDFTTxStorageAPI {
    async fn batch_append(
        &self,
        storage_canister_id: Principal,
        blocks: VecDeque<EncodedBlock>,
    ) -> CommonResult<()>;
}

pub struct DFTTxStorageAPI;

impl DFTTxStorageAPI {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl IDFTTxStorageAPI for DFTTxStorageAPI {
    async fn batch_append(
        &self,
        storage_canister_id: Principal,
        blocks: VecDeque<EncodedBlock>,
    ) -> CommonResult<()> {
        //save the txs to auto-scaling storage
        let res: Result<(BooleanResult, ), (RejectionCode, String)> =
            api::call::call(storage_canister_id, "batchAppend", (blocks, )).await;
        match res {
            Ok((res, )) => match res {
                BooleanResult::Ok(sucess) => {
                    if sucess {
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
