#[derive(Debug)]
pub struct Plan {
    pub plan_id: String,
    pub receiver_address: String,
    pub name: String,
    pub token_address: String,
}

impl Plan {
    pub fn new(
        plan_id: String,
        receiver_address: String,
        name: String,
        token_address: String,
    ) -> Self {
        Plan {
            plan_id,
            receiver_address,
            name,
            token_address,
        }
    }
}
