use candid::{CandidType, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, CandidType, Deserialize, Error)]
pub enum DFTError {
    #[error("DFT: call it anonymous is not allow")]
    NotAllowAnonymous,
    #[error("DFT: Caller is not the owner")]
    OnlyOwnerAllowCallIt,
    #[error("DFT: Caller is not the minter")]
    OnlyMinterAllowCallIt,
    #[error("DFT: Invalid spender")]
    InvalidSpender,
    #[error("DFT: Invalid arg format [from]")]
    InvalidArgFormatFrom,
    #[error("DFT: Invalid arg format [to]")]
    InvalidArgFormatTo,
    #[error("DFT: Invalid arg format [feeTo]")]
    InvalidArgFormatFeeTo,
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
    #[error("DFT: storage scaling failed, details: {detail:?}")]
    StorageScalingFailed { detail: String },
    #[error("DFT: move tx to scaling storage failed")]
    MoveTxToScalingStorageFailed,
    #[error("DFT: invalid type or format of logo")]
    InvalidTypeOrFormatOfLogo,
    #[error("DFT: cannot apply block because its parent hash doesn't match")]
    ApplyBlockFailedByParentHashDoesNotMatch,
    #[error("DFT: cannot apply block because its timestamp is older than the previous tip")]
    ApplyBlockFailedByInvalidTimestamp,
    #[error("DFT: tx too old")]
    TxTooOld,
    #[error("DFT: tx created in future")]
    TxCreatedInFuture,
    #[error("DFT: tx duplicate")]
    TxDuplicate,
    #[error("DFT: too many transactions in replay prevention window, token is throttling, please retry later")]
    TooManyTransactionsInReplayPreventionWindow,
    #[error("DFT_TX: non-existent block height")]
    NonExistentBlockHeight,
    #[error("DFT_TX: exceed the byte size limit of one request")]
    ExceedTheByteSizeLimitOfOneRequest,
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
            DFTError::OnlyMinterAllowCallIt => 3,
            DFTError::InvalidSpender => 4,
            DFTError::InvalidArgFormatFrom => 5,
            DFTError::InvalidArgFormatTo => 6,
            DFTError::InvalidArgFormatFeeTo => 7,
            DFTError::InsufficientBalance => 8,
            DFTError::InsufficientAllowance => 9,
            DFTError::TransferAmountExceedsAllowance => 10,
            DFTError::TransferAmountExceedsBalance => 11,
            DFTError::BurnValueTooSmall => 12,
            DFTError::BurnValueExceedsBalance => 13,
            DFTError::BurnValueExceedsAllowance => 14,
            DFTError::NotificationFailed => 15,
            DFTError::StorageScalingFailed { .. } => 16,
            DFTError::MoveTxToScalingStorageFailed => 17,
            DFTError::InvalidTypeOrFormatOfLogo => 18,
            DFTError::ApplyBlockFailedByParentHashDoesNotMatch => 19,
            DFTError::ApplyBlockFailedByInvalidTimestamp => 20,
            DFTError::TxTooOld => 21,
            DFTError::TxCreatedInFuture => 22,
            DFTError::TxDuplicate => 23,
            DFTError::TooManyTransactionsInReplayPreventionWindow => 24,
            DFTError::NonExistentBlockHeight => 25,
            DFTError::ExceedTheByteSizeLimitOfOneRequest => 26,
            DFTError::InvalidTxId => 27,
            DFTError::TxIdNotBelongToCurrentDft => 28,
            DFTError::OnlyAllowTokenCanisterCallThisFunction => 29,
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

impl From<ErrorInfo> for DFTError {
    fn from(error: ErrorInfo) -> Self {
        match error.code {
            1 => DFTError::NotAllowAnonymous,
            2 => DFTError::OnlyOwnerAllowCallIt,
            3 => DFTError::OnlyMinterAllowCallIt,
            4 => DFTError::InvalidSpender,
            5 => DFTError::InvalidArgFormatFrom,
            6 => DFTError::InvalidArgFormatTo,
            7 => DFTError::InvalidArgFormatFeeTo,
            8 => DFTError::InsufficientBalance,
            9 => DFTError::InsufficientAllowance,
            10 => DFTError::TransferAmountExceedsAllowance,
            11 => DFTError::TransferAmountExceedsBalance,
            12 => DFTError::BurnValueTooSmall,
            13 => DFTError::BurnValueExceedsBalance,
            14 => DFTError::BurnValueExceedsAllowance,
            15 => DFTError::NotificationFailed,
            16 => DFTError::StorageScalingFailed {
                detail: error.message.to_string(),
            },
            17 => DFTError::MoveTxToScalingStorageFailed,
            18 => DFTError::InvalidTypeOrFormatOfLogo,
            19 => DFTError::ApplyBlockFailedByParentHashDoesNotMatch,
            20 => DFTError::ApplyBlockFailedByInvalidTimestamp,
            21 => DFTError::TxTooOld,
            22 => DFTError::TxCreatedInFuture,
            23 => DFTError::TxDuplicate,
            24 => DFTError::TooManyTransactionsInReplayPreventionWindow,
            25 => DFTError::NonExistentBlockHeight,
            26 => DFTError::ExceedTheByteSizeLimitOfOneRequest,
            27 => DFTError::InvalidTxId,
            28 => DFTError::TxIdNotBelongToCurrentDft,
            29 => DFTError::OnlyAllowTokenCanisterCallThisFunction,
            _ => DFTError::Unknown {
                detail: error.message.to_string(),
            },
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
