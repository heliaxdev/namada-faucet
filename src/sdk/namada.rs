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
    rpc_client: HttpClient,
    wallet: Wallet<FsWalletUtils>,
    shielded_ctx: ShieldedContext<FsShieldedUtils>,
    io: NullIo,
}

impl Sdk {
    pub fn new(
        faucet_sk: SecretKey,
        rpc_client: HttpClient,
        wallet: Wallet<FsWalletUtils>,
        shielded_ctx: ShieldedContext<FsShieldedUtils>,
        io: NullIo,
    ) -> Self {
        Self {
            faucet_sk,
            rpc_client,
            wallet,
            shielded_ctx,
            io: NullIo,
        }
    }

    pub async fn namada_ctx(
        &mut self,
    ) -> NamadaImpl<'_, HttpClient, FsWalletUtils, FsShieldedUtils, NullIo> {
        NamadaImpl::new(
            &self.rpc_client,
            &mut self.wallet,
            &mut self.shielded_ctx,
            &self.io,
        )
        .await
        .unwrap()
    }
}
