use crate::{BlockHash, EncodedBlock};
use candid::Deserialize;
use serde::Serialize;
/// Stores a chain of transactions with their metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub blocks: Vec<EncodedBlock>,
    pub last_hash: Option<BlockHash>,
    pub last_timestamp: u64,
    /// How many blocks have been sent to the archive
    pub num_archived_blocks: u64,
}
