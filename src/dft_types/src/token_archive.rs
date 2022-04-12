use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

use crate::constants::{
    BLOCK_ARCHIVE_SIZE, BLOCK_ARCHIVE_TRIGGER_THRESHOLD, CYCLES_PER_AUTO_SCALING,
    MAX_CANISTER_STORAGE_BYTES,
};

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct TokenArchiveOptions {
    /// The number of blocks which, when exceeded, will trigger an archiving
    /// operation
    pub trigger_threshold: u32,
    /// The number of blocks to archive when trigger threshold is exceeded
    pub num_blocks_to_archive: u32,
    pub node_max_memory_size_bytes: Option<u32>,
    pub max_message_size_bytes: Option<u32>,
    pub controller_id: Principal,
    pub cycles_for_archive_creation: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Archive {
    storage_canisters: Vec<Principal>,
    storage_canisters_block_ranges: Vec<(Nat, Nat)>,
    node_max_memory_size_bytes: u32,
    max_message_size_bytes: u32,
    num_archived_blocks: Nat,
    pub trigger_threshold: u32,
    /// The number of blocks to archive when trigger threshold is exceeded
    pub num_blocks_to_archive: u32,
    // cycles to use for the call to create a new canister and to install the archive3
    pub cycles_for_archive_creation: u64,
    #[serde(skip)]
    archiving_in_progress: bool,
}

impl Default for Archive {
    fn default() -> Self {
        Archive {
            storage_canisters: Vec::new(),
            storage_canisters_block_ranges: Vec::new(),
            node_max_memory_size_bytes: MAX_CANISTER_STORAGE_BYTES,
            max_message_size_bytes: 0,
            num_archived_blocks: 0.into(),
            trigger_threshold: BLOCK_ARCHIVE_TRIGGER_THRESHOLD,
            num_blocks_to_archive: BLOCK_ARCHIVE_SIZE,
            cycles_for_archive_creation: CYCLES_PER_AUTO_SCALING,
            archiving_in_progress: false,
        }
    }
}

impl Archive {
    pub fn new(options: TokenArchiveOptions) -> Self {
        Self {
            storage_canisters: vec![],
            storage_canisters_block_ranges: vec![],
            node_max_memory_size_bytes: options
                .node_max_memory_size_bytes
                .unwrap_or(1024 * 1024 * 1024),
            max_message_size_bytes: options.max_message_size_bytes.unwrap_or(2 * 1024 * 1024),
            num_archived_blocks: 0.into(),
            trigger_threshold: options.trigger_threshold,
            num_blocks_to_archive: options.num_blocks_to_archive,
            cycles_for_archive_creation: options.cycles_for_archive_creation.unwrap_or(0),
            archiving_in_progress: false,
        }
    }

    pub fn last_storage_canister_id(&self) -> Option<Principal> {
        match self.storage_canisters.last() {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }

    pub fn index(&self) -> Vec<((Nat, Nat), Principal)> {
        self.storage_canisters_block_ranges
            .iter()
            .cloned()
            .zip(self.storage_canisters.clone())
            .collect()
    }

    pub fn scaling_storage_block_height_offset(&self) -> Nat {
        self.storage_canisters_block_ranges
            .last()
            .map(|(_, height_to)| height_to.clone() + 1u32)
            .unwrap_or(0u32.into())
    }

    pub fn storage_canisters(&self) -> &[Principal] {
        &self.storage_canisters
    }

    pub fn storage_canisters_block_ranges(&self) -> &[(Nat, Nat)] {
        &self.storage_canisters_block_ranges
    }

    pub fn append_scaling_storage_canister(&mut self, canister_id: Principal) {
        if self.archiving_in_progress {
            self.storage_canisters.push(canister_id);
        }
    }

    pub fn update_scaling_storage_blocks_range(
        &mut self,
        storage_index: usize,
        block_range: (Nat, Nat),
    ) {
        if !self.archiving_in_progress {
            return;
        }
        let last_range: Option<(Nat, Nat)> = self
            .storage_canisters_block_ranges
            .last()
            .map(|(start, end)| (start.clone(), end.clone()));

        let range = self.storage_canisters_block_ranges.get_mut(storage_index);

        match range {
            // We haven't inserted any Blocks into this archive node yet.
            None => {
                match last_range {
                    // If we haven't recorded any heights yet in any of the
                    // nodes then this is the **first archive node** and it
                    // starts with Block at height 0
                    None => self
                        .storage_canisters_block_ranges
                        .push((0u32.into(), block_range.1)),
                    // If we haven't recorded any heights for this node but
                    // a previous node exists then the current heights
                    // start one above those in the previous node
                    Some((_, _)) => self.storage_canisters_block_ranges.push(block_range),
                }
            }
            // We have already inserted some Blocks into this archive node.
            // Hence, we already have a value to work with
            Some(heights) => {
                heights.1 = block_range.1;
            }
        }
    }

    pub fn lock_for_archiving(&mut self) -> bool {
        if self.archiving_in_progress {
            return false;
        } else {
            self.archiving_in_progress = true;
            return true;
        }
    }

    pub fn unlock_after_archiving(&mut self) {
        if self.archiving_in_progress {
            self.archiving_in_progress = false;
        }
    }
}
