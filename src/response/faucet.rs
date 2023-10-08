use data_encoding::HEXLOWER;
use serde::{Deserialize, Serialize};

use crate::entity::faucet::Faucet;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FaucetResponse {
    pub challenge: String,
    pub tag: String,
}

impl From<Faucet> for FaucetResponse {
    fn from(value: Faucet) -> Self {
        Self {
            challenge: HEXLOWER.encode(&value.challenge),
            tag: HEXLOWER.encode(&value.tag),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FaucetResponseStatus {
    pub token: String,
    pub amount: u64,
    pub target: String,
    pub sent: bool,
}
