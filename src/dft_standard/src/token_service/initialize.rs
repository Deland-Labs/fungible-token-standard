use super::TokenService;
use crate::state::STATE;
use candid::Principal;
use dft_types::*;
use dft_utils::*;

impl TokenService {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::clone_on_copy)]
    pub fn token_initialize(
        &self,
        owner: &Principal,
        token_id: Principal,
        logo: Option<Vec<u8>>,
        name: String,
        symbol: String,
        decimals: u8,
        fee: TokenFee,
        fee_to: TokenHolder,
        archive_options: Option<ArchiveOptions>,
    ) {
        // check logo type
        if logo.is_some() {
            let _ = get_logo_type(&logo.clone().unwrap())
                .map_err(|_| DFTError::InvalidTypeOrFormatOfLogo)
                .unwrap();
        }

        STATE.with(|s| {
            let mut token_settings = s.token_setting.borrow_mut();

            // set the parameters to token's properties
            *token_settings = TokenSetting::new(
                token_id,
                logo,
                name,
                symbol,
                decimals,
                owner.clone(),
                fee,
                fee_to,
            );

            let mut blockchain = s.blockchain.borrow_mut();
            if let Some(options) = archive_options {
                blockchain.archive = Archive::new(options);
            }
        });
    }
}
