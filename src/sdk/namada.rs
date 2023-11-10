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
    wallet::{fs::FsWalletUtils, Wallet},
    NamadaImpl,
};
use tendermint_rpc::HttpClient;

pub struct Sdk<'a> {
    pub faucet_sk: SecretKey,
    pub namada: NamadaImpl<'a, HttpClient, FsWalletUtils, FsShieldedUtils, NullIo>,
}

impl<'a> Sdk<'a> {
    pub async fn new(
        chain_id: &str,
        faucet_sk: SecretKey,
        http_client: &'a HttpClient,
        wallet: &'a mut Wallet<FsWalletUtils>,
        shielded_ctx: &'a mut ShieldedContext<FsShieldedUtils>,
        io: &'a NullIo,
    ) -> Sdk<'a> {
        
        let namada = NamadaImpl::new(http_client, wallet, shielded_ctx, io)
            .await
            .expect("unable to construct Namada object")
            .chain_id(ChainId::from_str(&chain_id).unwrap());

        Self {
            faucet_sk,
            namada,
        }
    }
}
