use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, utils::auth::validate_token};

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyTokenResquest {
    token: String,
}

pub async fn verify_token(
    State(state): State<AppState>,
    Json(body): Json<VerifyTokenResquest>,
) -> impl IntoResponse {
    let VerifyTokenResquest { token } = body;
    let banned_token_store = state.banned_token_store.clone();

    match validate_token(&token, banned_token_store).await {
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
        Ok(_) => StatusCode::OK.into_response(),
    }
}
