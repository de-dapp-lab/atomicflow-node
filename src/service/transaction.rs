extern crate intmax;

use crate::domain::error::ServiceError;
use crate::domain::payer::Payer;
use crate::domain::plan::Plan;
use crate::domain::transaction::Transaction;
use crate::infrastructure::external_service::intmax::IntmaxService;
use crate::infrastructure::repository::payer::PayerRepository;
use crate::infrastructure::repository::payment_status::PaymentStatusRepository;
use crate::infrastructure::repository::plan::PlanRepository;
use crate::infrastructure::repository::transaction::TransactionRepository;
use crate::infrastructure::repository::wallet::WalletRepository;
use chrono::{DateTime, Local};
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;

#[derive(Clone)]
pub struct TransactionService {
    payment_status_repo: PaymentStatusRepository,
    transaction_repo: TransactionRepository,
    plan_repo: PlanRepository,
    l2_service: IntmaxService,
    payer_repo: PayerRepository,

    //Temp
    wallet_repo: WalletRepository,
}

impl TransactionService {
    pub fn new(
        payment_status_repo: PaymentStatusRepository,
        transaction_repo: TransactionRepository,
        plan_repo: PlanRepository,
        l2_service: IntmaxService,
        payer_repo: PayerRepository,
        wallet_repo: WalletRepository,
    ) -> Self {
        Self {
            payment_status_repo,
            transaction_repo,
            plan_repo,
            l2_service,
            payer_repo,
            wallet_repo,
        }
    }

    pub async fn get_latest_tx(
        &self,
        plan_id: &str,
        receiver_address: &str,
    ) -> anyhow::Result<Option<Transaction>> {
        // FIXME

        // match self.plan_repo.get_all().await? {
        //     Some(plan) => {
        //         self.transaction_repo
        //             .get_latest(
        //                 &plan.receiver_address.to_string(),
        //                 receiver_address,
        //                 &plan.token_address,
        //             )
        //             .await
        //     }
        //     None => Ok(None),
        // }
        Ok(None)
    }

    pub async fn bulk_transfer(&self) -> anyhow::Result<()> {
        let payments = self.payment_status_repo.get_all().await?;

        // TODO: multiple payer
        let mut payer: Payer;
        if let Some(p) = self
            .payer_repo
            .get_by_address(&payments[0].payer_address)
            .await?
        {
            payer = p
        } else {
            anyhow::bail!("Payer is not found")
        }

        let mut wallet = self.wallet_repo.decode_wallet(&payer.assets)?;

        let plans = self.plan_repo.get_all().await?;

        let mut plan_hashmap: HashMap<String, Plan> = HashMap::new();
        for p in plans {
            plan_hashmap.insert(p.plan_key.to_string(), p.clone());
        }

        let now: DateTime<Local> = Local::now();
        let mut transactions = vec![];
        payments
            .into_iter()
            .try_for_each(|tx| -> anyhow::Result<()> {
                let transaction_id = Uuid::new_v4();
                let plan = plan_hashmap.get(&tx.plan_key);
                if let Some(plan) = plan {
                    let tx = Transaction::new(
                        transaction_id.to_string(),
                        tx.payer_address.to_string(),
                        plan.receiver_address.to_string(),
                        plan.token_address.to_string(),
                        plan.amount_per_month,
                        0,
                        now,
                    );

                    transactions.push(tx);
                    Ok(())
                } else {
                    anyhow::bail!("")
                }
            })?;

        let bulk_mint_result = self
            .l2_service
            .bulk_transfer(
                &mut wallet,
                &transactions[0].payer_address,
                transactions.to_vec(),
            )
            .await;

        let assets = self.wallet_repo.encode_wallet(wallet)?;
        payer.update_assets(assets);
        self.payer_repo.update_asset(payer).await?;
        // let bulk_mint_result: anyhow::Result<()> = Ok(());
        // let bulk_mint_result: anyhow::Result<()> = Err(anyhow!("test"));

        // TODO: use transaction of DB

        match bulk_mint_result {
            Ok(..) => {
                // Get latest tx and sum calc cumulative amount
                // TODO:fix amount

                self.transaction_repo
                    .bulk_create(transactions.to_vec())
                    .await?;

                Ok(())
            }
            Err(err) => {
                match err.downcast_ref::<ServiceError>() {
                    Some(service_err) => {
                        match service_err {
                            ServiceError::FailedTransaction { .. } => {
                                debug!("failed send assets: {:?}", service_err);

                                // TODO: add queue and run call to contract

                                // Save status to contract
                                // let payment_key = keccak256(encode_packed(&[
                                //     Token::String(plan_key.to_string()),
                                //     Token::String(payer_address.to_string()),
                                // ])?);
                                //
                                // let payment_key = U256::from(payment_key);
                                // self.payment_status_repo
                                //     .save_state(payment_key, false)
                                //     .await?;
                                Ok(())
                            }
                        }
                    }
                    None => anyhow::bail!(err),
                }
            }
        }
    }
}
