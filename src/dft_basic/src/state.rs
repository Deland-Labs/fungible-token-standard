use dft_types::*;
use ic_cdk::api::stable::{stable_bytes, StableWriter};
use ic_cdk_macros::*;
use log::{error, info};
use std::cell::RefCell;

thread_local! {
      pub static STATE : State = State::default();
}
#[derive(Default)]
pub struct State {
    pub token_setting: RefCell<TokenSetting>,
    pub token_desc: RefCell<TokenDescription>,
    pub blockchain: RefCell<Blockchain>,
    pub balances: RefCell<TokenBalances>,
    pub allowances: RefCell<TokenAllowances>,
}

impl State {
    pub fn replace(&self, new_state: State) {
        self.token_setting.replace(new_state.token_setting.take());
        self.token_desc.replace(new_state.token_desc.take());
        self.blockchain.replace(new_state.blockchain.take());
        self.balances.replace(new_state.balances.take());
        self.allowances.replace(new_state.allowances.take());
    }
}

impl StableState for State {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(&(
            self.token_setting.borrow().encode(),
            self.token_desc.borrow().encode(),
            self.blockchain.borrow().encode(),
            self.balances.borrow().encode(),
            self.allowances.borrow().encode(),
        ))
        .unwrap()
    }

    #[allow(clippy::type_complexity)]
    fn decode(bytes: Vec<u8>) -> Result<Self, String> {
        let (
            token_setting_bytes,
            token_desc_bytes,
            blockchain_bytes,
            balances_bytes,
            allowances_bytes,
        ): (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) = bincode::deserialize(&bytes).unwrap();

        Ok(State {
            token_setting: RefCell::new(TokenSetting::decode(token_setting_bytes)?),
            token_desc: RefCell::new(TokenDescription::decode(token_desc_bytes)?),
            blockchain: RefCell::new(Blockchain::decode(blockchain_bytes)?),
            balances: RefCell::new(TokenBalances::decode(balances_bytes)?),
            allowances: RefCell::new(TokenAllowances::decode(allowances_bytes)?),
        })
    }
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|s| {
        let bytes = s.encode();
        match StableWriter::default().write(bytes.as_slice()) {
            Ok(size) => {
                info!("after pre_upgrade stable_write size{}", size);
            }
            Err(_) => {
                error!("{}", "stable_write error");
            }
        }
    })
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|s| {
        let bytes = stable_bytes();
        let restore_state = State::decode(bytes).expect("Decoding stable memory failed");
        s.replace(restore_state);
    })
}
