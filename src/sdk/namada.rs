use std::path::PathBuf;

use namada::{
    proto::Tx,
    sdk::{
        args::{self, InputAmount},
        signing::{self, SigningTxData},
        tx::ProcessTxResponse,
    },
    types::{
        address::Address,
        chain::ChainId,
        io::DefaultIo,
        key::common::{self, SecretKey},
        masp::{TransferSource, TransferTarget},
        token::{self, DenominatedAmount, NATIVE_MAX_DECIMAL_PLACES},
        transaction::GasLimit,
    },
};

use super::{client::SdkClient, masp::SdkShieldedCtx, wallet::SdkWallet};

pub struct NamadaSdk {
    pub http_client: SdkClient,
    pub wallet: SdkWallet,
    pub shielded_ctx: SdkShieldedCtx,
}

impl NamadaSdk {
    pub fn new(url: String, sk: SecretKey, nam_address: Address) -> Self {
        Self {
            http_client: SdkClient::new(url),
            wallet: SdkWallet::new(sk, nam_address),
            shielded_ctx: SdkShieldedCtx::default(),
        }
    }

    pub fn get_secret_key(&mut self) -> SecretKey {
        self.wallet.wallet.find_key("my_faucet", None).unwrap()
    }

    pub fn get_address(&self, alias: String) -> Address {
        self.wallet.wallet.find_address(alias).unwrap().clone()
    }

    pub fn default_args(
        &self,
        chain_id: String,
        signing_keys: Vec<SecretKey>,
        fee_payer: Option<SecretKey>,
        fee_token: Address,
    ) -> args::Tx {
        args::Tx {
            dry_run: false,
            dump_tx: false,
            output_folder: None,
            force: false,
            broadcast_only: true,
            ledger_address: (),
            initialized_account_alias: None,
            wallet_alias_force: false,
            wrapper_fee_payer: fee_payer,
            fee_amount: Some(InputAmount::Validated(token::DenominatedAmount {
                amount: token::Amount::from_u64(0),
                denom: NATIVE_MAX_DECIMAL_PLACES.into(),
            })),
            fee_token,
            gas_limit: GasLimit::from(20_000),
            expiration: None,
            chain_id: Some(ChainId(chain_id)),
            signing_keys,
            signatures: Vec::default(),
            tx_reveal_code_path: PathBuf::from("tx_reveal_pk.wasm"),
            verification_key: None,
            password: None,
            dry_run_wrapper: false,
            fee_unshield: None,
            disposable_signing_key: false,
        }
    }

    pub async fn compute_signing_data(
        &mut self,
        owner: Option<Address>,
        default_signer: Option<Address>,
        args: &args::Tx,
    ) -> SigningTxData {
        signing::aux_signing_data::<_, _, DefaultIo>(
            &self.http_client,
            &mut self.wallet.wallet,
            args,
            owner,
            default_signer,
        )
        .await
        .unwrap()
    }

    pub fn sign_tx(&mut self, tx: &mut Tx, signing_data: SigningTxData, args: &args::Tx) {
        signing::sign_tx(&mut self.wallet.wallet, args, tx, signing_data).unwrap();
    }

    pub async fn process_tx(&mut self, tx: Tx, args: &args::Tx) -> ProcessTxResponse {
        namada::sdk::tx::process_tx::<_, _, DefaultIo>(
            &self.http_client,
            &mut self.wallet.wallet,
            args,
            tx,
        )
        .await
        .unwrap()
    }

    pub fn build_transfer_args(
        &self,
        source: Address,
        target: Address,
        token: Address,
        amount: u64,
        native_token: Address,
        args: args::Tx,
    ) -> args::TxTransfer {
        args::TxTransfer {
            tx: args,
            source: TransferSource::Address(source),
            target: TransferTarget::Address(target),
            token,
            amount: InputAmount::Validated(DenominatedAmount {
                amount: token::Amount::from_u64(amount),
                denom: NATIVE_MAX_DECIMAL_PLACES.into(),
            }),
            native_token,
            tx_code_path: PathBuf::from("tx_transfer.wasm"),
        }
    }

    pub async fn build_transfer_tx(
        &mut self,
        args: args::TxTransfer,
        fee_payer: common::PublicKey,
    ) -> Tx {
        let (tx, _) = namada::sdk::tx::build_transfer::<_, _, _, DefaultIo>(
            &self.http_client,
            &mut self.wallet.wallet,
            &mut self.shielded_ctx.shielded_context,
            args,
            fee_payer,
        )
        .await
        .unwrap();

        tx
    }
}
