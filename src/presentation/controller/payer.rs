use crate::container::Container;
use crate::service::payer::PayerService;
use axum::extract::State;
use axum::http::{ StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use axum_macros::debug_handler;
use serde::Deserialize;
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

// #[derive(Debug, Serialize, Validate)]
#[derive(Deserialize)]
pub struct CreatePayerRequest {
    pub plan_key: String,
    pub evm_address:String
}

#[debug_handler]
pub async fn create_payer(
    State(container): State<Container>,
    Json(req): Json<CreatePayerRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = container
        .payer_controller
        .payer_service
        .create(&req.plan_key, &req.evm_address)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        });
    res
}
