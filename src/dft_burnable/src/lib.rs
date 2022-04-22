use candid::Principal;
use dft_basic::{service::basic_service::verified_created_at, state::STATE};
use dft_types::*;

pub fn burn(
    caller: &Principal,
    owner: &TokenHolder,
    value: TokenAmount,
    created_at: Option<u64>,
    now: u64,
) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
    verified_created_at(&created_at, &now)?;
    STATE.with(|s| {
        let settings = s.token_setting.borrow();
        settings.not_allow_anonymous(caller)?;

        let mut blockchain = s.blockchain.borrow_mut();

        let num_purged = blockchain.tx_window.purge_old_transactions(now);
        if num_purged == 0 {
            blockchain.tx_window.throttle_check(now)?
        }

        let created_at = created_at.unwrap_or(now);
        // calc the transfer fee,if the burn amount small than minimum fee,return error
        if value < settings.fee().minimum {
            return Err(DFTError::BurnValueTooSmall);
        }

        let mut balances = s.balances.borrow_mut();
        //check the burn from holder's balance, if balance is not enough, return error
        if balances.balance_of(owner) < value {
            return Err(DFTError::InsufficientBalance);
        }

        let tx = Transaction {
            operation: Operation::Transfer {
                caller: *owner,
                from: *owner,
                to: TokenHolder::empty(),
                value: value.clone(),
                fee: 0u32.into(),
            },
            created_at,
        };
        let res = blockchain.add_tx_to_block(settings.token_id(), tx, now)?;
        // burn does not charge the transfer fee
        // debit the burn from holder's balance
        balances.debit_balance(owner, value)?;
        Ok(res)
    })
}
pub fn burn_from(
    caller: &Principal,
    owner: &TokenHolder,
    spender: &TokenHolder,
    value: TokenAmount,
    created_at: Option<u64>,
    now: u64,
) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
    verified_created_at(&created_at, &now)?;
    STATE.with(|s| {
        let settings = s.token_setting.borrow();
        settings.not_allow_anonymous(caller)?;

        let mut blockchain = s.blockchain.borrow_mut();

        let num_purged = blockchain.tx_window.purge_old_transactions(now);
        if num_purged == 0 {
            blockchain.tx_window.throttle_check(now)?
        }

        let created_at = created_at.unwrap_or(now);
        if value < settings.fee().minimum {
            return Err(DFTError::BurnValueTooSmall);
        }

        let mut balances = s.balances.borrow_mut();
        //check the burn from holder's balance, if balance is not enough, return error
        if balances.balance_of(owner) < value {
            Err(DFTError::InsufficientBalance)
        } else {
            let tx = Transaction {
                operation: Operation::Transfer {
                    caller: *spender,
                    from: *owner,
                    to: TokenHolder::empty(),
                    value: value.clone(),
                    fee: 0u32.into(),
                },
                created_at,
            };
            let res = blockchain.add_tx_to_block(settings.token_id(), tx, now)?;
            s.allowances
                .borrow_mut()
                .debit(owner, spender, value.clone())?;
            // burn does not charge the transfer fee
            // debit the burn from holder's balance
            balances.debit_balance(owner, value)?;
            Ok(res)
        }
    })
}
