use candid::{CandidType, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, CandidType, Deserialize, Error)]
pub enum DFTError {
    #[error("DFT: call it anonymous is not allow")]
    NotAllowAnonymous,
    #[error("DFT: Caller is not the owner")]
    OnlyOwnerAllowCallIt,
    #[error("DFT: Invalid spender")]
    InvalidSpender,
    #[error("DFT: Invalid arg format [from]")]
    InvalidArgFormatFrom,
    #[error("DFT: Invalid arg format [to]")]
    InvalidArgFormatTo,
    #[error("DFT: Invalid arg format [feeTo]")]
    InvalidArgFormatFeeTo,
    #[error("DFT: nonce not match")]
    NonceNotMatch,
    #[error("DFT: insufficient balance")]
    InsufficientBalance,
    #[error("DFT: insufficient allowance")]
    InsufficientAllowance,
    #[error("DFT: transfer amount exceeds allowance")]
    TransferAmountExceedsAllowance,
    #[error("DFT: transfer amount exceeds balance")]
    TransferAmountExceedsBalance,
    #[error("DFT: burn value is too small")]
    BurnValueTooSmall,
    #[error("DFT: burn value exceeds balance")]
    BurnValueExceedsBalance,
    #[error("DFT: burn value exceeds allowance")]
    BurnValueExceedsAllowance,
    #[error("DFT: notification failed")]
    NotificationFailed,
    #[error("DFT: storage scaling failed")]
    StorageScalingFailed,
    #[error("DFT: move tx to scaling storage failed")]
    MoveTxToScalingStorageFailed,
    #[error("DFT: invalid type or format of logo")]
    InvalidTypeOrFormatOfLogo,
    #[error("DFT_TX: invalid tx index")]
    InvalidTxIndex,
    #[error("DFT_TX: invalid tx id")]
    InvalidTxId,
    #[error("DFT_TX: tx id does not belong to current dft")]
    TxIdNotBelongToCurrentDft,
    #[error("DFT_TX: only allow token canister call this function")]
    OnlyAllowTokenCanisterCallThisFunction,
    #[error("Unknown error, detail: {detail:?}")]
    Unknown { detail: String },
}

impl DFTError {
    pub(crate) fn code(&self) -> u32 {
        match self {
            DFTError::NotAllowAnonymous => 1,
            DFTError::OnlyOwnerAllowCallIt => 2,
            DFTError::InvalidSpender => 3,
            DFTError::InvalidArgFormatFrom => 4,
            DFTError::InvalidArgFormatTo => 5,
            DFTError::InvalidArgFormatFeeTo => 6,
            DFTError::NonceNotMatch => 7,
            DFTError::InsufficientBalance => 8,
            DFTError::InsufficientAllowance => 9,
            DFTError::TransferAmountExceedsAllowance => 10,
            DFTError::TransferAmountExceedsBalance => 11,
            DFTError::BurnValueTooSmall => 12,
            DFTError::BurnValueExceedsBalance => 13,
            DFTError::BurnValueExceedsAllowance => 14,
            DFTError::NotificationFailed => 15,
            DFTError::StorageScalingFailed => 16,
            DFTError::MoveTxToScalingStorageFailed => 17,
            DFTError::InvalidTypeOrFormatOfLogo => 18,
            DFTError::InvalidTxIndex => 19,
            DFTError::InvalidTxId => 20,
            DFTError::TxIdNotBelongToCurrentDft => 21,
            DFTError::OnlyAllowTokenCanisterCallThisFunction => 22,
            DFTError::Unknown { .. } => 10000,
        }
    }
}

impl From<DFTError> for ErrorInfo {
    fn from(error: DFTError) -> Self {
        ErrorInfo {
            code: error.code(),
            message: error.to_string(),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, CandidType, Deserialize)]
pub struct ErrorInfo {
    code: u32,
    message: String,
}

pub type CommonResult<T> = anyhow::Result<T, DFTError>;
pub type ActorResult<T> = Result<T, ErrorInfo>;