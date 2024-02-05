use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::{
    app_state::AppState, repository::faucet::FaucetRepository,
    repository::faucet::FaucetRepositoryTrait, services::faucet::FaucetService,
};
use tokio::sync::RwLock;

use namada_sdk::{
    io::NullIo, masp::fs::FsShieldedUtils, types::address::Address, wallet::fs::FsWalletUtils,
    NamadaImpl,
};
use tendermint_rpc::HttpClient;

type PlayerId = String;

#[derive(Clone)]
pub struct FaucetState {
    pub faucet_service: FaucetService,
    pub faucet_repo: FaucetRepository,
    pub faucet_address: Address,
    pub sdk: Arc<NamadaImpl<HttpClient, FsWalletUtils, FsShieldedUtils, NullIo>>,
    pub auth_key: String,
    pub difficulty: u64,
    pub chain_id: String,
    pub chain_start: i64,
    pub withdraw_limit: u64,
    pub request_frequency: Duration,
    pub last_requests: HashMap<PlayerId, Instant>,
    pub webserver_host: String,
}

impl FaucetState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        data: &Arc<RwLock<AppState>>,
        faucet_address: Address,
        sdk: NamadaImpl<HttpClient, FsWalletUtils, FsShieldedUtils, NullIo>,
        auth_key: String,
        chain_id: String,
        chain_start: i64,
        withdraw_limit: u64,
        webserver_host: String,
        request_frequency: u64,
    ) -> Self {
        Self {
            faucet_service: FaucetService::new(data),
            faucet_repo: FaucetRepository::new(data),
            faucet_address,
            sdk: Arc::new(sdk),
            auth_key,
            difficulty: 0,
            chain_id,
            chain_start,
            withdraw_limit,
            webserver_host,
            request_frequency: Duration::from_secs(request_frequency),
            last_requests: HashMap::new(),
        }
    }
}
