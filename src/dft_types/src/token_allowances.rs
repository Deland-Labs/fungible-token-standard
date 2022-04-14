use crate::{CommonResult, DFTError, TokenHolder};
use candid::{CandidType, Deserialize, Nat};
use std::collections::HashMap;

#[derive(CandidType, Clone, Default, Debug, Deserialize)]
pub struct TokenAllowances(HashMap<TokenHolder, HashMap<TokenHolder, Nat>>);

impl TokenAllowances {
    pub fn new() -> Self {
        TokenAllowances(HashMap::new())
    }

    pub fn allowance_size(&self) -> Nat {
        Nat::from(match self.0.len() {
            0 => 0,
            _ => self.0.values().map(|v| v.len()).sum(),
        })
    }

    pub fn allowance(&self, owner: &TokenHolder, spender: &TokenHolder) -> Nat {
        self.0
            .get(owner)
            .unwrap_or(&HashMap::new())
            .get(spender)
            .unwrap_or(&Nat::from(0))
            .clone()
    }

    pub fn allowances_of(&self, owner: &TokenHolder) -> Vec<(TokenHolder, Nat)> {
        let mut vec = Vec::new();
        for (spender, allowance) in self.0.get(owner).unwrap_or(&HashMap::new()).iter() {
            vec.push((spender.clone(), allowance.clone()));
        }
        vec
    }

    //debit token holder's allowance
    pub fn debit(
        &mut self,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: Nat,
    ) -> CommonResult<()> {
        // get spenders allowance
        let spender_allowance = self.allowance(owner, spender);
        // check allowance
        if spender_allowance < value {
            return Err(DFTError::InsufficientAllowance);
        }
        let new_spender_allowance = spender_allowance - value.clone();
        match self.0.get(&owner) {
            Some(inner) => {
                let mut temp = inner.clone();
                if value == 0 {
                    temp.remove(&spender);
                    if temp.len() > 0 {
                        self.0.insert(owner.clone(), temp);
                    } else {
                        self.0.remove(&owner);
                    }
                } else {
                    temp.insert(spender.clone(), new_spender_allowance);
                    self.0.insert(owner.clone(), temp);
                }
            }
            None => {
                if value.gt(&Nat::from(0)) {
                    let mut inner = HashMap::new();
                    inner.insert(spender.clone(), new_spender_allowance);
                    self.0.insert(owner.clone(), inner);
                }
            }
        };
        Ok(())
    }

    // credit token spender's allowance
    pub fn credit(&mut self, owner: &TokenHolder, spender: &TokenHolder, value: Nat) {
        match self.0.get(&owner) {
            Some(inner) => {
                let mut temp = inner.clone();
                if value == 0 {
                    temp.remove(&spender);
                    if temp.len() > 0 {
                        self.0.insert(owner.clone(), temp);
                    } else {
                        self.0.remove(&owner);
                    }
                } else {
                    temp.insert(spender.clone(), value.clone());
                    self.0.insert(owner.clone(), temp);
                }
            }
            None => {
                if value.gt(&Nat::from(0)) {
                    let mut inner = HashMap::new();
                    inner.insert(spender.clone(), value.clone());
                    self.0.insert(owner.clone(), inner);
                }
            }
        };
    }

    // to vec
    pub fn to_vec(&self) -> Vec<(TokenHolder, Vec<(TokenHolder, Nat)>)> {
        let mut allowances = Vec::new();
        for (th, v) in self.0.iter() {
            let mut allow_item = Vec::new();
            for (sp, val) in v.iter() {
                allow_item.push((sp.clone(), val.clone()));
            }
            allowances.push((th.clone(), allow_item));
        }
        allowances
    }

    // restore from
    pub fn restore_from(&mut self, allowances: Vec<(TokenHolder, Vec<(TokenHolder, Nat)>)>) {
        for (th, v) in allowances.iter() {
            let mut allow_item = HashMap::new();
            for (sp, val) in v.iter() {
                allow_item.insert(sp.clone(), val.clone());
            }
            self.0.insert(th.clone(), allow_item);
        }
    }
}
