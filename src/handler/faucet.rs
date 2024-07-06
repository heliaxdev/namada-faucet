use std::collections::HashMap;

use axum::{extract::State, Json};
use axum_macros::debug_handler;
use namada_sdk::{
    address::Address, args::InputAmount, rpc, signing::default_sign, tendermint::abci::Code,
    tx::data::ResultCode, Namada,
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

    if payload.transfer.amount > state.withdraw_limit {
        return Err(FaucetError::InvalidWithdrawLimit(state.withdraw_limit).into());
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


    use namada_sdk::args::TxTransparentTransferData;

    // Create instances of TxTransparentTransferData
    let transfer_data = vec![
        TxTransparentTransferData {
            source: faucet_address,
            target: target_address,
            token: token_address.clone(),
            amount: InputAmount::Unvalidated(denominated_amount),
        },
    ];
    
    // Call the new_transparent_transfer function with the vector
    let mut transfer_tx_builder = state.sdk.new_transparent_transfer(transfer_data);
    

    transfer_tx_builder.tx.memo = Some("Transfer from faucet".to_string().as_bytes().to_vec());

    let (mut transfer_tx, signing_data) = transfer_tx_builder
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
                (r.code.eq(&ResultCode::Ok), Some(r.hash.to_string()))
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
