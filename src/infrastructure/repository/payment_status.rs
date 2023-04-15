use crate::infrastructure::external_service::contract::status::StatusContract;
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
}
