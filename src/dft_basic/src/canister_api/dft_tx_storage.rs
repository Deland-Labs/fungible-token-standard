<<<<<<< HEAD
<<<<<<< HEAD
use std::collections::VecDeque;
=======
>>>>>>> ebc4cf1 (Refactor: auto_scaling_storage for unit test)
=======
use std::collections::VecDeque;
>>>>>>> 202560d (Unit Test: auto scaling storage)
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
<<<<<<< HEAD
<<<<<<< HEAD
        blocks: VecDeque<EncodedBlock>,
    ) -> CommonResult<()>;
}
#[derive(Default)]
pub struct DFTTxStorageAPI;

=======
        blocks: Vec<EncodedBlock>,
=======
        blocks: VecDeque<EncodedBlock>,
>>>>>>> 202560d (Unit Test: auto scaling storage)
    ) -> CommonResult<()>;
}

pub struct DFTTxStorageAPI;

impl DFTTxStorageAPI {
    pub fn new() -> Self {
        Self
    }
}

>>>>>>> ebc4cf1 (Refactor: auto_scaling_storage for unit test)
#[async_trait]
impl IDFTTxStorageAPI for DFTTxStorageAPI {
    async fn batch_append(
        &self,
        storage_canister_id: Principal,
<<<<<<< HEAD
<<<<<<< HEAD
        blocks: VecDeque<EncodedBlock>,
    ) -> CommonResult<()> {
        //save the txs to auto-scaling storage
        let res: Result<(BooleanResult, ), (RejectionCode, String)> =
            api::call::call(storage_canister_id, "batchAppend", (blocks, )).await;
        match res {
            Ok((res, )) => match res {
=======
        blocks: Vec<EncodedBlock>,
=======
        blocks: VecDeque<EncodedBlock>,
>>>>>>> 202560d (Unit Test: auto scaling storage)
    ) -> CommonResult<()> {
        //save the txs to auto-scaling storage
        let res: Result<(BooleanResult, ), (RejectionCode, String)> =
            api::call::call(storage_canister_id, "batchAppend", (blocks, )).await;
        match res {
<<<<<<< HEAD
            Ok((res,)) => match res {
>>>>>>> ebc4cf1 (Refactor: auto_scaling_storage for unit test)
=======
            Ok((res, )) => match res {
>>>>>>> 202560d (Unit Test: auto scaling storage)
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
