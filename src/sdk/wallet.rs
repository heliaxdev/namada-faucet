use std::path::PathBuf;

use namada::{ledger::wallet::{alias::Alias, ConfirmationResponse, GenRestoreKeyError, WalletUtils, Store, Wallet, StoredKeypair}, types::key::{common::SecretKey, PublicKeyHash}};
use rand::rngs::OsRng;

pub struct SdkWallet {
    pub wallet: Wallet<SdkWalletUtils>
}

impl SdkWallet {
    pub fn new(sk: SecretKey) -> Self {
        let store = Store::default();
        let mut wallet = Wallet::new(PathBuf::new(), store);
        let stored_keypair = StoredKeypair::Raw(sk.clone());
        let pk_hash = PublicKeyHash::from(&sk.to_public());
        let alias = "fake_faucet".to_string();
        wallet.insert_keypair(alias, stored_keypair, pk_hash, true);
        Self {
            wallet
        }
    }
}

pub struct SdkWalletUtils {}

impl WalletUtils for SdkWalletUtils {
    type Storage = PathBuf;

    type Rng = OsRng;

    fn read_decryption_password() -> zeroize::Zeroizing<std::string::String> {
        panic!("attempted to prompt for password in non-interactive mode");
    }

    fn read_encryption_password() -> zeroize::Zeroizing<std::string::String> {
        panic!("attempted to prompt for password in non-interactive mode");
    }

    fn read_alias(_prompt_msg: &str) -> std::string::String {
        panic!("attempted to prompt for alias in non-interactive mode");
    }

    fn read_mnemonic_code() -> std::result::Result<namada::bip39::Mnemonic, GenRestoreKeyError> {
        panic!("attempted to prompt for mnemonic in non-interactive mode");
    }

    fn read_mnemonic_passphrase(_confirm: bool) -> zeroize::Zeroizing<std::string::String> {
        panic!("attempted to prompt for mnemonic in non-interactive mode");
    }

    fn show_overwrite_confirmation(
        _alias: &Alias,
        _alias_for: &str,
    ) -> namada::ledger::wallet::store::ConfirmationResponse {
        // Automatically replace aliases in non-interactive mode
        ConfirmationResponse::Replace
    }
}