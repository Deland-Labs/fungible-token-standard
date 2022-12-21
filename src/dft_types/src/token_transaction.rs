use crate::{InnerTokenFee, TokenAmount, TokenFee, TokenHolder, TokenReceiver, TransactionHash};
use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InnerOperation {
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
        new_fee: InnerTokenFee,
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
pub enum Operation {
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

impl From<InnerOperation> for Operation {
    fn from(operation: InnerOperation) -> Self {
        match operation {
            InnerOperation::Approve {
                caller,
                owner,
                spender,
                value,
                fee,
            } => Operation::Approve {
                caller,
                owner,
                spender,
                value: value.into(),
                fee: fee.into(),
            },
            InnerOperation::Transfer {
                caller,
                from,
                to,
                value,
                fee,
            } => Operation::Transfer {
                caller,
                from,
                to,
                value: value.into(),
                fee: fee.into(),
            },
            InnerOperation::FeeModify { caller, new_fee } => Operation::FeeModify {
                caller,
                new_fee: new_fee.into(),
            },
            InnerOperation::OwnerModify { caller, new_owner } => {
                Operation::OwnerModify { caller, new_owner }
            }
            InnerOperation::FeeToModify { caller, new_fee_to } => {
                Operation::FeeToModify { caller, new_fee_to }
            }
            InnerOperation::AddMinter { caller, minter } => Operation::AddMinter { caller, minter },
            InnerOperation::RemoveMinter { caller, minter } => {
                Operation::RemoveMinter { caller, minter }
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
pub struct InnerTransaction {
    pub operation: InnerOperation,
    pub created_at: u64,
}

impl InnerTransaction {
    // hash token id + tx bytes, make sure tx hash unique
    pub fn hash_with_token_id(&self, token_id: &Principal) -> TransactionHash {
        let tx_bytes = bincode::serialize(&self).unwrap();
        let combine_bytes = [token_id.as_slice(), &tx_bytes[..]].concat();
        dft_utils::sha256::compute_hash(&combine_bytes)
    }
}

#[derive(CandidType, Deserialize, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transaction {
    pub operation: Operation,
    /// The time this transaction was created.
    #[serde(rename = "createdAt")]
    pub created_at: u64,
}

impl From<InnerTransaction> for Transaction {
    fn from(tx: InnerTransaction) -> Self {
        Transaction {
            operation: Operation::from(tx.operation),
            created_at: tx.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_with_token_id() {
        let tx = InnerTransaction {
            operation: InnerOperation::Approve {
                caller: "czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae"
                    .parse()
                    .unwrap(),
                owner: "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                    .parse()
                    .unwrap(),
                spender: "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                    .parse()
                    .unwrap(),
                value: 1u32.into(),
                fee: 1u32.into(),
            },
            created_at: 1,
        };
        let token_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let tx_hash = tx.hash_with_token_id(&token_id);
        assert_eq!(
            hex::encode(&tx_hash),
            "2d90ad32cab94625bcde25ae30eb9c9ddd9a48b2041c32678144fec3aa15e0c6"
        );
    }

    #[test]
    fn test_transaction_to_candid_transaction() {
        let tx = InnerTransaction {
            operation: InnerOperation::Approve {
                caller: "czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae"
                    .parse()
                    .unwrap(),
                owner: "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                    .parse()
                    .unwrap(),
                spender: "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                    .parse()
                    .unwrap(),
                value: 1u32.into(),
                fee: 1u32.into(),
            },
            created_at: 1,
        };
        let candid_tx = Transaction::from(tx.clone());
        assert_eq!(candid_tx.created_at, tx.created_at);
    }

    #[test]
    fn test_operation_to_candid_operation() {
        let operation = InnerOperation::Transfer {
            caller: "czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae"
                .parse()
                .unwrap(),
            from: "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap(),
            to: "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap(),
            value: 1u32.into(),
            fee: 1u32.into(),
        };
        let candid_operation = Operation::from(operation);
        assert_eq!(
            candid_operation,
            Operation::Transfer {
                caller: "czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae"
                    .parse()
                    .unwrap(),
                from: "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                    .parse()
                    .unwrap(),
                to: "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                    .parse()
                    .unwrap(),
                value: 1u32.into(),
                fee: 1u32.into(),
            }
        );
    }
}
