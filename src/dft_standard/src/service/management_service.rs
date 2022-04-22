use super::internal_service::verified_created_at;
use crate::state::STATE;
use candid::Principal;
use dft_types::*;
use dft_utils::*;
use std::collections::HashMap;

pub fn set_owner(
    caller: &Principal,
    new_owner: Principal,
    created_at: Option<u64>,
    now: u64,
) -> CommonResult<bool> {
    verified_created_at(&created_at, &now)?;

    STATE.with(|s| {
        let mut settings = s.token_setting.borrow_mut();
        let mut blockchain = s.blockchain.borrow_mut();
        settings.only_owner(caller)?;
        if settings.owner() == new_owner {
            return Ok(true);
        }

        let num_purged = blockchain.tx_window.purge_old_transactions(now);
        if num_purged == 0 {
            blockchain.tx_window.throttle_check(now)?
        }

        let created_at = created_at.unwrap_or(now);
        let tx = Transaction {
            operation: Operation::OwnerModify {
                caller: *caller,
                new_owner,
            },
            created_at,
        };

        blockchain.add_tx_to_block(settings.token_id(), tx, now)?;
        settings.set_owner(new_owner);
        Ok(true)
    })
}

pub fn set_fee(
    caller: &Principal,
    new_fee: TokenFee,
    created_at: Option<u64>,
    now: u64,
) -> CommonResult<bool> {
    verified_created_at(&created_at, &now)?;

    STATE.with(|s| {
        let mut settings = s.token_setting.borrow_mut();
        let mut blockchain = s.blockchain.borrow_mut();
        settings.only_owner(caller)?;

        let num_purged = blockchain.tx_window.purge_old_transactions(now);
        if num_purged == 0 {
            blockchain.tx_window.throttle_check(now)?
        }

        let created_at = created_at.unwrap_or(now);
        let tx = Transaction {
            operation: Operation::FeeModify {
                caller: *caller,
                new_fee: new_fee.clone(),
            },
            created_at,
        };

        blockchain.add_tx_to_block(settings.token_id(), tx, now)?;
        settings.set_fee(new_fee);
        Ok(true)
    })
}

pub fn set_fee_to(
    caller: &Principal,
    new_fee_to: TokenHolder,
    created_at: Option<u64>,
    now: u64,
) -> CommonResult<bool> {
    verified_created_at(&created_at, &now)?;

    STATE.with(|s| {
        let mut settings = s.token_setting.borrow_mut();
        let mut blockchain = s.blockchain.borrow_mut();
        settings.only_owner(caller)?;

        let num_purged = blockchain.tx_window.purge_old_transactions(now);
        if num_purged == 0 {
            blockchain.tx_window.throttle_check(now)?
        }

        let created_at = created_at.unwrap_or(now);
        let tx = Transaction {
            operation: Operation::FeeToModify {
                caller: *caller,
                new_fee_to,
            },
            created_at,
        };

        blockchain.add_tx_to_block(settings.token_id(), tx, now)?;
        settings.set_fee_to(new_fee_to);
        Ok(true)
    })
}

pub fn set_desc(caller: &Principal, descriptions: HashMap<String, String>) -> CommonResult<bool> {
    STATE.with(|s| {
        let settings = s.token_setting.borrow_mut();
        let mut token_desc = s.token_desc.borrow_mut();
        settings.only_owner(caller)?;
        token_desc.set_all(descriptions);
        Ok(true)
    })
}

pub fn set_logo(caller: &Principal, logo: Option<Vec<u8>>) -> CommonResult<bool> {
    STATE.with(|s| {
        let mut settings = s.token_setting.borrow_mut();
        settings.only_owner(caller)?;
        if logo.is_some() {
            get_logo_type(&logo.clone().unwrap())
                .map_err(|_| DFTError::InvalidTypeOrFormatOfLogo)?;
        }
        settings.set_logo(logo);
        Ok(true)
    })
}
