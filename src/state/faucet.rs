use crate::{
    app_state::AppState, repository::faucet::FaucetRepository,
    repository::faucet::FaucetRepositoryTrait,
    services::faucet::FaucetService, sdk::namada::Sdk,
};
use std::sync::{Arc, RwLock, Mutex};

#[derive(Clone)]
pub struct FaucetState<'a> {
    pub faucet_service: FaucetService,
    pub faucet_repo: FaucetRepository,
    pub sdk: Arc<Mutex<Sdk<'a>>>,
    pub auth_key: String,
    pub difficulty: u64,
    pub chain_id: String,
    pub chain_start: i64,
}

impl<'a> FaucetState<'a> {
    pub fn new(
        data: &Arc<RwLock<AppState>>,
        sdk: Sdk<'a>,
        auth_key: String,
        difficulty: u64,
        chain_id: String,
        chain_start: i64,
    ) -> Self {
        Self {
            faucet_service: FaucetService::new(data),
            faucet_repo: FaucetRepository::new(data),
            sdk: Arc::new(Mutex::new(sdk)),
            auth_key,
            difficulty,
            chain_id,
            chain_start,
        }
    }
}
