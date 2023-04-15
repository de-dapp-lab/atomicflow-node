use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PaymentStatus {
    pub plan_id: String,
    pub payer_address: String,
    pub latest_tx_fail_at: Option<DateTime<Local>>,
}

impl PaymentStatus {
    pub fn new(
        plan_id: String,
        payer_address: String,
        latest_tx_fail_at: Option<DateTime<Local>>,
    ) -> Self {
        PaymentStatus {
            plan_id,
            payer_address,
            latest_tx_fail_at,
        }
    }
}
