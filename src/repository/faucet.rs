use std::sync::{Arc, RwLock};

use async_trait::async_trait;

use crate::app_state::AppState;

#[derive(Clone)]
pub struct FaucetRepository {
    pub(crate) data: Arc<RwLock<AppState>>,
}

#[async_trait]
pub trait FaucetRepositoryTrait {
    fn new(data: &Arc<RwLock<AppState>>) -> Self;
    fn add(&mut self, challenge: String);
    fn contains(&self, challenge: &str) -> bool;
}

#[async_trait]
impl FaucetRepositoryTrait for FaucetRepository {
    fn new(data: &Arc<RwLock<AppState>>) -> Self {
        Self { data: data.clone() }
    }

    fn add(&mut self, challenge: String) {
        let mut state = self.data.write().expect("Should be able to lock appstate.");
        state.add(challenge)
    }

    fn contains(&self, challenge: &str) -> bool {
        let state = self.data.read().expect("Should be able to lock appstate.");
        state.contains(&challenge.to_string())
    }
}
