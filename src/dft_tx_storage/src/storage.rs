use std::convert::TryInto;

use candid::Nat;
use candid::{CandidType, Deserialize, Principal};
use dft_types::*;

#[derive(CandidType, Debug, Deserialize)]
pub struct StorageInfo {
    #[serde(rename = "tokenId")]
    pub token_id: Principal,
    #[serde(rename = "blockHeightOffset")]
    pub block_height_offset: Nat,
    #[serde(rename = "totalBlocksCount")]
    pub total_blocks_count: Nat,
    #[serde(rename = "totalBlockSizeBytes")]
    pub total_block_size_bytes: Nat,
    pub cycles: u64,
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct AutoScalingStorage {
    pub token_id: Principal,
    pub block_height_offset: Nat,
    pub blocks: Vec<EncodedBlock>,
    pub total_block_size_bytes: usize,
    pub last_update_timestamp: u64,
}

impl Default for AutoScalingStorage {
    fn default() -> Self {
        AutoScalingStorage {
            token_id: Principal::anonymous(),
            block_height_offset: 0.into(),
            blocks: Vec::new(),
            total_block_size_bytes: 0,
            last_update_timestamp: 0,
        }
    }
}

impl AutoScalingStorage {
    // fn init
    pub fn initialize(&mut self, token_id: Principal, block_height_offset: Nat) {
        self.token_id = token_id;
        self.block_height_offset = block_height_offset;
    }

    // fn only allow token canister
    fn _only_allow_token_canister(&self, caller: &Principal) -> CommonResult<()> {
        if &self.token_id != caller {
            return Err(DFTError::OnlyAllowTokenCanisterCallThisFunction);
        }
        Ok(())
    }

    // fn batch_append
    pub fn batch_append(
        &mut self,
        caller: &Principal,
        blocks: Vec<EncodedBlock>,
    ) -> CommonResult<bool> {
        self._only_allow_token_canister(caller)?;

        // update total block_size_bytes
        for block in &blocks {
            self.total_block_size_bytes += block.size_bytes();
        }

        self.blocks.extend(blocks);
        Ok(true)
    }

    pub fn get_block_by_height(&self, block_height: Nat) -> BlockResult {
        if block_height < self.block_height_offset
            || block_height > (self.block_height_offset.clone() + self.blocks.len())
        {
            BlockResult::Err(DFTError::NonExistentBlockHeight.into())
        } else {
            let inner_index: usize = (block_height - self.block_height_offset.clone())
                .0
                .try_into()
                .unwrap();

            match self.blocks.get(inner_index) {
                Some(block) => match block.decode() {
                    Ok(de_block) => BlockResult::Ok(de_block.into()),
                    Err(e) => BlockResult::Err(e.into()),
                },
                None => BlockResult::Err(DFTError::NonExistentBlockHeight.into()),
            }
        }
    }

    pub fn get_blocks_by_query(&self, start: Nat, size: usize) -> BlockListResult {
        if start < self.block_height_offset
            || start > (self.block_height_offset.clone() + self.blocks.len() as u64)
        {
            BlockListResult::Err(DFTError::NonExistentBlockHeight.into())
        } else {
            let inner_index_start: usize = (start - self.block_height_offset.clone())
                .0
                .try_into()
                .unwrap();

            let inner_index_end = inner_index_start + size;
            let inner_index_end = if inner_index_end > self.blocks.len() {
                self.blocks.len()
            } else {
                inner_index_end
            };

            let mut res: Vec<CandidBlock> = Vec::new();
            for i in inner_index_start..inner_index_end {
                res.push(self.blocks[i].decode().unwrap().into());
            }
            BlockListResult::Ok(res)
        }
    }

    // fn get storage info
    pub fn get_storage_info(&self) -> StorageInfo {
        StorageInfo {
            token_id: self.token_id,
            block_height_offset: self.block_height_offset.clone(),
            total_blocks_count: self.blocks.len().into(),
            total_block_size_bytes: self.total_block_size_bytes.into(),
            cycles: self.last_update_timestamp,
        }
    }

    // fn restore
    pub fn restore(&mut self, data: AutoScalingStorage) {
        self.token_id = data.token_id;
        self.block_height_offset = data.block_height_offset;
        self.blocks = data.blocks;
        self.total_block_size_bytes = data.total_block_size_bytes;
        self.last_update_timestamp = data.last_update_timestamp;
    }
}

//test
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn test_token_id() -> Principal {
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
    }

    #[fixture]
    fn other_token_id() -> Principal {
        Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap()
    }

    #[fixture]
    fn test_start_block_height() -> Nat {
        Nat::from(1234)
    }

    #[fixture]
    fn next_tx_index(test_start_block_height: Nat) -> Nat {
        test_start_block_height + 1
    }

    #[fixture]
    fn now() -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        now as u64
    }

    #[fixture]
    fn test_storage(test_token_id: Principal, test_start_block_height: Nat) -> AutoScalingStorage {
        let mut storage = AutoScalingStorage::default();
        storage.initialize(test_token_id, test_start_block_height);
        storage
    }

    #[fixture]
    fn test_block(test_token_id: Principal, now: u64) -> Block {
        let from =
            Principal::from_text("o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae")
                .unwrap();
        let to =
            Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe")
                .unwrap();
        let from_holder = TokenHolder::new(from.clone(), None);
        let to_holder = TokenHolder::new(to.clone(), None);
        let genesis_block_hash = dft_utils::sha256::compute_hash(test_token_id.as_slice());
        Block::new_from_transaction(
            &test_token_id,
            Some(genesis_block_hash),
            Transaction {
                operation: Operation::Transfer {
                    caller: from_holder.clone(),
                    from: from_holder,
                    to: to_holder,
                    value: 1000u32.into(),
                    fee: 1u32.into(),
                },
                created_at: now,
            },
            now,
        )
    }

    #[fixture]
    fn test_blocks(test_token_id: Principal, now: u64) -> Vec<EncodedBlock> {
        let from =
            Principal::from_text("o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae")
                .unwrap();
        let to =
            Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe")
                .unwrap();
        let from_holder = TokenHolder::new(from.clone(), None);
        let to_holder = TokenHolder::new(to.clone(), None);
        // generate 3 tx records
        let mut blocks = Vec::new();
        let mut last_block: Option<Block> = None;
        for i in 0..3 {
            let parent_hash = if last_block.is_none() {
                dft_utils::sha256::compute_hash(test_token_id.as_slice())
            } else {
                last_block
                    .unwrap()
                    .encode()
                    .unwrap()
                    .hash_with_token_id(&test_token_id)
            };
            let block = Block::new_from_transaction(
                &test_token_id,
                Some(parent_hash),
                Transaction {
                    operation: Operation::Transfer {
                        caller: from_holder.clone(),
                        from: from_holder.clone(),
                        to: to_holder.clone(),
                        value: (1000u32 + i).into(),
                        fee: 1u32.into(),
                    },
                    created_at: now,
                },
                now,
            );
            last_block = Some(block.clone());
            blocks.push(block.encode().unwrap());
        }

        blocks
    }

    // test batch_append
    #[rstest]
    fn test_batch_append(
        test_storage: AutoScalingStorage,
        test_token_id: Principal,
        other_token_id: Principal,
        test_blocks: Vec<EncodedBlock>,
    ) {
        let mut storage = test_storage.clone();
        // append with other token id should fail
        let res = storage.batch_append(&other_token_id, test_blocks.clone());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            DFTError::OnlyAllowTokenCanisterCallThisFunction
        );
        // append with invalid tx record should fail

        // append with test token id should succeed
        let res = storage.batch_append(&test_token_id, test_blocks.clone());
        assert!(res.is_ok());
        assert_eq!(storage.blocks.len(), test_blocks.len());
    }
}
