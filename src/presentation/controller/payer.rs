use crate::container::Container;
use crate::service::payer::PayerService;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Clone)]
pub struct PayerController {
    payer_service: PayerService,
}

impl PayerController {
    pub fn new(payer_service: PayerService) -> Self {
        Self { payer_service }
    }
}

#[derive(Deserialize)]
pub struct CreatePayerRequest {
    pub evm_address: String,
}

#[derive(Deserialize)]
pub struct GetTokenAmountQuery {
    pub evm_address: String,
    pub token_address: String,
}

#[derive(Debug, Serialize)]
pub struct GetTokenAmountResponse {
    pub amount: String,
}

#[debug_handler]
pub async fn create_payer(
    State(container): State<Container>,
    Json(req): Json<CreatePayerRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = container
        .payer_controller
        .payer_service
        .create(&req.evm_address)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        });
    res
}

#[debug_handler]
pub async fn get_token_amount(
    State(container): State<Container>,
    Query(query): Query<GetTokenAmountQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = container
        .payer_controller
        .payer_service
        .get_token_amount(&query.evm_address, &query.token_address)
        .await
        .map(|result| {
            (
                StatusCode::OK,
                Json(GetTokenAmountResponse { amount: result }),
            )
        })
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        });
    res
}
