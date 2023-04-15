use atomicflow_operator::container::Container;
use atomicflow_operator::presentation::controller::payer::{create_payer, get_token_amount};
use atomicflow_operator::presentation::controller::transaction::{
    bulk_transfer, get_latest_transaction,
};
use axum::routing::get;
use axum::{routing::post, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;

const DEFAULT_AGGREGATOR_URL: &str = "http://localhost:8080";

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // settings
    dotenv().unwrap();
    let env_url = dotenvy::var("AGGREGATOR_URL");
    let url = match &env_url {
        Ok(result) => result,
        Err(_) => DEFAULT_AGGREGATOR_URL,
    };
    tracing::info!("aggregator url: {}", url);

    let container = Container::new(url).await.unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/tx", post(bulk_transfer))
        .route("/tx/latest", get(get_latest_transaction))
        .route("/payers", post(create_payer))
        .route("/payer/tokens", get(get_token_amount))
        .with_state(container);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
