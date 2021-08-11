use candid::CandidType;
use ic_cdk::export::Principal;
use serde::{de, de::Error, Deserialize, Serialize};
use sha2::{Digest, Sha224};
use std::fmt;
use std::{
    collections::HashMap,
    convert::TryInto,
    fmt::{Display, Formatter},
    str::FromStr,
    string::String,
};

pub type TransactionId = u128;
pub type ExtendData = HashMap<String, String>;
pub type Balances = HashMap<TokenHolder, u128>;
pub type Allowances = HashMap<TokenHolder, HashMap<TokenHolder, u128>>;
#[derive(CandidType, Debug, Deserialize)]
pub struct TokenPayload {
    pub initialized: bool,
    pub owner: Principal,
    pub fee_cashier: TokenHolder,
    pub meta: MetaData,
    pub extend: Vec<(String, String)>,
    pub logo: Vec<u8>,
    pub balances: Vec<(TokenHolder, u128)>,
    pub allowances: Vec<(TokenHolder, Vec<(TokenHolder, u128)>)>,
    pub total_fee: u128,
    pub tx_id_cursor: u128,
    pub storage_canister_id: Principal,
}
// Rate decimals = 8
// transferFee = amount * rate / 1000000
#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum Fee {
    Fixed(u128),
    RateWithLowestLimit(u128, u8),
}

impl fmt::Display for Fee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            Fee::Fixed(_fee) => _fee.to_string(),
            Fee::RateWithLowestLimit(_fee, rate) => format!("{{lowest:{0},rate:{1}}}", _fee, rate),
        };
        write!(f, "{}", s)
    }
}

#[derive(CandidType, Debug, Deserialize)]
pub struct MetaData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub fee: Fee,
}

#[derive(CandidType, Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TokenHolder {
    Account(AccountIdentifier),
    Principal(Principal),
}

#[derive(CandidType, Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct KeyValuePair {
    pub k: String,
    pub v: String,
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
impl fmt::Display for TokenHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            TokenHolder::Account(_ai) => _ai.to_string(),
            TokenHolder::Principal(_pid) => _pid.to_string(),
        };
        write!(f, "{}", s)
    }
}

pub type TransferFrom = TokenHolder;
pub type TokenReceiver = TokenHolder;

#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
pub struct CallData {
    pub method: String,
    pub args: Vec<u8>,
}

#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
pub enum TransferResult {
    //transfer succeed, but call failed & notify failed
    Ok(TransactionId, Option<Vec<String>>),
    Err(String),
}

#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
pub enum BurnResult {
    Ok,
    Err(String),
}

// Invalid data: Invalid IDL blob by candid 0.6.21
#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
pub enum ApproveResult {
    Ok(Option<String>),
    Err(String),
}

#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
pub enum TxRecord {
    // caller, owner, spender, value, fee, timestamp
    Approve(Principal, TokenHolder, TokenReceiver, u128, u128, u64),
    // caller, from, to, value, fee, timestamp
    Transfer(Principal, TokenHolder, TokenReceiver, u128, u128, u64),
    // caller, from, value, timestamp
    Burn(Principal, TokenHolder, u128, u64),
}

/// While this is backed by an array of length 28, it's canonical representation
/// is a hex string of length 64. The first 8 characters are the CRC-32 encoded
/// hash of the following 56 characters of hex. Both, upper and lower case
/// characters are valid in the input string and can even be mixed.
///
/// When it is encoded or decoded it will always be as a string to make it
/// easier to use from DFX.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountIdentifier {
    pub hash: [u8; 28],
}

impl AccountIdentifier {}

pub static SUB_ACCOUNT_ZERO: Subaccount = Subaccount([0; 32]);
static ACCOUNT_DOMAIN_SEPERATOR: &[u8] = b"\x0Aaccount-id";

impl AccountIdentifier {
    pub fn new(account: Principal, sub_account: Option<Subaccount>) -> AccountIdentifier {
        let mut hash = Sha224::new();
        hash.update(ACCOUNT_DOMAIN_SEPERATOR);
        hash.update(account.as_slice());

        let sub_account = sub_account.unwrap_or(SUB_ACCOUNT_ZERO);
        hash.update(&sub_account.0[..]);

        AccountIdentifier {
            hash: hash.finalize().into(),
        }
    }

    pub fn from_hex(hex_str: &str) -> Result<AccountIdentifier, String> {
        let hex: Vec<u8> = hex::decode(hex_str).map_err(|e| e.to_string())?;
        Self::from_slice(&hex[..])
    }

    /// Goes from the canonical format (with checksum) encoded in bytes rather
    /// than hex to AccountIdentifier
    pub fn from_slice(v: &[u8]) -> Result<AccountIdentifier, String> {
        // Trim this down when we reach rust 1.48
        let hex: Box<[u8; 32]> = match v.to_vec().into_boxed_slice().try_into() {
            Ok(h) => h,
            Err(_) => {
                let hex_str = hex::encode(v);
                return Err(format!(
                    "{} has a length of {} but we expected a length of 64",
                    hex_str,
                    hex_str.len()
                ));
            }
        };
        check_sum(*hex)
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_vec())
    }

    pub fn to_vec(&self) -> Vec<u8> {
        [&self.generate_checksum()[..], &self.hash[..]].concat()
    }

    pub fn generate_checksum(&self) -> [u8; 4] {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&self.hash);
        hasher.finalize().to_be_bytes()
    }
}

impl Display for AccountIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_hex().fmt(f)
    }
}

impl FromStr for AccountIdentifier {
    type Err = String;

    fn from_str(s: &str) -> Result<AccountIdentifier, String> {
        AccountIdentifier::from_hex(s)
    }
}

impl Serialize for AccountIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_hex().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AccountIdentifier {
    // This is the canonical way to read a this from string
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
        D::Error: de::Error,
    {
        let hex: [u8; 32] = hex::serde::deserialize(deserializer)?;
        check_sum(hex).map_err(D::Error::custom)
    }
}

impl From<Principal> for AccountIdentifier {
    fn from(pid: Principal) -> Self {
        AccountIdentifier::new(pid, None)
    }
}

fn check_sum(hex: [u8; 32]) -> Result<AccountIdentifier, String> {
    // Get the checksum provided
    let found_checksum = &hex[0..4];

    // Copy the hash into a new array
    let mut hash = [0; 28];
    hash.copy_from_slice(&hex[4..32]);

    let account_id = AccountIdentifier { hash };
    let expected_checksum = account_id.generate_checksum();

    // Check the generated checksum matches
    if expected_checksum == found_checksum {
        Ok(account_id)
    } else {
        Err(format!(
            "Checksum failed for {}, expected check bytes {} but found {}",
            hex::encode(&hex[..]),
            hex::encode(expected_checksum),
            hex::encode(found_checksum),
        ))
    }
}

impl CandidType for AccountIdentifier {
    // The type expected for account identifier is
    fn _ty() -> candid::types::Type {
        String::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        self.to_hex().idl_serialize(serializer)
    }
}

/// Subaccounts are arbitrary 32-byte values.
#[derive(Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq, Copy)]
#[serde(transparent)]
pub struct Subaccount(pub [u8; 32]);

#[test]
fn check_round_trip() {
    let ai = AccountIdentifier { hash: [7; 28] };
    let res = ai.to_hex();
    assert_eq!(
        res.parse(),
        Ok(ai),
        "The account identifier doesn't change after going back and forth between a string"
    )
}

#[test]
fn check_encoding() {
    let ai = AccountIdentifier { hash: [7; 28] };

    let en1 = candid::encode_one(ai).unwrap();
    let en2 = candid::encode_one(ai.to_string()).unwrap();

    assert_eq!(
        &en1, &en2,
        "Candid encoding of an account identifier and a string should be identical"
    );

    let de1: String = candid::decode_one(&en1[..]).unwrap();
    let de2: AccountIdentifier = candid::decode_one(&en2[..]).unwrap();

    assert_eq!(
        de1.parse(),
        Ok(de2),
        "The types are the same after decoding, even through a different type"
    );

    assert_eq!(de2, ai, "And the value itself hasn't changed");
}
