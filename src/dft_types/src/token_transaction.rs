use crate::{CandidTokenFee, TokenAmount, TokenFee, TokenHolder, TokenReceiver, TransactionHash};
use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    Approve {
        caller: Principal,
        owner: TokenHolder,
        spender: TokenHolder,
        value: TokenAmount,
        fee: TokenAmount,
    },
    Transfer {
        caller: TokenHolder,
        from: TokenHolder,
        to: TokenReceiver,
        value: TokenAmount,
        fee: TokenAmount,
    },
    FeeModify {
        caller: Principal,
        #[serde(rename = "newFee")]
        new_fee: TokenFee,
    },
    OwnerModify {
        caller: Principal,
        #[serde(rename = "newOwner")]
        new_owner: Principal,
    },
    FeeToModify {
        caller: Principal,
        #[serde(rename = "newFeeTo")]
        new_fee_to: TokenHolder,
    },
    AddMinter {
        caller: Principal,
        minter: Principal,
    },
    RemoveMinter {
        caller: Principal,
        minter: Principal,
    },
}

#[derive(CandidType, Deserialize, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CandidOperation {
    Approve {
        caller: Principal,
        owner: TokenHolder,
        spender: TokenHolder,
        value: Nat,
        fee: Nat,
    },
    Transfer {
        caller: TokenHolder,
        from: TokenHolder,
        to: TokenReceiver,
        value: Nat,
        fee: Nat,
    },
    FeeModify {
        caller: Principal,
        #[serde(rename = "newFee")]
        new_fee: CandidTokenFee,
    },
    OwnerModify {
        caller: Principal,
        #[serde(rename = "newOwner")]
        new_owner: Principal,
    },
    FeeToModify {
        caller: Principal,
        #[serde(rename = "newFeeTo")]
        new_fee_to: TokenHolder,
    },
    AddMinter {
        caller: Principal,
        minter: Principal,
    },
    RemoveMinter {
        caller: Principal,
        minter: Principal,
    },
}

impl From<Operation> for CandidOperation {
    fn from(operation: Operation) -> Self {
        match operation {
            Operation::Approve {
                caller,
                owner,
                spender,
                value,
                fee,
            } => CandidOperation::Approve {
                caller,
                owner,
                spender,
                value: value.into(),
                fee: fee.into(),
            },
            Operation::Transfer {
                caller,
                from,
                to,
                value,
                fee,
            } => CandidOperation::Transfer {
                caller,
                from,
                to,
                value: value.into(),
                fee: fee.into(),
            },
            Operation::FeeModify { caller, new_fee } => CandidOperation::FeeModify {
                caller,
                new_fee: new_fee.into(),
            },
            Operation::OwnerModify { caller, new_owner } => {
                CandidOperation::OwnerModify { caller, new_owner }
            }
            Operation::FeeToModify { caller, new_fee_to } => {
                CandidOperation::FeeToModify { caller, new_fee_to }
            }
            Operation::AddMinter { caller, minter } => {
                CandidOperation::AddMinter { caller, minter }
            }
            Operation::RemoveMinter { caller, minter } => {
                CandidOperation::RemoveMinter { caller, minter }
            }
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionInfo {
    pub block_timestamp: u64,
    pub tx_hash: TransactionHash,
}

#[derive(Deserialize, Serialize, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transaction {
    pub operation: Operation,
    pub created_at: u64,
}

impl Transaction {
    // hash token id + tx bytes, make sure tx hash unique
    pub fn hash_with_token_id(&self, token_id: &Principal) -> TransactionHash {
        let tx_bytes = bincode::serialize(&self).unwrap();
        let combine_bytes = [token_id.as_slice(), &tx_bytes[..]].concat();
        dft_utils::sha256::compute_hash(&combine_bytes)
    }
}

#[derive(CandidType, Deserialize, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CandidTransaction {
    pub operation: CandidOperation,
    /// The time this transaction was created.
    #[serde(rename = "createdAt")]
    pub created_at: u64,
}

impl From<Transaction> for CandidTransaction {
    fn from(tx: Transaction) -> Self {
        CandidTransaction {
            operation: CandidOperation::from(tx.operation),
            created_at: tx.created_at,
        }
    }
}
