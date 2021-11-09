use candid::{Nat, Principal};
use dft_types::{message::*, *};
use dft_utils::decode_tx_id;
use std::collections::HashMap;
const MAX_GET_TXS_SIZE: usize = 200;
const FEE_RATE_DIV: u64 = 100_000_000;
pub trait TokenStandard {
    // token id
    fn id(&self) -> Principal;
    // get/set owner
    fn owner(&self) -> Principal;
    fn set_owner(&mut self, caller: &Principal, owner: Principal) -> Result<bool, String>;
    // name
    fn name(&self) -> String;
    // symbol
    fn symbol(&self) -> String;
    // decimals
    fn decimals(&self) -> u8;
    // total supply
    fn total_supply(&self) -> Nat;
    // get/set fee
    fn fee(&self) -> Fee;
    fn set_fee(&mut self, caller: &Principal, fee: Fee) -> Result<bool, String>;
    // set fee to
    fn set_fee_to(&mut self, caller: &Principal, fee_to: TokenHolder) -> Result<bool, String>;
    // get metadata
    fn metadata(&self) -> Metadata;
    // get/set desc info
    fn desc(&self) -> HashMap<String, String>;
    fn set_desc(
        &mut self,
        caller: &Principal,
        descriptions: HashMap<String, String>,
    ) -> Result<bool, String>;
    // get/set logo
    fn logo(&self) -> Vec<u8>;
    fn set_logo(&mut self, caller: &Principal, logo: Vec<u8>) -> Result<bool, String>;
    // balance of
    fn balance_of(&self, owner: &TokenHolder) -> Nat;
    // allowance
    fn allowance(&self, owner: &TokenHolder, spender: &TokenHolder) -> Nat;
    // allowances of
    fn allowances_of(&self, owner: &TokenHolder) -> Vec<(TokenHolder, Nat)>;
    // approve
    fn approve(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String>;
    // transfer from
    fn transfer_from(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        spender: &TokenHolder,
        to: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String>;
    // transfer
    fn transfer(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        to: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String>;
    // token info
    fn token_info(&self) -> TokenInfo;
    // transaction by index
    fn transaction_by_index(&self, index: &Nat) -> TxRecordResult;
    // transaction by id
    fn transaction_by_id(&self, id: &String) -> TxRecordResult;
    // last transactions
    fn last_transactions(&self, count: usize) -> Result<Vec<TxRecord>, String>;
}

pub trait MintableExtension {
    //mint
    fn mint(
        &mut self,
        caller: &Principal,
        to: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String>;
}

#[derive(Debug)]
pub struct TokenBasic {
    // token id
    token_id: Principal,
    // owner
    owner: Principal,
    // fee to
    fee_to: TokenHolder,
    // storage canister ids
    storage_canister_ids: HashMap<Nat, Principal>,
    // next tx index
    next_tx_index: Nat,
    // tx store inside
    txs: Vec<TxRecord>,
    // balances
    balances: HashMap<TokenHolder, Nat>,
    // allowances
    allowances: HashMap<TokenHolder, HashMap<TokenHolder, Nat>>,
    // token's logo
    logo: Option<Vec<u8>>,
    // token's name
    name: String,
    // token's symbol
    symbol: String,
    // token's decimals
    decimals: u8,
    // token's total supply
    total_supply: Nat,
    // token's fee
    fee: Fee,
    // token's extend info : social media, description etc
    desc: HashMap<String, String>,
}

impl Default for TokenBasic {
    fn default() -> Self {
        TokenBasic {
            token_id: Principal::anonymous(),
            owner: Principal::anonymous(),
            fee_to: TokenHolder::None,
            storage_canister_ids: HashMap::new(),
            next_tx_index: Nat::from(0),
            txs: Vec::new(),
            balances: HashMap::new(),
            allowances: HashMap::new(),
            logo: None,
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0,
            total_supply: Nat::from(0),
            fee: Fee {
                minimum: Nat::from(0),
                rate: Nat::from(0),
            },
            desc: HashMap::new(),
        }
    }
}

impl TokenBasic {
    // check if the caller is anonymous
    pub fn not_allow_anonymous(&self, caller: &Principal) -> Result<(), String> {
        if caller == &Principal::anonymous() {
            return Err(MSG_NOT_ALLOW_ANONYMOUS.to_owned());
        }
        Ok(())
    }
    // check if the caller is the owner
    pub fn only_owner(&self, caller: &Principal) -> Result<(), String> {
        if &self.owner != caller {
            return Err(MSG_ONLY_OWNER.to_owned());
        }
        Ok(())
    }
    //generate new tx index
    fn generate_new_tx_index(&mut self) -> Nat {
        let rtn = self.next_tx_index.clone();
        self.next_tx_index = rtn.clone() + 1;
        rtn
    }
    //debit token holder's allowance
    pub fn debit_allowance(
        &mut self,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: Nat,
    ) -> Result<(), String> {
        // get spenders allowance
        let spender_allowance = self._allowance(owner, spender);
        // check allowance
        if spender_allowance < value {
            return Err(MSG_INSUFFICIENT_BALANCE.to_string());
        }
        let new_spender_allowance = spender_allowance - value.clone();
        match self.allowances.get(&owner) {
            Some(inner) => {
                let mut temp = inner.clone();
                if value == 0 {
                    temp.remove(&spender);
                    if temp.len() > 0 {
                        self.allowances.insert(owner.clone(), temp);
                    } else {
                        self.allowances.remove(&owner);
                    }
                } else {
                    temp.insert(spender.clone(), new_spender_allowance);
                    self.allowances.insert(owner.clone(), temp);
                }
            }
            None => {
                if value.gt(&Nat::from(0)) {
                    let mut inner = HashMap::new();
                    inner.insert(spender.clone(), new_spender_allowance);
                    self.allowances.insert(owner.clone(), inner);
                }
            }
        };
        Ok(())
    }

    //credit token spender's allowance
    pub fn credit_allowance(&mut self, owner: &TokenHolder, spender: &TokenHolder, value: Nat) {
        match self.allowances.get(&owner) {
            Some(inner) => {
                let mut temp = inner.clone();
                if value == 0 {
                    temp.remove(&spender);
                    if temp.len() > 0 {
                        self.allowances.insert(owner.clone(), temp);
                    } else {
                        self.allowances.remove(&owner);
                    }
                } else {
                    temp.insert(spender.clone(), value.clone());
                    self.allowances.insert(owner.clone(), temp);
                }
            }
            None => {
                if value.gt(&Nat::from(0)) {
                    let mut inner = HashMap::new();
                    inner.insert(spender.clone(), value.clone());
                    self.allowances.insert(owner.clone(), inner);
                }
            }
        };
    }

    // debit token holder's balance
    pub fn debit_balance(&mut self, holder: &TokenHolder, value: Nat) -> Result<(), String> {
        if self._balance_of(holder) < value {
            Err(MSG_INSUFFICIENT_BALANCE.to_string())
        } else {
            // calc new balance
            let new_balance = self._balance_of(holder) - value;

            if new_balance > Nat::from(0) {
                self.balances.insert(holder.clone(), new_balance);
            } else {
                self.balances.remove(holder);
            }

            Ok(())
        }
    }
    // credit token holder's balance
    pub fn credit_balance(&mut self, holder: &TokenHolder, value: Nat) {
        let new_balance = self._balance_of(holder) + value;
        self.balances.insert(holder.clone(), new_balance);
    }
    //charge approve fee
    fn charge_approve_fee(&mut self, approver: &TokenHolder) -> Result<Nat, String> {
        // check the approver's balance
        // if balance is not enough, return error
        if self.balances.get(approver).unwrap_or(&Nat::from(0)) < &self.fee.minimum {
            Err(MSG_INSUFFICIENT_BALANCE.to_string())
        } else {
            // charge the approver's balance as approve fee
            let fee = self.fee.minimum.clone();
            let fee_to = self.fee_to.clone();
            self.debit_balance(&approver, fee.clone())?;
            self.credit_balance(&fee_to, fee.clone());
            Ok(fee)
        }
    }

    // chare transfer fee
    fn charge_transfer_fee(
        &mut self,
        transfer_from: &TokenHolder,
        transfer_value: &Nat,
    ) -> Result<Nat, String> {
        // calc the transfer fee: rate * value
        // compare the transfer fee and minumum fee,get the max value
        let rate_fee = self.fee.rate.clone() * transfer_value.clone() / FEE_RATE_DIV;
        let min_fee = self.fee.minimum.clone();
        let transfer_fee = if rate_fee > min_fee {
            rate_fee
        } else {
            min_fee
        };

        // check the transfer_from's balance
        // if balance is not enough, return error
        if self.balances.get(transfer_from).unwrap_or(&Nat::from(0)) < &transfer_fee {
            Err(MSG_INSUFFICIENT_BALANCE.to_string())
        } else {
            let fee_to = self.fee_to.clone();
            self.debit_balance(&transfer_from, transfer_fee.clone())?;
            self.credit_balance(&fee_to, transfer_fee.clone());
            Ok(transfer_fee)
        }
    }
    // calc transfer fee
    fn calc_transfer_fee(&self, transfer_value: &Nat) -> Nat {
        // calc the transfer fee: rate * value
        // compare the transfer fee and minumum fee,get the max value
        let fee = self.fee.rate.clone() * transfer_value.clone() / FEE_RATE_DIV;
        let min_fee = self.fee.minimum.clone();
        let max_fee = if fee > min_fee { fee } else { min_fee };
        max_fee
    }

    pub fn get_inner_txs(&self) -> Vec<TxRecord> {
        self.txs.clone()
    }

    pub fn get_storage_canister_ids(&self) -> HashMap<Nat, Principal> {
        self.storage_canister_ids.clone()
    }

    pub fn add_storage_canister_ids(&mut self, tx_index_start: Nat, canister_id: Principal) {
        self.storage_canister_ids
            .insert(tx_index_start, canister_id);
    }

    pub fn remove_inner_txs(&mut self, index: usize) {
        self.txs.remove(index);
    }
    pub fn get_tx_index(&self, tx: &TxRecord) -> Nat {
        match tx {
            TxRecord::Approve(ti, _, _, _, _, _, _) => ti.clone(),
            TxRecord::Transfer(ti, _, _, _, _, _, _) => ti.clone(),
        }
    }

    fn _balance_of(&self, owner: &TokenHolder) -> Nat {
        self.balances.get(owner).unwrap_or(&Nat::from(0)).clone()
    }
    fn _allowance(&self, owner: &TokenHolder, spender: &TokenHolder) -> Nat {
        self.allowances
            .get(owner)
            .unwrap_or(&HashMap::new())
            .get(spender)
            .unwrap_or(&Nat::from(0))
            .clone()
    }
    //transfer token
    fn _transfer(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        to: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String> {
        // calc the transfer fee
        let transfer_fee = self.calc_transfer_fee(&value);
        //check the transfer_from's balance, if balance is not enough, return error
        if self._balance_of(from) < value.clone() + transfer_fee.clone() {
            Err(MSG_INSUFFICIENT_BALANCE.to_string())
        } else {
            // charge the transfer fee
            self.charge_transfer_fee(from, &value)?;
            // debit the transfer_from's balance
            self.debit_balance(from, value.clone())?;
            // credit the transfer_to's balance
            self.credit_balance(to, value.clone());
            // add the transfer tx to txs
            let tx_index = self.generate_new_tx_index();
            let tx = TxRecord::Transfer(
                tx_index.clone(),
                caller.clone(),
                from.clone(),
                to.clone(),
                value.clone(),
                transfer_fee,
                now,
            );
            self.txs.push(tx);
            Ok(tx_index)
        }
    }
    // _mint
    pub fn _mint(
        &mut self,
        caller: &Principal,
        to: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String> {
        self.credit_balance(to, value.clone());
        // increase the total supply
        self.total_supply = self.total_supply.clone() + value.clone();
        // add the mint tx to txs
        let tx_index = self.generate_new_tx_index();
        let tx = TxRecord::Transfer(
            tx_index.clone(),
            caller.clone(),
            TokenHolder::None,
            to.clone(),
            value.clone(),
            Nat::from(0),
            now,
        );
        self.txs.push(tx);
        Ok(tx_index)
    }

    // _burn
    pub fn _burn(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String> {
        // calc the transfer fee,if the fee smaller than minimum fee,return error
        let fee = self.calc_transfer_fee(&value);
        if fee < self.fee.minimum.clone() {
            return Err(MSG_BURN_VALUE_TOO_SMALL.to_string());
        }
        //check the burn from's balance, if balance is not enough, return error
        if self._balance_of(from) < value.clone() {
            return Err(MSG_INSUFFICIENT_BALANCE.to_string());
        } else {
            // burn does not charge the transfer fee
            // debit the burn from's balance
            self.debit_balance(from, value.clone())?;
            // decrease the total supply
            self.total_supply = self.total_supply.clone() - value.clone();
            // add the burn tx to txs
            let tx_index = self.generate_new_tx_index();
            let tx = TxRecord::Transfer(
                tx_index.clone(),
                caller.clone(),
                from.clone(),
                TokenHolder::None,
                value.clone(),
                fee,
                now,
            );
            self.txs.push(tx);
            Ok(tx_index)
        }
    }
}

//from/to TokenPayload
impl TokenBasic {
    // initialize
    pub fn initialize(
        &mut self,
        owner: &Principal,
        token_id: Principal,
        logo: Option<Vec<u8>>,
        name: String,
        symbol: String,
        decimals: u8,
        fee: Fee,
        fee_to: TokenHolder,
    ) {
        // set the parameters to token's prepoty
        self.owner = owner.clone();
        self.token_id = token_id.clone();
        self.logo = logo;
        self.name = name.clone();
        self.symbol = symbol.clone();
        self.decimals = decimals;
        self.fee = fee;
        self.fee_to = fee_to;
    }
    pub fn from_token_payload(&mut self, payload: TokenPayload) {
        self.token_id = payload.token_id;
        self.owner = payload.owner;
        self.logo = Some(payload.logo);
        self.name = payload.meta.name;
        self.symbol = payload.meta.symbol;
        self.decimals = payload.meta.decimals;
        self.fee = payload.meta.fee;
        self.fee_to = payload.fee_to;

        for (k, v) in payload.extend {
            self.desc.insert(k, v);
        }
        for (k, v) in payload.balances {
            self.balances.insert(k, v);
        }
        for (k, v) in payload.allowances {
            let mut inner = HashMap::new();
            for (ik, iv) in v {
                inner.insert(ik, iv);
            }
            self.allowances.insert(k, inner);
        }
        for (k, v) in payload.storage_canister_ids {
            self.storage_canister_ids.insert(k, v);
        }

        for v in payload.txs_inner {
            self.txs.push(v);
        }
    }
    pub fn to_token_payload(&self) -> TokenPayload {
        let mut extend = Vec::new();
        let mut balances = Vec::new();
        let mut allowances = Vec::new();
        let mut storage_canister_ids = Vec::new();
        let mut txs = Vec::new();
        for (k, v) in self.desc.iter() {
            extend.push((k.to_string(), v.to_string()));
        }
        for (k, v) in self.balances.iter() {
            balances.push((k.clone(), v.clone()));
        }
        for (th, v) in self.allowances.iter() {
            let mut allow_item = Vec::new();
            for (sp, val) in v.iter() {
                allow_item.push((sp.clone(), val.clone()));
            }
            allowances.push((th.clone(), allow_item));
        }
        for (k, v) in self.storage_canister_ids.iter() {
            storage_canister_ids.push((k.clone(), *v));
        }
        for v in self.txs.iter() {
            txs.push(v.clone());
        }
        TokenPayload {
            token_id: self.token_id.clone(),
            owner: self.owner,
            fee_to: self.fee_to.clone(),
            meta: self.metadata(),
            extend,
            logo: self.logo.clone().unwrap_or_else(|| vec![]),
            balances,
            allowances,
            tx_index_cursor: self.next_tx_index.clone(),
            storage_canister_ids,
            txs_inner: txs,
        }
    }
}

impl TokenStandard for TokenBasic {
    fn id(&self) -> Principal {
        self.token_id.clone()
    }

    fn owner(&self) -> Principal {
        self.owner.clone()
    }

    fn set_owner(&mut self, caller: &Principal, owner: Principal) -> Result<bool, String> {
        self.only_owner(caller)?;
        self.owner = owner;
        Ok(true)
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn symbol(&self) -> String {
        self.symbol.clone()
    }

    fn decimals(&self) -> u8 {
        self.decimals.clone()
    }

    fn total_supply(&self) -> Nat {
        self.total_supply.clone()
    }

    fn fee(&self) -> Fee {
        self.fee.clone()
    }

    fn set_fee(&mut self, caller: &Principal, fee: Fee) -> Result<bool, String> {
        self.only_owner(caller)?;
        self.fee = fee;
        Ok(true)
    }

    fn set_fee_to(&mut self, caller: &Principal, fee_to: TokenHolder) -> Result<bool, String> {
        self.only_owner(caller)?;
        self.fee_to = fee_to;
        Ok(true)
    }

    fn metadata(&self) -> Metadata {
        Metadata {
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            decimals: self.decimals,
            total_supply: self.total_supply.clone(),
            fee: self.fee.clone(),
        }
    }

    fn desc(&self) -> HashMap<String, String> {
        self.desc.clone()
    }

    fn set_desc(
        &mut self,
        caller: &Principal,
        descriptions: HashMap<String, String>,
    ) -> Result<bool, String> {
        self.only_owner(caller)?;
        for (key, value) in descriptions.iter() {
            if EXTEND_KEYS.contains(&key.as_str()) {
                self.desc.insert(key.clone(), value.clone());
            }
        }
        Ok(true)
    }

    fn logo(&self) -> Vec<u8> {
        self.logo.clone().unwrap_or_else(|| vec![])
    }

    fn set_logo(&mut self, caller: &Principal, logo: Vec<u8>) -> Result<bool, String> {
        self.only_owner(caller)?;
        self.logo = Some(logo);
        Ok(true)
    }

    fn balance_of(&self, holder: &TokenHolder) -> Nat {
        self._balance_of(holder)
    }

    fn allowance(&self, holder: &TokenHolder, spender: &TokenHolder) -> Nat {
        self._allowance(holder, spender)
    }

    fn allowances_of(&self, owner: &TokenHolder) -> Vec<(TokenHolder, Nat)> {
        match self.allowances.get(owner) {
            Some(allowances) => allowances
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            None => Vec::new(),
        }
    }

    fn approve(
        &mut self,
        caller: &Principal,
        owner: &TokenHolder,
        spender: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String> {
        self.not_allow_anonymous(caller)?;
        let approve_fee = self.charge_approve_fee(owner)?;
        //credit the spender's allowance
        self.credit_allowance(owner, spender, value.clone());
        let tx_index = self.generate_new_tx_index();

        let approve_tx = TxRecord::Approve(
            tx_index.clone(),
            caller.clone(),
            owner.clone(),
            spender.clone(),
            value.clone(),
            approve_fee,
            now,
        );
        self.txs.push(approve_tx);
        return Ok(tx_index);
    }

    fn transfer_from(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        spender: &TokenHolder,
        to: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String> {
        self.not_allow_anonymous(caller)?;
        let transfer_fee = self.calc_transfer_fee(&value);
        // get spenders allowance
        let spender_allowance = self._allowance(from, spender);
        let decreased_allowance = value.clone() + transfer_fee.clone();
        // check allowance
        if spender_allowance < decreased_allowance.clone() {
            return Err(MSG_INSUFFICIENT_ALLOWANCE.to_string());
        }
        // debit the spender's allowance
        self.debit_allowance(from, spender, decreased_allowance.clone())?;

        return self._transfer(caller, from, to, value, now);
    }

    fn transfer(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        to: &TokenHolder,
        value: Nat,
        now: u64,
    ) -> Result<TransactionIndex, String> {
        self.not_allow_anonymous(caller)?;
        self._transfer(caller, from, to, value, now)
    }

    fn token_info(&self) -> TokenInfo {
        //get the allowances size
        let allowances_size = match self.allowances.len() {
            0 => 0,
            _ => self.allowances.values().map(|v| v.len()).sum(),
        };

        TokenInfo {
            owner: self.owner.clone(),
            holders: Nat::from(self.balances.len()),
            allowance_size: Nat::from(allowances_size),
            fee_to: self.fee_to.clone(),
            tx_count: self.next_tx_index.clone(),
            cycles: 0,
            storages: self
                .storage_canister_ids
                .values()
                .map(|v| v.clone())
                .collect(),
        }
    }

    fn transaction_by_index(&self, index: &Nat) -> TxRecordResult {
        let inner_start_tx_index = self.get_tx_index(&self.txs[0]);
        let inner_end_tx_index = self.next_tx_index.clone() - 1;

        // if index > inner_end_tx_index, return error
        if index > &inner_end_tx_index {
            return TxRecordResult::Err(MSG_INVALID_TX_INDEX.to_string());
        }

        // if the tx record exist in self.txs which has the same index,return it
        // else find the key in self.storage_canister_ids which has the biggest value
        // that less than index, get the value of the key ,return it
        if index < &inner_start_tx_index {
            let mut index_map = self.storage_canister_ids.clone();
            index_map.retain(|k, _| k <= index);
            let key = index_map.keys().last().unwrap();
            let value = index_map.get(key).unwrap();
            return TxRecordResult::Forward(*value);
        }
        if let Some(tx_record) = self.txs.iter().find(|tx| &self.get_tx_index(tx) == index) {
            return TxRecordResult::Ok(tx_record.clone());
        }
        return TxRecordResult::Err(MSG_INVALID_TX_INDEX.to_string());
    }

    fn transaction_by_id(&self, id: &String) -> TxRecordResult {
        match decode_tx_id(id.clone()) {
            Ok((token_id, tx_index)) => {
                if token_id != self.token_id {
                    return TxRecordResult::Err(MSG_NOT_BELONG_DFT_TX_ID.to_string());
                } else {
                    self.transaction_by_index(&tx_index)
                }
            }
            Err(_) => TxRecordResult::Err(MSG_INVALID_TX_ID.to_string()),
        }
    }

    fn last_transactions(&self, count: usize) -> Result<Vec<TxRecord>, String> {
        if count > MAX_GET_TXS_SIZE {
            return Err(MSG_GET_LAST_TXS_SIZE_TOO_LARGE.to_string());
        }
        if self.txs.len() < count {
            return Ok(self.txs.clone());
        } else {
            let start = self.txs.len() - count;
            return Ok(self.txs[start..].to_vec());
        }
    }
}
