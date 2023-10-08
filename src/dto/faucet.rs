
use serde::{Deserialize, Serialize};
use validator::Validate;



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
    #[validate(range(min = 1, max = 1_000_000_000))]
    pub amount: u64,
}