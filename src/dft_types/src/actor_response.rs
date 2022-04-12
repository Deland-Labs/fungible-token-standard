use crate::{
    ActorResult, Block, BlockHash, BlockHeight, CommonResult, ErrorInfo, Transaction,
    TransactionHash, TransactionId,
};
use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Debug, Deserialize)]
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
            Err(error) => BooleanResult::Err(error.into()),
        }
    }
}

#[derive(CandidType, Debug, Deserialize)]
pub enum OperationResult {
    Ok {
        #[serde(rename = "txId")]
        tx_id: TransactionId,
        #[serde(rename = "blockHeight")]
        block_height: BlockHeight,
        error: Option<ErrorInfo>,
    },
    Err(ErrorInfo),
}

impl From<CommonResult<(BlockHeight, BlockHash, TransactionHash)>> for OperationResult {
    fn from(result: CommonResult<(BlockHeight, BlockHash, TransactionHash)>) -> Self {
        match result {
            Ok((block_height, _, tx_hash)) => OperationResult::Ok {
                tx_id: hex::encode(tx_hash.as_ref()),
                block_height: block_height,
                error: None,
            },
            Err(error) => OperationResult::Err(error.into()),
        }
    }
}

#[derive(CandidType, Debug, Clone)]
pub enum BlockResult {
    // Return tx record if exist in the DFT cache txs
    Ok(Block),
    // If not storage in DFT cache txs, return the storage canister id
    Forward(Principal),
    // Such as out of tx index or tx id not exist
    Err(ErrorInfo),
}

#[derive(CandidType, Debug, Clone)]
pub enum BlockListResult {
    // Return tx record if exist in the DFT cache txs
    Ok(Vec<Block>),
    // Such as out of tx index or tx id not exist
    Err(ErrorInfo),
}

#[derive(Debug, CandidType, Deserialize)]
pub struct ArchivedBlocksRange {
    pub start: BlockHeight,
    pub length: u64,
    #[serde(rename = "storageCanisterId")]
    pub storage_canister_id: Principal,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct QueryBlocksResult {
    #[serde(rename = "chainLength")]
    pub chain_length: Nat,
    pub certificate: Option<serde_bytes::ByteBuf>,
    pub blocks: Vec<Block>,
    #[serde(rename = "firstBlockIndex")]
    pub first_block_index: BlockHeight,
    #[serde(rename = "archivedBlocks")]
    pub archived_blocks: Vec<ArchivedBlocksRange>,
}

pub type TransactionList = Vec<Transaction>;

#[derive(CandidType, Debug, Clone)]
pub enum TransactionResult {
    // Return tx if exist in the DFT cache txs
    Ok(Transaction),
    // If not storage in DFT cache txs, return the storage canister id
    Forward(Principal),
    // Such as out of tx index or tx id not exist
    Err(ErrorInfo),
}
#[derive(CandidType)]
pub enum TransactionListResult {
    Ok(TransactionList),
    Err(ErrorInfo),
}

impl From<CommonResult<TransactionList>> for TransactionListResult {
    fn from(result: CommonResult<TransactionList>) -> Self {
        match result {
            Ok(value) => TransactionListResult::Ok(value),
            Err(error) => TransactionListResult::Err(error.into()),
        }
    }
}
