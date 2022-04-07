use crate::{CommonResult, DFTError, TokenHolder};
use candid::{CandidType, Deserialize, Nat};
use std::collections::HashMap;

#[derive(CandidType, Clone, Default, Debug, Deserialize)]
pub struct TokenBalances {
    balances: HashMap<TokenHolder, Nat>,
    total_supply: Nat,
}

impl TokenBalances {
    pub fn new() -> Self {
        TokenBalances {
            balances: HashMap::new(),
            total_supply: Nat::from(0),
        }
    }

    // holder count
    pub fn holder_count(&self) -> usize {
        self.balances.len()
    }

    // total supply
    pub fn total_supply(&self) -> Nat {
        self.total_supply.clone()
    }

    pub fn balance_of(&self, holder: &TokenHolder) -> Nat {
        self.balances.get(holder).unwrap_or(&Nat::from(0)).clone()
    }

    // debit token holder's balance
    pub fn debit_balance(&mut self, holder: &TokenHolder, value: Nat) -> CommonResult<()> {
        if self.balance_of(holder) < value {
            Err(DFTError::InsufficientBalance)
        } else {
            // calc new balance
            let new_balance = self.balance_of(holder) - value.clone();

            if new_balance > Nat::from(0) {
                self.balances.insert(holder.clone(), new_balance);
            } else {
                self.balances.remove(holder);
            }
            self.total_supply = self.total_supply.clone() - value;

            Ok(())
        }
    }

    // credit token holder's balance
    pub fn credit_balance(&mut self, holder: &TokenHolder, value: Nat) {
        let new_balance = self.balance_of(holder) + value.clone();
        self.balances.insert(holder.clone(), new_balance);
        self.total_supply = self.total_supply.clone() + value;
    }

    // to vec
    pub fn to_vec(&self) -> Vec<(TokenHolder, Nat)> {
        let mut vec = Vec::new();
        for (holder, balance) in self.balances.iter() {
            vec.push((holder.clone(), balance.clone()));
        }
        vec
    }

    // restore from
    pub fn restore_from(&mut self, vec: Vec<(TokenHolder, Nat)>) {
        self.balances = HashMap::new();
        for (holder, balance) in vec {
            self.balances.insert(holder.clone(), balance.clone());
        }
    }
}
