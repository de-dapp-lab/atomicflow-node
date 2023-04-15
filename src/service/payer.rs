use crate::domain::payer::Payer;
use crate::infrastructure::external_service::intmax::IntmaxService;
use crate::infrastructure::repository::payer::PayerRepository;
use crate::infrastructure::repository::plan::PlanRepository;
use crate::infrastructure::repository::wallet::WalletRepository;
use intmax::service::builder::calc_merge_witnesses;
use intmax_rollup_interface::intmax_zkp_core::zkdsa::account::Address;
use std::str::FromStr;

#[derive(Clone)]
pub struct PayerService {
    l2_service: IntmaxService,
    payer_repo: PayerRepository,
    wallet_repo: WalletRepository,
}

impl PayerService {
    pub fn new(
        l2_service: IntmaxService,
        payer_repo: PayerRepository,
        wallet_repo: WalletRepository,
    ) -> Self {
        Self {
            l2_service,
            payer_repo,
            wallet_repo,
        }
    }

    pub async fn create(&self, evm_address: &str) -> anyhow::Result<()> {
        // Create payer
        let (wallet, address) = self.l2_service.new_encoded_wallet().await?;

        let payer = Payer::new(address, evm_address.to_string(), wallet);
        self.payer_repo.create(payer).await?;

        Ok(())
    }

    pub async fn get_token_amount(
        &self,
        evm_address: &str,
        token_address: &str,
    ) -> anyhow::Result<String> {
        if let Some(payer) = self.payer_repo.get_by_evm_address(evm_address).await? {
            let mut wallet = self.wallet_repo.decode_wallet(&payer.assets)?;
            let address = Address::from_str(&payer.address)?;
            if let Some(user_state) = wallet.data.get_mut(&address) {
                self.l2_service
                    .sync_sent_transaction(user_state, address)
                    .await?;

                // NOTICE: Changes to `user_state` here are not saved to file.
                calc_merge_witnesses(user_state, user_state.rest_received_assets.clone()).await;

                let total_amount_map = user_state.assets.calc_total_amount();

                for ((_, variable_index), total_amount) in total_amount_map {
                    if token_address == variable_index {
                        return Ok(total_amount.to_string());
                    }
                }

                anyhow::bail!("Token is not found")
            } else {
                anyhow::bail!("Cannot get user state")
            }
        } else {
            anyhow::bail!("Payer is not found")
        }
    }
}
