use crate::{Archive, Block, BlockHash, BlockHeight, CommonResult, DFTError, EncodedBlock};
use candid::{Deserialize, Principal};
use serde::Serialize;
use std::collections::VecDeque;
use std::convert::TryInto;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Blockchain {
    pub blocks: Vec<EncodedBlock>,
    pub last_hash: Option<BlockHash>,
    pub last_timestamp: u64,
    pub archive: Archive,
    pub num_archived_blocks: BlockHeight,
}

impl Default for Blockchain {
    fn default() -> Self {
        Blockchain {
            blocks: Vec::new(),
            last_hash: None,
            last_timestamp: 0,
            archive: Archive::default(),
            num_archived_blocks: 0u32.into(),
        }
    }
}

impl Blockchain {
    pub fn add_block(&mut self, token_id: &Principal, block: Block) -> CommonResult<BlockHeight> {
        let raw_block = block.clone().encode()?;
        self.add_block_with_encoded(token_id, block, raw_block)
    }

    pub fn add_block_with_encoded(
        &mut self,
        token_id: &Principal,
        block: Block,
        encoded_block: EncodedBlock,
    ) -> CommonResult<BlockHeight> {
        if block.parent_hash != self.last_hash {
            return Err(DFTError::ApplyBlockFailedByParentHashDoesNotMatch);
        }
        if block.timestamp < self.last_timestamp {
            return Err(DFTError::ApplyBlockFailedByInvalidTimestamp);
        }
        self.last_hash = Some(encoded_block.hash_with_token_id(token_id));
        self.last_timestamp = block.timestamp;
        self.blocks.push(encoded_block);
        Ok(self.chain_length() - 1u32)
    }

    pub fn get(&self, height: BlockHeight) -> Option<&EncodedBlock> {
        if height < self.num_archived_blocks() {
            None
        } else {
            let index: usize = (height - self.num_archived_blocks()).try_into().unwrap();
            self.blocks.get(index)
        }
    }

    pub fn last(&self) -> Option<&EncodedBlock> {
        self.blocks.last()
    }

    pub fn num_archived_blocks(&self) -> BlockHeight {
        self.num_archived_blocks.clone()
    }

    pub fn num_unarchived_blocks(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn local_block_range(&self) -> std::ops::Range<BlockHeight> {
        self.num_archived_blocks.clone()..self.num_archived_blocks.clone() + self.blocks.len()
    }

    pub fn chain_length(&self) -> BlockHeight {
        self.num_archived_blocks() + self.num_unarchived_blocks()
    }

    pub fn remove_archived_blocks(&mut self, len: usize) {
        if len > self.blocks.len() {
            panic!(
                "Asked to remove more blocks than present. Present: {}, to remove: {}",
                self.blocks.len(),
                len.to_string()
            );
        }
        self.blocks = self.blocks.split_off(len.clone());
        self.num_archived_blocks += len;
    }

    pub fn get_blocks_for_archiving(
        &self,
        trigger_threshold: usize,
        num_blocks_to_archive: usize,
    ) -> VecDeque<EncodedBlock> {
        let num_blocks_unarchived = self.num_unarchived_blocks() as usize;
        if num_blocks_unarchived < trigger_threshold {
            return VecDeque::new();
        }

        let blocks_to_archive: VecDeque<EncodedBlock> = VecDeque::from(
            self.blocks[0..num_blocks_to_archive.min(num_blocks_unarchived)].to_vec(),
        );
        blocks_to_archive
    }
}
