use std::collections::VecDeque;
use std::ops::{Add, Sub};

use crate::ic_management::*;
use crate::token_service::blockchain_service;
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
use log::{debug, error, info};

const STORAGE_WASM: &[u8] =
    std::include_bytes!("../../target/wasm32-unknown-unknown/release/dft_tx_storage_opt.wasm");

pub async fn exec_auto_scaling_strategy() -> CommonResult<()> {
    let blocks_to_archive = blockchain_service::get_blocks_for_archiving();

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

    // if lock failed, return, lock failed means the archiving is already in progress
    if !blockchain_service::lock_for_archiving() {
        return Ok(());
    }

    if (send_blocks_to_archive(blocks_to_archive).await).is_ok() {
        info!(
            "Archive size: {} bytes,max_msg_size: {} bytes,total blocks: {}",
            archive_size_bytes, max_msg_size, num_blocks
        );
        let last_storage_index = blockchain_service::last_storage_canister_index();
        let archived_end_block_height = blockchain_service::archived_blocks_num()
            .add(num_blocks)
            .sub(1u32);

        blockchain_service::update_scaling_storage_blocks_range(
            last_storage_index,
            archived_end_block_height,
        );
        blockchain_service::remove_archived_blocks(num_blocks);
    };

    // Ensure unlock
    blockchain_service::unlock_after_archiving();

    Ok(())
}

async fn get_or_create_available_storage_id(archive_size_bytes: u32) -> CommonResult<Principal> {
    let mut last_storage_id = blockchain_service::last_auto_scaling_storage_canister_id();

    let mut is_necessary_create_new_storage_canister = last_storage_id.is_none();

    // check storage remain size
    if last_storage_id.is_some() {
        let req = CanisterIdRecord {
            canister_id: last_storage_id.unwrap(),
        };
        let status = get_canister_status(req).await;
        match status {
            Ok(res) => {
                info!(
                    "current scaling storage used memory_size is {}",
                    res.memory_size
                );
                if (Nat::from(MAX_CANISTER_STORAGE_BYTES) - res.memory_size).lt(&archive_size_bytes)
                {
                    is_necessary_create_new_storage_canister = true;
                } else {
                    return Ok(last_storage_id.unwrap());
                }
            }
            Err(msg) => {
                let msg = format!("check storage canister status failed. details:{}", msg);
                error!("{}", msg);
                return Err(DFTError::StorageScalingFailed { detail: msg });
            }
        };
    }

    if is_necessary_create_new_storage_canister {
        last_storage_id = blockchain_service::latest_storage_canister();
        let token_id = api::id();
        let block_height_offset: Nat =
            blockchain_service::scaling_storage_block_height_offset().into();

        // avoid re-create storage canister when install code failed
        if last_storage_id.is_some() {
            install_storage_canister_and_append_to_storage_records(
                last_storage_id.unwrap(),
                token_id,
                block_height_offset,
            )
            .await?;
        } else {
            let new_scaling_storage_canister_id =
                create_new_scaling_storage_canister(token_id, block_height_offset).await?;
            last_storage_id = Some(new_scaling_storage_canister_id);
        }
    }
    Ok(last_storage_id.unwrap())
}

async fn create_new_scaling_storage_canister(
    token_id: Principal,
    block_height_offset: Nat,
) -> CommonResult<Principal> {
    let create_args = CreateCanisterArgs {
        cycles: CYCLES_PER_AUTO_SCALING,
        settings: CanisterSettings {
            controllers: Some(vec![token_id]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        },
    };
    debug!("creating token storage...");
    let create_result = create_canister(create_args).await;

    match create_result {
        Ok(cdr) => {
            blockchain_service::pre_append_scaling_storage_canister(cdr.canister_id);
            debug!(
                "token new storage canister id : {} , block height offset : {}",
                cdr.canister_id,
                block_height_offset.clone()
            );
            install_storage_canister_and_append_to_storage_records(
                cdr.canister_id,
                token_id,
                block_height_offset,
            )
            .await?;
            Ok(cdr.canister_id)
        }
        Err(msg) => {
            let msg = format!("create new storage canister failed {}", msg);
            error!("{}", msg);
            Err(DFTError::StorageScalingFailed { detail: msg })
        }
    }
}

async fn install_storage_canister_and_append_to_storage_records(
    canister_id: Principal,
    token_id: Principal,
    block_height_offset: Nat,
) -> CommonResult<()> {
    match encode_args((token_id, block_height_offset.clone())) {
        Ok(install_args) => {
            match install_canister(&canister_id, STORAGE_WASM.to_vec(), install_args).await {
                Ok(_) => {
                    debug!("install storage canister success");
                    blockchain_service::append_scaling_storage_canister(canister_id);
                    Ok(())
                }
                Err(msg) => {
                    let msg = format!(
                        "install auto-scaling storage canister failed. details:{}",
                        msg
                    );
                    error!("{}", msg);
                    Err(DFTError::StorageScalingFailed { detail: msg })
                }
            }
        }
        Err(msg) => {
            let msg = format!("encode_args failed. details:{:?}", msg);
            error!("{}", msg);
            Err(DFTError::StorageScalingFailed { detail: msg })
        }
    }
}

async fn send_blocks_to_archive(blocks_to_archive: VecDeque<EncodedBlock>) -> CommonResult<()> {
    let storage_canister_id =
        get_or_create_available_storage_id(blocks_to_archive.len() as u32).await?;

    debug!("storage_canister_id is {}", storage_canister_id.to_text());
    //save the txs to auto-scaling storage
    let res: Result<(BooleanResult,), (RejectionCode, String)> =
        api::call::call(storage_canister_id, "batchAppend", (blocks_to_archive,)).await;
    match res {
        Ok((res,)) => match res {
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
