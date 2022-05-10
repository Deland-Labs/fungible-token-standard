use crate::{
    ActorResult, BlockHash, BlockHeight, CandidBlock, CandidTransaction, CommonResult, ErrorInfo,
    Transaction, TransactionHash, TransactionId,
};
use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum BooleanResult {
    Ok(bool),
    Err(ErrorInfo),
}

impl From<CommonResult<bool>> for BooleanResult {
    fn from(result: CommonResult<bool>) -> Self {
        match result {
            Ok(value) => BooleanResult::Ok(value),
            Err(error) => BooleanResult::Err(error.into()),
        }
    }
}

impl From<ActorResult<bool>> for BooleanResult {
    fn from(result: ActorResult<bool>) -> Self {
        match result {
            Ok(value) => BooleanResult::Ok(value),
            Err(error) => BooleanResult::Err(error),
        }
    }
}

#[derive(CandidType, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperationResult {
    Ok {
        #[serde(rename = "txId")]
        tx_id: TransactionId,
        #[serde(rename = "blockHeight")]
        block_height: Nat,
    },
    Err(ErrorInfo),
}

impl From<CommonResult<(BlockHeight, BlockHash, TransactionHash)>> for OperationResult {
    fn from(result: CommonResult<(BlockHeight, BlockHash, TransactionHash)>) -> Self {
        match result {
            Ok((block_height, _, tx_hash)) => OperationResult::Ok {
                tx_id: hex::encode(tx_hash.as_ref()),
                block_height: block_height.into(),
            },
            Err(error) => OperationResult::Err(error.into()),
        }
    }
}

#[derive(CandidType, Debug, Clone)]
pub enum BlockResult {
    // Return tx record if exist in the DFT cache txs
    Ok(CandidBlock),
    // If not storage in DFT cache txs, return the storage canister id
    Forward(Principal),
    // Such as out of tx index or tx id not exist
    Err(ErrorInfo),
}

#[derive(CandidType, Debug, Clone)]
pub enum BlockListResult {
    // Return tx record if exist in the DFT cache txs
    Ok(Vec<CandidBlock>),
    // Such as out of tx index or tx id not exist
    Err(ErrorInfo),
}

#[derive(Debug, CandidType, Deserialize)]
pub struct ArchivedBlocksRange {
    pub start: Nat,
    pub length: u64,
    #[serde(rename = "storageCanisterId")]
    pub storage_canister_id: Principal,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct QueryBlocksResult {
    #[serde(rename = "chainLength")]
    pub chain_length: Nat,
    pub certificate: Option<serde_bytes::ByteBuf>,
    pub blocks: Vec<CandidBlock>,
    #[serde(rename = "firstBlockIndex")]
    pub first_block_index: Nat,
    #[serde(rename = "archivedBlocks")]
    pub archived_blocks: Vec<ArchivedBlocksRange>,
}

pub type TransactionList = Vec<Transaction>;
pub type CandidTransactionList = Vec<CandidTransaction>;

#[derive(CandidType, Debug, Clone)]
pub enum TransactionResult {
    // Return tx if exist in the DFT cache txs
    Ok(CandidTransaction),
    // If not storage in DFT cache txs, return the storage canister id
    Forward(Principal),
    // Such as out of tx index or tx id not exist
    Err(ErrorInfo),
}

#[derive(CandidType)]
pub enum TransactionListResult {
    Ok(CandidTransactionList),
    Err(ErrorInfo),
}

impl From<CommonResult<TransactionList>> for TransactionListResult {
    fn from(result: CommonResult<TransactionList>) -> Self {
        match result {
            Ok(value) => {
                let mut txs = Vec::with_capacity(value.len());
                for tx in value {
                    txs.push(tx.into());
                }
                TransactionListResult::Ok(txs)
            }
            Err(error) => TransactionListResult::Err(error.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CandidOperation, CandidTokenFee, Operation, TokenFee};
    use dft_utils::sha256::compute_hash;
    use num_bigint::BigUint;

    #[test]
    fn test_common_result_bool_to_boolean_result() {
        let result = CommonResult::Ok(true);
        let boolean_result: BooleanResult = result.into();
        assert_eq!(boolean_result, BooleanResult::Ok(true));

        let result = CommonResult::Ok(false);
        let boolean_result: BooleanResult = result.into();
        assert_eq!(boolean_result, BooleanResult::Ok(false));
    }

    #[test]
    fn test_actor_result_bool_to_boolean_result() {
        let result = ActorResult::Ok(true);
        let boolean_result: BooleanResult = result.into();
        assert_eq!(boolean_result, BooleanResult::Ok(true));

        let result = ActorResult::Ok(false);
        let boolean_result: BooleanResult = result.into();
        assert_eq!(boolean_result, BooleanResult::Ok(false));
    }

    #[test]
    fn test_common_result_to_operation_result() {
        let block_height = BigUint::from(1u32);
        let block_hash = compute_hash("block".as_bytes());
        let tx_hash = compute_hash("tx00000001".as_bytes());

        let result: CommonResult<(BlockHeight, BlockHash, TransactionHash)> =
            CommonResult::Ok((block_height.clone(), block_hash.clone(), tx_hash.clone()));
        let operation_result: OperationResult = result.into();
        assert_eq!(
            operation_result,
            OperationResult::Ok {
                tx_id: hex::encode(tx_hash),
                block_height: block_height.into()
            }
        );
    }

    #[test]
    fn test_transaction_list_common_result_to_transaction_list_result() {
        let tx_list = vec![
            Transaction {
                operation: Operation::AddMinter {
                    caller: "czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae"
                        .parse()
                        .unwrap(),
                    minter: "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                        .parse()
                        .unwrap(),
                },
                created_at: 1,
            },
            Transaction {
                operation: Operation::FeeModify {
                    caller: "czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae"
                        .parse()
                        .unwrap(),
                    new_fee: TokenFee::new(1u32.into(), 1u32, 8),
                },
                created_at: 2,
            },
        ];

        let result = CommonResult::Ok(tx_list);
        let tx_list_res: TransactionListResult = result.clone().into();
        match tx_list_res {
            TransactionListResult::Ok(tx_list) => {
                assert_eq!(tx_list.len(), 2);
                assert_eq!(
                    tx_list[0].operation,
                    CandidOperation::AddMinter {
                        caller: "czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae"
                            .parse()
                            .unwrap(),
                        minter: "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                            .parse()
                            .unwrap(),
                    }
                );
                assert_eq!(
                    tx_list[1].operation,
                    CandidOperation::FeeModify {
                        caller: "czjfo-ddpvm-6sibl-6zbox-ee5zq-bx3hc-e336t-s6pka-dupmy-wcxqi-fae"
                            .parse()
                            .unwrap(),
                        new_fee: CandidTokenFee {
                            minimum: 1u32.into(),
                            rate: 1u32.into(),
                            rate_decimals: 8,
                        },
                    }
                );
            }
            _ => panic!("should be ok"),
        };
    }
}
