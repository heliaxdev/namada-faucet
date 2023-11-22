use std::sync::{Arc};
use tokio::sync::RwLock;

use async_trait::async_trait;

use crate::app_state::AppState;

#[derive(Clone)]
pub struct FaucetRepository {
    pub(crate) data: Arc<RwLock<AppState>>,
}

#[async_trait]
pub trait FaucetRepositoryTrait {
    fn new(data: &Arc<RwLock<AppState>>) -> Self;
    async fn add(&mut self, challenge: String);
    async fn contains(&self, challenge: &str) -> bool;
}

#[async_trait]
impl FaucetRepositoryTrait for FaucetRepository {
    fn new(data: &Arc<RwLock<AppState>>) -> Self {
        Self { data: data.clone() }
    }

    async fn add(&mut self, challenge: String) {
        let mut state = self.data.write().await;
        state.add(challenge)
    }

    async fn contains(&self, challenge: &str) -> bool {
        let state = self.data.read().await;
        state.contains(&challenge.to_string())
    }
}
