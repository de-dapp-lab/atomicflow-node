use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct DB(pub(crate) Arc<PgPool>);

impl DB {
    pub async fn new() -> DB {
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(
                &dotenvy::var("DATABASE_URL")
                    .unwrap_or_else(|_| panic!("DATABASE_URL must be set!")),
            )
            .await
            .unwrap_or_else(|_| {
                panic!("Cannot connect to the database. Please check your configuration.")
            });
        DB(Arc::new(pool))
    }
}
