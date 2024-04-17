use sea_orm::DbErr;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DbErr(#[from] DbErr),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Invalid date range")]
    InvalidDateRange,
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}
