use crate::domain::payment_status::PaymentStatus;
use crate::infrastructure::external_service::contract::status::StatusContract;
use chrono::Local;
use ethers::prelude::*;

#[derive(Clone)]
pub struct PaymentStatusRepository {
    contract: StatusContract,
}

impl PaymentStatusRepository {
    pub fn new(contract: StatusContract) -> Self {
        Self { contract }
    }

    pub async fn save_state(&self, payment_key: U256, status: bool) -> anyhow::Result<()> {
        self.contract
            .contract
            .method::<_, H256>("saveStatus", (payment_key, status))?
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_all(&self) -> anyhow::Result<Vec<PaymentStatus>> {
        let payer_address = "0x3d68111635a765a6";

        let payment_status = vec![
            PaymentStatus::new(
                "1".to_string(),
                payer_address.to_string(),
                "payer_evm_address".to_string(),
                Local::now(),
                true,
                5,
            ),
            PaymentStatus::new(
                "2".to_string(),
                payer_address.to_string(),
                "payer_evm_address".to_string(),
                Local::now(),
                true,
                8,
            ),
            PaymentStatus::new(
                "3".to_string(),
                payer_address.to_string(),
                "payer_evm_address".to_string(),
                Local::now(),
                true,
                10,
            ),
        ];
        Ok(payment_status)
    }
}
