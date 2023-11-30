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
            namada_impl: NamadaImpl::new(
            rpc_client,
            wallet,
            shielded_ctx,
            io,
        )
        .await
        .unwrap()
        }
    }

    pub async fn namada_ctx(
        &self,
    ) -> &NamadaImpl<HttpClient, FsWalletUtils, FsShieldedUtils, NullIo> {
        &self.namada_impl
    }
}
