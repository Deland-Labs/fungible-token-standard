use crate::*;
use candid::{Deserialize, Principal};
use serde::Serialize;

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_setting_encode_decode() {
        let token_setting = TokenSetting::default();
        let encoded = token_setting.encode();
        let decoded = TokenSetting::decode(encoded).unwrap();
        assert_eq!(token_setting, decoded);
    }

    #[test]
    fn test_token_setting_set_logo() {
        let mut token_setting = TokenSetting::default();
        token_setting.set_logo(Some(vec![1, 2, 3]));
        assert_eq!(token_setting.logo(), Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_token_setting_set_owner() {
        let mut token_setting = TokenSetting::default();
        let owner = "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
            .parse()
            .unwrap();
        token_setting.set_owner(owner);
        assert_eq!(token_setting.owner(), owner);
    }

    #[test]
    fn test_token_setting_set_fee() {
        let mut token_setting = TokenSetting::default();
        let fee = TokenFee::new(1u32.into(), 2, 3);
        token_setting.set_fee(fee.clone());
        assert_eq!(token_setting.fee(), fee);
    }

    #[test]
    fn test_token_setting_set_fee_to() {
        let mut token_setting = TokenSetting::default();
        let fee_to = TokenHolder::new(
            "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
                .parse()
                .unwrap(),
            None,
        );
        token_setting.set_fee_to(fee_to.clone());
        assert_eq!(token_setting.fee_to(), fee_to);
    }

    #[test]
    fn test_token_setting_set_minter() {
        let mut token_setting = TokenSetting::default();
        let minter: Principal = "o5y7v-htz2q-vk7fc-cqi4m-bqvwa-eth75-sc2wz-ubuev-curf2-rbipe-tae"
            .parse()
            .unwrap();
        token_setting.add_minter(minter.clone());
        assert_eq!(token_setting.minters(), vec![minter]);
    }

    #[test]
    fn test_token_setting_default() {
        let mut token_setting = TokenSetting::default();

        assert_eq!(token_setting.token_id(), &Principal::anonymous());
        assert_eq!(token_setting.logo(), None);
        assert_eq!(token_setting.name(), "");
        assert_eq!(token_setting.symbol(), "");
        assert_eq!(token_setting.decimals(), 0);
        assert_eq!(token_setting.owner(), Principal::anonymous());
        assert_eq!(token_setting.minters(), Vec::new());
        assert_eq!(token_setting.fee(), TokenFee::default());
        assert_eq!(token_setting.fee_to(), TokenHolder::empty());
    }
}
