use candid::{Nat, Principal};
use dft_types::*;
use dft_utils::{decode_tx_id, get_logo_type};
use getset::{Getters, Setters};
use std::{collections::HashMap, time::Duration};

pub trait TokenStandard {
    fn set_owner(
        &mut self,
        caller: &Principal,
        owner: Principal,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool>;
    fn set_fee(
        &mut self,
        caller: &Principal,
        fee: Fee,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool>;
    // set fee to
    fn set_fee_to(
        &mut self,
        caller: &Principal,
        fee_to: TokenHolder,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool>;
    fn set_desc(
        &mut self,
        caller: &Principal,
        descriptions: HashMap<String, String>,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool>;
    fn set_logo(
        &mut self,
        caller: &Principal,
        logo: Option<Vec<u8>>,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool>;
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
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex>;
    // transfer from
    fn transfer_from(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        spender: &TokenHolder,
        to: &TokenHolder,
        value: Nat,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex>;
    // transfer
    fn transfer(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        to: &TokenHolder,
        value: Nat,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex>;
    // token info
    fn token_info(&self) -> TokenInfo;
    fn token_metrics(&self) -> TokenMetrics;
    // transaction by index
    fn transaction_by_index(&self, index: &Nat) -> TxRecordCommonResult;
    // transaction by id
    fn transaction_by_id(&self, id: &String) -> TxRecordCommonResult;
    // last transactions
    fn last_transactions(&self, count: usize) -> CommonResult<Vec<TxRecord>>;
}
#[derive(Getters, Setters)]
#[getset(get = "pub")]
#[derive(Debug, Clone)]
pub struct TokenBasic {
    // token id
    token_id: Principal,
    // owner
    owner: Principal,
    // fee to
    fee_to: TokenHolder,
    // meta data
    metadata: Metadata,
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
    // token's desc info : social media, description etc
    desc: HashMap<String, String>,
}

impl Default for TokenBasic {
    fn default() -> Self {
        TokenBasic {
            token_id: Principal::anonymous(),
            metadata: Metadata::default(),
            owner: Principal::anonymous(),
            fee_to: TokenHolder::None,
            storage_canister_ids: HashMap::new(),
            next_tx_index: Nat::from(0),
            txs: Vec::new(),
            balances: HashMap::new(),
            allowances: HashMap::new(),
            logo: None,
            desc: HashMap::new(),
        }
    }
}

impl TokenBasic {
    // check if the caller is anonymous
    pub fn not_allow_anonymous(&self, caller: &Principal) -> CommonResult<()> {
        if caller == &Principal::anonymous() {
            return Err(DFTError::NotAllowAnonymous);
        }
        Ok(())
    }
    // check if the caller is the owner
    pub fn only_owner(&self, caller: &Principal) -> CommonResult<()> {
        self.not_allow_anonymous(caller)?;
        if &self.owner != caller {
            return Err(DFTError::OnlyOwnerAllowCallIt);
        }
        Ok(())
    }

    // verify created at
    pub fn verified_created_at(&self, created_at: &Option<u64>, now: &u64) -> CommonResult<()> {
        if created_at.is_none() {
            return Ok(());
        }
        let created_at_time = Duration::from_nanos(created_at.unwrap());
        let now = Duration::from_nanos(now.clone());
        if created_at_time + constants::DEFAULT_MAX_TRANSACTIONS_IN_WINDOW < now {
            return Err(DFTError::TxTooOld);
        }
        if created_at_time > now + constants::PERMITTED_DRIFT {
            return Err(DFTError::TxCreatedInFuture);
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
    ) -> CommonResult<()> {
        // get spenders allowance
        let spender_allowance = self._allowance(owner, spender);
        // check allowance
        if spender_allowance < value {
            return Err(DFTError::InsufficientAllowance);
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
    pub fn debit_balance(&mut self, holder: &TokenHolder, value: Nat) -> CommonResult<()> {
        if self._balance_of(holder) < value {
            Err(DFTError::InsufficientBalance)
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
    fn charge_approve_fee(&mut self, approver: &TokenHolder) -> CommonResult<Nat> {
        // check the approver's balance
        // if balance is not enough, return error
        if self.balances.get(approver).unwrap_or(&Nat::from(0)) < &self.metadata().fee().minimum {
            Err(DFTError::InsufficientBalance)
        } else {
            // charge the approver's balance as approve fee
            let fee = self.metadata().fee().minimum.clone();
            let fee_to = self.fee_to.clone();
            self.debit_balance(&approver, fee.clone())?;
            self.credit_balance(&fee_to, fee.clone());
            Ok(fee)
        }
    }

    // charge transfer fee
    fn charge_transfer_fee(
        &mut self,
        transfer_from: &TokenHolder,
        transfer_value: &Nat,
    ) -> CommonResult<Nat> {
        // calc the transfer fee: rate * value
        // compare the transfer fee and minimum fee,get the max value
        let rate_fee = self.metadata().fee().rate.clone() * transfer_value.clone()
            / 10u64.pow(self.metadata().fee().rate_decimals.into());
        let min_fee = self.metadata().fee().minimum.clone();
        let transfer_fee = if rate_fee > min_fee {
            rate_fee
        } else {
            min_fee
        };

        // check the transfer_from's balance
        // if balance is not enough, return error
        if self.balances.get(transfer_from).unwrap_or(&Nat::from(0)) < &transfer_fee {
            Err(DFTError::InsufficientBalance)
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
        // compare the transfer fee and minimum fee,get the max value
        let fee = self.metadata().fee().rate.clone() * transfer_value.clone()
            / 10u64.pow(self.metadata().fee().rate_decimals.into());
        let min_fee = self.metadata().fee().minimum.clone();
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
        tx_invoker: &TokenHolder,
        from: &TokenHolder,
        to: &TokenHolder,
        value: Nat,
        created_at: u64,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        // calc the transfer fee
        let transfer_fee = self.calc_transfer_fee(&value);
        //check the transfer_from's balance, if balance is not enough, return error
        if self._balance_of(from) < value.clone() + transfer_fee.clone() {
            Err(DFTError::InsufficientBalance)
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
                tx_invoker.clone(),
                from.clone(),
                to.clone(),
                value.clone(),
                transfer_fee,
                created_at,
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
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        self.verified_created_at(&created_at, &now)?;
        self.credit_balance(to, value.clone());
        let created_at = created_at.unwrap_or(now.clone());
        // increase the total supply
        self.metadata
            .set_total_supply(self.metadata().total_supply().clone() + value.clone());
        // add the mint tx to txs
        let tx_index = self.generate_new_tx_index();
        let tx = TxRecord::Transfer(
            tx_index.clone(),
            TokenHolder::new(caller.clone(), None),
            TokenHolder::None,
            to.clone(),
            value.clone(),
            Nat::from(0),
            created_at,
            now,
        );
        self.txs.push(tx);
        Ok(tx_index)
    }

    // _burn
    pub fn _burn(
        &mut self,
        caller: &Principal,
        burner: &TokenHolder,
        from: &TokenHolder,
        value: Nat,
        created_at: u64,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        // calc the transfer fee,if the burn amount small than minimum fee,return error
        let fee = self.calc_transfer_fee(&value);
        if value < self.metadata().fee().minimum.clone() {
            return Err(DFTError::BurnValueTooSmall);
        }
        //check the burn from holder's balance, if balance is not enough, return error
        if self._balance_of(from) < value.clone() {
            return Err(DFTError::InsufficientBalance);
        } else {
            // burn does not charge the transfer fee
            // debit the burn from holder's balance
            self.debit_balance(from, value.clone())?;
            // decrease the total supply
            self.metadata
                .set_total_supply(self.metadata().total_supply().clone() - value.clone());
            // add the burn tx to txs
            let tx_index = self.generate_new_tx_index();
            let tx = TxRecord::Transfer(
                tx_index.clone(),
                burner.clone(),
                from.clone(),
                TokenHolder::None,
                value.clone(),
                fee,
                created_at,
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
        // check logo type
        if logo.is_some() {
            let _ = get_logo_type(&logo.clone().unwrap())
                .map_err(|_| DFTError::InvalidTypeOrFormatOfLogo)
                .unwrap();
        }

        // set the parameters to token's properties
        self.owner = owner.clone();
        self.token_id = token_id.clone();
        self.metadata = Metadata::new(
            name.clone(),
            symbol.clone(),
            decimals.clone(),
            0.into(),
            fee.clone(),
        );
        self.logo = logo;
        self.fee_to = fee_to;
    }
    pub fn load_from_token_payload(&mut self, payload: TokenPayload) {
        self.token_id = payload.token_id;
        self.owner = payload.owner;
        self.logo = if payload.logo.len() > 0 {
            Some(payload.logo)
        } else {
            None
        };
        self.metadata = payload.meta;
        self.fee_to = payload.fee_to;

        for (k, v) in payload.desc {
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
        let mut desc = Vec::new();
        let mut balances = Vec::new();
        let mut allowances = Vec::new();
        let mut storage_canister_ids = Vec::new();
        let mut txs = Vec::new();
        for (k, v) in self.desc.iter() {
            desc.push((k.to_string(), v.to_string()));
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
            owner: self.owner.clone(),
            fee_to: self.fee_to.clone(),
            meta: self.metadata.clone(),
            desc,
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
    fn set_owner(
        &mut self,
        caller: &Principal,
        owner: Principal,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool> {
        self.only_owner(caller)?;
        self.verified_created_at(&created_at, &now)?;
        self.owner = owner;
        // create OwnerModifyTx
        let tx_index = self.generate_new_tx_index();
        let created_at = created_at.unwrap_or(now.clone());
        let tx = TxRecord::OwnerModify(
            tx_index.clone(),
            caller.clone(),
            owner.clone(),
            created_at,
            now,
        );
        self.txs.push(tx);
        Ok(true)
    }

    fn set_fee(
        &mut self,
        caller: &Principal,
        fee: Fee,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool> {
        self.only_owner(caller)?;
        self.verified_created_at(&created_at, &now)?;
        self.metadata.set_fee(fee.clone());
        // create FeeModifyTx
        let tx_index = self.generate_new_tx_index();
        let created_at = created_at.unwrap_or(now.clone());
        let tx = TxRecord::FeeModify(tx_index.clone(), caller.clone(), fee, created_at, now);
        self.txs.push(tx);
        Ok(true)
    }

    fn set_fee_to(
        &mut self,
        caller: &Principal,
        fee_to: TokenHolder,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool> {
        self.only_owner(caller)?;
        self.verified_created_at(&created_at, &now)?;
        self.fee_to = fee_to.clone();
        // create FeeToModifyTx
        let tx_index = self.generate_new_tx_index();
        let created_at = created_at.unwrap_or(now.clone());
        let tx = TxRecord::FeeToModify(
            tx_index.clone(),
            caller.clone(),
            fee_to.clone(),
            created_at,
            now,
        );
        self.txs.push(tx);
        Ok(true)
    }

    fn set_desc(
        &mut self,
        caller: &Principal,
        descriptions: HashMap<String, String>,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool> {
        self.only_owner(caller)?;
        self.verified_created_at(&created_at, &now)?;
        for (key, value) in descriptions.iter() {
            if DESC_KEYS.contains(&key.as_str()) {
                self.desc.insert(key.clone(), value.clone());
            }
        }

        let tx_index = self.generate_new_tx_index();
        let created_at = created_at.unwrap_or(now.clone());
        let modify_desc_tx = TxRecord::DescModify(
            tx_index.clone(),
            self.owner.clone(),
            descriptions
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            created_at,
            now,
        );
        self.txs.push(modify_desc_tx);
        Ok(true)
    }

    fn set_logo(
        &mut self,
        caller: &Principal,
        logo: Option<Vec<u8>>,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<bool> {
        self.only_owner(caller)?;
        self.verified_created_at(&created_at, &now)?;
        if logo.is_some() {
            get_logo_type(&logo.clone().unwrap())
                .map_err(|_| DFTError::InvalidTypeOrFormatOfLogo)?;
        }

        self.logo = logo.clone();
        let tx_index = self.generate_new_tx_index();
        let created_at = created_at.unwrap_or(now.clone());
        let modify_logo_tx = TxRecord::LogoModify(
            tx_index.clone(),
            self.owner.clone(),
            if logo.is_some() {
                logo.unwrap()
            } else {
                vec![]
            },
            created_at,
            now,
        );
        self.txs.push(modify_logo_tx);
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
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        self.not_allow_anonymous(caller)?;
        self.verified_created_at(&created_at, &now)?;
        let approve_fee = self.charge_approve_fee(owner)?;
        //credit the spender's allowance
        self.credit_allowance(owner, spender, value.clone());
        let tx_index = self.generate_new_tx_index();
        let created_at = created_at.unwrap_or(now.clone());
        let approve_tx = TxRecord::Approve(
            tx_index.clone(),
            owner.clone(),
            owner.clone(),
            spender.clone(),
            value.clone(),
            approve_fee,
            created_at,
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
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        self.not_allow_anonymous(caller)?;
        self.verified_created_at(&created_at, &now)?;
        let created_at = created_at.unwrap_or(now.clone());
        let transfer_fee = self.calc_transfer_fee(&value);
        // get spenders allowance
        let spender_allowance = self._allowance(from, spender);
        let decreased_allowance = value.clone() + transfer_fee.clone();
        // check allowance
        if spender_allowance < decreased_allowance.clone() {
            return Err(DFTError::InsufficientAllowance);
        }
        // debit the spender's allowance
        self.debit_allowance(from, spender, decreased_allowance.clone())?;

        return self._transfer(caller, spender, from, to, value, created_at, now);
    }

    fn transfer(
        &mut self,
        caller: &Principal,
        from: &TokenHolder,
        to: &TokenHolder,
        value: Nat,
        created_at: Option<u64>,
        now: u64,
    ) -> CommonResult<TransactionIndex> {
        self.not_allow_anonymous(caller)?;
        self.verified_created_at(&created_at, &now)?;
        let created_at = created_at.unwrap_or(now.clone());
        self._transfer(caller, &from, from, to, value, created_at, now)
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

    fn token_metrics(&self) -> TokenMetrics {
        let allowances_size = match self.allowances.len() {
            0 => 0,
            _ => self.allowances.values().map(|v| v.len()).sum(),
        };
        TokenMetrics {
            holders: Nat::from(self.balances.len()),
            total_tx_count: self.next_tx_index.clone(),
            inner_tx_count: Nat::from(self.txs.len()),
            allowance_size: Nat::from(allowances_size),
        }
    }

    fn transaction_by_index(&self, index: &Nat) -> TxRecordCommonResult {
        let inner_start_tx_index = &self.txs[0].get_tx_index();
        let inner_end_tx_index = self.next_tx_index.clone() - 1;

        // if index > inner_end_tx_index, return error
        if index > &inner_end_tx_index {
            return TxRecordCommonResult::Err(DFTError::InvalidTxIndex);
        }

        // if the tx record exist in self.txs which has the same index,return it
        // else find the key in self.storage_canister_ids which has the biggest value
        // that less than index, get the value of the key ,return it
        if index < inner_start_tx_index {
            let mut index_map = self.storage_canister_ids.clone();
            index_map.retain(|k, _| k <= index);
            let key = index_map.keys().last().unwrap();
            let value = index_map.get(key).unwrap();
            return TxRecordCommonResult::Forward(*value);
        }
        if let Some(tx_record) = self.txs.iter().find(|tx| &tx.get_tx_index() == index) {
            return TxRecordCommonResult::Ok(tx_record.clone());
        }
        return TxRecordCommonResult::Err(DFTError::InvalidTxIndex);
    }

    fn transaction_by_id(&self, id: &String) -> TxRecordCommonResult {
        match decode_tx_id(id.clone()) {
            Ok((token_id, tx_index)) => {
                if token_id != self.token_id {
                    return TxRecordCommonResult::Err(DFTError::TxIdNotBelongToCurrentDft);
                } else {
                    return self.transaction_by_index(&tx_index);
                }
            }
            Err(_) => TxRecordCommonResult::Err(DFTError::InvalidTxId),
        }
    }

    fn last_transactions(&self, count: usize) -> CommonResult<Vec<TxRecord>> {
        // max return count is 100
        let count = if count > 100 { 100 } else { count };

        if self.txs.len() < count {
            let mut txs = self.txs.clone();
            txs.reverse();
            return Ok(txs);
        } else {
            let start = self.txs.len() - count;
            let mut txs = self.txs[start..].to_vec();
            txs.reverse();
            Ok(txs)
        }
    }
}
