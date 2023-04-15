use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub transaction_id: String,
    pub payer_address: String,
    pub receiver_address: String,
    pub token_address: String,
    pub amount: u64,
    pub cumulative_amount: u64,
    pub created_at: DateTime<Local>,
}

impl Transaction {
    pub fn new(
        transaction_id: String,
        payer_address: String,
        receiver_address: String,
        token_address: String,
        amount: u64,
        cumulative_amount: u64,
        created_at: DateTime<Local>,
    ) -> Self {
        Transaction {
            transaction_id,
            payer_address,
            receiver_address,
            token_address,
            amount,
            cumulative_amount,
            created_at,
        }
    }
}
