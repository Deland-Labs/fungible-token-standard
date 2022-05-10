use crate::{
    constants::{DEFAULT_MAX_TRANSACTIONS_IN_WINDOW, DEFAULT_TRANSACTION_WINDOW},
    *,
};
use candid::Deserialize;
use serde::Serialize;
use std::collections::{BTreeMap, VecDeque};
use std::convert::TryFrom;

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenTransactionWindow {
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
}

impl TokenTransactionWindow {
    pub fn new() -> Self {
        TokenTransactionWindow {
            max_transactions_in_window: usize::try_from(DEFAULT_MAX_TRANSACTIONS_IN_WINDOW)
                .unwrap(),
            transaction_window: DEFAULT_TRANSACTION_WINDOW,
            transactions_by_hash: BTreeMap::new(),
            transactions_by_height: VecDeque::new(),
        }
    }

    pub fn max_transactions_in_window(&self) -> usize {
        self.max_transactions_in_window
    }

    pub fn transactions_count_in_window(&self) -> usize {
        self.transactions_by_height.len()
    }

    pub fn transaction_window(&self) -> u64 {
        self.transaction_window
    }

    pub fn contains_transaction(&self, transaction_hash: TransactionHash) -> bool {
        self.transactions_by_hash.contains_key(&transaction_hash)
    }

    pub fn front_transaction(&self) -> Option<&TransactionInfo> {
        self.transactions_by_height.front()
    }

    pub fn push_transaction(&mut self, block_height: BlockHeight, transaction: TransactionInfo) {
        self.transactions_by_hash
            .insert(transaction.tx_hash, block_height);
        self.transactions_by_height.push_back(transaction);
    }

    /// Removes at most [MAX_TRANSACTIONS_TO_PURGE] transactions older
    /// than `now - transaction_window` and returns the number of pruned
    /// transactions.
    pub fn purge_old_transactions(&mut self, now: u64) -> usize {
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

    pub fn throttle_check(&self, now: u64) -> CommonResult<()> {
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

            assert!(
                false,
                "throttle check,{}  {}",
                self.transactions_by_height
                    .get(num_in_window.saturating_sub(max_rate))
                    .map(|x| x.block_timestamp)
                    .unwrap_or_else(|| 0)
                    + 10u64.pow(9),
                &now
            );
        }

        Ok(())
    }
}

impl StableState for TokenTransactionWindow {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(&(
            &self.max_transactions_in_window,
            &self.transaction_window,
            &self.transactions_by_hash,
            &self.transactions_by_height,
        ))
        .unwrap()
    }

    fn decode(bytes: Vec<u8>) -> Result<Self, String> {
        let (
            max_transactions_in_window,
            transaction_window,
            transactions_by_hash,
            transactions_by_height,
        ): (
            usize,
            u64,
            BTreeMap<TransactionHash, BlockHeight>,
            VecDeque<TransactionInfo>,
        ) = bincode::deserialize(&bytes).unwrap();

        Ok(TokenTransactionWindow {
            max_transactions_in_window,
            transaction_window,
            transactions_by_hash,
            transactions_by_height,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use dft_utils::sha256::compute_hash;

    use super::*;

    #[test]
    fn test_token_transaction_window() {
        let mut window = TokenTransactionWindow::new();
        assert_eq!(window.transactions_by_height.len(), 0);
        assert_eq!(window.transactions_by_hash.len(), 0);

        let block_height = BigUint::from(5u32);
        let block_timestamp = 168_000_000_000u64;
        let tx_info: TransactionInfo = TransactionInfo {
            block_timestamp: block_timestamp.clone(),
            tx_hash: compute_hash("test".as_bytes()),
        };
        window.push_transaction(block_height.clone(), tx_info.clone());
        assert_eq!(window.transactions_by_height.len(), 1);
        assert_eq!(window.transactions_by_hash.len(), 1);
        assert_eq!(
            window
                .transactions_by_height
                .front()
                .unwrap()
                .block_timestamp,
            block_timestamp
        );
        assert_eq!(
            window.transactions_by_hash.get(&tx_info.tx_hash).unwrap(),
            &block_height
        );
    }

    #[test]
    fn test_purge_old_transactions() {
        let mut window = TokenTransactionWindow::new();

        let block_height = BigUint::from(5u32);
        // get now timestamp
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();
        let block_timestamp = now - window.transaction_window - constants::PERMITTED_DRIFT - 1000;
        let tx_info: TransactionInfo = TransactionInfo {
            block_timestamp: block_timestamp.clone(),
            tx_hash: compute_hash("test".as_bytes()),
        };
        window.push_transaction(block_height.clone(), tx_info.clone());
        assert_eq!(window.transactions_by_height.len(), 1);
        assert_eq!(window.transactions_by_hash.len(), 1);

        let block_height = BigUint::from(6u32);
        let block_timestamp = now;
        let tx_info: TransactionInfo = TransactionInfo {
            block_timestamp: block_timestamp.clone(),
            tx_hash: compute_hash("test2".as_bytes()),
        };
        window.push_transaction(block_height.clone(), tx_info.clone());
        assert_eq!(window.transactions_by_height.len(), 2);
        assert_eq!(window.transactions_by_hash.len(), 2);

        let removed = window.purge_old_transactions(now);
        assert_eq!(removed, 1);
        assert_eq!(window.transactions_by_height.len(), 1);
        assert_eq!(window.transactions_by_hash.len(), 1);

        let removed = window.purge_old_transactions(
            now + window.transaction_window + constants::PERMITTED_DRIFT + 1,
        );
        assert_eq!(removed, 1);
        assert_eq!(window.transactions_by_height.len(), 0);
        assert_eq!(window.transactions_by_hash.len(), 0);
    }

    #[test]
    fn test_throttle_check() {
        let mut window = TokenTransactionWindow::new();
        let max_txs_in_window = window.max_transactions_in_window();

        let push_txs_count = max_txs_in_window / 2 - 1;

        // get now timestamp
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();

        for i in 0..push_txs_count {
            let block_height = BigUint::from(i as u32);
            let block_timestamp = now - push_txs_count as u64 + 1;
            let tx_info: TransactionInfo = TransactionInfo {
                block_timestamp: block_timestamp.clone(),
                tx_hash: compute_hash(format!("test{}", i).as_bytes()),
            };
            window.push_transaction(block_height.clone(), tx_info.clone());

            assert_eq!(window.transactions_by_height.len(), i + 1);
            assert_eq!(window.transactions_by_hash.len(), i + 1);

            let result = window.throttle_check(now);
            assert_eq!(result, Ok(()));
        }

        // push new transaction
        let block_height = BigUint::from(push_txs_count + 1);
        let block_timestamp = now + 1;
        let tx_info: TransactionInfo = TransactionInfo {
            block_timestamp: block_timestamp.clone(),
            tx_hash: compute_hash(format!("test{}", block_height).as_bytes()),
        };
        window.push_transaction(block_height.clone(), tx_info.clone());

        assert_eq!(window.transactions_by_height.len(), push_txs_count + 1);
        assert_eq!(window.transactions_by_hash.len(), push_txs_count + 1);

        // check throttling
        let result = window.throttle_check(now);
        assert_eq!(
            result,
            Err(DFTError::TooManyTransactionsInReplayPreventionWindow)
        );
    }

    #[test]
    fn test_decode_encode() {
        let mut window = TokenTransactionWindow::new();
        let push_txs_count = 100;

        // get now timestamp
        let now: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap();

        for i in 0..push_txs_count {
            let block_height = BigUint::from(i as u32);
            let block_timestamp = now - push_txs_count as u64 + 1;
            let tx_info: TransactionInfo = TransactionInfo {
                block_timestamp: block_timestamp.clone(),
                tx_hash: compute_hash(format!("test{}", i).as_bytes()),
            };
            window.push_transaction(block_height.clone(), tx_info.clone());
            assert_eq!(window.transactions_by_height.len(), i + 1);
            assert_eq!(window.transactions_by_hash.len(), i + 1);
        }

        let encoded = window.encode();
        let decoded = TokenTransactionWindow::decode(encoded);
        assert_eq!(decoded.unwrap(), window);
    }
}
