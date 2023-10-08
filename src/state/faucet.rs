use crate::{
    app_state::AppState, repository::faucet::FaucetRepository,
    repository::faucet::FaucetRepositoryTrait, sdk::namada::NamadaSdk,
    services::faucet::FaucetService,
};
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct FaucetState {
    pub faucet_service: FaucetService,
    pub faucet_repo: FaucetRepository,
    pub sdk: Arc<Mutex<NamadaSdk>>,
    pub auth_key: String,
    pub difficulty: u64,
    pub webserver_host: String
}

impl FaucetState {
    pub fn new(
        data: &Arc<RwLock<AppState>>,
        sdk: NamadaSdk,
        auth_key: String,
        difficulty: u64,
        webserver_host: String
    ) -> Self {
        Self {
            faucet_service: FaucetService::new(data),
            faucet_repo: FaucetRepository::new(data),
            sdk: Arc::new(Mutex::new(sdk)),
            auth_key,
            difficulty,
            webserver_host
        }
    }
}
