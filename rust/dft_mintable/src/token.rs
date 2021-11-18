use candid::{Nat, Principal};
use dft_standard::token::TokenBasic;
use dft_types::*;

pub trait MintableExtension {
    //mint
    fn mint(
        &mut self,
        caller: &Principal,
        to: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> CommonResult<TransactionIndex>;
}

// impl MintableExtension for TokenBasic
impl MintableExtension for TokenBasic {
    fn mint(
        &mut self,
        caller: &Principal,
        to: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        self.only_owner(caller)?;
        self._mint(caller, to, value, now)
    }
}
