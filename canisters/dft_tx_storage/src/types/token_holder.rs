use candid::Principal;
use ic_cdk::export::candid::{CandidType, Deserialize};
use std::{
    fmt::{self, Display},
    str::FromStr,
    string::String,
};

use super::AccountIdentifier;
#[derive(CandidType, Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
pub enum TokenHolder {
    Account(AccountIdentifier),
    Principal(Principal),
}

impl FromStr for TokenHolder {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pid = s.parse::<Principal>();
        match pid {
            Ok(_principal) => Ok(TokenHolder::Principal(_principal)),
            _ => {
                let account_identity = s.parse::<AccountIdentifier>();
                match account_identity {
                    Ok(_ai) => Ok(TokenHolder::Account(_ai)),
                    _ => Err("invalid token holder format".to_string()),
                }
            }
        }
    }
}

impl Display for TokenHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            TokenHolder::Account(_ai) => _ai.to_string(),
            TokenHolder::Principal(_pid) => _pid.to_string(),
        };
        write!(f, "{}", s)
    }
}
