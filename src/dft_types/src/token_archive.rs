use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

use crate::{
    constants::{
        BLOCK_ARCHIVE_SIZE, BLOCK_ARCHIVE_TRIGGER_THRESHOLD, CYCLES_PER_AUTO_SCALING,
        MAX_CANISTER_STORAGE_BYTES,
    },
    BlockHeight,
};

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ArchiveOptions {
    /// The number of blocks which, when exceeded, will trigger an archiving
    /// operation
    pub trigger_threshold: u32,
    /// The number of blocks to archive when trigger threshold is exceeded
    pub num_blocks_to_archive: u32,
    pub node_max_memory_size_bytes: Option<u32>,
    pub max_message_size_bytes: Option<u32>,
    pub cycles_for_archive_creation: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ArchiveInfo {
    #[serde(rename = "canisterId")]
    canister_id: Principal,
    #[serde(rename = "startBlockHeight")]
    start_block_height: Nat,
    #[serde(rename = "endBlockHeight")]
    end_block_height: Nat,
    #[serde(rename = "numBlocks")]
    num_blocks: Nat,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Archive {
    storage_canisters: Vec<Principal>,
    // Temporary storage of newly created auto-scaling storage canister
    // id to avoid duplicate creation due to failed installation code
    latest_storage_canister: Option<Principal>,
    storage_canisters_block_ranges: Vec<(BlockHeight, BlockHeight)>,
    node_max_memory_size_bytes: u32,
    max_message_size_bytes: u32,
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
            latest_storage_canister: None,
            storage_canisters_block_ranges: Vec::new(),
            node_max_memory_size_bytes: MAX_CANISTER_STORAGE_BYTES,
            max_message_size_bytes: 0,
            trigger_threshold: BLOCK_ARCHIVE_TRIGGER_THRESHOLD,
            num_blocks_to_archive: BLOCK_ARCHIVE_SIZE,
            cycles_for_archive_creation: CYCLES_PER_AUTO_SCALING,
            archiving_in_progress: false,
        }
    }
}

impl Archive {
    pub fn new(options: ArchiveOptions) -> Self {
        Self {
            storage_canisters: vec![],
            latest_storage_canister: None,
            storage_canisters_block_ranges: vec![],
            node_max_memory_size_bytes: options
                .node_max_memory_size_bytes
                .unwrap_or(1024 * 1024 * 1024),
            max_message_size_bytes: options.max_message_size_bytes.unwrap_or(2 * 1024 * 1024),
            trigger_threshold: options.trigger_threshold,
            num_blocks_to_archive: options.num_blocks_to_archive,
            cycles_for_archive_creation: options.cycles_for_archive_creation.unwrap_or(0),
            archiving_in_progress: false,
        }
    }

    pub fn latest_storage_canister(&self) -> Option<Principal> {
        self.latest_storage_canister
    }
    pub fn last_storage_canister_id(&self) -> Option<Principal> {
        self.storage_canisters.last().copied()
    }

    pub fn last_storage_canister_index(&self) -> usize {
        self.storage_canisters.len() - 1
    }

    pub fn last_storage_canister_range(&self) -> Option<(BlockHeight, BlockHeight)> {
        self.storage_canisters_block_ranges.last().cloned()
    }

    pub fn index(&self) -> Vec<((BlockHeight, BlockHeight), Principal)> {
        self.storage_canisters_block_ranges
            .iter()
            .cloned()
            .zip(self.storage_canisters.clone())
            .collect()
    }

    pub fn archives(&self) -> Vec<ArchiveInfo> {
        self.storage_canisters_block_ranges
            .iter()
            .cloned()
            .zip(self.storage_canisters.clone())
            .map(|((start, end), id)| ArchiveInfo {
                canister_id: id,
                start_block_height: start.clone().into(),
                end_block_height: end.clone().into(),
                num_blocks: (end - start + 1u32).into(),
            })
            .collect()
    }

    pub fn scaling_storage_block_height_offset(&self) -> BlockHeight {
        self.storage_canisters_block_ranges
            .last()
            .map(|(_, height_to)| height_to.clone() + 1u32)
            .unwrap_or_else(|| 0u32.into())
    }

    pub fn storage_canisters(&self) -> &[Principal] {
        &self.storage_canisters
    }

    pub fn storage_canisters_block_ranges(&self) -> &[(BlockHeight, BlockHeight)] {
        &self.storage_canisters_block_ranges
    }

    pub fn pre_append_storage_canister(&mut self, canister_id: Principal) {
        assert!(self.archiving_in_progress);
        assert!(self.latest_storage_canister.is_none());
        self.latest_storage_canister = Some(canister_id);
    }
    pub fn append_scaling_storage_canister(&mut self, canister_id: Principal) {
        assert!(self.archiving_in_progress);
        assert_eq!(canister_id, self.latest_storage_canister.unwrap());
        if self.archiving_in_progress {
            assert!(!self.storage_canisters.contains(&canister_id));
            self.storage_canisters.push(canister_id);
            self.latest_storage_canister = None;
        }
    }

    pub fn update_scaling_storage_blocks_range(
        &mut self,
        storage_index: usize,
        end_block_height: BlockHeight,
    ) {
        assert!(self.archiving_in_progress);

        let last_range: Option<(BlockHeight, BlockHeight)> = self
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
                        .push((0u32.into(), end_block_height)),
                    // If we haven't recorded any heights for this node but
                    // a previous node exists then the current heights
                    // start one above those in the previous node
                    Some((_, last_range_end_block_height)) => self
                        .storage_canisters_block_ranges
                        .push((last_range_end_block_height + 1u32, end_block_height)),
                }
            }
            // We have already inserted some Blocks into this archive node.
            // Hence, we already have a value to work with
            Some(heights) => {
                heights.1 = end_block_height;
            }
        }
    }

    pub fn lock_for_archiving(&mut self) -> bool {
        if self.archiving_in_progress {
            false
        } else {
            self.archiving_in_progress = true;
            true
        }
    }

    pub fn unlock_after_archiving(&mut self) {
        if self.archiving_in_progress {
            self.archiving_in_progress = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    #[should_panic]
    fn test_latest_storage_canister() {
        let archive = Archive::default();
        assert!(archive.latest_storage_canister().is_none());
        assert!(archive.last_storage_canister_id().is_none());
        assert!(archive.storage_canisters().is_empty());
        assert_eq!(archive.storage_canisters_block_ranges().len(), 0);
        assert_eq!(archive.last_storage_canister_index(), 0);
    }

    #[test]
    fn test_lock_for_archiving() {
        let mut archive = Archive::default();

        let lock_res = archive.lock_for_archiving();
        assert!(lock_res);
        let lock_res = archive.lock_for_archiving();
        assert!(!lock_res);
    }

    #[test]
    fn test_unlock_after_archiving() {
        let mut archive = Archive::default();
        archive.lock_for_archiving();
        archive.unlock_after_archiving();
        let res = archive.lock_for_archiving();
        assert!(res);
    }

    #[test]
    #[should_panic]
    fn test_prepend_storage_canister_without_lock_should_panic() {
        let mut archive = Archive::default();
        let storage_canister_id: Principal =
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap();
        archive.pre_append_storage_canister(storage_canister_id);
    }

    #[test]
    fn test_prepend_storage_canister() {
        let mut archive = Archive::default();
        archive.lock_for_archiving();
        let storage_canister_id: Principal =
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap();
        archive.pre_append_storage_canister(storage_canister_id);
        assert_eq!(archive.storage_canisters().len(), 0);
        assert_eq!(archive.storage_canisters_block_ranges().len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_append_storage_canister_without_lock_should_panic() {
        let mut archive = Archive::default();
        archive.lock_for_archiving();
        let storage_canister_id: Principal =
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap();
        archive.pre_append_storage_canister(storage_canister_id);
        archive.unlock_after_archiving();
        archive.append_scaling_storage_canister(storage_canister_id);
    }

    #[test]
    #[should_panic]
    fn test_append_storage_canister_without_pre_append_should_panic() {
        let mut archive = Archive::default();
        archive.lock_for_archiving();
        let storage_canister_id: Principal =
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap();
        archive.append_scaling_storage_canister(storage_canister_id);
    }

    #[test]
    fn test_append_storage_canister() {
        let mut archive = Archive::default();
        archive.lock_for_archiving();
        let storage_canister_id: Principal =
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap();
        archive.pre_append_storage_canister(storage_canister_id);
        archive.append_scaling_storage_canister(storage_canister_id);
        assert_eq!(archive.storage_canisters().len(), 1);
        assert_eq!(archive.storage_canisters_block_ranges().len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_append_storage_canister_with_same_id_should_panic() {
        let mut archive = Archive::default();
        archive.lock_for_archiving();
        let storage_canister_id: Principal =
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap();
        archive.pre_append_storage_canister(storage_canister_id);
        archive.append_scaling_storage_canister(storage_canister_id);
        archive.append_scaling_storage_canister(storage_canister_id);
    }

    #[test]
    fn test_update_storage_canister_block_range() {
        let mut archive = Archive::default();
        archive.lock_for_archiving();
        let storage_canister_id: Principal =
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap();
        archive.pre_append_storage_canister(storage_canister_id);
        archive.append_scaling_storage_canister(storage_canister_id);
        let last_storage_index = archive.last_storage_canister_index();
        assert_eq!(last_storage_index, 0);

        let archived_end_block_height = BigUint::from(100u32);
        archive.update_scaling_storage_blocks_range(
            last_storage_index,
            archived_end_block_height.clone(),
        );

        assert_eq!(archive.storage_canisters().len(), 1);
        assert_eq!(archive.storage_canisters_block_ranges().len(), 1);
        assert_eq!(
            archive.storage_canisters_block_ranges()[last_storage_index].0,
            BigUint::from(0u32)
        );
        assert_eq!(
            archive.storage_canisters_block_ranges()[last_storage_index].1,
            archived_end_block_height
        );

        let archived_end_block_height = BigUint::from(200u32);
        archive.update_scaling_storage_blocks_range(
            last_storage_index,
            archived_end_block_height.clone(),
        );
        assert_eq!(
            archive.storage_canisters_block_ranges()[last_storage_index].0,
            BigUint::from(0u32)
        );
        assert_eq!(
            archive.storage_canisters_block_ranges()[last_storage_index].1,
            archived_end_block_height
        );

        assert_eq!(
            archive.scaling_storage_block_height_offset(),
            archived_end_block_height.clone() + 1u32
        );

        archive.unlock_after_archiving();

        let new_storage_canister_id: Principal =
            "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                .parse()
                .unwrap();

        archive.lock_for_archiving();
        archive.pre_append_storage_canister(new_storage_canister_id);
        archive.append_scaling_storage_canister(new_storage_canister_id);
        let last_storage_index = archive.last_storage_canister_index();
        assert_eq!(last_storage_index, 1);

        let new_archived_end_block_height = BigUint::from(300u32);
        archive.update_scaling_storage_blocks_range(
            last_storage_index,
            new_archived_end_block_height.clone(),
        );

        assert_eq!(archive.storage_canisters().len(), 2);
        assert_eq!(archive.storage_canisters_block_ranges().len(), 2);

        let indexes = archive.index();

        assert_eq!(indexes.len(), 2);
        assert_eq!(indexes[0].0, (BigUint::from(0u32), BigUint::from(200u32)));
        assert_eq!(indexes[0].1, storage_canister_id);
        assert_eq!(indexes[1].1, new_storage_canister_id);

        let archives = archive.archives();

        assert_eq!(archives.len(), 2);
        assert_eq!(archives[0].start_block_height, Nat::from(0u32));
        assert_eq!(archives[1].start_block_height, Nat::from(201u32));
        assert_eq!(archives[0].end_block_height, Nat::from(200u32));
        assert_eq!(archives[0].canister_id, storage_canister_id);
        assert_eq!(archives[1].canister_id, new_storage_canister_id);
        assert_eq!(
            archives[0].num_blocks,
            Nat::from(archived_end_block_height.clone() + 1u32)
        );
        assert_eq!(
            archives[1].num_blocks,
            Nat::from(new_archived_end_block_height - archived_end_block_height)
        );
    }
}
