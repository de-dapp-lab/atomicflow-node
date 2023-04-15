use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PaymentStatus {
    pub plan_key: String,
    pub payer_address: String,
    pub payer_evm_address: String,
    pub start_time: DateTime<Local>,
    pub status: bool,
    pub member_count: u64,
}

impl PaymentStatus {
    pub fn new(
        plan_key: String,
        payer_address: String,
        payer_evm_address: String,
        start_time: DateTime<Local>,
        status: bool,
        member_count: u64,
    ) -> Self {
        PaymentStatus {
            plan_key,
            payer_address,
            payer_evm_address,
            start_time,
            status,
            member_count,
        }
    }
}
