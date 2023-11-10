use namada_sdk::{
    core::types::key::common::SecretKey,
    io::NullIo,
    masp::{fs::FsShieldedUtils, ShieldedContext},
    wallet::{fs::FsWalletUtils, Wallet},
    NamadaImpl,
};
use tendermint_rpc::HttpClient;

pub struct Sdk {
    pub faucet_sk: SecretKey,
    pub namada: NamadaImpl<HttpClient, FsWalletUtils, FsShieldedUtils, NullIo>,
}

impl Sdk {
    pub async fn new(
        faucet_sk: SecretKey,
        rpc_client: HttpClient,
        wallet: Wallet<FsWalletUtils>,
        shielded_ctx: ShieldedContext<FsShieldedUtils>,
        io: NullIo,
    ) -> Self {
        let namada = NamadaImpl::new(rpc_client, wallet, shielded_ctx, io)
            .await
            .unwrap();
        Self { faucet_sk, namada }
    }
}
