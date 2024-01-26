use namada_sdk::{
    io::NullIo,
    masp::{fs::FsShieldedUtils, ShieldedContext},
    types::key::common::SecretKey,
    wallet::{fs::FsWalletUtils, Wallet},
    NamadaImpl,
};
use tendermint_rpc::HttpClient;

pub struct Sdk {
    pub faucet_sk: SecretKey,
    namada_impl: NamadaImpl<HttpClient, FsWalletUtils, FsShieldedUtils, NullIo>,
}

impl Sdk {
    pub async fn new(
        faucet_sk: SecretKey,
        rpc_client: HttpClient,
        wallet: Wallet<FsWalletUtils>,
        shielded_ctx: ShieldedContext<FsShieldedUtils>,
        io: NullIo,
    ) -> Self {
        Self {
            faucet_sk,
            namada_impl: NamadaImpl::new(rpc_client, wallet, shielded_ctx, io)
                .await
                .unwrap(),
        }
    }

    pub async fn namada_ctx(
        &self,
    ) -> &NamadaImpl<HttpClient, FsWalletUtils, FsShieldedUtils, NullIo> {
        &self.namada_impl
    }
}
