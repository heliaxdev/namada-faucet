use std::collections::HashMap;
use std::time::Instant;

use axum::extract::Path;
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use namada_sdk::types::string_encoding::Format;
use namada_sdk::{
    args::InputAmount,
    rpc,
    signing::default_sign,
    tendermint::abci::Code,
    tx::data::ResultCode,
    types::{
        address::Address,
        key::{common, SigScheme},
        masp::{TransferSource, TransferTarget},
    },
    Namada,
};

use crate::{
    dto::faucet::{
        FaucetRequestDto, FaucetResponseDto, FaucetResponseStatusDto, FaucetSettingResponse,
    },
    error::{api::ApiError, faucet::FaucetError, validate::ValidatedRequest},
    repository::faucet::FaucetRepositoryTrait,
    state::faucet::FaucetState,
};

pub async fn faucet_settings(
    State(state): State<FaucetState>,
) -> Result<Json<FaucetSettingResponse>, ApiError> {
    let nam_token_address = rpc::query_native_token(state.sdk.client()).await.unwrap();

    let response = FaucetSettingResponse {
        difficulty: state.difficulty,
        chain_id: state.chain_id,
        start_at: state.chain_start,
        withdraw_limit: state.withdraw_limit,
        tokens_alias_to_address: HashMap::from([(
            "NAM".to_string(),
            nam_token_address.to_string(),
        )]),
    };

    Ok(Json(response))
}

pub async fn request_challenge(
    State(mut state): State<FaucetState>,
    Path(player_id): Path<String>,
) -> Result<Json<FaucetResponseDto>, ApiError> {
    let is_player = match reqwest::get(format!(
        "https://{}/api/v1/player/exists/{}",
        state.webserver_host, player_id
    ))
    .await
    .map(|response| response.status().is_success())
    {
        Ok(is_success) if is_success => true,
        _ => false,
    };
    if !is_player {
        return Err(FaucetError::NotPlayer(player_id).into());
    }

    let now = Instant::now();
    let too_many_requests = 'result: {
        let Some(last_request_instant) = state.last_requests.get(&player_id) else {
            break 'result false;
        };
        let elapsed_request_time = now.duration_since(*last_request_instant);
        elapsed_request_time <= state.request_frequency
    };

    if too_many_requests {
        return Err(FaucetError::TooManyRequests.into());
    }
    state.last_requests.insert(player_id.clone(), now);

    let faucet_request = state
        .faucet_service
        .generate_faucet_request(state.auth_key, player_id)
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

    if payload.transfer.amount > state.withdraw_limit {
        return Err(FaucetError::InvalidWithdrawLimit(state.withdraw_limit).into());
    }

    let player_id_pk: common::PublicKey = if let Ok(pk) = payload.player_id.parse() {
        pk
    } else {
        return Err(FaucetError::InvalidPublicKey.into());
    };

    let challenge_signature = if let Ok(hex_decoded_sig) = hex::decode(payload.challenge_signature)
    {
        if let Ok(sig) = common::Signature::decode_bytes(&hex_decoded_sig) {
            sig
        } else {
            return Err(FaucetError::InvalidSignature.into());
        }
    } else {
        return Err(FaucetError::InvalidSignature.into());
    };

    if common::SigScheme::verify_signature(
        &player_id_pk,
        // NOTE: signing over the hex encoded challenge data
        &payload.challenge.as_bytes(),
        &challenge_signature,
    )
    .is_err()
    {
        return Err(FaucetError::InvalidSignature.into());
    }

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

    let is_valid_proof = state.faucet_service.verify_tag(
        &auth_key,
        &payload.challenge,
        &payload.player_id,
        &payload.tag,
    );
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

    if let Ok(balance) =
        rpc::get_token_balance(state.sdk.client(), &token_address, &faucet_address).await
    {
        if balance < payload.transfer.amount.into() {
            return Err(FaucetError::FaucetOutOfBalance.into());
        }
    } else {
        return Err(FaucetError::SdkError("Can't query faucet balance".to_string()).into());
    }

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

    transfer_tx_builder.tx.memo = Some("Transfer from faucet".to_string().as_bytes().to_vec());

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
            namada_sdk::tx::ProcessTxResponse::Applied(r) => {
                (r.code.eq(&ResultCode::Ok), Some(r.hash))
            }
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
