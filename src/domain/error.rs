use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Transaction fail")]
    FailedTransaction,
}
