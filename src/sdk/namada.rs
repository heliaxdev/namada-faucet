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
        token::{self, DenominatedAmount, Denomination, NATIVE_MAX_DECIMAL_PLACES},
        transaction::GasLimit,
    },
};

use super::{client::SdkClient, error::NamadaError, masp::SdkShieldedCtx, wallet::SdkWallet};

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

    pub fn get_secret_key(&mut self) -> Result<SecretKey, NamadaError> {
        let sk = self
            .wallet
            .wallet
            .find_key("my_faucet", None)
            .map_err(|_e| NamadaError::InvalidSecretKey)?;

        Ok(sk)
    }

    pub fn get_address(&self, alias: String) -> Result<Address, NamadaError> {
        let address = self.wallet.wallet.find_address(alias);

        if let Some(address) = address {
            Ok(address.clone())
        } else {
            Err(NamadaError::ConversionInvalid(
                "Can't convert string to address".to_string(),
            ))
        }
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
    ) -> Result<SigningTxData, NamadaError> {
        let signing_data = signing::aux_signing_data::<_, _, DefaultIo>(
            &self.http_client,
            &mut self.wallet.wallet,
            args,
            owner,
            default_signer,
        )
        .await
        .map_err(|e| NamadaError::SigningDataInvalid(e.to_string()))?;

        Ok(signing_data)
    }

    pub fn sign_tx(&mut self, tx: &mut Tx, signing_data: SigningTxData, args: &args::Tx) {
        signing::sign_tx(&mut self.wallet.wallet, args, tx, signing_data).unwrap();
    }

    pub async fn process_tx(
        &mut self,
        tx: Tx,
        args: &args::Tx,
    ) -> Result<ProcessTxResponse, NamadaError> {
        let broadcast_error = namada::sdk::tx::process_tx::<_, _, DefaultIo>(
            &self.http_client,
            &mut self.wallet.wallet,
            args,
            tx,
        )
        .await
        .map_err(|e| NamadaError::TxBroadcastingInvalid(e.to_string()))?;

        Ok(broadcast_error)
    }

    pub async fn build_transfer_args(
        &self,
        source: Address,
        target: Address,
        token: Address,
        amount: u64,
        native_token: Address,
        args: args::Tx,
    ) -> Result<args::TxTransfer, NamadaError> {
        let unvalidated_amount = InputAmount::Unvalidated(DenominatedAmount {
            amount: token::Amount::from_u64(amount),
            denom: Denomination(0),
        });
        let denominated_amount = namada::sdk::rpc::validate_amount::<_, DefaultIo>(
            &self.http_client,
            unvalidated_amount,
            &token,
            true,
        )
        .await
        .map_err(|e| NamadaError::DenominationInvalid(e.to_string()))?;

        let tranfer_args = args::TxTransfer {
            tx: args,
            source: TransferSource::Address(source),
            target: TransferTarget::Address(target),
            token,
            amount: InputAmount::Validated(denominated_amount),
            native_token,
            tx_code_path: PathBuf::from("tx_transfer.wasm"),
        };

        Ok(tranfer_args)
    }

    pub async fn build_transfer_tx(
        &mut self,
        args: args::TxTransfer,
        fee_payer: common::PublicKey,
    ) -> Result<Tx, NamadaError> {
        let (tx, _) = namada::sdk::tx::build_transfer::<_, _, _, DefaultIo>(
            &self.http_client,
            &mut self.wallet.wallet,
            &mut self.shielded_ctx.shielded_context,
            args,
            fee_payer,
        )
        .await
        .map_err(|e| NamadaError::TxBuildingInvalid(e.to_string()))?;

        Ok(tx)
    }
}
