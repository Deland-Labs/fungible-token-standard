use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::{
    fmt::{self, Display},
    str::FromStr,
    string::String,
};

use super::{AccountIdentifier, Subaccount};

#[derive(
    CandidType, Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum TokenHolder {
    Account(AccountIdentifier),
    Principal(Principal),
    None,
}

impl TokenHolder {
    pub fn new(principal: Principal, sub_account: Option<Subaccount>) -> TokenHolder {
        match sub_account {
            Some(_) => {
                let account_identity = AccountIdentifier::new(principal, sub_account);
                TokenHolder::Account(account_identity)
            }
            _ => TokenHolder::Principal(principal),
        }
    }
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
            TokenHolder::Account(_ai) => _ai.to_hex(),
            TokenHolder::Principal(_pid) => _pid.to_text(),
            TokenHolder::None => "".into(),
        };
        write!(f, "{}", s)
    }
}

#[test]
fn test_token_holder_size() {
    let token_holder_size = std::mem::size_of::<TokenHolder>();
    assert_eq!(
        31, token_holder_size,
        "token_holder_size is  not {}",
        token_holder_size
    );
}
