use super::internal_service::{
    _transfer, calc_transfer_fee, charge_approve_fee, verified_created_at,
};
use crate::state::STATE;
use candid::Principal;
use dft_types::constants::MAX_BLOCKS_PER_REQUEST;
use dft_types::*;
use dft_utils::*;
use std::collections::HashMap;
use std::convert::TryInto;

pub fn token_id() -> Principal {
    STATE.with(|s| *s.token_setting.borrow().token_id())
}
pub fn metadata() -> TokenMetadata {
    STATE.with(|s| s.token_setting.borrow().metadata())
}
pub fn owner() -> Principal {
    STATE.with(|s| s.token_setting.borrow().owner())
}

pub fn fee() -> TokenFee {
    STATE.with(|s| s.token_setting.borrow().fee())
}

pub fn fee_to() -> TokenHolder {
    STATE.with(|s| s.token_setting.borrow().fee_to())
}
pub fn desc() -> HashMap<String, String> {
    STATE.with(|s| s.token_desc.borrow().get_all())
}
pub fn logo() -> Option<Vec<u8>> {
    STATE.with(|s| s.token_setting.borrow().logo())
}
pub fn name() -> String {
    STATE.with(|s| s.token_setting.borrow().name())
}

pub fn symbol() -> String {
    STATE.with(|s| s.token_setting.borrow().symbol())
}

pub fn decimals() -> u8 {
    STATE.with(|s| s.token_setting.borrow().decimals())
}

pub fn total_supply() -> TokenAmount {
    STATE.with(|s| s.balances.borrow().total_supply())
}

pub fn balance_of(holder: &TokenHolder) -> TokenAmount {
    STATE.with(|s| s.balances.borrow().balance_of(holder))
}

pub fn allowance(holder: &TokenHolder, spender: &TokenHolder) -> TokenAmount {
    STATE.with(|s| s.allowances.borrow().allowance(holder, spender))
}

pub fn allowances_of(owner: &TokenHolder) -> Vec<(TokenHolder, TokenAmount)> {
    STATE.with(|s| s.allowances.borrow().allowances_of(owner))
}
#[allow(clippy::too_many_arguments)]
pub fn approve(
    caller: &Principal,
    owner: &TokenHolder,
    spender: &TokenHolder,
    value: TokenAmount,
    created_at: Option<u64>,
    now: u64,
) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
    verified_created_at(&created_at, &now)?;
    let mut approve_fee: TokenAmount = 0u32.into();
    let res = STATE.with(|s| {
        let settings = s.token_setting.borrow();
        let mut blockchain = s.blockchain.borrow_mut();
        let mut allowances = s.allowances.borrow_mut();
        let balances = s.balances.borrow_mut();
        settings.not_allow_anonymous(caller)?;
        let num_purged = blockchain.tx_window.purge_old_transactions(now);
        if num_purged == 0 {
            blockchain.tx_window.throttle_check(now)?
        }

        let created_at = created_at.unwrap_or(now);
        approve_fee = settings.fee().calc_approve_fee(&value);
        if balances.balance_of(owner) < approve_fee {
            Err(DFTError::InsufficientBalance)
        } else {
            let tx = Transaction {
                operation: Operation::Approve {
                    caller: *caller,
                    owner: *owner,
                    spender: *spender,
                    value: value.clone(),
                    fee: approve_fee.clone(),
                },
                created_at,
            };
            let res = blockchain.add_tx_to_block(settings.token_id(), tx, now)?;
            allowances.credit(owner, spender, value.clone());
            Ok(res)
        }
    })?;
    if approve_fee > 0u32.into() {
        charge_approve_fee(owner, approve_fee)?;
    }
    Ok(res)
}

#[allow(clippy::too_many_arguments)]
pub fn transfer_from(
    caller: &Principal,
    from: &TokenHolder,
    spender: &TokenHolder,
    to: &TokenHolder,
    value: TokenAmount,
    created_at: Option<u64>,
    now: u64,
) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
    verified_created_at(&created_at, &now)?;
    let decreased_allowance = STATE.with(|s| {
        let settings = s.token_setting.borrow();
        let allowances = s.allowances.borrow();
        settings.not_allow_anonymous(caller)?;
        let transfer_fee = calc_transfer_fee(&value);
        // get spenders allowance
        let spender_allowance = allowances.allowance(from, spender);
        let decreased_allowance = value.clone() + transfer_fee;
        // check allowance
        if spender_allowance < decreased_allowance {
            return Err(DFTError::InsufficientAllowance);
        }
        Ok(decreased_allowance)
    })?;
    let created_at = created_at.unwrap_or(now);

    let transfer_res = _transfer(spender, from, to, value, created_at, now)?;

    STATE.with(|s| {
        let mut allowances = s.allowances.borrow_mut();
        // debit the spender's allowance
        allowances.debit(from, spender, decreased_allowance)
    })?;

    Ok(transfer_res)
}

pub fn transfer(
    caller: &Principal,
    from: &TokenHolder,
    to: &TokenHolder,
    value: TokenAmount,
    created_at: Option<u64>,
    now: u64,
) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
    verified_created_at(&created_at, &now)?;
    STATE.with(|s| s.token_setting.borrow().not_allow_anonymous(caller))?;
    let created_at = created_at.unwrap_or(now);
    _transfer(from, from, to, value, created_at, now)
}

pub fn token_info() -> TokenInfo {
    STATE.with(|s| {
        let settings = s.token_setting.borrow();
        let allowances = s.allowances.borrow();
        let balances = s.balances.borrow();
        let blockchain = s.blockchain.borrow();
        TokenInfo {
            owner: settings.owner(),
            holders: balances.holder_count(),
            allowance_size: allowances.allowance_size(),
            fee_to: settings.fee_to(),
            block_height: blockchain.chain_length().into(),
            storages: blockchain.archive.storage_canisters().to_vec(),
            cycles: 0,
            certificate: None,
        }
    })
}

pub fn token_metrics() -> TokenMetrics {
    STATE.with(|s| {
        let balances = s.balances.borrow();
        let allowances = s.allowances.borrow();
        let blockchain = s.blockchain.borrow();
        TokenMetrics {
            holders: balances.holder_count(),
            total_block_count: blockchain.chain_length().into(),
            local_block_count: (blockchain.blocks.len() as u64).into(),
            allowance_size: allowances.allowance_size(),
        }
    })
}

pub fn block_by_height(block_height: BlockHeight) -> BlockResult {
    STATE.with(|s| {
        let blockchain = s.blockchain.borrow();
        if block_height > blockchain.chain_length() {
            return BlockResult::Err(DFTError::NonExistentBlockHeight.into());
        }
        if block_height < blockchain.num_archived_blocks() {
            let index = blockchain.archive.index();
            let result = index.binary_search_by(|((from, to), _)| {
                // If within the range we've found the right node
                if *from <= block_height && block_height <= *to {
                    std::cmp::Ordering::Equal
                } else if *from < block_height {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            return match result {
                Ok(i) => BlockResult::Forward(index[i].1),
                Err(_) => BlockResult::Err(DFTError::NonExistentBlockHeight.into()),
            };
        }

        let inner_index: usize = (block_height - blockchain.num_archived_blocks())
            .try_into()
            .unwrap();

        match blockchain.blocks.get(inner_index) {
            Some(encoded_block) => match encoded_block.decode() {
                Ok(block) => BlockResult::Ok(block.into()),
                Err(e) => BlockResult::Err(e.into()),
            },
            _ => BlockResult::Err(DFTError::NonExistentBlockHeight.into()),
        }
    })
}

pub fn blocks_by_query(start: BlockHeight, count: usize) -> QueryBlocksResult {
    let requested_range = range_utils::make_range(start, count);
    STATE.with(|s| {
        let blockchain = s.blockchain.borrow();
        let local_range = blockchain.local_block_range();
        let effective_local_range = range_utils::head(
            &range_utils::intersect(&requested_range, &local_range),
            MAX_BLOCKS_PER_REQUEST as usize,
        );

        let local_start: usize = (effective_local_range.start.clone() - local_range.start)
            .try_into()
            .unwrap();
        let range_len: usize = range_utils::range_len(&effective_local_range)
            .try_into()
            .unwrap();
        let local_end = local_start + range_len;

        let local_blocks: Vec<CandidBlock> = blockchain.blocks[local_start..local_end]
            .iter()
            .map(|enc_block| -> CandidBlock {
                enc_block
                    .decode()
                    .expect("bug: failed to decode encoded block")
                    .into()
            })
            .collect();

        let archived_blocks_range = requested_range.start..effective_local_range.start.clone();

        let archived_blocks = blockchain
            .archive
            .index()
            .iter()
            .filter_map(|((from, to), canister_id)| {
                let slice = range_utils::intersect(
                    &(from.clone()..to.clone() + 1u32),
                    &archived_blocks_range,
                );
                (!slice.is_empty()).then(|| ArchivedBlocksRange {
                    start: slice.start.clone().into(),
                    length: range_utils::range_len(&slice).try_into().unwrap(),
                    storage_canister_id: *canister_id,
                })
            })
            .collect();

        let chain_length = blockchain.chain_length();

        QueryBlocksResult {
            chain_length: chain_length.into(),
            certificate: None,
            blocks: local_blocks,
            first_block_index: effective_local_range.start.into(),
            archived_blocks,
        }
    })
}

pub fn archives() -> Vec<ArchiveInfo> {
    STATE.with(|s| {
        let blockchain = s.blockchain.borrow();
        blockchain.archive.archives()
    })
}
