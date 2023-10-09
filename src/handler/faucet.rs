use axum::{extract::State, Json};
use axum_macros::debug_handler;
use namada::{tendermint::abci::Code, types::address::Address};

use crate::{
    dto::faucet::{FaucetRequestDto, FaucetResponseDto, FaucetResponseStatusDto},
    error::{api::ApiError, faucet::FaucetError, validate::ValidatedRequest},
    repository::faucet::FaucetRepositoryTrait,
    state::faucet::FaucetState,
};

pub async fn request_challenge(
    State(mut state): State<FaucetState>,
) -> Result<Json<FaucetResponseDto>, ApiError> {
    let faucet_request = state
        .faucet_service
        .generate_faucet_request(state.auth_key)
        .await?;
    let response = FaucetResponseDto::from(faucet_request);

    Ok(Json(response))
}

#[debug_handler]
pub async fn request_transfer(
    State(mut state): State<FaucetState>,
    ValidatedRequest(payload): ValidatedRequest<FaucetRequestDto>,
) -> Result<Json<FaucetResponseStatusDto>, ApiError> {
    let auth_key: String = state.auth_key.clone();
    let chain_id = state.chain_id.clone();

    let token_address = Address::decode(payload.transfer.token.clone());
    let token_address = if let Ok(address) = token_address {
        address
    } else {
        return Err(FaucetError::InvalidAddress.into());
    };
    let target_address = Address::decode(payload.transfer.target.clone());
    let target_address = if let Ok(address) = target_address {
        address
    } else {
        return Err(FaucetError::InvalidAddress.into());
    };

    if state.faucet_repo.contains(&payload.challenge) {
        return Err(FaucetError::DuplicateChallenge.into());
    }

    let is_valid_proof =
        state
            .faucet_service
            .verify_tag(&auth_key, &payload.challenge, &payload.tag);
    if !is_valid_proof {
        return Err(FaucetError::InvalidProof.into());
    }

    let is_valid_pow =
        state
            .faucet_service
            .verify_pow(&payload.challenge, &payload.solution, state.difficulty);
    if !is_valid_pow {
        return Err(FaucetError::InvalidPoW.into());
    }

    let mut locked_sdk = state.sdk.lock().await;

    let sk = locked_sdk
        .get_secret_key()
        .map_err(|e| FaucetError::SdkError(e.to_string()))?;
    let nam_address = locked_sdk
        .get_address("nam".to_string())
        .map_err(|e| FaucetError::SdkError(e.to_string()))?;

    let owner = Address::from(&sk.to_public());
    let tx_args = locked_sdk.default_args(chain_id, vec![sk], None, nam_address.clone());
    let signing_data = locked_sdk
        .compute_signing_data(Some(owner.clone()), None, &tx_args)
        .await
        .map_err(|e| FaucetError::SdkError(e.to_string()))?;
    let tx_data = locked_sdk
        .build_transfer_args(
            owner,
            target_address,
            token_address,
            payload.transfer.amount,
            nam_address,
            tx_args.clone(),
        )
        .await
        .map_err(|e| FaucetError::SdkError(e.to_string()))?;
    let mut tx = locked_sdk
        .build_transfer_tx(tx_data, signing_data.fee_payer.clone())
        .await
        .map_err(|e| FaucetError::SdkError(e.to_string()))?;
    locked_sdk.sign_tx(&mut tx, signing_data, &tx_args);
    let process_tx_response = locked_sdk
        .process_tx(tx, &tx_args)
        .await
        .map_err(|e| FaucetError::SdkError(e.to_string()))?;
    drop(locked_sdk);

    let transfer_result = match process_tx_response {
        namada::sdk::tx::ProcessTxResponse::Applied(r) => r.code.eq(&"0"),
        namada::sdk::tx::ProcessTxResponse::Broadcast(r) => r.code.eq(&Code::Ok),
        _ => false,
    };

    if transfer_result {
        state.faucet_repo.add(payload.challenge.clone());
    }

    let response = FaucetResponseStatusDto {
        token: payload.transfer.token.clone(),
        amount: payload.transfer.amount,
        target: payload.transfer.target.clone(),
        sent: transfer_result,
    };

    Ok(Json(response))
}
