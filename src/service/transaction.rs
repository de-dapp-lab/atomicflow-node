extern crate intmax;
use crate::domain::error::ServiceError;
use crate::domain::transaction::Transaction;
use crate::infrastructure::external_service::intmax::IntmaxService;
use crate::infrastructure::repository::payment_status::PaymentStatusRepository;
use crate::infrastructure::repository::plan::PlanRepository;
use crate::infrastructure::repository::transaction::TransactionRepository;
use crate::infrastructure::repository::wallet::WalletRepository;
use chrono::{DateTime, Local};
use ethers::abi::{encode_packed, Token};
use ethers::types::U256;
use ethers::utils::keccak256;
use intmax::utils::key_management::memory::SerializableWalletOnMemory;
use tracing::debug;
use uuid::Uuid;

#[derive(Clone)]
pub struct TransactionService {
    payment_status_repo: PaymentStatusRepository,
    transaction_repo: TransactionRepository,
    plan_repo: PlanRepository,
    l2_service: IntmaxService,

    //Temp
    wallet_repo: WalletRepository,
}

impl TransactionService {
    pub fn new(
        payment_status_repo: PaymentStatusRepository,
        transaction_repo: TransactionRepository,
        plan_repo: PlanRepository,
        l2_service: IntmaxService,
        wallet_repo: WalletRepository,
    ) -> Self {
        Self {
            payment_status_repo,
            transaction_repo,
            plan_repo,
            l2_service,
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
        // TODO: get encoded wallet from DB
        let wallet = self.wallet_repo.get_wallet()?;
        let raw = SerializableWalletOnMemory {
            data: wallet.data.values().cloned().collect::<Vec<_>>(),
            default_account: wallet.default_account,
        };
        let assets = serde_json::to_string(&raw).unwrap();

        // TODO: get transfers in condition
        let payer_address = "0x3d68111635a765a6";
        let receiver_address = "0x1909a02279691d0a";
        let amount = 100;
        let token_address = &0u8.to_string();
        let plan_id = "1";
        let transaction_id = Uuid::new_v4();
        let now: DateTime<Local> = Local::now();

        let transactions = vec![Transaction::new(
            transaction_id.to_string(),
            payer_address.to_string(),
            receiver_address.to_string(),
            token_address.to_string(),
            amount,
            0,
            now,
        )];

        let bulk_mint_result = self
            .l2_service
            .bulk_transfer(&assets, payer_address, transactions)
            .await;

        // let bulk_mint_result: anyhow::Result<()> = Ok(());
        // let bulk_mint_result: anyhow::Result<()> = Err(anyhow!("test"));

        // TODO: use transaction of DB

        match bulk_mint_result {
            Ok(..) => {
                // Get latest tx and sum calc cumulative amount
                let mut cumulative_amount = i64::try_from(amount)?;
                if let Some(latest_tx) = self
                    .transaction_repo
                    .get_latest(
                        &payer_address.to_string(),
                        &receiver_address.to_string(),
                        token_address,
                    )
                    .await?
                {
                    cumulative_amount += i64::try_from(latest_tx.cumulative_amount)?
                }

                let transaction = Transaction::new(
                    transaction_id.to_string(),
                    payer_address.to_string(),
                    receiver_address.to_string(),
                    token_address.to_string(),
                    amount,
                    u64::try_from(cumulative_amount)?,
                    now,
                );

                // Save transaction history
                self.transaction_repo.save(transaction).await?;

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
                                let payment_key = keccak256(encode_packed(&[
                                    Token::String(plan_id.to_string()),
                                    Token::String(payer_address.to_string()),
                                ])?);

                                let payment_key = U256::from(payment_key);
                                self.payment_status_repo
                                    .save_state(payment_key, false)
                                    .await?;
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
