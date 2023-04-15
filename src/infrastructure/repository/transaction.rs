use crate::domain::transaction::Transaction;
use crate::infrastructure::external_service::postgres::DB;
use anyhow::anyhow;
use chrono::NaiveDateTime;
use chrono::{Local, TimeZone};
use sqlx::{query, query_as};

#[derive(Clone)]
pub struct TransactionRepository {
    db: DB,
}

#[derive(Debug)]
pub struct TransactionTable {
    pub transaction_id: String,
    pub payer_address: String,
    pub receiver_address: String,
    pub token_address: String,
    pub amount: i64,
    pub cumulative_amount: i64,
    pub created_at: NaiveDateTime,
}

impl TryFrom<TransactionTable> for Transaction {
    type Error = anyhow::Error;
    fn try_from(tt: TransactionTable) -> Result<Self, Self::Error> {
        Ok(Transaction::new(
            tt.transaction_id,
            tt.payer_address,
            tt.receiver_address,
            tt.token_address,
            u64::try_from(tt.amount)?,
            u64::try_from(tt.cumulative_amount)?,
            Local
                .from_local_datetime(&tt.created_at)
                .earliest()
                .ok_or(anyhow!(
                    "Cannot parse value {:?}, confirm your format.",
                    tt.created_at
                ))?,
        ))
    }
}

impl TryFrom<Transaction> for TransactionTable {
    type Error = anyhow::Error;
    fn try_from(t: Transaction) -> Result<Self, Self::Error> {
        Ok(TransactionTable {
            transaction_id: t.transaction_id,
            payer_address: t.payer_address.to_string(),
            receiver_address: t.receiver_address.to_string(),
            token_address: t.token_address,
            amount: i64::try_from(t.amount)?,
            cumulative_amount: i64::try_from(t.cumulative_amount)?,
            created_at: t.created_at.naive_utc(),
        })
    }
}

impl TransactionRepository {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub async fn bulk_create(&self, transactions: Vec<Transaction>) -> anyhow::Result<()> {
        let pool = self.db.0.clone();

        let  query_str = "INSERT INTO transactions(transaction_id, payer_address, receiver_address, token_address, amount, cumulative_amount, created_at) VALUES "
            .to_owned()
            + &(0..transactions.len())
            .map(|i| format!("(${}, ${}, ${}, ${}, ${}::bigint, ${}::bigint, ${}::timestamp)", i * 7 + 1, i * 7 + 2,i * 7 + 3,i * 7 + 4,i * 7 + 5,i * 7 + 6,i * 7 + 7,))
            .collect::<Vec<_>>()
            .join(", ");

        let mut query = sqlx::query(&query_str);

        for tx in transactions {
            query = query
                .bind(tx.transaction_id)
                .bind(tx.payer_address)
                .bind(tx.receiver_address)
                .bind(tx.token_address)
                .bind(tx.amount.to_string())
                .bind(tx.cumulative_amount.to_string())
                .bind(tx.created_at.to_string())
        }

        query.execute(&*pool).await?;

        Ok(())
    }

    pub async fn save(&self, transaction: Transaction) -> anyhow::Result<()> {
        let pool = self.db.0.clone();

        let tx: TransactionTable = transaction.try_into()?;

        query!(
            "INSERT INTO transactions(transaction_id, payer_address, receiver_address, token_address, amount, cumulative_amount, created_at)
VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT (transaction_id) DO UPDATE SET payer_address     = $2,
                                           receiver_address  = $3,
                                           token_address     = $4,
                                           amount            = $5,
                                           cumulative_amount = $6,
                                           created_at = $7;",
            tx.transaction_id,
            tx.payer_address,
            tx.receiver_address,
            tx.token_address,
            tx.amount,
            tx.cumulative_amount,
            tx.created_at
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }

    pub async fn get_latest(
        &self,
        payer_address: &str,
        receiver_address: &str,
        token_address: &str,
    ) -> anyhow::Result<Option<Transaction>> {
        let pool = self.db.0.clone();

        let transaction = query_as!(
            TransactionTable,
            "SELECT transaction_id, payer_address, receiver_address, token_address, amount, cumulative_amount, created_at
FROM transactions
WHERE payer_address = $1
  AND receiver_address = $2
  AND token_address = $3
ORDER BY created_at DESC
LIMIT 1;",
            payer_address,
            receiver_address,
            token_address
        )
        .fetch_optional(&*pool)
        .await?;

        match transaction {
            Some(tx) => {
                let tx = tx.try_into()?;
                Ok(Some(tx))
            }
            None => Ok(None),
        }
    }
}
