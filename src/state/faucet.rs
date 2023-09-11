use namada::types::{key::common::SecretKey, address::Address};

use crate::{
    app_state::AppState, repository::faucet::FaucetRepository,
    repository::faucet::FaucetRepositoryTrait, services::faucet::FaucetService, sdk::namada::NamadaSdk};
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct FaucetState {
    pub faucet_service: FaucetService,
    pub faucet_repo: FaucetRepository,
    pub sdk: Arc<Mutex<NamadaSdk>>,
    pub sk: SecretKey,
    pub nam_address: Address,
    pub auth_key: String,
    pub difficulty: u64,
    pub chain_id: String
}

impl FaucetState {
    pub fn new(data: &Arc<RwLock<AppState>>, sdk: NamadaSdk, sk: SecretKey, nam_address: Address, auth_key: String, difficulty: u64, chain_id: String) -> Self {
        Self {
            faucet_service: FaucetService::new(data),
            faucet_repo: FaucetRepository::new(data),
            sdk: Arc::new(Mutex::new(sdk)),
            sk,
            nam_address,
            auth_key,
            difficulty,
            chain_id,
        }
    }
}
