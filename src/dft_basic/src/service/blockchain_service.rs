use crate::state::STATE;
use candid::Principal;
use dft_types::*;
use num_bigint::BigUint;
use std::collections::VecDeque;

pub fn get_blocks_for_archiving() -> VecDeque<EncodedBlock> {
    STATE.with(|s| {
        let blockchain = s.blockchain.borrow();
        blockchain.get_blocks_for_archiving(
            blockchain.archive.trigger_threshold as usize,
            blockchain.archive.num_blocks_to_archive as usize,
        )
    })
}
pub fn archived_blocks_num() -> BigUint {
    STATE.with(|s| {
        let blockchain = s.blockchain.borrow();
        blockchain.num_archived_blocks.clone()
    })
}
pub fn last_storage_canister_index() -> usize {
    STATE.with(|s| {
        let blockchain = s.blockchain.borrow();
        blockchain.archive.last_storage_canister_index()
    })
}

pub fn latest_storage_canister() -> Option<Principal> {
    STATE.with(|s| {
        let blockchain = s.blockchain.borrow();
        blockchain.archive.latest_storage_canister()
    })
}

pub fn last_auto_scaling_storage_canister_id() -> Option<Principal> {
    STATE.with(|s| s.blockchain.borrow().archive.last_storage_canister_id())
}

pub fn scaling_storage_block_height_offset() -> BlockHeight {
    STATE.with(|s| {
        s.blockchain
            .borrow()
            .archive
            .scaling_storage_block_height_offset()
    })
}

pub fn remove_archived_blocks(num_archived: usize) {
    STATE.with(|s| {
        s.blockchain
            .borrow_mut()
            .remove_archived_blocks(num_archived)
    })
}

pub fn lock_for_archiving() -> bool {
    STATE.with(|s| s.blockchain.borrow_mut().archive.lock_for_archiving())
}

pub fn unlock_after_archiving() {
    STATE.with(|s| s.blockchain.borrow_mut().archive.unlock_after_archiving())
}

pub fn pre_append_scaling_storage_canister(canister_id: Principal) {
    STATE.with(|s| {
        s.blockchain
            .borrow_mut()
            .archive
            .pre_append_storage_canister(canister_id)
    })
}
pub fn append_scaling_storage_canister(storage_canister_id: Principal) {
    STATE.with(|s| {
        s.blockchain
            .borrow_mut()
            .archive
            .append_scaling_storage_canister(storage_canister_id)
    })
}

pub fn update_scaling_storage_blocks_range(
    storage_canister_index: usize,
    end_block_height: BlockHeight,
) {
    STATE.with(|s| {
        s.blockchain
            .borrow_mut()
            .archive
            .update_scaling_storage_blocks_range(storage_canister_index, end_block_height)
    })
}
