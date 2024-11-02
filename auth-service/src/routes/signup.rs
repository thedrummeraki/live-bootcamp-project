use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{
        error::AuthAPIError,
        user::{Email, Password, User},
    },
};

use super::utils::map_user_store_error_to_api_error;

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(map_user_store_error_to_api_error)?;

    let password = Password::parse(request.password).map_err(map_user_store_error_to_api_error)?;

    let user = User::new(email, password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;

    user_store
        .add_user(user)
        .map_err(map_user_store_error_to_api_error)?;

    let response = Json(SignupResponse {
        message: "User created successfully!".into(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
