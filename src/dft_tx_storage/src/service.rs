use crate::{state::STATE, types::StorageInfo};
use candid::Principal;
use dft_types::constants::MAX_BLOCKS_PER_REQUEST;
use dft_types::{BlockListResult, BlockResult, CandidBlock, CommonResult, DFTError, EncodedBlock};
use num_bigint::BigUint;
use std::convert::TryInto;

pub fn init(dft_id: Principal, dft_tx_start_index: BigUint, now: u64) {
    STATE.with(|s| {
        let mut setting = s.storage_setting.borrow_mut();
        setting.initialize(dft_id, dft_tx_start_index, now);
    });
}

pub fn batch_append(caller: &Principal, blocks: Vec<EncodedBlock>, now: u64) -> CommonResult<()> {
    STATE.with(|s| {
        let setting = s.storage_setting.borrow();
        setting.only_allow_token_canister(caller)?;

        let mut block_archive = s.block_archive.borrow_mut();
        block_archive.batch_append(blocks, now);
        Ok(())
    })
}

pub fn get_block_by_height(block_height: BigUint) -> BlockResult {
    STATE.with(|s| {
        let setting = s.storage_setting.borrow();
        let block_archive = s.block_archive.borrow();

        if block_height < *setting.block_height_offset()
            || block_height
                > (setting.block_height_offset().clone() + block_archive.total_blocks_count())
        {
            BlockResult::Err(DFTError::NonExistentBlockHeight.into())
        } else {
            let inner_index: usize = (block_height - setting.block_height_offset())
                .try_into()
                .unwrap();

            match block_archive.blocks().get(inner_index) {
                Some(block) => match block.decode() {
                    Ok(de_block) => BlockResult::Ok(de_block.into()),
                    Err(e) => BlockResult::Err(e.into()),
                },
                None => BlockResult::Err(DFTError::NonExistentBlockHeight.into()),
            }
        }
    })
}

pub fn get_blocks_by_query(start_block_height: BigUint, size: usize) -> BlockListResult {
    let size = MAX_BLOCKS_PER_REQUEST.min(size as u32);
    STATE.with(|s| {
        let setting = s.storage_setting.borrow();
        let block_archive = s.block_archive.borrow();
        let total_blocks_count = block_archive.total_blocks_count();

        if start_block_height < *setting.block_height_offset()
            || start_block_height > (setting.block_height_offset() + total_blocks_count)
        {
            BlockListResult::Err(DFTError::NonExistentBlockHeight.into())
        } else {
            let inner_index_start: u64 = (start_block_height - setting.block_height_offset())
                .try_into()
                .unwrap();

            let inner_index_end = inner_index_start + size as u64;
            let inner_index_end = if inner_index_end > total_blocks_count {
                block_archive.total_blocks_count()
            } else {
                inner_index_end
            };

            let mut res: Vec<CandidBlock> = Vec::new();
            for i in inner_index_start..inner_index_end {
                let block = block_archive.get_block(i).unwrap();
                res.push(block.decode().unwrap().into());
            }
            BlockListResult::Ok(res)
        }
    })
}

pub fn get_storage_info() -> StorageInfo {
    STATE.with(|s| {
        let setting = s.storage_setting.borrow();
        let block_archive = s.block_archive.borrow();

        StorageInfo {
            token_id: *setting.token_id(),
            block_height_offset: setting.block_height_offset().clone().into(),
            total_blocks_count: block_archive.total_blocks_count().into(),
            total_block_size_bytes: *block_archive.total_block_size_bytes() as u64,
            cycles: 0,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Nat;
    use dft_types::{Block, CandidOperation, ErrorInfo, Operation, TokenHolder, Transaction};
    use std::ops::Add;

    #[test]
    fn test_init() {
        let test_token_id: Principal = "rwlgt-iiaaa-aaaaa-aaaaa-cai".parse().unwrap();
        let block_height_offset = BigUint::from(1u8);
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();
        init(test_token_id.clone(), block_height_offset.clone(), now);

        let storage_info = get_storage_info();
        assert_eq!(storage_info.token_id, test_token_id);
        assert_eq!(storage_info.block_height_offset.0, block_height_offset);
        assert_eq!(storage_info.total_blocks_count.0, BigUint::from(0u8));
        assert_eq!(storage_info.total_block_size_bytes, 0);
    }

    #[test]
    #[should_panic]
    fn test_init_twice_should_panic() {
        let test_token_id: Principal = "rwlgt-iiaaa-aaaaa-aaaaa-cai".parse().unwrap();
        let block_height_offset = BigUint::from(1u8);
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();
        init(
            test_token_id.clone(),
            block_height_offset.clone(),
            now.clone(),
        );
        init(test_token_id, block_height_offset, now);
    }

    #[test]
    fn test_get_block() {
        let test_token_id: Principal = "rwlgt-iiaaa-aaaaa-aaaaa-cai".parse().unwrap();
        let block_height_offset = BigUint::from(110u8);
        let from: Principal = "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
            .parse()
            .unwrap();
        let to: Principal = "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
            .parse()
            .unwrap();
        let from_holder = TokenHolder::new(from.clone(), None);
        let to_holder = TokenHolder::new(to.clone(), None);

        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();
        init(test_token_id, block_height_offset.clone(), now);

        let mut blocks = Vec::new();
        let mut pre_block: Option<Block> = None;
        let loop_times = 500u64;

        for i in 0..loop_times {
            let pre_block_hash = if pre_block.is_some() {
                Some(
                    pre_block
                        .unwrap()
                        .encode()
                        .unwrap()
                        .hash_with_token_id(&test_token_id),
                )
            } else {
                None
            };
            let block = Block::new_from_transaction(
                &test_token_id,
                pre_block_hash,
                Transaction {
                    operation: Operation::Transfer {
                        caller: from_holder.clone(),
                        from: from_holder,
                        to: to_holder,
                        value: (1000u64 + i).into(),
                        fee: 1u32.into(),
                    },
                    created_at: now.clone() + i,
                },
                now.clone() + i,
            );

            pre_block = Some(block.clone());
            blocks.push(block.encode().unwrap());
        }

        let res = batch_append(&test_token_id, blocks, now);
        assert_eq!(res.is_ok(), true);

        for i in 0..loop_times {
            let block_height = BigUint::from(i);
            let block = get_block_by_height(block_height_offset.clone() + block_height.clone());
            match block {
                BlockResult::Ok(b) => match b.transaction.operation {
                    CandidOperation::Transfer { value, .. } => {
                        assert_eq!(value, Nat::from(1000u64 + i));
                    }
                    _ => panic!("unexpected operation"),
                },
                _ => {
                    assert!(
                        false,
                        "unexpected result,i={},add ={}",
                        i,
                        block_height_offset.clone() + block_height.clone()
                    );
                }
            }
        }

        let block = get_block_by_height(block_height_offset.clone().add(loop_times));

        match block {
            BlockResult::Err(e) => {
                let comp_err: ErrorInfo = DFTError::NonExistentBlockHeight.into();
                assert_eq!(e, comp_err);
            }
            _ => panic!("unexpected result"),
        }

        let blocks = get_blocks_by_query(block_height_offset.clone().add(100u32), 300);

        match blocks {
            BlockListResult::Ok(b) => {
                assert_eq!(b.len(), 100);
            }
            BlockListResult::Err(e) => {
                assert!(false, "unexpected result,{:?}", e);
            }
        }

        let blocks = get_blocks_by_query(block_height_offset.clone().add(1000000u32), 300);

        match blocks {
            BlockListResult::Err(e) => {
                let comp_err: ErrorInfo = DFTError::NonExistentBlockHeight.into();
                assert_eq!(e, comp_err);
            }
            _ => panic!("unexpected result"),
        }

        let storage_info = get_storage_info();
        assert_eq!(storage_info.total_blocks_count, loop_times as u32);
    }
}
