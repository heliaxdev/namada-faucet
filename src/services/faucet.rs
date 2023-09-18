use data_encoding::HEXLOWER;
use orion::auth::{self, Tag};
use rand::prelude::*;
use rand::rngs::adapter::ReseedingRng;
use rand::SeedableRng;
use rand_chacha::rand_core::OsRng;
use rand_chacha::ChaCha20Core;

use std::sync::{Arc, RwLock};

use crate::{
    app_state::AppState, entity::faucet::Faucet, error::api::ApiError,
    repository::faucet::FaucetRepository, repository::faucet::FaucetRepositoryTrait, utils,
};

#[derive(Clone)]
pub struct FaucetService {
    _faucet_repo: FaucetRepository,
    r: RndGen,
}

impl FaucetService {
    pub fn new(data: &Arc<RwLock<AppState>>) -> Self {
        Self {
            _faucet_repo: FaucetRepository::new(data),
            r: RndGen::default(),
        }
    }

    pub async fn generate_faucet_request(&mut self, auth_key: String) -> Result<Faucet, ApiError> {
        let challenge = self.r.generate();
        let tag = self.compute_tag(&auth_key, &challenge);

        Ok(Faucet::request(challenge, tag))
    }

    fn compute_tag(&self, auth_key: &String, challenge: &[u8]) -> Vec<u8> {
        let key = auth::SecretKey::from_slice(auth_key.as_bytes())
            .expect("Should be able to convert key to bytes");
        let tag = auth::authenticate(&key, challenge).expect("Should be able to compute tag");

        tag.unprotected_as_bytes().to_vec()
    }

    pub fn verify_tag(&self, auth_key: &String, challenge: &String, tag: &String) -> bool {
        let key = auth::SecretKey::from_slice(auth_key.as_bytes())
            .expect("Should be able to convert key to bytes");

        let decoded_tag = if let Ok(decoded_tag) = HEXLOWER.decode(tag.as_bytes()) {
            let tag = Tag::from_slice(&decoded_tag);
            match tag {
                Ok(tag) => {
                    let tag_bytes = tag.unprotected_as_bytes().to_vec();
                    tag_bytes
                }
                Err(_) => return false,
            }
        } else {
            return false;
        };

        let tag = Tag::from_slice(&decoded_tag).expect("Should be able to convert bytes to tag");

        let decoded_challenge = HEXLOWER.decode(challenge.as_bytes()).expect("Test");

        auth::authenticate_verify(&tag, &key, &decoded_challenge).is_ok()
    }

    pub fn verify_pow(&self, challenge: &String, solution: &String, difficulty: u64) -> bool {
        utils::pow::is_valid_proof_of_work(challenge, solution, difficulty)
    }
}

#[derive(Clone)]
pub struct RndGen {
    r: ReseedingRng<ChaCha20Core, OsRng>,
}

impl Default for RndGen {
    fn default() -> Self {
        let prng = ChaCha20Core::from_entropy();
        Self {
            r: ReseedingRng::new(prng, 0, OsRng),
        }
    }
}

impl RndGen {
    pub fn generate(&mut self) -> Vec<u8> {
        let random_one = self.r.next_u64();
        let random_two = self.r.next_u64();

        [random_one.to_be_bytes(), random_two.to_be_bytes()].concat()
    }
}
