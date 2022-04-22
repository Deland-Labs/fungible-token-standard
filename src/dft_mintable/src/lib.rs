use candid::Principal;
use dft_basic::service::basic_service::verified_created_at;
use dft_basic::state::STATE;
use dft_types::*;

pub fn mint(
    caller: &Principal,
    to: &TokenHolder,
    value: TokenAmount,
    created_at: Option<u64>,
    now: u64,
) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
    verified_created_at(&created_at, &now)?;

    STATE.with(|s| {
        let settings = s.token_setting.borrow();
        settings.only_minter(caller)?;

        let mut blockchain = s.blockchain.borrow_mut();
        let num_purged = blockchain.tx_window.purge_old_transactions(now);
        if num_purged == 0 {
            blockchain.tx_window.throttle_check(now)?
        }

        let created_at = created_at.unwrap_or(now);
        let tx = Transaction {
            operation: Operation::Transfer {
                caller: TokenHolder::new(*caller, None),
                from: TokenHolder::empty(),
                to: *to,
                value: value.clone(),
                fee: 0u32.into(),
            },
            created_at,
        };
        let res = blockchain.add_tx_to_block(settings.token_id(), tx, now)?;
        let mut balances = s.balances.borrow_mut();
        balances.credit_balance(to, value);
        Ok(res)
    })
}

pub fn minters() -> Vec<Principal> {
    STATE.with(|s| {
        let settings = s.token_setting.borrow();
        settings.minters()
    })
}
pub fn add_minter(
    caller: &Principal,
    minter: Principal,
    created_at: Option<u64>,
    now: u64,
) -> CommonResult<bool> {
    verified_created_at(&created_at, &now)?;
    STATE.with(|s| {
        let mut settings = s.token_setting.borrow_mut();
        settings.only_owner(caller)?;
        if settings.minters().contains(&minter) {
            return Ok(true);
        }
        let created_at = created_at.unwrap_or(now);
        let tx = Transaction {
            operation: Operation::AddMinter {
                caller: *caller,
                minter,
            },
            created_at,
        };
        let mut blockchain = s.blockchain.borrow_mut();
        blockchain.add_tx_to_block(settings.token_id(), tx, now)?;
        settings.add_minter(minter);
        Ok(true)
    })
}
pub fn remove_minter(
    caller: &Principal,
    minter: Principal,
    created_at: Option<u64>,
    now: u64,
) -> CommonResult<bool> {
    verified_created_at(&created_at, &now)?;
    STATE.with(|s| {
        let mut settings = s.token_setting.borrow_mut();
        settings.only_owner(caller)?;
        if !settings.minters().contains(&minter) {
            return Ok(true);
        }
        let created_at = created_at.unwrap_or(now);
        let tx = Transaction {
            operation: Operation::RemoveMinter {
                caller: *caller,
                minter,
            },
            created_at,
        };
        let mut blockchain = s.blockchain.borrow_mut();
        blockchain.add_tx_to_block(settings.token_id(), tx, now)?;
        settings.remove_minter(minter);
        Ok(true)
    })
}
