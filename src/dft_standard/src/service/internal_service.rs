use crate::state::STATE;
use dft_types::*;

pub(crate) fn verified_created_at(created_at: &Option<u64>, now: &u64) -> CommonResult<()> {
    if created_at.is_none() {
        return Ok(());
    }
    let created_at_time = created_at.unwrap();
    let now = *now;
    if created_at_time + constants::DEFAULT_TRANSACTION_WINDOW < now {
        return Err(DFTError::TxTooOld);
    }
    if created_at_time > now + constants::PERMITTED_DRIFT {
        return Err(DFTError::TxCreatedInFuture);
    }
    Ok(())
}

//charge approve fee
pub(crate) fn charge_approve_fee(
    approver: &TokenHolder,
    approve_fee: TokenAmount,
) -> CommonResult<()> {
    STATE.with(|s| {
        let settings = s.token_setting.borrow();

        let mut balances = s.balances.borrow_mut();
        if balances.balance_of(approver) < approve_fee {
            Err(DFTError::InsufficientBalance)
        } else {
            balances.debit_balance(approver, approve_fee.clone())?;
            balances.credit_balance(&settings.fee_to(), approve_fee.clone());
            Ok(())
        }
    })
}

// charge transfer fee
pub(crate) fn charge_transfer_fee(
    transfer_from: &TokenHolder,
    transfer_fee: TokenAmount,
) -> CommonResult<()> {
    STATE.with(|s| {
        // calc the transfer fee: rate * value
        // compare the transfer fee and minimum fee,get the max value
        let settings = s.token_setting.borrow();

        let mut balances = s.balances.borrow_mut();
        // check the transfer_from's balance
        // if balance is not enough, return error
        if balances.balance_of(transfer_from) < transfer_fee {
            Err(DFTError::InsufficientBalance)
        } else {
            balances.debit_balance(transfer_from, transfer_fee.clone())?;
            balances.credit_balance(&settings.fee_to(), transfer_fee.clone());
            Ok(())
        }
    })
}

// calc transfer fee
pub(crate) fn calc_transfer_fee(transfer_value: &TokenAmount) -> TokenAmount {
    STATE.with(|s| {
        // calc the transfer fee: rate * value
        // compare the transfer fee and minimum fee,get the max value
        let settings = s.token_setting.borrow();
        let fee_setting = settings.fee();
        fee_setting.calc_transfer_fee(transfer_value)
    })
}

//transfer token
pub(crate) fn _transfer(
    tx_invoker: &TokenHolder,
    from: &TokenHolder,
    to: &TokenHolder,
    value: TokenAmount,
    created_at: u64,
    now: u64,
) -> CommonResult<(BlockHeight, BlockHash, TransactionHash)> {
    // calc the transfer fee
    let transfer_fee = calc_transfer_fee(&value);
    let res = STATE.with(|s| {
        let settings = s.token_setting.borrow();
        let mut balances = s.balances.borrow_mut();
        let mut blockchain = s.blockchain.borrow_mut();

        let num_purged = blockchain.tx_window.purge_old_transactions(now);
        if num_purged == 0 {
            blockchain.tx_window.throttle_check(now)?
        }

        //check the transfer_from's balance, if balance is not enough, return error
        if balances.balance_of(from) < value.clone() + transfer_fee.clone() {
            Err(DFTError::InsufficientBalance)
        } else {
            let tx = Transaction {
                operation: Operation::Transfer {
                    caller: *tx_invoker,
                    from: *from,
                    to: *to,
                    value: value.clone(),
                    fee: transfer_fee.clone(),
                },
                created_at,
            };
            let res = blockchain.add_tx_to_block(settings.token_id(), tx, now)?;
            // debit the transfer_from's balance
            balances.debit_balance(from, value.clone())?;
            // credit the transfer_to's balance
            balances.credit_balance(to, value.clone());
            Ok(res)
        }
    });

    if res.is_ok() && transfer_fee > 0u32.into() {
        // charge the transfer fee
        charge_transfer_fee(from, transfer_fee)?;
    }
    res
}
