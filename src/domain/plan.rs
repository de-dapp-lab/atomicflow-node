#[derive(Debug, Clone)]
pub struct Plan {
    pub plan_key: String,
    pub receiver_address: String,
    pub name: String,
    pub token_address: String,
    pub amount_per_month: u64,
    pub max_member: u64,
}

impl Plan {
    pub fn new(
        plan_key: String,
        receiver_address: String,
        name: String,
        token_address: String,
        amount_per_month: u64,
        max_member: u64,
    ) -> Self {
        Plan {
            plan_key,
            receiver_address,
            name,
            token_address,
            amount_per_month,
            max_member,
        }
    }
}
