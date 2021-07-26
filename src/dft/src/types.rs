use crate::utils;
use candid::CandidType;
use ic_types::{CanisterId, PrincipalId};
use ledger_canister::account_identifier::AccountIdentifier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::string::String;

pub type TransactionId = u128;
// Rate decimals = 6
// transferFee = amount * rate / 1000000
#[derive(CandidType, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Fee {
    Fixed(u128),
    Rate(u8),
    RateWithLowestLimit(u128, u8),
}

#[derive(CandidType, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct MetaData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub fee: Fee,
}

pub type ExtendData = HashMap<String, String>;
#[derive(CandidType, Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TokenHolder {
    Account(AccountIdentifier),
    Principal(PrincipalId),
    Canister(CanisterId),
}

#[derive(CandidType, Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct KeyValuePair {
    pub k: String,
    pub v: String,
}

impl FromStr for TokenHolder {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pid = s.parse::<PrincipalId>();
        match pid {
            Ok(_principal) => match _principal {
                _principal if utils::is_canister(&_principal) => {
                    let cid = CanisterId::new(_principal).unwrap();
                    Ok(TokenHolder::Canister(cid))
                }
                _principal if utils::is_user_principal(&_principal) => {
                    Ok(TokenHolder::Principal(_principal))
                }
                _ => Err(Error::InvalidReceiver),
            },
            _ => {
                let account_identity = s.parse::<AccountIdentifier>();
                match account_identity {
                    Ok(_ai) => Ok(TokenHolder::Account(_ai)),
                    _ => Err(Error::InvalidReceiver),
                }
            }
        }
    }
}

pub type TransferFrom = TokenHolder;
pub type TokenReceiver = TokenHolder;

#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallData {
    pub method: String,
    pub args: Vec<u8>,
}

#[derive(Deserialize, Debug, Clone, CandidType)]
#[serde(rename_all = "camelCase")]
pub enum Error {
    InvalidSubaccount,
    InvalidTokenHolder,
    InvalidSpender,
    InvalidReceiver,
    InsufficientBalance,
    InsufficientAllowance,
    RejectedByHolder,
    RejectedByReceiver,
    CallFailed,
    NotifyFailed,
    QuantityTooSmall,
    Unknown,
}

#[derive(Deserialize, Debug, Clone, CandidType)]
#[serde(rename_all = "camelCase")]
pub enum TransferResult {
    //transfer succeed, but call failed & notify failed
    Ok(TransactionId, Option<Vec<Error>>),
    Err(Error),
}
 
#[derive(Deserialize, Debug, CandidType)]
#[serde(rename_all = "camelCase")]
pub enum BurnResult {
    Ok,
    Err(Error),
}

#[derive(Deserialize, Debug, CandidType)]
#[serde(rename_all = "camelCase")]
pub enum ApproveResult {
    Ok(Error),
    Err(Error),
}

#[derive(Deserialize, Debug, Clone, CandidType)]
#[serde(rename_all = "camelCase")]
pub enum TxRecord {
    // caller, owner, decimals, total_supply, timestamp
    Init(PrincipalId, PrincipalId, u8, u128, u64),
    // caller, from, to, value, fee, timestamp
    Approve(PrincipalId, TokenHolder, TokenReceiver, u128, u128, u64),
    // caller, from, to, value, fee, timestamp
    Transfer(PrincipalId, TokenHolder, TokenReceiver, u128, u128, u64),
    // caller, from, value, timestamp
    Burn(PrincipalId, TokenHolder, u128, u64),
}
