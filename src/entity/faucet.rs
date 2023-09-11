use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Faucet {
    pub challenge: Vec<u8>,
    pub tag: Vec<u8>,
    pub solution: Option<String>,
}

impl Faucet {
    pub fn request(challenge: Vec<u8>, tag: Vec<u8>) -> Self {
        Self {
            challenge,
            tag,
            solution: None,
        }
    }
}
