#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum CargoEnv {
    Development,
    Production,
}

#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env, value_enum)]
    pub cargo_env: CargoEnv,

    #[clap(long, env, default_value = "5000")]
    pub port: u16,

    #[clap(long, env)]
    pub difficulty: u64,

    #[clap(long, env)]
    pub private_key: String,

    #[clap(long, env)]
    pub chain_id: String,

    #[clap(long, env)]
    pub rpcs: Vec<String>,

    #[clap(long, env)]
    pub nam_address: String,

    #[clap(long, env)]
    pub auth_key: Option<String>,

    #[clap(long, env)]
    pub rps: Option<u64>,
}
