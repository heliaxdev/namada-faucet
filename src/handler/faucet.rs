use axum::{extract::State, Json};
use axum_macros::debug_handler;
use namada_sdk::{Namada, core::types::{address::Address, masp::{TransferSource, TransferTarget}, token::{DenominatedAmount, self}}, tendermint::abci::Code, args::InputAmount};

use crate::{
    dto::faucet::{FaucetRequestDto, FaucetResponseDto, FaucetResponseStatusDto},
    error::{api::ApiError, faucet::FaucetError, validate::ValidatedRequest},
    repository::faucet::FaucetRepositoryTrait,
    state::faucet::FaucetState,
};

pub async fn request_challenge<'a>(
    State(mut state): State<FaucetState<'a>>,
) -> Result<Json<FaucetResponseDto>, ApiError> {
    let current_timestamp = chrono::offset::Utc::now().timestamp();

    if current_timestamp < state.chain_start {
        return Err(FaucetError::ChainNotStarted.into());
    }

    let faucet_request = state
        .faucet_service
        .generate_faucet_request(state.auth_key)
        .await?;
    let response = FaucetResponseDto::from(faucet_request);

    Ok(Json(response))
}

// #[debug_handler]
pub async fn request_transfer<'a>(
    State(mut state): State<FaucetState<'a>>,
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

    let sdk = state.sdk.lock().unwrap();

    let nam_address = sdk.namada.native_token();
    let faucet_key = sdk.faucet_sk.clone();
    let faucet_pk = faucet_key.to_public();
    let faucet_address = Address::from(&faucet_pk);

    let mut transfer_tx_builder = sdk.namada.new_transfer(
        TransferSource::Address(faucet_address),
        TransferTarget::Address(target_address),
        token_address.clone(),
        InputAmount::Unvalidated(DenominatedAmount::native(token::Amount::from_u64(
            payload.transfer.amount,
        ))),
    );

    // let (mut transfer_tx, signing_data, _epoch) = transfer_tx_builder
    //         .build(&sdk.namada)
    //         .await
    //         .expect("unable to build transfer");
    //     sdk.namada
    //         .sign(&mut transfer_tx, &transfer_tx_builder.tx, signing_data)
    //         .await
    //         .expect("unable to sign reveal pk tx");
    //     let process_tx_response = sdk
    //         .namada
    //         .submit(transfer_tx, &transfer_tx_builder.tx)
    //         .await;

    // let transfer_result = if let Ok(response) = process_tx_response {
    //     match response {
    //         namada_sdk::tx::ProcessTxResponse::Applied(r) => r.code.eq(&"0"),
    //         namada_sdk::tx::ProcessTxResponse::Broadcast(r) => r.code.eq(&Code::Ok),    
    //         _ => false,
    //     }
    // } else {
    //     false
    // };

    // if transfer_result {
    //     state.faucet_repo.add(payload.challenge.clone());
    // }

    let response = FaucetResponseStatusDto {
        token: payload.transfer.token.clone(),
        amount: payload.transfer.amount,
        target: payload.transfer.target.clone(),
        sent: false,
    };

    Ok(Json(response))
}
