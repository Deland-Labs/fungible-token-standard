use crate::{
    ActorError, ActorResult, CommonResult, TransactionResponse, TxRecord, TxRecordCommonResult,
};
use candid::{CandidType, Principal};

#[derive(CandidType)]
pub enum BooleanResult {
    Ok(bool),
    Err(ActorError),
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

#[derive(CandidType)]
pub enum TransactionResult {
    Ok(TransactionResponse),
    Err(ActorError),
}

impl From<CommonResult<TransactionResponse>> for TransactionResult {
    fn from(result: CommonResult<TransactionResponse>) -> Self {
        match result {
            Ok(value) => TransactionResult::Ok(value),
            Err(error) => TransactionResult::Err(error.into()),
        }
    }
}

impl From<ActorResult<TransactionResponse>> for TransactionResult {
    fn from(result: ActorResult<TransactionResponse>) -> Self {
        match result {
            Ok(value) => TransactionResult::Ok(value),
            Err(error) => TransactionResult::Err(error.into()),
        }
    }
}

#[derive(CandidType, Debug, Clone)]
pub enum TxRecordResult {
    // Return tx record if exist in the DFT cache txs
    Ok(TxRecord),
    // If not storage in DFT cache txs, return the storage canister id
    Forward(Principal),
    // Such as out of tx index or tx id not exist
    Err(ActorError),
}

impl From<TxRecordCommonResult> for TxRecordResult {
    fn from(r: TxRecordCommonResult) -> Self {
        match r {
            TxRecordCommonResult::Ok(tx) => TxRecordResult::Ok(tx),
            TxRecordCommonResult::Forward(p) => TxRecordResult::Forward(p),
            TxRecordCommonResult::Err(e) => TxRecordResult::Err(e.into()),
        }
    }
}

#[derive(CandidType)]
pub enum TxRecordListResult {
    Ok(Vec<TxRecord>),
    Err(ActorError),
}

impl From<CommonResult<Vec<TxRecord>>> for TxRecordListResult {
    fn from(result: CommonResult<Vec<TxRecord>>) -> Self {
        match result {
            Ok(value) => TxRecordListResult::Ok(value),
            Err(error) => TxRecordListResult::Err(error.into()),
        }
    }
}

impl From<ActorResult<Vec<TxRecord>>> for TxRecordListResult {
    fn from(result: ActorResult<Vec<TxRecord>>) -> Self {
        match result {
            Ok(value) => TxRecordListResult::Ok(value),
            Err(error) => TxRecordListResult::Err(error.into()),
        }
    }
}
