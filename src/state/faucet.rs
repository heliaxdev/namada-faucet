use crate::{
    app_state::AppState, repository::faucet::FaucetRepository,
    repository::faucet::FaucetRepositoryTrait,
    services::faucet::FaucetService, sdk::namada::Sdk,
};
use std::sync::{Arc};
use tokio::sync::RwLock;
use std::{path::PathBuf, str::FromStr};

use namada_sdk::{
    args::TxBuilder,
    core::types::{
        address::{Address, ImplicitAddress},
        chain::ChainId,
        key::{common::SecretKey, PublicKeyHash},
    },
    io::NullIo,
    masp::{fs::FsShieldedUtils, ShieldedContext},
    wallet::{fs::FsWalletUtils, Wallet, WalletIo},
    NamadaImpl,
};
use tendermint_rpc::{HttpClient, Url};

#[derive(Clone)]
pub struct FaucetState {
    pub faucet_service: FaucetService,
    pub faucet_repo: FaucetRepository,
    pub faucet_sk: SecretKey,
    pub sdk: Arc<NamadaImpl<HttpClient, FsWalletUtils, FsShieldedUtils, NullIo>>,
    pub auth_key: String,
    pub difficulty: u64,
    pub chain_id: String,
    pub chain_start: i64,
}

impl FaucetState {
    pub fn new(
        data: &Arc<RwLock<AppState>>,
        faucet_sk: SecretKey,
        sdk: NamadaImpl<HttpClient, FsWalletUtils, FsShieldedUtils, NullIo>,
        auth_key: String,
        difficulty: u64,
        chain_id: String,
        chain_start: i64,
    ) -> Self {
        Self {
            faucet_service: FaucetService::new(data),
            faucet_repo: FaucetRepository::new(data),
            faucet_sk,
            sdk: Arc::new(sdk),
            auth_key,
            difficulty,
            chain_id,
            chain_start,
        }
    }
}
