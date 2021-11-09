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
        now: u64,
    ) -> Result<TransactionIndex, String>;
    //burn from
    fn burn_from(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String>;
}

// imple BurnableExtension for TokenBasic
impl BurnableExtension for TokenBasic {
    fn burn(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String> {
        self._burn(caller, owner, value, now)
    }
    fn burn_from(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String> {
        // debit spender's allowance
        self.debit_allowance(owner, spender, value.clone())?;
        self._burn(caller, owner, value, now)
    }
}
