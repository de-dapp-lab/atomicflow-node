use crate::domain::plan::Plan;
use crate::infrastructure::external_service::contract::status::StatusContract;

#[derive(Clone)]
pub struct PlanRepository {
    contract: StatusContract,
}

impl PlanRepository {
    pub fn new(contract: StatusContract,) -> Self {
        Self { contract }
    }

    pub async fn get_all(&self) -> anyhow::Result<Vec<Plan>> {
        // TODO: impl
        Ok(vec![])
    }
}
