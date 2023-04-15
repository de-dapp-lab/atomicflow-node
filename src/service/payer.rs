use crate::domain::payer::Payer;
use crate::infrastructure::external_service::intmax::IntmaxService;
use crate::infrastructure::repository::payer::PayerRepository;
use crate::infrastructure::repository::plan::PlanRepository;

#[derive(Clone)]
pub struct PayerService {
    l2_service: IntmaxService,
    plan_repo: PlanRepository,
    payer_repo: PayerRepository,
}

impl PayerService {
    pub fn new(
        l2_service: IntmaxService,
        plan_repo: PlanRepository,
        payer_repo: PayerRepository,
    ) -> Self {
        Self {
            l2_service,
            plan_repo,
            payer_repo,
        }
    }

    pub async fn create(&self, plan_key: &str, evm_address: &str) -> anyhow::Result<()> {
        // TODO: create transaction of DB

        // Check plan
        let plan = self.plan_repo.get(plan_key).await?;
        match plan {
            None => anyhow::bail!("plan {} is not found", plan_key),
            _ => {}
        }

        // Create payer
        let (wallet, address) = self.l2_service.new_encoded_wallet().await?;

        let payer = Payer::new(address, evm_address.to_string(), wallet);
        self.payer_repo.create(payer).await?;

        Ok(())
    }
}
