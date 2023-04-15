use crate::domain::plan::Plan;
use crate::infrastructure::external_service::contract::status::StatusContract;
use ethers::core::k256::U256;
use tracing::debug;

#[derive(Clone)]
pub struct PlanRepository {
    contract: StatusContract,
}

// struct RawPlan {
//     planKey: U256,
//     amountPerMonth: U256,
//     receiverWallet: U256,
//     maxMember: U256,
//     tokenAddress: U256,
// }
//
// impl TryFrom<RawPlan> for Plan {
//     type Error = anyhow::Error;
//     fn try_from(r: RawPlan) -> Result<Self, Self::Error> {
//         Ok(Plan {
//             plan_key: r.planKey.to_string(),
//             receiver_address: r.receiverWallet.to_string(),
//             amount_per_month: r.amountPerMonth,
//             max_member: r.maxMember,
//         })
//     }
// }

impl PlanRepository {
    pub fn new(contract: StatusContract) -> Self {
        Self { contract }
    }

    pub async fn get(&self, plan_key: &str) -> anyhow::Result<Option<Plan>> {
        // let res = self
        //     .contract
        //     .contract
        //     .method::<_, _>("getPlan", plan_key.to_string())?
        //     .call()
        //     .await?;
        //
        // debug!("res:{:?}", res);

        let plan = match plan_key {
            "1" => Some(Plan::new(plan_key.to_string(), "".to_string(), 8, 5)),
            "2" => Some(Plan::new(plan_key.to_string(), "".to_string(), 10, 10)),
            "3" => Some(Plan::new(plan_key.to_string(), "".to_string(), 15, 20)),
            _ => None,
        };

        // match res {
        //     Some(res) => {
        //         let plan:Plan =res.try_into()?;
        //         Ok(plan)
        //     },
        //     None => Ok(None)
        // }
        Ok(plan)
    }

    pub async fn get_all(&self) -> anyhow::Result<Vec<Plan>> {
        let plans = vec![
            Plan::new("1".to_string(), "".to_string(), 8, 5),
            Plan::new("2".to_string(), "".to_string(), 10, 10),
            Plan::new("3".to_string(), "".to_string(), 15, 20),
        ];
        Ok(plans)
    }
}
