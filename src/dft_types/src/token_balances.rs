use std::collections::HashMap;

use candid::Deserialize;
use num_traits::CheckedSub;
use serde::Serialize;

use crate::{CommonResult, DFTError, StableState, TokenAmount, TokenHolder};

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
            let new_balance = self.balance_of(holder).checked_sub(&value).unwrap();

            if new_balance > TokenAmount::from(0u32) {
                self.balances.insert(*holder, new_balance);
            } else {
                self.balances.remove(holder);
            }
            self.total_supply = self.total_supply.clone().checked_sub(&value).unwrap();

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

#[cfg(test)]
mod tests {
    use crate::{TokenAmount, TokenHolder};

    use super::*;

    #[test]
    fn test_token_balances() {
        let mut balances = TokenBalances::new();
        let holder1 = TokenHolder::new(
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap(),
            None,
        );
        let holder2 = TokenHolder::new(
            "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                .parse()
                .unwrap(),
            None,
        );

        let value1 = TokenAmount::from(100u32);
        let value2 = TokenAmount::from(200u32);

        balances.credit_balance(&holder1, value1.clone());
        balances.credit_balance(&holder2, value2.clone());

        assert_eq!(balances.balance_of(&holder1), value1);
        assert_eq!(balances.balance_of(&holder2), value2);
        assert_eq!(balances.total_supply(), value1.clone() + value2.clone());
        assert_eq!(balances.holder_count(), 2);

        let res = balances.debit_balance(&holder1, value1.clone() + 1u32);
        assert_eq!(res, Err(DFTError::InsufficientBalance));

        balances.debit_balance(&holder1, value1.clone()).unwrap();
        balances.debit_balance(&holder2, value2.clone()).unwrap();

        assert_eq!(balances.balance_of(&holder1), TokenAmount::from(0u32));
        assert_eq!(balances.balance_of(&holder2), TokenAmount::from(0u32));
        assert_eq!(balances.total_supply(), 0u32.into());
        assert_eq!(balances.holder_count(), 0);
    }

    #[test]
    fn test_token_balances_to_vec() {
        let mut balances = TokenBalances::new();
        let holder1 = TokenHolder::new(
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap(),
            None,
        );
        let holder2 = TokenHolder::new(
            "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                .parse()
                .unwrap(),
            None,
        );
        let value1 = TokenAmount::from(100u32);
        let value2 = TokenAmount::from(200u32);

        balances.credit_balance(&holder1, value1.clone());
        balances.credit_balance(&holder2, value2.clone());
        let vec = balances.to_vec();
        assert_eq!(vec.len(), 2);
    }
}
