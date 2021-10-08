#![allow(dead_code)]
pub const MSG_ONLY_OWNER: &str = "DFT: caller is not the owner";
pub const MSG_INVALID_SPENDER: &str = "DFT: invalid spender";
pub const MSG_INVALID_FROM: &str = "DFT: invalid format [from]";
pub const MSG_INVALID_TO: &str = "DFT: invalid format [to]";
pub const MSG_INVALID_FEE_TO: &str = "DFT: invalid format [feeTo]";
pub const MSG_INSUFFICIENT_BALANCE: &str = "DFT: insufficient balance";
pub const MSG_ALLOWANCE_EXCEEDS: &str = "DFT: transfer amount exceeds allowance";
pub const MSG_BALANCE_EXCEEDS: &str = "DFT: transfer amount exceeds balance";
pub const MSG_BURN_VALUE_TOO_SMALL: &str = "DFT: burn value is too small";
pub const MSG_BURN_VALUE_EXCEEDS: &str = "DFT: burn value exceeds balance";
pub const MSG_BURN_FROM_VALUE_EXCEEDS: &str = "DFT: burn amount exceeds allowance";
pub const MSG_NOTIFICATION_FAILED: &str = "DFT: notification failed";
pub const MSG_STORAGE_SCALING_FAILED: &str = "DFT: storage scaling failed";

pub const MSG_OUT_OF_TX_INDEX_RANGE: &str = "DFT: out of tx index range";
pub const MSG_GET_LAST_TXS_SIZE_TOO_LARGE: &str = "DFT: size too large, max size is 100";
pub const MSG_INVALID_TX_ID: &str = "DFT_TX: invalid tx id";
pub const MSG_NOT_BELONG_DFT_TX_ID: &str = "DFT_TX: tx id not belong to the current dft";
