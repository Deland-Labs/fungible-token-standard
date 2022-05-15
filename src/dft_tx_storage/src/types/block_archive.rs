use dft_types::*;
use getset::{Getters, Setters};

#[derive(Default, Debug, Getters, Setters)]
#[getset(get = "pub")]
pub struct BlockArchive {
    blocks: Vec<EncodedBlock>,
    total_block_size_bytes: usize,
    last_update_timestamp: u64,
}

impl BlockArchive {
    #[allow(dead_code)]
    pub fn append(&mut self, block: EncodedBlock, now: u64) {
        self.total_block_size_bytes += block.size_bytes();
        self.blocks.push(block);
        self.last_update_timestamp = now;
    }

    pub fn batch_append(&mut self, blocks: Vec<EncodedBlock>, now: u64) {
        // update total block_size_bytes
        for block in &blocks {
            self.total_block_size_bytes += block.size_bytes();
        }
        self.blocks.extend(blocks);
        self.last_update_timestamp = now;
    }

    pub fn total_blocks_count(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn get_block(&self, index: u64) -> Option<EncodedBlock> {
        self.blocks.get(index as usize).cloned()
    }
}

impl StableState for BlockArchive {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(&(
            &self.blocks,
            &self.total_block_size_bytes,
            &self.last_update_timestamp,
        ))
        .unwrap()
    }

    fn decode(bytes: Vec<u8>) -> Result<Self, String> {
        let (blocks, total_block_size_bytes, last_update_timestamp): (
            Vec<EncodedBlock>,
            usize,
            u64,
        ) = bincode::deserialize(&bytes).unwrap();

        Ok(BlockArchive {
            blocks,
            total_block_size_bytes,
            last_update_timestamp,
        })
    }
}

#[cfg(test)]
mod tests {
    use candid::Principal;
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn test_block_archive() {
        let mut block_archive = BlockArchive::default();
        let test_token_id: Principal = "rwlgt-iiaaa-aaaaa-aaaaa-cai".parse().unwrap();
        let from: Principal = "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
            .parse()
            .unwrap();
        let to: Principal = "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
            .parse()
            .unwrap();
        let from_holder = TokenHolder::new(from.clone(), None);
        let to_holder = TokenHolder::new(to.clone(), None);
        let genesis_block_hash = dft_utils::sha256::compute_hash(test_token_id.as_slice());
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();

        let mut total_byte_size: usize = 0;
        for i in 0..5u64 {
            let block = Block::new_from_transaction(
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
                    created_at: now.clone() + i,
                },
                now.clone() + i,
            );
            let encoded_block = block.encode().unwrap();
            total_byte_size += encoded_block.size_bytes();
            block_archive.append(encoded_block, now.clone() + i);
        }
        assert_eq!(block_archive.total_blocks_count(), 5);
        assert_eq!(block_archive.total_block_size_bytes, total_byte_size);
        assert_eq!(block_archive.last_update_timestamp, now.clone() + 4);

        let encoded_block_archive = block_archive.encode();
        let decoded_block_archive = BlockArchive::decode(encoded_block_archive).unwrap();
        assert_eq!(decoded_block_archive.blocks.len(), 5);
        assert_eq!(
            decoded_block_archive.total_block_size_bytes,
            total_byte_size
        );
        assert_eq!(decoded_block_archive.last_update_timestamp, now.clone() + 4);

        let mut block_archive_2 = BlockArchive::default();
        block_archive_2.batch_append(decoded_block_archive.blocks, now.clone() + 5);

        assert_eq!(block_archive_2.total_blocks_count(), 5);
        assert_eq!(block_archive_2.total_block_size_bytes, total_byte_size);
        assert_eq!(block_archive_2.last_update_timestamp, now.clone() + 5);
    }
}
