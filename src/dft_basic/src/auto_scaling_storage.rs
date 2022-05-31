use crate::canister_api::*;
use crate::service::blockchain_service;
use candid::encode_args;
use dft_types::constants::*;
use dft_types::*;
use ic_cdk::export::{candid::Nat, Principal};
use log::{debug, error, info};
use std::collections::VecDeque;
use std::ops::{Add, Sub};
use std::sync::Arc;

// Auto-scaling tx  storage canister wasm package bytes
const AUTO_SCALING_STORAGE_CANISTER_WASM: &[u8] =
    std::include_bytes!("../../target/wasm32-unknown-unknown/release/dft_tx_storage.wasm");

pub struct AutoScalingStorageService {
    pub token_id: Principal,
    pub ic_management: Arc<dyn IICManagementAPI>,
    pub dft_tx_storage: Arc<dyn IDFTTxStorageAPI>,
}

impl AutoScalingStorageService {
    pub fn new(token_id: Principal) -> Self {
        Self {
            token_id,
            ic_management: Arc::new(ICManagementAPI::default()),
            dft_tx_storage: Arc::new(DFTTxStorageAPI::default()),
        }
    }
    pub async fn exec_auto_scaling_strategy(&self) {
        let blocks_to_archive = blockchain_service::get_blocks_for_archiving();

        let archive_size_bytes = blocks_to_archive
            .iter()
            .fold(0, |acc, block| acc + block.size_bytes());
        let max_msg_size = MAX_MESSAGE_SIZE_BYTES;
        if archive_size_bytes > max_msg_size as usize {
            error!(
                "batch_mint exec_auto_scaling_strategy failed: {}",
                DFTError::ExceedTheByteSizeLimitOfOneRequest.to_string()
            );
            return;
        }

        let num_blocks = blocks_to_archive.len();

        if num_blocks == 0 {
            return;
        }

        // if lock failed, return, lock failed means the archiving is already in progress
        if !blockchain_service::lock_for_archiving() {
            return;
        }

        if (self.send_blocks_to_archive(blocks_to_archive, archive_size_bytes).await).is_ok() {
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
    }

    async fn get_or_create_available_storage_id(
        &self,
        archive_size_bytes: usize,
    ) -> CommonResult<Principal> {
        let mut last_storage_id = blockchain_service::last_auto_scaling_storage_canister_id();

        let mut is_necessary_create_new_storage_canister = last_storage_id.is_none();

        // check storage remain size
        if last_storage_id.is_some() {
            let req = CanisterIdRecord {
                canister_id: last_storage_id.unwrap(),
            };
            let status = self.ic_management.canister_status(req).await;
            match status {
                Ok(res) => {
                    debug!(
                        "current scaling storage used memory_size is {},max is {},available memory_size is {},archive_size_bytes is {}",
                        res.memory_size, MAX_CANISTER_STORAGE_BYTES,MAX_CANISTER_STORAGE_BYTES - (res.memory_size.clone().0 +  archive_size_bytes),
                        archive_size_bytes.clone()
                    );
                    if MAX_CANISTER_STORAGE_BYTES <= res.memory_size + archive_size_bytes
                    {
                        debug!("is_necessary_create_new_storage_canister");
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
            let token_id = self.token_id;
            let block_height_offset: Nat =
                blockchain_service::scaling_storage_block_height_offset().into();

            // avoid re-create storage canister when install code failed
            if last_storage_id.is_some() {
                self.install_storage_canister_and_append_to_storage_records(
                    last_storage_id.unwrap(),
                    token_id,
                    block_height_offset,
                )
                    .await?;
            } else {
                let new_scaling_storage_canister_id = self
                    .create_new_scaling_storage_canister(token_id, block_height_offset)
                    .await?;
                last_storage_id = Some(new_scaling_storage_canister_id);
            }
        }
        Ok(last_storage_id.unwrap())
    }

    async fn create_new_scaling_storage_canister(
        &self,
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
        let create_result = self.ic_management.create_canister(create_args).await;

        match create_result {
            Ok(cdr) => {
                blockchain_service::pre_append_scaling_storage_canister(cdr.canister_id);
                debug!(
                    "token new storage canister id : {} , block height offset : {}",
                    cdr.canister_id,
                    block_height_offset.clone()
                );
                self.install_storage_canister_and_append_to_storage_records(
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
        &self,
        canister_id: Principal,
        token_id: Principal,
        block_height_offset: Nat,
    ) -> CommonResult<()> {
        match encode_args((token_id, block_height_offset.clone())) {
            Ok(install_args) => {
                match self
                    .ic_management
                    .canister_install(
                        &canister_id,
                        AUTO_SCALING_STORAGE_CANISTER_WASM.to_vec(),
                        install_args,
                    )
                    .await
                {
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

    async fn send_blocks_to_archive(
        &self,
        blocks_to_archive: VecDeque<EncodedBlock>,
        archive_size_bytes: usize,
    ) -> CommonResult<()> {
        let storage_canister_id = self
            .get_or_create_available_storage_id(archive_size_bytes)
            .await?;

        debug!("storage_canister_id is {}", storage_canister_id.to_text());
        self.dft_tx_storage
            .batch_append(storage_canister_id, blocks_to_archive)
            .await
    }
}

#[cfg(test)]
mod tests;
