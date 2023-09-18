use data_encoding::HEXLOWER;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entity::faucet::Faucet;

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct FaucetRequestDto {
    #[validate(length(min = 1, max = 128, message = "Invalid solution"))]
    pub solution: String,
    #[validate(length(equal = 32, message = "Invalid challenge"))]
    pub challenge: String,
    #[validate(length(equal = 64, message = "Invalid proof"))]
    pub tag: String,
    pub transfer: Transfer,
}

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct Transfer {
    #[validate(length(min = 1, max = 50, message = "Invalid token address"))]
    pub token: String,
    #[validate(length(min = 1, max = 50, message = "Invalid target address"))]
    pub target: String,
    #[validate(range(min = 1, max = 1000))]
    pub amount: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FaucetResponseDto {
    pub challenge: String,
    pub tag: String,
}

impl From<Faucet> for FaucetResponseDto {
    fn from(value: Faucet) -> Self {
        Self {
            challenge: HEXLOWER.encode(&value.challenge),
            tag: HEXLOWER.encode(&value.tag),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FaucetResponseStatusDto {
    pub token: String,
    pub amount: u64,
    pub target: String,
    pub sent: bool,
}
