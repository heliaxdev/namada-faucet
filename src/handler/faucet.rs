use axum::{extract::State, Json};
use axum_macros::debug_handler;
use namada_sdk::{
    args::InputAmount,
    core::types::{
        address::Address,
        masp::{TransferSource, TransferTarget},
    },
    rpc,
    signing::default_sign,
    tendermint::abci::Code,
    Namada,
};

use crate::{
    dto::faucet::{FaucetRequestDto, FaucetResponseDto, FaucetResponseStatusDto},
    error::{api::ApiError, faucet::FaucetError, validate::ValidatedRequest},
    repository::faucet::FaucetRepositoryTrait,
    state::faucet::FaucetState,
};

pub async fn request_challenge(
    State(mut state): State<FaucetState>,
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

#[debug_handler]
pub async fn request_transfer(
    State(mut state): State<FaucetState>,
    ValidatedRequest(payload): ValidatedRequest<FaucetRequestDto>,
) -> Result<Json<FaucetResponseStatusDto>, ApiError> {
    let auth_key: String = state.auth_key.clone();

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

    if state.faucet_repo.contains(&payload.challenge).await {
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

    let faucet_address = state.faucet_address.clone();

    let denominated_amount = rpc::denominate_amount(
        state.sdk.client(),
        state.sdk.io(),
        &token_address,
        payload.transfer.amount.into(),
    )
    .await;

    let mut transfer_tx_builder = state.sdk.new_transfer(
        TransferSource::Address(faucet_address),
        TransferTarget::Address(target_address),
        token_address.clone(),
        InputAmount::Unvalidated(denominated_amount),
    );

    let (mut transfer_tx, signing_data, _epoch) = transfer_tx_builder
        .build(&*state.sdk)
        .await
        .expect("unable to build transfer");
    state
        .sdk
        .sign(
            &mut transfer_tx,
            &transfer_tx_builder.tx,
            signing_data,
            default_sign,
            (),
        )
        .await
        .expect("unable to sign reveal pk tx");

    let process_tx_response = state.sdk.submit(transfer_tx, &transfer_tx_builder.tx).await;

    let (transfer_result, tx_hash) = if let Ok(response) = process_tx_response {
        match response {
            namada_sdk::tx::ProcessTxResponse::Applied(r) => (r.code.eq(&"0"), Some(r.hash)),
            namada_sdk::tx::ProcessTxResponse::Broadcast(r) => {
                (r.code.eq(&Code::Ok), Some(r.hash.to_string()))
            }
            _ => (false, None),
        }
    } else {
        (false, None)
    };

    if transfer_result {
        state.faucet_repo.add(payload.challenge.clone()).await;
    }

    let response = FaucetResponseStatusDto {
        token: payload.transfer.token.clone(),
        amount: payload.transfer.amount,
        target: payload.transfer.target.clone(),
        sent: transfer_result,
        tx_hash,
    };

    Ok(Json(response))
}
