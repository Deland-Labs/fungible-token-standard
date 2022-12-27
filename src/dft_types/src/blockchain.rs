use std::collections::VecDeque;
use std::convert::TryInto;

use candid::{Deserialize, Principal};
use serde::Serialize;

use crate::*;

#[derive(Clone, Deserialize, Serialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Blockchain {
    pub blocks: Vec<EncodedBlock>,
    pub tx_window: TokenTransactionWindow,
    pub last_hash: Option<BlockHash>,
    pub last_timestamp: u64,
    pub archive: Archive,
    pub num_archived_blocks: BlockHeight,
}

impl Default for Blockchain {
    fn default() -> Self {
        Blockchain {
            blocks: Vec::new(),
            tx_window: TokenTransactionWindow::new(),
            last_hash: None,
            last_timestamp: 0,
            archive: Archive::default(),
            num_archived_blocks: 0u32.into(),
        }
    }
}

impl Blockchain {
    pub fn add_tx_to_block(
        &mut self,
        token_id: &Principal,
        tx: InnerTransaction,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
        let block = InnerBlock::new_from_transaction(token_id, self.last_hash, tx, now);
        self.add_block(token_id, block)
    }
    fn add_block(
        &mut self,
        token_id: &Principal,
        block: InnerBlock,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
        let tx_hash = block.transaction.hash_with_token_id(token_id);
        if self.tx_window.contains_transaction(tx_hash) {
            return Err(DFTError::TxDuplicate);
        }
        let raw_block = block.clone().encode()?;
        let block_timestamp = block.timestamp;
        let height = self.add_block_with_encoded(token_id, block, raw_block)?;
        self.tx_window.push_transaction(
            height.clone(),
            TransactionInfo {
                block_timestamp,
                tx_hash,
            },
        );
        let res = (height, self.last_hash.unwrap(), tx_hash);
        Ok(res)
    }

    fn add_block_with_encoded(
        &mut self,
        token_id: &Principal,
        block: InnerBlock,
        encoded_block: EncodedBlock,
    ) -> CommonResult<BlockHeight> {
        if self.last_hash.is_some() && block.parent_hash != self.last_hash.unwrap() {
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

    pub fn num_archived_blocks(&self) -> BlockHeight {
        self.num_archived_blocks.clone()
    }

    pub fn num_unarchived_blocks(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn local_block_range(&self) -> std::ops::Range<BlockHeight> {
        self.num_archived_blocks.clone()
            ..self.num_archived_blocks.clone() + self.blocks.len() - 1u32
    }

    pub fn chain_length(&self) -> BlockHeight {
        self.num_archived_blocks() + self.num_unarchived_blocks()
    }

    pub fn remove_archived_blocks(&mut self, len: usize) {
        if len > self.blocks.len() {
            panic!(
                "Asked to remove more blocks than present. Present: {}, to remove: {}",
                self.blocks.len(),
                len
            );
        }
        self.blocks = self.blocks.split_off(len);
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

impl StableState for Blockchain {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(&(
            &self.blocks,
            &self.tx_window,
            &self.last_hash,
            &self.last_timestamp,
            &self.archive,
            &self.num_archived_blocks,
        ))
        .unwrap()
    }

    fn decode(bytes: Vec<u8>) -> Result<Self, String> {
        let (blocks, tx_window, last_hash, last_timestamp, archive, num_archived_blocks): (
            Vec<EncodedBlock>,
            TokenTransactionWindow,
            Option<BlockHash>,
            u64,
            Archive,
            BlockHeight,
        ) = bincode::deserialize(&bytes).unwrap();

        Ok(Blockchain {
            blocks,
            tx_window,
            last_hash,
            last_timestamp,
            archive,
            num_archived_blocks,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use dft_utils::range_utils::make_range;

    use super::*;

    #[test]
    fn test_blockchain_encode_decode() {
        let mut blockchain = Blockchain::default();
        let token_id: Principal = "rkp4c-7iaaa-aaaaa-aaaca-cai".parse().unwrap();
        let caller: Principal = "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
            .parse()
            .unwrap();
        let new_owner: Principal =
            "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                .parse()
                .unwrap();
        let now: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();

        let timestamp = now.clone();
        let transaction = InnerTransaction {
            operation: InnerOperation::OwnerModify {
                caller: caller.clone().into(),
                new_owner: new_owner.clone().into(),
            },
            created_at: timestamp.clone(),
        };
        let _ = blockchain.add_tx_to_block(&token_id, transaction.clone(), timestamp.clone());

        // Encode and decode the blockchain
        let encoded_blockchain = blockchain.clone().encode();
        let decoded_blockchain = Blockchain::decode(encoded_blockchain).unwrap();

        // Check that the decoded blockchain is the same as the original
        assert_eq!(blockchain, decoded_blockchain);

        let res = blockchain.add_tx_to_block(&token_id, transaction.clone(), timestamp.clone());

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), DFTError::TxDuplicate)
    }

    #[test]
    fn test_blockchain_archive() {
        let mut blockchain = Blockchain::default();
        let token_id: Principal = "rkp4c-7iaaa-aaaaa-aaaca-cai".parse().unwrap();
        let caller: Principal = "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
            .parse()
            .unwrap();
        let new_owner: Principal =
            "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                .parse()
                .unwrap();
        let now: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();

        // add 3000 txs to blockchain
        for i in 0..=3000u32 {
            let timestamp = now.clone() + i as u64;
            let transaction = InnerTransaction {
                operation: InnerOperation::OwnerModify {
                    caller: caller.clone().into(),
                    new_owner: new_owner.clone().into(),
                },
                created_at: timestamp.clone(),
            };
            let _ = blockchain.add_tx_to_block(&token_id, transaction.clone(), timestamp);
            let inside_block = blockchain.get(i.into()).unwrap();
            assert_eq!(inside_block.decode().unwrap().timestamp, timestamp);
            if i == 1000 {
                // archive the first 1000 txs
                let blocks = blockchain.get_blocks_for_archiving(
                    blockchain.archive.trigger_threshold as usize,
                    blockchain.archive.num_blocks_to_archive as usize,
                );
                assert_eq!(blocks.len(), 0);
                assert_eq!(blockchain.num_archived_blocks(), BigUint::from(0u64));
                assert_eq!(blockchain.num_unarchived_blocks(), (i + 1) as u64);
                assert_eq!(blockchain.chain_length(), BigUint::from((i + 1) as u64));
                assert_eq!(
                    blockchain.local_block_range(),
                    make_range(0u32.into(), 1000)
                );
            }

            if i == 2000 {
                // archive the next 1000 txs
                let blocks = blockchain.get_blocks_for_archiving(
                    blockchain.archive.trigger_threshold as usize,
                    blockchain.archive.num_blocks_to_archive as usize,
                );
                assert_eq!(blocks.len(), 1000);
                blockchain.remove_archived_blocks(blocks.len());
                assert_eq!(
                    blockchain.num_archived_blocks(),
                    BigUint::from(blocks.len() as u64)
                );
                assert_eq!(
                    blockchain.num_unarchived_blocks(),
                    i as u64 - (blocks.len() - 1) as u64
                );
                assert_eq!(blockchain.chain_length(), BigUint::from((i + 1) as u64));
                assert_eq!(
                    blockchain.local_block_range(),
                    make_range(1000u32.into(), 1000)
                );
            }

            if i == 3000 {
                // archive the last 1000 txs
                let blocks = blockchain.get_blocks_for_archiving(
                    blockchain.archive.trigger_threshold as usize,
                    blockchain.archive.num_blocks_to_archive as usize,
                );
                assert_eq!(blocks.len(), 1000);
                blockchain.remove_archived_blocks(blocks.len());
                assert_eq!(blockchain.num_archived_blocks(), BigUint::from(2000u64));
                assert_eq!(blockchain.num_unarchived_blocks(), 1001u64);
                assert_eq!(blockchain.chain_length(), BigUint::from((i + 1) as u64));
                assert_eq!(
                    blockchain.local_block_range(),
                    make_range(2000u32.into(), 1000)
                );
            }
        }
    }
}
