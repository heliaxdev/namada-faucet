use serde::Deserialize;
use thiserror::Error;

use crate::dto::network::NetworkWithId;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Unknown error for network id {0}: {1}")]
    Unknown(String, String),
    #[error("Unknown error: {0}")]
    UnknownWithoutId(String),
}

#[derive(Debug, Deserialize)]
pub struct NetworkListResponse {
    pub networks: Vec<NetworkWithId>
}

pub async fn list_networks(host: &str) -> Result<NetworkListResponse, RequestError> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/v1/network", host);

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| RequestError::UnknownWithoutId(e.to_string()))?
        .json::<NetworkListResponse>()
        .await
        .map_err(|e| RequestError::UnknownWithoutId(e.to_string()))?;

    Ok(response)
}