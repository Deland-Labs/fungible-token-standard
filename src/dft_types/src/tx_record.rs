use super::{TokenHolder, TokenReceiver};
use crate::{DFTError, Fee};
use candid::{CandidType, Deserialize, Nat, Principal};
use std::convert::TryInto;
//
// #[derive(CandidType, Clone, Debug)]
// pub enum Operation {
//     Approve {
//         #[prost(message, tag = "1")]
//         caller: Principal,
//         #[prost(message, tag = "2")]
//         owner: TokenHolder,
//         #[prost(message, tag = "3")]
//         spender: TokenHolder,
//         #[prost(message, tag = "4")]
//         value: Nat,
//         #[prost(message, tag = "5")]
//         fee: Nat,
//         #[prost(message, tag = "6")]
//         timestamp: u64,
//     }
// }
// prost_into_vec!(Operation, 32);
// vec_try_into_prost!(Operation);
// // test serialization
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{DFTError, Fee};
//     use candid::{CandidType, Deserialize, Nat, Principal};
//     use std::convert::TryInto;
//     use std::str::FromStr;
//     use std::time::SystemTime;
//
//     #[test]
//     fn test_serialize_approve() {
//         let caller = Principal::from_text("czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae").unwrap();
//         let owner = TokenHolder::new(caller, None);
//         let spender = TokenHolder::new(caller, None);
//         let value = Nat::from(1);
//         let fee = Nat::from(1);
//         let timestamp = 1u64;
//         let op = Operation::Approve {
//             caller,
//             owner,
//             spender,
//             value,
//             fee,
//             timestamp,
//         };
//         let data: Vec<u8> = op.into();
//         let deserialized: Operation = data.try_into().unwrap();
//         //compare op and deserialized
//         match deserialized {
//             Operation::Approve {
//                 caller: _caller,
//                 owner: _owner,
//                 spender: _spender,
//                 value: _value,
//                 fee: _fee,
//                 timestamp: _timestamp,
//             } => {
//                 match op {
//                     Operation::Approve {
//                         caller: _caller,
//                         owner: _owner,
//                         spender: _spender,
//                         value: _value,
//                         fee: _fee,
//                         timestamp: _timestamp,
//                     } => {
//                         assert_eq!(_caller, caller);
//                         assert_eq!(_owner, owner);
//                         assert_eq!(_spender, spender);
//                         assert_eq!(_value, value);
//                         assert_eq!(_fee, fee);
//                         assert_eq!(_timestamp, timestamp);
//                     }
//                     _ => panic!("deserialized op is not an approve"),
//                 }
//             },
//             _ => panic!("deserialized op is not an approve operation"),
//         }
//         assert_eq!(op, deserialized);
//     }
// }

// #[derive(CandidType, Clone,  PartialEq, Eq, PartialOrd, Ord, Hash, Message)]
// pub struct Transaction {
//     pub caller: Principal,
//     pub operation: Operation,
//     pub created_at: Nat,
// }

#[derive(CandidType, Debug, Clone, Deserialize, Eq, PartialEq)]
pub enum TxRecord {
    // tx_index, caller, owner, spender, value, fee,created_at , timestamp
    Approve(
        Nat,
        TokenHolder,
        TokenHolder,
        TokenReceiver,
        Nat,
        Nat,
        u64,
        u64,
    ),
    // tx_index, caller, from, to, value, fee, created_at, timestamp
    Transfer(
        Nat,
        TokenHolder,
        TokenHolder,
        TokenReceiver,
        Nat,
        Nat,
        u64,
        u64,
    ),
    // tx_index, caller (owner), new fee setting, created_at, timestamp
    FeeModify(
        Nat,
        Principal,
        Fee,
        u64,
        u64,
    ),
    // tx_index, caller (owner), new owner, created_at, timestamp
    OwnerModify(
        Nat,
        Principal,
        Principal,
        u64,
        u64,
    ),
    // tx_index, caller (owner), new feeTo, created_at, timestamp
    FeeToModify(
        Nat,
        Principal,
        TokenHolder,
        u64,
        u64,
    ),
    // tx_index, caller (owner), new logo, created_at, timestamp
    LogoModify(
        Nat,
        Principal,
        Vec<u8>,
        u64,
        u64,
    ),
    // tx_index, caller (owner), new logo, created_at, timestamp
    DescModify(
        Nat,
        Principal,
        Vec<(String, String)>,
        u64,
        u64,
    ),
}

impl TxRecord {
    pub fn get_tx_index(&self) -> Nat {
        match self {
            TxRecord::Approve(tx_index, _, _, _, _, _, _, _) => tx_index.clone(),
            TxRecord::Transfer(tx_index, _, _, _, _, _, _, _) => tx_index.clone(),
            TxRecord::FeeModify(tx_index, _, _, _, _) => tx_index.clone(),
            TxRecord::OwnerModify(tx_index, _, _, _, _) => tx_index.clone(),
            TxRecord::FeeToModify(tx_index, _, _, _, _) => tx_index.clone(),
            TxRecord::LogoModify(tx_index, _, _, _, _) => tx_index.clone(),
            TxRecord::DescModify(tx_index, _, _, _, _) => tx_index.clone(),
        }
    }
    pub fn get_caller(&self) -> TokenHolder {
        match self {
            TxRecord::Approve(_, caller, _, _, _, _, _, _) => caller.clone(),
            TxRecord::Transfer(_, caller, _, _, _, _, _, _) => caller.clone(),
            TxRecord::FeeModify(_, caller, _, _, _) => TokenHolder::new(caller.clone(), None),
            TxRecord::OwnerModify(_, caller, _, _, _) => TokenHolder::new(caller.clone(), None),
            TxRecord::FeeToModify(_, caller, _, _, _) => TokenHolder::new(caller.clone(), None),
            TxRecord::LogoModify(_, caller, _, _, _) => TokenHolder::new(caller.clone(), None),
            TxRecord::DescModify(_, caller, _, _, _) => TokenHolder::new(caller.clone(), None),
        }
    }

    pub fn get_created_at(&self) -> u64 {
        match self {
            TxRecord::Approve(_, _, _, _, _, _, created_at, _) => created_at.clone(),
            TxRecord::Transfer(_, _, _, _, _, _, created_at, _) => created_at.clone(),
            TxRecord::FeeModify(_, _, _, created_at, _) => created_at.clone(),
            TxRecord::OwnerModify(_, _, _, created_at, _) => created_at.clone(),
            TxRecord::FeeToModify(_, _, _, created_at, _) => created_at.clone(),
            TxRecord::LogoModify(_, _, _, created_at, _) => created_at.clone(),
            TxRecord::DescModify(_, _, _, created_at, _) => created_at.clone(),
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

#[test]
fn test_tx_record_size() {
    let tx_record_size = std::mem::size_of::<TxRecord>();
    assert_eq!(
        184, tx_record_size,
        "tx_record_size is not {}",
        tx_record_size
    );
}
