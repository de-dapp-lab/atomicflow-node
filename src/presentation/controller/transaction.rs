use crate::container::Container;
use crate::domain::transaction::Transaction;
use crate::service::transaction::TransactionService;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Clone)]
pub struct TransactionController {
    transaction_service: TransactionService,
}

impl TransactionController {
    pub fn new(transaction_service: TransactionService) -> Self {
        Self {
            transaction_service,
        }
    }
}

#[debug_handler]
pub async fn bulk_transfer(
    State(container): State<Container>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = container
        .transaction_controller
        .transaction_service
        .bulk_transfer()
        .await
        .map(|_| StatusCode::OK)
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        });
    res
}

#[derive(Debug, Deserialize)]
pub struct GetLatestTransactionQuery {
    pub plan_id: String,
    pub receiver_address: String,
}

#[derive(Debug, Serialize)]
pub struct GetLatestTransactionResponse {
    pub transaction: Option<Transaction>,
}

#[debug_handler]
pub async fn get_latest_transaction(
    State(container): State<Container>,
    Query(query): Query<GetLatestTransactionQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = container
        .transaction_controller
        .transaction_service
        .get_latest_tx(&query.plan_id, &query.receiver_address)
        .await
        .map(|result| {
            (
                StatusCode::OK,
                Json(GetLatestTransactionResponse {
                    transaction: result,
                }),
            )
        })
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        });
    res
}
