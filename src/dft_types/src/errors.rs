use candid::{CandidType, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, CandidType, Deserialize, Error)]
pub enum DFTError {
    #[error("DFT: call it anonymous is not allowed")]
    NotAllowAnonymous,
    #[error("DFT: caller is not the owner")]
    OnlyOwnerAllowCallIt,
    #[error("DFT: caller is not the minter")]
    OnlyMinterAllowCallIt,
    #[error("DFT: invalid spender")]
    InvalidSpender,
    #[error("DFT: invalid arg format [from]")]
    InvalidArgFormatFrom,
    #[error("DFT: invalid arg format [to]")]
    InvalidArgFormatTo,
    #[error("DFT: invalid arg format [feeTo]")]
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
    #[error("DFT: storage scaling failed, details {detail:?}")]
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
                detail: error.message,
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
                detail: error.message,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_info() {
        let error = DFTError::NotAllowAnonymous;
        let error_info: ErrorInfo = error.into();
        assert_eq!(error_info.code, 1);
        assert_eq!(error_info.message, "DFT: call it anonymous is not allowed");
    }

    #[test]
    fn test_error_into() {
        let error_info = ErrorInfo {
            code: 1,
            message: "DFT: call it anonymous is not allowed".to_string(),
        };
        let error: DFTError = error_info.into();
        assert_eq!(error, DFTError::NotAllowAnonymous);
    }

    #[test]
    fn test_error_code() {
        assert_eq!(DFTError::NotAllowAnonymous.code(), 1);
        assert_eq!(DFTError::OnlyOwnerAllowCallIt.code(), 2);
        assert_eq!(DFTError::OnlyMinterAllowCallIt.code(), 3);
        assert_eq!(DFTError::InvalidSpender.code(), 4);
        assert_eq!(DFTError::InvalidArgFormatFrom.code(), 5);
        assert_eq!(DFTError::InvalidArgFormatTo.code(), 6);
        assert_eq!(DFTError::InvalidArgFormatFeeTo.code(), 7);
        assert_eq!(DFTError::InsufficientBalance.code(), 8);
        assert_eq!(DFTError::InsufficientAllowance.code(), 9);
        assert_eq!(DFTError::TransferAmountExceedsAllowance.code(), 10);
        assert_eq!(DFTError::TransferAmountExceedsBalance.code(), 11);
        assert_eq!(DFTError::BurnValueTooSmall.code(), 12);
        assert_eq!(DFTError::BurnValueExceedsBalance.code(), 13);
        assert_eq!(DFTError::BurnValueExceedsAllowance.code(), 14);
        assert_eq!(DFTError::NotificationFailed.code(), 15);
        assert_eq!(
            DFTError::StorageScalingFailed {
                detail: "test".to_owned()
            }
            .code(),
            16
        );
        assert_eq!(DFTError::MoveTxToScalingStorageFailed.code(), 17);
        assert_eq!(DFTError::InvalidTypeOrFormatOfLogo.code(), 18);
        assert_eq!(
            DFTError::ApplyBlockFailedByParentHashDoesNotMatch.code(),
            19
        );
        assert_eq!(DFTError::ApplyBlockFailedByInvalidTimestamp.code(), 20);
        assert_eq!(DFTError::TxTooOld.code(), 21);
        assert_eq!(DFTError::TxCreatedInFuture.code(), 22);
        assert_eq!(DFTError::TxDuplicate.code(), 23);
        assert_eq!(
            DFTError::TooManyTransactionsInReplayPreventionWindow.code(),
            24
        );
        assert_eq!(DFTError::NonExistentBlockHeight.code(), 25);
        assert_eq!(DFTError::ExceedTheByteSizeLimitOfOneRequest.code(), 26);
        assert_eq!(DFTError::InvalidTxId.code(), 27);
        assert_eq!(DFTError::TxIdNotBelongToCurrentDft.code(), 28);
        assert_eq!(DFTError::OnlyAllowTokenCanisterCallThisFunction.code(), 29);
        assert_eq!(
            DFTError::Unknown {
                detail: "test".to_owned()
            }
            .code(),
            10000
        );
    }

    #[test]
    fn test_error_message() {
        assert_eq!(
            DFTError::NotAllowAnonymous.to_string(),
            "DFT: call it anonymous is not allowed"
        );
        assert_eq!(
            DFTError::OnlyOwnerAllowCallIt.to_string(),
            "DFT: caller is not the owner"
        );
        assert_eq!(
            DFTError::OnlyMinterAllowCallIt.to_string(),
            "DFT: caller is not the minter"
        );
        assert_eq!(DFTError::InvalidSpender.to_string(), "DFT: invalid spender");
        assert_eq!(
            DFTError::InvalidArgFormatFrom.to_string(),
            "DFT: invalid arg format [from]"
        );
        assert_eq!(
            DFTError::InvalidArgFormatTo.to_string(),
            "DFT: invalid arg format [to]"
        );
        assert_eq!(
            DFTError::InvalidArgFormatFeeTo.to_string(),
            "DFT: invalid arg format [feeTo]"
        );
        assert_eq!(
            DFTError::InsufficientBalance.to_string(),
            "DFT: insufficient balance"
        );
        assert_eq!(
            DFTError::InsufficientAllowance.to_string(),
            "DFT: insufficient allowance"
        );
        assert_eq!(
            DFTError::TransferAmountExceedsAllowance.to_string(),
            "DFT: transfer amount exceeds allowance"
        );
        assert_eq!(
            DFTError::TransferAmountExceedsBalance.to_string(),
            "DFT: transfer amount exceeds balance"
        );
        assert_eq!(
            DFTError::BurnValueTooSmall.to_string(),
            "DFT: burn value is too small"
        );
        assert_eq!(
            DFTError::BurnValueExceedsBalance.to_string(),
            "DFT: burn value exceeds balance"
        );
        assert_eq!(
            DFTError::BurnValueExceedsAllowance.to_string(),
            "DFT: burn value exceeds allowance"
        );
        assert_eq!(
            DFTError::NotificationFailed.to_string(),
            "DFT: notification failed"
        );
        assert_eq!(
            DFTError::StorageScalingFailed {
                detail: "test".to_owned()
            }
            .to_string(),
            "DFT: storage scaling failed, details \"test\""
        );
        assert_eq!(
            DFTError::MoveTxToScalingStorageFailed.to_string(),
            "DFT: move tx to scaling storage failed"
        );
        assert_eq!(
            DFTError::InvalidTypeOrFormatOfLogo.to_string(),
            "DFT: invalid type or format of logo"
        );
        assert_eq!(
            DFTError::ApplyBlockFailedByParentHashDoesNotMatch.to_string(),
            "DFT: cannot apply block because its parent hash doesn't match"
        );
        assert_eq!(
            DFTError::ApplyBlockFailedByInvalidTimestamp.to_string(),
            "DFT: cannot apply block because its timestamp is older than the previous tip"
        );
        assert_eq!(DFTError::TxTooOld.to_string(), "DFT: tx too old");
        assert_eq!(
            DFTError::TxCreatedInFuture.to_string(),
            "DFT: tx created in future"
        );
        assert_eq!(DFTError::TxDuplicate.to_string(), "DFT: tx duplicate");
        assert_eq!(DFTError::TooManyTransactionsInReplayPreventionWindow.to_string(), "DFT: too many transactions in replay prevention window, token is throttling, please retry later");
        assert_eq!(
            DFTError::NonExistentBlockHeight.to_string(),
            "DFT_TX: non-existent block height"
        );
        assert_eq!(
            DFTError::ExceedTheByteSizeLimitOfOneRequest.to_string(),
            "DFT_TX: exceed the byte size limit of one request"
        );
        assert_eq!(DFTError::InvalidTxId.to_string(), "DFT_TX: invalid tx id");
        assert_eq!(
            DFTError::TxIdNotBelongToCurrentDft.to_string(),
            "DFT_TX: tx id does not belong to current dft"
        );
        assert_eq!(
            DFTError::OnlyAllowTokenCanisterCallThisFunction.to_string(),
            "DFT_TX: only allow token canister call this function"
        );
        assert_eq!(
            DFTError::Unknown {
                detail: "test".to_owned()
            }
            .to_string(),
            "Unknown error, detail: \"test\""
        );
    }
}
