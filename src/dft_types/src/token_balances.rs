use crate::{CommonResult, DFTError, StableState, TokenAmount, TokenHolder};
use candid::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct TokenBalances {
    balances: HashMap<TokenHolder, TokenAmount>,
    total_supply: TokenAmount,
}

impl TokenBalances {
    pub fn new() -> Self {
        TokenBalances {
            balances: HashMap::new(),
            total_supply: TokenAmount::default(),
        }
    }

    // holder count
    pub fn holder_count(&self) -> usize {
        self.balances.len()
    }

    // total supply
    pub fn total_supply(&self) -> TokenAmount {
        self.total_supply.clone()
    }

    pub fn balance_of(&self, holder: &TokenHolder) -> TokenAmount {
        if let Some(balance) = self.balances.get(holder) {
            balance.clone()
        } else {
            TokenAmount::default()
        }
    }

    // debit token holder's balance
    pub fn debit_balance(&mut self, holder: &TokenHolder, value: TokenAmount) -> CommonResult<()> {
        if self.balance_of(holder) < value {
            Err(DFTError::InsufficientBalance)
        } else {
            // calc new balance
            let new_balance = self.balance_of(holder) - value.clone();

            if new_balance > TokenAmount::from(0u32) {
                self.balances.insert(*holder, new_balance);
            } else {
                self.balances.remove(holder);
            }
            self.total_supply = self.total_supply.clone() - value;

            Ok(())
        }
    }

    // credit token holder's balance
    pub fn credit_balance(&mut self, holder: &TokenHolder, value: TokenAmount) {
        let new_balance = self.balance_of(holder) + value.clone();
        self.balances.insert(*holder, new_balance);
        self.total_supply = self.total_supply.clone() + value;
    }

    // to vec
    pub fn to_vec(&self) -> Vec<(TokenHolder, TokenAmount)> {
        let mut vec = Vec::new();
        for (holder, balance) in self.balances.iter() {
            vec.push((*holder, balance.clone()));
        }
        vec
    }

    // restore from
    pub fn restore_from(&mut self, vec: Vec<(TokenHolder, TokenAmount)>) {
        self.balances = HashMap::new();
        for (holder, balance) in vec {
            self.balances.insert(holder, balance);
        }
    }
}

impl StableState for TokenBalances {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(&(&self.balances, &self.total_supply)).unwrap()
    }

    fn decode(bytes: Vec<u8>) -> Result<Self, String> {
        let (balances, total_supply): (HashMap<TokenHolder, TokenAmount>, TokenAmount) =
            bincode::deserialize(&bytes).unwrap();

        Ok(TokenBalances {
            balances,
            total_supply,
        })
    }
}
