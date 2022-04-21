use crate::*;
use candid::{Deserialize, Principal};
use serde::Serialize;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct TokenSetting {
    inner: TokenSettingInner,
}

impl TokenSetting {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        token_id: Principal,
        logo: Option<Vec<u8>>,
        name: String,
        symbol: String,
        decimals: u8,
        owner: Principal,
        fee: TokenFee,
        fee_to: TokenHolder,
    ) -> Self {
        TokenSetting {
            inner: TokenSettingInner {
                token_id,
                logo,
                name,
                symbol,
                decimals,
                owner,
                minters: Vec::new(),
                fee,
                fee_to,
            },
        }
    }
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
        if &self.inner.owner != caller {
            return Err(DFTError::OnlyOwnerAllowCallIt);
        }
        Ok(())
    }

    // check if the caller is the minter
    pub fn only_minter(&self, caller: &Principal) -> CommonResult<()> {
        self.not_allow_anonymous(caller)?;
        if &self.inner.owner == caller {
            return Ok(());
        }
        if !self.inner.minters.contains(caller) {
            return Err(DFTError::OnlyMinterAllowCallIt);
        }
        Ok(())
    }
    pub fn token_id(&self) -> &Principal {
        &self.inner.token_id
    }

    pub fn logo(&self) -> Option<Vec<u8>> {
        self.inner.logo.clone()
    }

    pub fn set_logo(&mut self, logo: Option<Vec<u8>>) {
        self.inner.logo = logo;
    }

    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    pub fn symbol(&self) -> String {
        self.inner.symbol.clone()
    }

    pub fn decimals(&self) -> u8 {
        self.inner.decimals
    }

    pub fn owner(&self) -> Principal {
        self.inner.owner
    }

    pub fn set_owner(&mut self, owner: Principal) {
        self.inner.owner = owner;
    }

    pub fn minters(&self) -> Vec<Principal> {
        self.inner.minters.to_vec()
    }

    pub fn add_minter(&mut self, minter: Principal) {
        if self.inner.minters.contains(&minter) {
            return;
        }
        self.inner.minters.push(minter);
    }

    pub fn remove_minter(&mut self, minter: Principal) {
        if !self.inner.minters.contains(&minter) {
            return;
        }
        self.inner.minters.retain(|x| x != &minter);
    }

    pub fn fee(&self) -> TokenFee {
        self.inner.fee.clone()
    }

    pub fn set_fee(&mut self, fee: TokenFee) {
        self.inner.fee = fee;
    }

    pub fn fee_to(&self) -> TokenHolder {
        self.inner.fee_to
    }
    pub fn set_fee_to(&mut self, fee_to: TokenHolder) {
        self.inner.fee_to = fee_to;
    }
    pub fn metadata(&self) -> TokenMetadata {
        TokenMetadata::new(self.name(), self.symbol(), self.decimals(), self.fee())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct TokenSettingInner {
    token_id: Principal,
    logo: Option<Vec<u8>>,
    name: String,
    symbol: String,
    decimals: u8,
    owner: Principal,
    minters: Vec<Principal>,
    fee: TokenFee,
    fee_to: TokenHolder,
}

impl Default for TokenSettingInner {
    fn default() -> Self {
        TokenSettingInner {
            token_id: Principal::anonymous(),
            logo: None,
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0,
            owner: Principal::anonymous(),
            minters: Vec::new(),
            fee: TokenFee::default(),
            fee_to: TokenHolder::empty(),
        }
    }
}

impl StableState for TokenSetting {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(&self.inner).unwrap()
    }

    fn decode(bytes: Vec<u8>) -> Result<Self, String> {
        let token_setting_inner: TokenSettingInner = bincode::deserialize(&bytes).unwrap();

        Ok(TokenSetting {
            inner: token_setting_inner,
        })
    }
}
