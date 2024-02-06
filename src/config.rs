#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum CargoEnv {
    Development,
    Production,
}

#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env, value_enum)]
    pub cargo_env: CargoEnv,

    /// Port to bind the crawler's HTTP server to
    #[clap(long, env, default_value = "5000")]
    pub port: u16,

    /// Faucet's private key in Namada
    #[clap(long, env)]
    pub private_key: String,

    #[clap(long, env)]
    pub chain_start: i64,

    /// Chain id of Namada
    #[clap(long, env)]
    pub chain_id: String,

    /// URL of the Namada RPC
    #[clap(long, env)]
    pub rpc: String,

    /// Withdraw limit given in base units of NAAN
    #[clap(long, env)]
    pub withdraw_limit: Option<u64>,

    /// Authentication key for faucet challenges
    #[clap(long, env)]
    pub auth_key: Option<String>,

    /// Max number of requests per second
    #[clap(long, env)]
    pub rps: Option<u64>,

    /// URL of the Shielded Expedition's webserver
    #[clap(long, env)]
    pub webserver_host: String,

    /// User request frequency given in seconds
    ///
    /// If more than one request is performed during this
    /// interval, the faucet denies the request
    #[clap(long, env)]
    pub request_frequency: u64,
}
