use std::sync::{Arc, RwLock};

use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;
use namada_faucet::{app::ApplicationServer, app_state::AppState, config::AppConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());
    let db = Arc::new(RwLock::new(AppState::default()));
    
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    ApplicationServer::serve(config, db)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
