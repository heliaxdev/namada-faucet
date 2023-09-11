use crate::error::faucet::FaucetError;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    FaucetError(#[from] FaucetError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::FaucetError(error) => error.into_response(),
        }
    }
}
