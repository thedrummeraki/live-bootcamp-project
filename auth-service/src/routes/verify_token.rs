use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::utils::auth::validate_token;

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyTokenResquest {
    token: String,
}

pub async fn verify_token(Json(body): Json<VerifyTokenResquest>) -> impl IntoResponse {
    let VerifyTokenResquest { token } = body;

    match validate_token(&token).await {
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
        Ok(_) => StatusCode::OK.into_response(),
    }
}
