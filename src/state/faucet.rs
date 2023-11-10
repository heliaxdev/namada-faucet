use crate::{
    app_state::AppState, repository::faucet::FaucetRepository,
    repository::faucet::FaucetRepositoryTrait, sdk::namada::Sdk, services::faucet::FaucetService,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct FaucetState {
    pub faucet_service: FaucetService,
    pub faucet_repo: FaucetRepository,
    pub sdk: Arc<Sdk>,
    pub auth_key: String,
    pub difficulty: u64,
    pub chain_id: String,
    pub chain_start: i64,
}

impl FaucetState {
    pub fn new(
        data: &Arc<RwLock<AppState>>,
        sdk: Sdk,
        auth_key: String,
        difficulty: u64,
        chain_id: String,
        chain_start: i64,
    ) -> Self {
        Self {
            faucet_service: FaucetService::new(data),
            faucet_repo: FaucetRepository::new(data),
            sdk: Arc::new(sdk),
            auth_key,
            difficulty,
            chain_id,
            chain_start,
        }
    }
}
