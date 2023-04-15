use crate::infrastructure::external_service::contract::client::Client;
use crate::infrastructure::external_service::contract::status::StatusContract;
use crate::infrastructure::external_service::intmax::IntmaxService;
use crate::infrastructure::external_service::postgres::DB;
use crate::infrastructure::repository::payment_status::PaymentStatusRepository;
use crate::infrastructure::repository::plan::PlanRepository;
use crate::infrastructure::repository::transaction::TransactionRepository;
use crate::infrastructure::repository::wallet::WalletRepository;
use crate::presentation::controller::transaction::TransactionController;
use crate::service::transaction::TransactionService;
use intmax::service::builder::ServiceBuilder;
use crate::infrastructure::repository::payer::PayerRepository;
use crate::presentation::controller::payer::PayerController;
use crate::service::payer::PayerService;

#[derive(Clone)]
pub struct Container {
    pub transaction_controller: TransactionController,
    pub payer_controller:PayerController
}

impl Container {
    pub async fn new(aggregator_url: &str) -> anyhow::Result<Self> {
        // Infrastructure

        // External service(Infrastructure)
        let intmax_service_builder = ServiceBuilder::new(aggregator_url);

        let aggregator_url = intmax_service_builder
            .aggregator_api_url("")
            .split("://")
            .last()
            .unwrap()
            .to_string();

        let wallet_repo = WalletRepository::new(&aggregator_url);

        let intmax_service =
            IntmaxService::new(intmax_service_builder.clone(), wallet_repo.clone());

        tracing::debug!("aggregator origin: {}", aggregator_url);

        let ethers_client = Client::new().await?;
        let status_contract = StatusContract::new(ethers_client.clone()).await?;

        let db = DB::new().await;

        // Repository(Infrastructure)
        let transaction_repo = TransactionRepository::new(db.clone());
        let payment_status_repo = PaymentStatusRepository::new(status_contract.clone());
        let plan_repo = PlanRepository::new(status_contract.clone());
        let payer_repo = PayerRepository::new(db.clone());

        // Application service
        let transaction_service = TransactionService::new(
            payment_status_repo.clone(),
            transaction_repo.clone(),
            plan_repo.clone(),
            intmax_service.clone(),
            wallet_repo.clone(),
        );
        let payer_service = PayerService::new(intmax_service.clone(),plan_repo.clone(),payer_repo.clone());

        // Presentation
        // Controller(Presentation)
        let transaction_controller = TransactionController::new(transaction_service.clone());
        let payer_controller = PayerController::new(payer_service.clone());

        Ok(Container {
            transaction_controller,
            payer_controller
        })
    }
}
