use candid::{CandidType, Nat, Principal};
use serde::Deserialize;

mod block_archive;
mod storage_setting;

pub use block_archive::BlockArchive;
pub use storage_setting::StorageSetting;

#[derive(CandidType, Debug, Deserialize)]
pub struct StorageInfo {
    #[serde(rename = "tokenId")]
    pub token_id: Principal,
    #[serde(rename = "blockHeightOffset")]
    pub block_height_offset: Nat,
    #[serde(rename = "totalBlocksCount")]
    pub total_blocks_count: Nat,
    #[serde(rename = "totalBlockSizeBytes")]
    pub total_block_size_bytes: u64,
    pub cycles: u64,
}
