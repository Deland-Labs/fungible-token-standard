use std::collections::VecDeque;

use crate::ic_management::*;
use crate::state::TOKEN;
use candid::encode_args;
use dft_types::constants::{
    CYCLES_PER_AUTO_SCALING, MAX_CANISTER_STORAGE_BYTES, MAX_MESSAGE_SIZE_BYTES,
};
use dft_types::*;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::{
    api,
    export::{candid::Nat, Principal},
};

const STORAGE_WASM: &[u8] =
    std::include_bytes!("../../target/wasm32-unknown-unknown/release/dft_tx_storage_opt.wasm");

pub async fn exec_auto_scaling_strategy() -> CommonResult<()> {
    let blocks_to_archive = TOKEN.with(|token| {
        let token = token.borrow();
        let blockchain = token.blockchain();
        let blocks_to_archive = blockchain.get_blocks_for_archiving(
            blockchain.archive.trigger_threshold as usize,
            blockchain.archive.num_blocks_to_archive as usize,
        );
        blocks_to_archive
    });

    let archive_size_bytes = blocks_to_archive
        .iter()
        .fold(0, |acc, block| acc + block.size_bytes());
    let max_msg_size = MAX_MESSAGE_SIZE_BYTES;
    if archive_size_bytes > max_msg_size as usize {
        return Err(DFTError::ExceedTheByteSizeLimitOfOneRequest);
    }

    let num_blocks = blocks_to_archive.len();

    if num_blocks == 0 {
        return Ok(());
    }

    api::print(format!(
        "Archive size: {} bytes,max_msg_size: {} bytes,total blocks: {}",
        archive_size_bytes,
        max_msg_size,
        blocks_to_archive.len()
    ));

    // mark archiving
    let lock_res = TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.lock_for_archiving()
    });

    // if lock failed, return, lock failed means the archiving is already in progress
    if lock_res == false {
        return Ok(());
    }

    send_blocks_to_archive(blocks_to_archive).await?;

    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let last_storage_index = token.blockchain().archive.last_storage_canister_index();
        let archived_end_block_height =
            token.blockchain().num_archived_blocks.clone() + num_blocks as u128 - 1u32;
        token.update_scaling_storage_blocks_range(last_storage_index, archived_end_block_height);
        token.remove_archived_blocks(num_blocks);
        token.unlock_after_archiving();
    });

    Ok(())
}

async fn get_or_create_available_storage_id(archive_size_bytes: u32) -> CommonResult<Principal> {
    let mut last_storage_id = TOKEN.with(|token| {
        let token = token.borrow();
        token.last_auto_scaling_storage_canister_id()
    });

    let mut is_necessary_create_new_storage_canister = last_storage_id.is_none();

    // check storage remain size
    if last_storage_id.is_some() {
        let req = CanisterIdRecord {
            canister_id: last_storage_id.unwrap(),
        };
        let status = get_canister_status(req).await;
        match status {
            Ok(res) => {
                ic_cdk::print(format!(
                    "current scaling storage used memory_size is {}",
                    res.memory_size
                ));
                if (Nat::from(MAX_CANISTER_STORAGE_BYTES) - res.memory_size).lt(&archive_size_bytes)
                {
                    is_necessary_create_new_storage_canister = true;
                } else {
                    return Ok(last_storage_id.unwrap());
                }
            }
            Err(emsg) => {
                let emsg = format!("check storage canister status failed. details:{}", emsg);
                api::print(emsg.clone());
                return Err(DFTError::StorageScalingFailed { detail: emsg });
            }
        };
    }

    if is_necessary_create_new_storage_canister {
        let new_scaling_storage_canister_id = create_new_scaling_storage_canister().await?;
        last_storage_id = Some(new_scaling_storage_canister_id);
    }
    return Ok(last_storage_id.unwrap());
}

async fn create_new_scaling_storage_canister() -> CommonResult<Principal> {
    let dft_id = api::id();
    let create_args = CreateCanisterArgs {
        cycles: CYCLES_PER_AUTO_SCALING,
        settings: CanisterSettings {
            controllers: Some(vec![dft_id.clone()]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        },
    };
    api::print("creating token storage...");
    let create_result = create_canister(create_args).await;

    match create_result {
        Ok(cdr) => {
            let block_height_offset: Nat = TOKEN.with(|token| {
                let token = token.borrow();
                token.scaling_storage_block_height_offset().into()
            });

            TOKEN.with(|token| {
                let mut token = token.borrow_mut();
                token.pre_append_scaling_storage_canister(cdr.canister_id);
            });

            api::print(format!(
                "token new storage canister id : {} , block height offset : {}",
                cdr.canister_id.clone().to_string(),
                block_height_offset.clone()
            ));
            match encode_args((dft_id.clone(), block_height_offset.clone())) {
                Ok(install_args) => {
                    match install_canister(&cdr.canister_id, STORAGE_WASM.to_vec(), install_args)
                        .await
                    {
                        Ok(_) => {
                            TOKEN.with(|token| {
                                let mut token = token.borrow_mut();
                                token.append_scaling_storage_canister(cdr.canister_id);
                            });
                            Ok(cdr.canister_id)
                        }
                        Err(emsg) => {
                            let emsg = format!(
                                "install auto-scaling storage canister failed. details:{}",
                                emsg
                            );
                            api::print(emsg.clone());
                            return Err(DFTError::StorageScalingFailed { detail: emsg }.into());
                        }
                    }
                }
                Err(emsg) => {
                    let emsg = format!("encode_args failed. details:{:?}", emsg);
                    api::print(emsg.clone());
                    return Err(DFTError::StorageScalingFailed { detail: emsg });
                }
            }
        }
        Err(emsg) => {
            let emsg = format!("create new storage canister failed {}", emsg);
            api::print(emsg.clone());
            return Err(DFTError::StorageScalingFailed { detail: emsg }.into());
        }
    }
}

async fn send_blocks_to_archive(blocks_to_archive: VecDeque<EncodedBlock>) -> CommonResult<bool> {
    let storage_canister_id =
        get_or_create_available_storage_id(blocks_to_archive.len() as u32).await?;

    api::print(format!(
        "storage_canister_id is {}",
        storage_canister_id.to_text()
    ));
    //save the txs to auto-scaling storage
    let res: Result<(BooleanResult, ), (RejectionCode, String)> =
        api::call::call(storage_canister_id, "batchAppend", (blocks_to_archive, )).await;
    match res {
        Ok((res, )) => match res {
            BooleanResult::Ok(sucess) => {
                if sucess {
                    api::print("batchAppend success");
                    Ok(true)
                } else {
                    api::print("batchAppend failed");
                    Err(DFTError::MoveTxToScalingStorageFailed)
                }
            }
            BooleanResult::Err(err) => Err(err.into()),
        },
        Err((_, emsg)) => {
            api::print(format!(
                "batchAppend: save to auto-scaling storage failed,{0}",
                emsg
            ));
            Err(DFTError::MoveTxToScalingStorageFailed)
        }
    }
}
