use candid::{Deserialize, Principal};
use dft_types::constants::{
    DEFAULT_MAX_TRANSACTIONS_IN_WINDOW, DEFAULT_TRANSACTION_WINDOW, MAX_BLOCKS_PER_REQUEST,
};
use dft_types::*;
use dft_utils::*;
use getset::{Getters, Setters};
use serde::Serialize;
use std::collections::HashMap;
use std::collections::{BTreeMap, VecDeque};
use std::convert::{TryFrom, TryInto};

pub trait TokenStandard {
    fn set_owner(
        &mut self,
        caller: &Principal,
        owner: Principal,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool>;
    fn set_fee(
        &mut self,
        caller: &Principal,
        fee: TokenFee,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool>;
    // set fee to
    fn set_fee_to(
        &mut self,
        caller: &Principal,
        fee_to: TokenHolder,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool>;
    fn set_desc(
        &mut self,
        caller: &Principal,
        descriptions: HashMap<String, String>,
    ) -> CommonResult<bool>;
    fn set_logo(&mut self, caller: &Principal, logo: Option<Vec<u8>>) -> CommonResult<bool>;
    // total supply
    fn total_supply(&self) -> TokenAmount;
    // balance of
    fn balance_of(&self, owner: &TokenHolder) -> TokenAmount;
    // allowance
    fn allowance(&self, owner: &TokenHolder, spender: &TokenHolder) -> TokenAmount;
    // allowances of
    fn allowances_of(&self, owner: &TokenHolder) -> Vec<(TokenHolder, TokenAmount)>;
    // approve
    fn approve(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: TokenAmount,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)>;
    // transfer from
    fn transfer_from(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        spender: &TokenHolder,
        to: &TokenHolder,
        value: TokenAmount,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)>;
    // transfer
    fn transfer(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        to: &TokenHolder,
        value: TokenAmount,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)>;
    // token info
    fn token_info(&self) -> TokenInfo;
    fn token_metrics(&self) -> TokenMetrics;
    fn block_by_height(&self, block_height: BlockHeight) -> BlockResult;
    fn blocks_by_query(&self, start: BlockHeight, count: usize) -> QueryBlocksResult;
    fn archives(&self) -> Vec<ArchiveInfo>;
}

#[derive(Getters, Setters)]
#[getset(get = "pub")]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenBasic {
    // token id
    token_id: Principal,
    // token's logo
    logo: Option<Vec<u8>>,
    owner: Principal,
    fee_to: TokenHolder,
    metadata: TokenMetadata,
    blockchain: Blockchain,
    balances: TokenBalances,
    allowances: TokenAllowances,
    /// Maximum number of transactions which ledger will accept
    /// within the transaction_window.
    max_transactions_in_window: usize,
    /// How long transactions are remembered to detect duplicates.
    transaction_window: u64,
    /// For each transaction, record the block in which the
    /// transaction was created. This only contains transactions from
    /// the last `transaction_window` period.
    transactions_by_hash: BTreeMap<TransactionHash, BlockHeight>,
    /// The transactions in the transaction window, sorted by block
    /// index / block timestamp. (Block timestamps are monotonically
    /// non-decreasing, so this is the same.)
    transactions_by_height: VecDeque<TransactionInfo>,
    // token's desc info : social media, description etc
    desc: TokenDescription,
}

impl Default for TokenBasic {
    fn default() -> Self {
        TokenBasic {
            token_id: Principal::anonymous(),
            logo: None,
            owner: Principal::anonymous(),
            fee_to: TokenHolder::None,
            blockchain: Blockchain::default(),
            metadata: TokenMetadata::default(),
            balances: TokenBalances::default(),
            allowances: TokenAllowances::default(),
            max_transactions_in_window: usize::try_from(DEFAULT_MAX_TRANSACTIONS_IN_WINDOW)
                .unwrap(),
            transaction_window: DEFAULT_TRANSACTION_WINDOW,
            transactions_by_hash: BTreeMap::new(),
            transactions_by_height: VecDeque::new(),
            desc: TokenDescription::default(),
        }
    }
}

impl TokenBasic {
    // check if the caller is anonymous
    pub fn not_allow_anonymous(&self, caller: &Principal) -> CommonResult<()> {
        if caller == &Principal::anonymous() {
            return Err(DFTError::NotAllowAnonymous);
        }
        Ok(())
    }
    // check if the caller is the owner
    pub fn only_owner(&self, caller: &Principal) -> CommonResult<()> {
        self.not_allow_anonymous(caller)?;
        if &self.owner != caller {
            return Err(DFTError::OnlyOwnerAllowCallIt);
        }
        Ok(())
    }

    // verify created at
    pub fn verified_created_at(&self, created_at: &Option<u64>, now: &u64) -> CommonResult<()> {
        if created_at.is_none() {
            return Ok(());
        }
        let created_at_time = created_at.unwrap();
        let now = now.clone();
        if created_at_time + constants::DEFAULT_TRANSACTION_WINDOW < now {
            return Err(DFTError::TxTooOld);
        }
        if created_at_time > now + constants::PERMITTED_DRIFT {
            return Err(DFTError::TxCreatedInFuture);
        }
        Ok(())
    }

    fn throttle_check(&self, now: u64) -> CommonResult<()> {
        let num_in_window = self.transactions_by_height.len();
        // We admit the first half of max_transactions_in_window freely.
        // After that we start throttling on per-second basis.
        // This way we guarantee that at most max_transactions_in_window will
        // get through within the transaction window.
        if num_in_window >= self.max_transactions_in_window / 2 {
            // max num of transactions allowed per second
            let max_rate = (0.5 * self.max_transactions_in_window as f64
                / self.transaction_window as f64)
                .ceil() as usize;

            if self
                .transactions_by_height
                .get(num_in_window.saturating_sub(max_rate))
                .map(|x| x.block_timestamp)
                .unwrap_or_else(|| 0)
                + 10u64.pow(9) // 1 second
                > now
            {
                return Err(DFTError::TooManyTransactionsInReplayPreventionWindow);
            }
        }

        Ok(())
    }
    //charge approve fee
    fn charge_approve_fee(&mut self, approver: &TokenHolder) -> CommonResult<TokenAmount> {
        // check the approver's balance
        // if balance is not enough, return error
        if self.balances.balance_of(approver) < self.metadata().fee().minimum {
            Err(DFTError::InsufficientBalance)
        } else {
            // charge the approver's balance as approve fee
            let fee = self.metadata().fee().minimum.clone();
            let fee_to = self.fee_to.clone();
            self.balances.debit_balance(&approver, fee.clone())?;
            self.balances.credit_balance(&fee_to, fee.clone());
            Ok(fee)
        }
    }

    // charge transfer fee
    fn charge_transfer_fee(
        &mut self,
        transfer_from: &TokenHolder,
        transfer_value: &TokenAmount,
    ) -> CommonResult<TokenAmount> {
        // calc the transfer fee: rate * value
        // compare the transfer fee and minimum fee,get the max value
        let rate_fee = self.metadata().fee().rate.clone() * transfer_value.clone()
            / 10u128.pow(self.metadata().fee().rate_decimals.into());
        let min_fee = self.metadata().fee().minimum.clone();
        let transfer_fee = if rate_fee > min_fee {
            rate_fee
        } else {
            min_fee
        };

        // check the transfer_from's balance
        // if balance is not enough, return error
        if self.balances.balance_of(transfer_from) < transfer_fee {
            Err(DFTError::InsufficientBalance)
        } else {
            let fee_to = self.fee_to.clone();
            self.balances
                .debit_balance(&transfer_from, transfer_fee.clone())?;
            self.balances.credit_balance(&fee_to, transfer_fee.clone());
            Ok(transfer_fee)
        }
    }
    // calc transfer fee
    fn calc_transfer_fee(&self, transfer_value: &TokenAmount) -> TokenAmount {
        // calc the transfer fee: rate * value
        // compare the transfer fee and minimum fee,get the max value
        let fee = self.metadata().fee().rate.clone() * transfer_value.clone()
            / 10u128.pow(self.metadata().fee().rate_decimals.into());
        let min_fee = self.metadata().fee().minimum.clone();
        let max_fee = if fee > min_fee { fee } else { min_fee };
        max_fee
    }

    pub fn last_auto_scaling_storage_canister_id(&self) -> Option<Principal> {
        self.blockchain.archive.last_storage_canister_id()
    }

    pub fn scaling_storage_block_height_offset(&self) -> BlockHeight {
        self.blockchain
            .archive
            .scaling_storage_block_height_offset()
    }

    pub fn remove_archived_blocks(&mut self, num_archived: usize) {
        self.blockchain.remove_archived_blocks(num_archived)
    }

    pub fn lock_for_archiving(&mut self) -> bool {
        self.blockchain.archive.lock_for_archiving()
    }

    pub fn unlock_after_archiving(&mut self) {
        self.blockchain.archive.unlock_after_archiving()
    }

    pub fn append_scaling_storage_canister(&mut self, storage_canister_id: Principal) {
        self.blockchain
            .archive
            .append_scaling_storage_canister(storage_canister_id)
    }

    pub fn update_scaling_storage_blocks_range(
        &mut self,
        storage_canister_index: usize,
        end_block_height: BlockHeight,
    ) {
        self.blockchain
            .archive
            .update_scaling_storage_blocks_range(storage_canister_index, end_block_height)
    }

    /// Removes at most [MAX_TRANSACTIONS_TO_PURGE] transactions older
    /// than `now - transaction_window` and returns the number of pruned
    /// transactions.
    fn purge_old_transactions(&mut self, now: u64) -> usize {
        let mut cnt = 0usize;
        while let Some(TransactionInfo {
                           block_timestamp,
                           tx_hash,
                       }) = self.transactions_by_height.front()
        {
            if *block_timestamp + self.transaction_window + constants::PERMITTED_DRIFT >= now {
                // Stop at a sufficiently recent block.
                break;
            }
            let removed = self.transactions_by_hash.remove(tx_hash);
            assert!(removed.is_some());

            self.transactions_by_height.pop_front();
            cnt += 1;
            if cnt >= constants::MAX_TRANSACTIONS_TO_PURGE {
                break;
            }
        }
        cnt
    }

    fn add_tx_to_block(
        &mut self,
        tx: Transaction,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
        let tx_hash = tx.hash_with_token_id(&self.token_id);

        if let Some(_) = self.transactions_by_hash.get(&tx_hash) {
            return Err(DFTError::TxDuplicate);
        }
        let block = Block::new_from_transaction(self.blockchain.last_hash, tx, now);
        let block_timestamp = block.timestamp;

        let height = self
            .blockchain
            .add_block(&self.token_id, block)
            .expect("failed to add block");

        self.transactions_by_hash.insert(tx_hash, height.clone());
        self.transactions_by_height.push_back(TransactionInfo {
            block_timestamp,
            tx_hash,
        });
        Ok((height, self.blockchain.last_hash.unwrap(), tx_hash))
    }

    //transfer token
    fn _transfer(
        &mut self,
        tx_invoker: &TokenHolder,
        from: &TokenHolder,
        to: &TokenHolder,
        value: TokenAmount,
        created_at: u64,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
        // calc the transfer fee
        let transfer_fee = self.calc_transfer_fee(&value);
        //check the transfer_from's balance, if balance is not enough, return error
        if self.balances.balance_of(from) < value.clone() + transfer_fee.clone() {
            Err(DFTError::InsufficientBalance)
        } else {
            let tx = Transaction {
                operation: Operation::Transfer {
                    caller: tx_invoker.clone(),
                    from: from.clone(),
                    to: to.clone(),
                    value: value.clone(),
                    fee: transfer_fee,
                },
                created_at,
            };
            let res = self.add_tx_to_block(tx, now)?;
            // charge the transfer fee
            self.charge_transfer_fee(from, &value)?;
            // debit the transfer_from's balance
            self.balances.debit_balance(from, value.clone())?;
            // credit the transfer_to's balance
            self.balances.credit_balance(to, value.clone());
            Ok(res)
        }
    }
    // _mint
    pub fn _mint(
        &mut self,
        caller: &Principal,
        to: &TokenHolder,
        value: TokenAmount,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
        self.verified_created_at(&created_at, &now)?;
        let created_at = created_at.unwrap_or(now.clone());
        let tx = Transaction {
            operation: Operation::Transfer {
                caller: TokenHolder::new(caller.clone(), None),
                from: TokenHolder::None,
                to: to.clone(),
                value: value.clone(),
                fee: 0u32.into(),
            },
            created_at,
        };
        let res = self.add_tx_to_block(tx, now)?;
        self.balances.credit_balance(to, value.clone());
        Ok(res)
    }

    // _burn
    pub fn _burn(
        &mut self,
        burner: &TokenHolder,
        value: TokenAmount,
        created_at: u64,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
        // calc the transfer fee,if the burn amount small than minimum fee,return error
        let fee = self.calc_transfer_fee(&value);
        if value < self.metadata().fee().minimum.clone() {
            return Err(DFTError::BurnValueTooSmall);
        }
        //check the burn from holder's balance, if balance is not enough, return error
        if self.balances.balance_of(burner) < value.clone() {
            return Err(DFTError::InsufficientBalance);
        }

        let tx = Transaction {
            operation: Operation::Transfer {
                caller: burner.clone(),
                from: burner.clone(),
                to: TokenHolder::None,
                value: value.clone(),
                fee: fee.clone(),
            },
            created_at,
        };
        let res = self.add_tx_to_block(tx, now)?;
        // burn does not charge the transfer fee
        // debit the burn from holder's balance
        self.balances.debit_balance(burner, value.clone())?;
        Ok(res)
    }

    // _burn_from
    pub fn _burn_from(
        &mut self,
        burner: &TokenHolder,
        from: &TokenHolder,
        value: TokenAmount,
        created_at: u64,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
        // calc the transfer fee,if the burn amount small than minimum fee,return error
        let fee = self.calc_transfer_fee(&value);
        if value < self.metadata().fee().minimum.clone() {
            return Err(DFTError::BurnValueTooSmall);
        }
        //check the burn from holder's balance, if balance is not enough, return error
        if self.balances.balance_of(from) < value.clone() {
            return Err(DFTError::InsufficientBalance);
        } else {
            let tx = Transaction {
                operation: Operation::Transfer {
                    caller: burner.clone(),
                    from: from.clone(),
                    to: TokenHolder::None,
                    value: value.clone(),
                    fee: fee.clone(),
                },
                created_at,
            };
            let res = self.add_tx_to_block(tx, now)?;
            self.allowances.debit(from, burner, value.clone())?;
            // burn does not charge the transfer fee
            // debit the burn from holder's balance
            self.balances.debit_balance(from, value.clone())?;
            Ok(res)
        }
    }
}

//from/to TokenPayload
impl TokenBasic {
    // initialize
    pub fn initialize(
        &mut self,
        owner: &Principal,
        token_id: Principal,
        logo: Option<Vec<u8>>,
        name: String,
        symbol: String,
        decimals: u8,
        fee: TokenFee,
        fee_to: TokenHolder,
        archive_options: Option<ArchiveOptions>,
    ) {
        // check logo type
        if logo.is_some() {
            let _ = get_logo_type(&logo.clone().unwrap())
                .map_err(|_| DFTError::InvalidTypeOrFormatOfLogo)
                .unwrap();
        }

        // set the parameters to token's properties
        self.owner = owner.clone();
        self.token_id = token_id.clone();
        self.metadata =
            TokenMetadata::new(name.clone(), symbol.clone(), decimals.clone(), fee.clone());
        self.logo = logo;
        self.fee_to = fee_to;
        if archive_options.is_some() {
            self.blockchain.archive = Archive::new(archive_options.unwrap());
        }
    }
}

impl TokenStandard for TokenBasic {
    fn set_owner(
        &mut self,
        caller: &Principal,
        new_owner: Principal,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool> {
        self.only_owner(caller)?;
        self.verified_created_at(&created_at, &now)?;

        if self.owner == new_owner {
            return Ok(true);
        }

        let num_purged = self.purge_old_transactions(now);
        if num_purged == 0 {
            self.throttle_check(now)?
        }

        let created_at = created_at.unwrap_or(now.clone());
        let tx = Transaction {
            operation: Operation::OwnerModify {
                caller: caller.clone(),
                new_owner: new_owner.clone(),
            },
            created_at,
        };

        self.add_tx_to_block(tx, now)?;
        self.owner = new_owner;
        Ok(true)
    }

    fn set_fee(
        &mut self,
        caller: &Principal,
        new_fee: TokenFee,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool> {
        self.only_owner(caller)?;
        self.verified_created_at(&created_at, &now)?;

        let num_purged = self.purge_old_transactions(now);
        if num_purged == 0 {
            self.throttle_check(now)?
        }

        let created_at = created_at.unwrap_or(now.clone());
        let tx = Transaction {
            operation: Operation::FeeModify {
                caller: caller.clone(),
                new_fee: new_fee.clone(),
            },
            created_at,
        };

        self.add_tx_to_block(tx, now)?;
        self.metadata.set_fee(new_fee);
        Ok(true)
    }

    fn set_fee_to(
        &mut self,
        caller: &Principal,
        new_fee_to: TokenHolder,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool> {
        self.only_owner(caller)?;
        self.verified_created_at(&created_at, &now)?;

        let num_purged = self.purge_old_transactions(now);
        if num_purged == 0 {
            self.throttle_check(now)?
        }

        let created_at = created_at.unwrap_or(now.clone());
        let tx = Transaction {
            operation: Operation::FeeToModify {
                caller: caller.clone(),
                new_fee_to: new_fee_to.clone(),
            },
            created_at,
        };

        self.add_tx_to_block(tx, now)?;
        self.fee_to = new_fee_to;
        Ok(true)
    }

    fn set_desc(
        &mut self,
        caller: &Principal,
        descriptions: HashMap<String, String>,
    ) -> CommonResult<bool> {
        self.only_owner(caller)?;
        self.desc.set_all(descriptions.clone());
        Ok(true)
    }

    fn set_logo(&mut self, caller: &Principal, logo: Option<Vec<u8>>) -> CommonResult<bool> {
        self.only_owner(caller)?;
        if logo.is_some() {
            get_logo_type(&logo.clone().unwrap())
                .map_err(|_| DFTError::InvalidTypeOrFormatOfLogo)?;
        }
        self.logo = logo.clone();
        Ok(true)
    }

    fn total_supply(&self) -> TokenAmount {
        self.balances.total_supply()
    }

    fn balance_of(&self, holder: &TokenHolder) -> TokenAmount {
        self.balances.balance_of(holder)
    }

    fn allowance(&self, holder: &TokenHolder, spender: &TokenHolder) -> TokenAmount {
        self.allowances.allowance(holder, spender)
    }

    fn allowances_of(&self, owner: &TokenHolder) -> Vec<(TokenHolder, TokenAmount)> {
        self.allowances.allowances_of(owner)
    }

    fn approve(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: TokenAmount,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
        self.not_allow_anonymous(caller)?;
        self.verified_created_at(&created_at, &now)?;

        let num_purged = self.purge_old_transactions(now);
        if num_purged == 0 {
            self.throttle_check(now)?
        }

        let created_at = created_at.unwrap_or(now.clone());
        let approve_fee = self.charge_approve_fee(owner)?;
        let tx = Transaction {
            operation: Operation::Approve {
                caller: caller.clone(),
                owner: owner.clone(),
                spender: spender.clone(),
                value: value.clone(),
                fee: approve_fee,
            },
            created_at,
        };
        let res = self.add_tx_to_block(tx, now)?;
        self.allowances.credit(owner, spender, value.clone());
        return Ok(res);
    }

    fn transfer_from(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        spender: &TokenHolder,
        to: &TokenHolder,
        value: TokenAmount,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
        self.not_allow_anonymous(caller)?;
        self.verified_created_at(&created_at, &now)?;
        let created_at = created_at.unwrap_or(now.clone());
        let transfer_fee = self.calc_transfer_fee(&value);
        // get spenders allowance
        let spender_allowance = self.allowances.allowance(from, spender);
        let decreased_allowance = value.clone() + transfer_fee.clone();
        // check allowance
        if spender_allowance < decreased_allowance.clone() {
            return Err(DFTError::InsufficientAllowance);
        }
        // debit the spender's allowance
        self.allowances
            .debit(from, spender, decreased_allowance.clone())?;

        return self._transfer(spender, from, to, value, created_at, now);
    }

    fn transfer(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        to: &TokenHolder,
        value: TokenAmount,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
        self.not_allow_anonymous(caller)?;
        self.verified_created_at(&created_at, &now)?;

        let num_purged = self.purge_old_transactions(now);
        if num_purged == 0 {
            self.throttle_check(now)?
        }
        let created_at = created_at.unwrap_or(now.clone());
        self._transfer(&from, from, to, value, created_at, now)
    }

    fn token_info(&self) -> TokenInfo {
        TokenInfo {
            owner: self.owner.clone(),
            holders: self.balances.holder_count(),
            allowance_size: self.allowances.allowance_size(),
            fee_to: self.fee_to.clone(),
            block_height: self.blockchain.chain_length().into(),
            storages: self.blockchain.archive.storage_canisters().to_vec(),
            cycles: 0,
            certificate: None,
        }
    }

    fn token_metrics(&self) -> TokenMetrics {
        TokenMetrics {
            holders: self.balances.holder_count(),
            total_block_count: self.blockchain.chain_length().into(),
            local_block_count: (self.blockchain.blocks.len() as u64).into(),
            allowance_size: self.allowances.allowance_size(),
        }
    }

    fn block_by_height(&self, block_height: BlockHeight) -> BlockResult {
        if block_height > self.blockchain.chain_length() {
            return BlockResult::Err(DFTError::NonExistentBlockHeight.into());
        }
        if block_height < self.blockchain.num_archived_blocks() {
            let index = self.blockchain.archive.index();
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
                Ok(i) => {
                    BlockResult::Forward(index[i].1)
                }
                Err(_) => {
                    BlockResult::Err(DFTError::NonExistentBlockHeight.into())
                }
            };
        }

        let inner_index: usize = (block_height - self.blockchain.num_archived_blocks())
            .try_into()
            .unwrap();

        match &self.blockchain.blocks.get(inner_index) {
            Some(encoded_block) => match encoded_block.clone().decode() {
                Ok(block) => BlockResult::Ok(block.into()),
                Err(e) => BlockResult::Err(e.into()),
            },
            _ => BlockResult::Err(DFTError::NonExistentBlockHeight.into()),
        }
    }

    fn blocks_by_query(&self, start: BlockHeight, count: usize) -> QueryBlocksResult {
        let requested_range = range_utils::make_range(start, count);

        let local_range = self.blockchain.local_block_range();
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

        let local_blocks: Vec<CandidBlock> = self.blockchain.blocks[local_start..local_end]
            .iter()
            .map(|enc_block| -> CandidBlock {
                enc_block
                    .decode()
                    .expect("bug: failed to decode encoded block")
                    .into()
            })
            .collect();

        let archived_blocks_range = requested_range.start..effective_local_range.start.clone();

        let archived_blocks = self
            .blockchain
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
                    storage_canister_id: canister_id.clone(),
                })
            })
            .collect();

        let chain_length = self.blockchain.chain_length();

        QueryBlocksResult {
            chain_length: chain_length.into(),
            certificate: None,
            blocks: local_blocks,
            first_block_index: effective_local_range.start.into(),
            archived_blocks,
        }
    }

    fn archives(&self) -> Vec<ArchiveInfo> {
        self.blockchain.archive.archives()
    }
}
