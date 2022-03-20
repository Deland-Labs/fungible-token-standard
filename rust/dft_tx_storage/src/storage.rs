use std::convert::TryInto;

use candid::Nat;
use candid::{CandidType, Deserialize, Principal};
use dft_types::*;
use dft_utils::decode_tx_id;

#[derive(CandidType, Debug, Deserialize)]
pub struct StorageInfo {
    pub dft_id: Principal,
    pub tx_start_index: Nat,
    pub txs_count: Nat,
    pub cycles: u64,
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct AutoScalingStorage {
    pub dft_id: Principal,
    pub tx_start_index: Nat,
    pub txs: Txs,
}

impl Default for AutoScalingStorage {
    fn default() -> Self {
        AutoScalingStorage {
            dft_id: Principal::anonymous(),
            tx_start_index: Nat::from(0),
            txs: Txs::new(),
        }
    }
}

impl AutoScalingStorage {
    // fn init
    pub fn initialize(&mut self, dft_id: Principal, tx_start_index: Nat) {
        self.dft_id = dft_id;
        self.tx_start_index = tx_start_index;
    }

    // fn only allow token canister
    fn _only_allow_token_canister(&self, caller: &Principal) -> CommonResult<()> {
        if &self.dft_id != caller {
            return Err(DFTError::OnlyAllowTokenCanisterCallThisFunction);
        }
        Ok(())
    }

    // fn append
    pub fn append(&mut self, caller: &Principal, tx: TxRecord) -> CommonResult<bool> {
        self._only_allow_token_canister(caller)?;
        // check tx index
        self._check_tx_index(&tx)?;
        self.txs.push(tx);
        Ok(true)
    }

    // fn batch_append
    pub fn batch_append(&mut self, caller: &Principal, txs: Vec<TxRecord>) -> CommonResult<bool> {
        self._only_allow_token_canister(caller)?;
        // insert txs to self.txs, check the tx index first
        for tx in &txs {
            self._check_tx_index(tx)?;
        }
        self.txs.extend(txs);
        Ok(true)
    }

    // check tx index
    fn _check_tx_index(&self, tx: &TxRecord) -> CommonResult<()> {
        if tx.get_tx_index() < self.tx_start_index {
            return Err(DFTError::InvalidTxIndex);
        }
        Ok(())
    }

    // fn get_tx_by_index
    pub fn get_tx_by_index(&self, tx_index: Nat) -> TxRecordCommonResult {
        if tx_index < self.tx_start_index
            || tx_index > (self.tx_start_index.clone() + self.txs.len() as u128)
        {
            TxRecordCommonResult::Err(DFTError::InvalidTxIndex)
        } else {
            let inner_index: usize = (tx_index - self.tx_start_index.clone())
                .0
                .try_into()
                .unwrap();
            TxRecordCommonResult::Ok(self.txs[inner_index].clone())
        }
    }

    // fn get_tx_by_id
    pub fn get_tx_by_id(&self, tx_id: String) -> TxRecordCommonResult {
        let decode_res = decode_tx_id(tx_id);
        match decode_res {
            Ok((dft_id, tx_index)) => {
                if dft_id != self.dft_id {
                    TxRecordCommonResult::Err(DFTError::TxIdNotBelongToCurrentDft)
                } else {
                    self.get_tx_by_index(tx_index)
                }
            }
            Err(_) => TxRecordCommonResult::Err(DFTError::InvalidTxId),
        }
    }

    // fn get_tx_by_index_range
    pub fn get_tx_by_index_range(
        &self,
        tx_index_start: Nat,
        size: usize,
    ) -> CommonResult<Vec<TxRecord>> {
        if tx_index_start < self.tx_start_index
            || tx_index_start > (self.tx_start_index.clone() + self.txs.len() as u128)
        {
            Err(DFTError::InvalidTxIndex)
        } else {
            let inner_index_start: usize = (tx_index_start - self.tx_start_index.clone())
                .0
                .try_into()
                .unwrap();
            let inner_index_end = inner_index_start + size;
            let inner_index_end = if inner_index_end > self.txs.len() {
                self.txs.len()
            } else {
                inner_index_end
            };

            let mut res = Vec::new();
            for i in inner_index_start..inner_index_end {
                res.push(self.txs[i].clone());
            }
            Ok(res)
        }
    }

    // fn get storage info
    pub fn get_storage_info(&self) -> StorageInfo {
        StorageInfo {
            dft_id: self.dft_id.clone(),
            tx_start_index: self.tx_start_index.clone(),
            txs_count: self.txs.len().into(),
            cycles: 0,
        }
    }

    // fn restore
    pub fn restore(&mut self, data: AutoScalingStorage) {
        self.dft_id = data.dft_id;
        self.tx_start_index = data.tx_start_index;
        self.txs = data.txs;
    }
}

//test
#[cfg(test)]
mod tests {
    use super::*;
    use dft_utils::encode_tx_id;
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
    fn test_start_index() -> Nat {
        Nat::from(1234)
    }

    #[fixture]
    fn next_tx_index(test_start_index: Nat) -> Nat {
        test_start_index + 1
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
    fn test_storage(test_token_id: Principal, test_start_index: Nat) -> AutoScalingStorage {
        let mut storage = AutoScalingStorage::default();
        storage.initialize(test_token_id, test_start_index);
        storage
    }

    #[fixture]
    fn test_tx_record(test_start_index: Nat, now: u64) -> TxRecord {
        let from =
            Principal::from_text("o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae")
                .unwrap();
        let to =
            Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe")
                .unwrap();
        let from_holder = TokenHolder::new(from.clone(), None);
        let to_holder = TokenHolder::new(to.clone(), None);
        TxRecord::Transfer(
            test_start_index,
            from_holder.clone(),
            from_holder,
            to_holder,
            Nat::from(1000),
            Nat::from(1),
            1u64,
            now,
        )
    }

    #[fixture]
    fn invalid_tx_record(test_start_index: Nat, now: u64) -> TxRecord {
        let from =
            Principal::from_text("o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae")
                .unwrap();
        let to =
            Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe")
                .unwrap();
        let from_holder = TokenHolder::new(from.clone(), None);
        let to_holder = TokenHolder::new(to.clone(), None);
        TxRecord::Transfer(
            test_start_index - 1,
            from_holder.clone(),
            from_holder,
            to_holder,
            Nat::from(1000),
            Nat::from(1),
            1u64,
            now,
        )
    }

    #[fixture]
    fn test_tx_records(test_start_index: Nat, now: u64) -> Vec<TxRecord> {
        let from =
            Principal::from_text("o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae")
                .unwrap();
        let to =
            Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe")
                .unwrap();
        let from_holder = TokenHolder::new(from.clone(), None);
        let to_holder = TokenHolder::new(to.clone(), None);
        // generate 3 tx records
        let mut tx_records = Vec::new();
        for i in 0..3 {
            tx_records.push(TxRecord::Transfer(
                test_start_index.clone() + i,
                from_holder.clone(),
                from_holder.clone(),
                to_holder.clone(),
                Nat::from(1000),
                Nat::from(1),
                1u64,
                now,
            ));
        }

        tx_records
    }

    #[fixture]
    fn test_invalid_tx_records(test_start_index: Nat, now: u64) -> Vec<TxRecord> {
        let from =
            Principal::from_text("o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae")
                .unwrap();
        let to =
            Principal::from_text("qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe")
                .unwrap();
        let from_holder = TokenHolder::new(from.clone(), None);
        let to_holder = TokenHolder::new(to.clone(), None);
        // generate 3 tx records
        let mut tx_records = Vec::new();
        for i in 0..3 {
            let ti = if i == 2 {
                test_start_index.clone() - 100
            } else {
                test_start_index.clone() + i
            };

            tx_records.push(TxRecord::Transfer(
                ti,
                from_holder.clone(),
                from_holder.clone(),
                to_holder.clone(),
                Nat::from(1000),
                Nat::from(1),
                1u64,
                now,
            ));
        }

        tx_records
    }

    // test append
    #[rstest]
    fn test_append(
        test_storage: AutoScalingStorage,
        test_token_id: Principal,
        other_token_id: Principal,
        test_tx_record: TxRecord,
        invalid_tx_record: TxRecord,
    ) {
        let mut storage = test_storage.clone();
        // append with other token id should fail
        let res = storage.append(&other_token_id, test_tx_record.clone());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            DFTError::OnlyAllowTokenCanisterCallThisFunction
        );
        // append with invalid tx record should fail
        let res = storage.append(&test_token_id, invalid_tx_record.clone());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), DFTError::InvalidTxIndex);
        // append with test token id should succeed
        let res = storage.append(&test_token_id, test_tx_record.clone());
        assert!(res.is_ok());
        assert_eq!(storage.txs.len(), 1);
        // test get tx by index
        let res = storage.get_tx_by_index(test_tx_record.get_tx_index());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), test_tx_record);
        // test get tx by id
        let tx_id = encode_tx_id(test_token_id, test_tx_record.get_tx_index());
        let res = storage.get_tx_by_id(tx_id);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), test_tx_record);
    }

    // test batch_append
    #[rstest]
    fn test_batch_append(
        test_storage: AutoScalingStorage,
        test_token_id: Principal,
        other_token_id: Principal,
        test_tx_records: Vec<TxRecord>,
        test_invalid_tx_records: Vec<TxRecord>,
    ) {
        let mut storage = test_storage.clone();
        // append with other token id should fail
        let res = storage.batch_append(&other_token_id, test_tx_records.clone());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            DFTError::OnlyAllowTokenCanisterCallThisFunction
        );
        // append with invalid tx record should fail
        let res = storage.batch_append(&test_token_id, test_invalid_tx_records);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), DFTError::InvalidTxIndex);
        // append with test token id should succeed
        let res = storage.batch_append(&test_token_id, test_tx_records.clone());
        assert!(res.is_ok());
        assert_eq!(storage.txs.len(), test_tx_records.len());
        // test get tx by index
        for tx_record in &test_tx_records {
            let res = storage.get_tx_by_index(tx_record.get_tx_index());
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), *tx_record);
        }
        // test get tx by id
        for tx_record in &test_tx_records {
            let tx_id = encode_tx_id(test_token_id, tx_record.get_tx_index());
            let res = storage.get_tx_by_id(tx_id);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), *tx_record);
        }
    }
}
