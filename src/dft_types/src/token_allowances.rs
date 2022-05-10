use crate::{CommonResult, DFTError, StableState, TokenAmount, TokenHolder};
use candid::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct TokenAllowances(HashMap<TokenHolder, HashMap<TokenHolder, TokenAmount>>);

impl TokenAllowances {
    pub fn new() -> Self {
        TokenAllowances(HashMap::new())
    }

    pub fn allowance_size(&self) -> usize {
        match self.0.len() {
            0 => 0,
            _ => self.0.values().map(|v| v.len()).sum(),
        }
    }

    pub fn allowance(&self, owner: &TokenHolder, spender: &TokenHolder) -> TokenAmount {
        if let Some(allowances) = self.0.get(owner) {
            if let Some(amount) = allowances.get(spender) {
                return amount.clone();
            }
        }
        TokenAmount::from(0u32)
    }

    pub fn allowances_of(&self, owner: &TokenHolder) -> Vec<(TokenHolder, TokenAmount)> {
        let mut vec = Vec::new();
        if let Some(allowances) = self.0.get(owner) {
            for (spender, amount) in allowances {
                vec.push((*spender, amount.clone()));
            }
        }
        vec
    }

    //debit token holder's allowance
    pub fn debit(
        &mut self,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: TokenAmount,
    ) -> CommonResult<()> {
        // get spenders allowance
        let spender_allowance = self.allowance(owner, spender);
        // check allowance
        if spender_allowance < value {
            return Err(DFTError::InsufficientAllowance);
        }
        let new_spender_allowance = spender_allowance - value.clone();
        match self.0.get(owner) {
            Some(inner) => {
                let mut temp = inner.clone();
                if new_spender_allowance == TokenAmount::from(0u32) {
                    temp.remove(spender);
                    if temp.is_empty() {
                        self.0.remove(owner);
                    } else {
                        self.0.insert(*owner, temp);
                    }
                } else {
                    temp.insert(*spender, new_spender_allowance);
                    self.0.insert(*owner, temp);
                }
            }
            None => {
                if value > TokenAmount::from(0u32) {
                    let mut inner = HashMap::new();
                    inner.insert(*spender, new_spender_allowance);
                    self.0.insert(*owner, inner);
                }
            }
        };
        Ok(())
    }

    // credit token spender's allowance
    pub fn credit(&mut self, owner: &TokenHolder, spender: &TokenHolder, value: TokenAmount) {
        match self.0.get(owner) {
            Some(inner) => {
                let mut temp = inner.clone();
                if value == TokenAmount::from(0u32) {
                    temp.remove(spender);
                    if temp.is_empty() {
                        self.0.remove(owner);
                    } else {
                        self.0.insert(*owner, temp);
                    }
                } else {
                    temp.insert(*spender, value);
                    self.0.insert(*owner, temp);
                }
            }
            None => {
                if value > TokenAmount::from(0u32) {
                    let mut inner = HashMap::new();
                    inner.insert(*spender, value);
                    self.0.insert(*owner, inner);
                }
            }
        };
    }

    // to vec
    pub fn to_vec(&self) -> Vec<(TokenHolder, Vec<(TokenHolder, TokenAmount)>)> {
        let mut allowances = Vec::new();
        for (th, v) in self.0.iter() {
            let mut allow_item = Vec::new();
            for (sp, val) in v.iter() {
                allow_item.push((*sp, val.clone()));
            }
            allowances.push((*th, allow_item));
        }
        allowances
    }

    // restore from
    pub fn restore_from(
        &mut self,
        allowances: Vec<(TokenHolder, Vec<(TokenHolder, TokenAmount)>)>,
    ) {
        for (th, v) in allowances.iter() {
            let mut allow_item = HashMap::new();
            for (sp, val) in v.iter() {
                allow_item.insert(*sp, val.clone());
            }
            self.0.insert(*th, allow_item);
        }
    }
}

impl StableState for TokenAllowances {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(&self.0).unwrap()
    }

    fn decode(bytes: Vec<u8>) -> Result<Self, String> {
        let allowances: HashMap<TokenHolder, HashMap<TokenHolder, TokenAmount>> =
            bincode::deserialize(&bytes).unwrap();

        Ok(TokenAllowances(allowances))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TokenAmount, TokenHolder};

    #[test]
    fn test_token_allowances() {
        let mut allowances = TokenAllowances::new();
        let owner = TokenHolder::new(
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap(),
            None,
        );
        let spender = TokenHolder::new(
            "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                .parse()
                .unwrap(),
            None,
        );
        let value = TokenAmount::from(100u32);
        allowances.credit(&owner, &spender, value.clone());
        assert_eq!(allowances.allowance(&owner, &spender), value);
        assert_eq!(allowances.allowance_size(), 1);
        assert_eq!(
            allowances.allowances_of(&owner),
            vec![(spender, value.clone())]
        );

        let res = allowances.debit(&owner, &spender, value.clone() + 1u32);
        assert_eq!(res, Err(DFTError::InsufficientAllowance));
        assert_eq!(allowances.allowance(&owner, &spender), value);
        assert_eq!(allowances.allowance_size(), 1);
        assert_eq!(allowances.to_vec().len(), 1);
        assert_eq!(
            allowances.allowances_of(&owner),
            vec![(spender, value.clone())]
        );

        allowances.debit(&owner, &spender, value.clone()).unwrap();
        assert_eq!(
            allowances.allowance(&owner, &spender),
            TokenAmount::from(0u32)
        );
        assert_eq!(allowances.allowance_size(), 0, "{:?}", allowances.to_vec());
        assert_eq!(allowances.allowances_of(&owner), vec![]);
        assert_eq!(allowances.to_vec().len(), 0);
    }
    #[test]
    fn test_decode_encode() {
        let mut allowances = TokenAllowances::new();
        let owner = TokenHolder::new(
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap(),
            None,
        );
        let spender = TokenHolder::new(
            "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                .parse()
                .unwrap(),
            None,
        );
        let value = TokenAmount::from(100u32);
        allowances.credit(&owner, &spender, value.clone());
        let encoded = allowances.encode();
        let decoded = TokenAllowances::decode(encoded).unwrap();
        assert_eq!(decoded.allowance(&owner, &spender), value);
    }

    #[test]
    fn test_restore_from() {
        let mut allowances = TokenAllowances::new();
        let owner = TokenHolder::new(
            "qupnt-ohzy3-npshw-oba2m-sttkq-tyawc-vufye-u5fbz-zb6yu-conr3-tqe"
                .parse()
                .unwrap(),
            None,
        );
        let spender = TokenHolder::new(
            "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                .parse()
                .unwrap(),
            None,
        );
        let value = TokenAmount::from(100u32);
        allowances.credit(&owner, &spender, value.clone());

        let mut allowances2 = TokenAllowances::new();
        allowances2.restore_from(allowances.to_vec());

        assert_eq!(allowances2.allowance(&owner, &spender), value);
        assert_eq!(allowances2.allowance_size(), 1);
    }
}
