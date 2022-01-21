use super::{TokenHolder, TokenReceiver};
use candid::{CandidType, Deserialize, Nat, Principal};
use crate::{ActorError, DFTError};

#[derive(CandidType, Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub enum TxRecord {
    // tx_index, caller, owner, spender, value, fee, timestamp
    Approve(Nat, Principal, TokenHolder, TokenReceiver, Nat, Nat, u64),
    // tx_index, caller, from, to, value, fee, timestamp
    Transfer(Nat, Principal, TokenHolder, TokenReceiver, Nat, Nat, u64),
}

impl TxRecord {
    pub fn get_tx_index(&self) -> Nat {
        match self {
            TxRecord::Approve(tx_index, _, _, _, _, _, _) => tx_index.clone(),
            TxRecord::Transfer(tx_index, _, _, _, _, _, _) => tx_index.clone(),
        }
    }
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum TxRecordCommonResult {
    // Return tx record if exist in the DFT cache txs
    Ok(TxRecord),
    // If not storage in DFT cache txs, return the storage canister id
    Forward(Principal),
    // Such as out of tx index or tx id not exist
    Err(DFTError),
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum TxRecordResult {
    // Return tx record if exist in the DFT cache txs
    Ok(TxRecord),
    // If not storage in DFT cache txs, return the storage canister id
    Forward(Principal),
    // Such as out of tx index or tx id not exist
    Err(ActorError),
}

impl From<TxRecordCommonResult> for TxRecordResult
{
    fn from(r: TxRecordCommonResult) -> Self {
        match r {
            TxRecordCommonResult::Ok(tx) => TxRecordResult::Ok(tx),
            TxRecordCommonResult::Forward(p) => TxRecordResult::Forward(p),
            TxRecordCommonResult::Err(e) => TxRecordResult::Err(e.into()),
        }
    }
}

#[test]
fn test_tx_record_size() {
    let tx_record_size = std::mem::size_of::<TxRecord>();
    assert_eq!(
        176, tx_record_size,
        "tx_record_size is not {}",
        tx_record_size
    );
}
