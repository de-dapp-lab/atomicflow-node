use crate::domain::payer::Payer;
use crate::infrastructure::external_service::postgres::DB;
use sqlx::{query};

#[derive(Clone)]
pub struct PayerRepository {
    db: DB,
}

#[derive(Debug, Clone)]
pub struct PayerTable {
    pub address: String,
    pub evm_address: String,
    pub assets: String,
}

impl TryFrom<PayerTable> for Payer {
    type Error = anyhow::Error;
    fn try_from(pt: PayerTable) -> Result<Self, Self::Error> {
        Ok(Payer::new(
            pt.address,
            pt.evm_address,
            pt.assets,
        ))
    }
}

impl TryFrom<Payer> for PayerTable {
    type Error = anyhow::Error;
    fn try_from(p: Payer) -> Result<Self, Self::Error> {
        Ok(PayerTable {
            address: p.address.to_string(),
            evm_address: p.evm_address,
            assets: p.assets,
        })
    }
}

impl PayerRepository {
    pub fn new(db: DB) -> Self {
        Self { db }
    }



    pub async fn create(&self, payer: Payer) -> anyhow::Result<()> {
        let pool = self.db.0.clone();

        let p: PayerTable = payer.try_into()?;

        query!(
            "INSERT INTO payers (address, evm_address, assets)
VALUES ($1, $2, $3);",
            p.address,
            p.evm_address,
            p.assets,
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }


    pub async fn update_asset(&self, payer: Payer) -> anyhow::Result<()> {
        let pool = self.db.0.clone();

        let p: PayerTable = payer.try_into()?;

        query!(
            "UPDATE payers
    SET assets = $2
    WHERE address = $1;",
            p.address,
            p.assets
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }
}
