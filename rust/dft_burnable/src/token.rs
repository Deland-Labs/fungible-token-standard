use candid::{Nat, Principal};
use dft_standard::token::TokenBasic;
use dft_types::*;

pub trait BurnableExtension {
    //burn
    fn burn(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        value: Nat,
        nonce: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex>;
    //burn from
    fn burn_from(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: Nat,
        nonce: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex>;
}

// imple BurnableExtension for TokenBasic
impl BurnableExtension for TokenBasic {
    fn burn(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        value: Nat,
        nonce: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        self.not_allow_anonymous(caller)?;
        self._burn(caller, owner, value, nonce, now)
    }
    fn burn_from(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: Nat,
        nonce: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        self.not_allow_anonymous(caller)?;
        // debit spender's allowance
        self.debit_allowance(owner, spender, value.clone())?;
        self._burn(caller, owner, value, nonce, now)
    }
}
