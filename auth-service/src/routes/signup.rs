use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{
        data_stores::user::UserStoreError,
        error::AuthAPIError,
        user::{Email, Password, User},
    },
};

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email =
        Email::parse(request.email.to_owned()).map_err(map_user_store_error_to_api_error)?;

    let password =
        Password::parse(request.password.to_owned()).map_err(map_user_store_error_to_api_error)?;

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

fn map_user_store_error_to_api_error(user_error: UserStoreError) -> AuthAPIError {
    match user_error {
        UserStoreError::InvalidCredentials(details) => AuthAPIError::InvalidCredentials(details),
        UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
        _ => AuthAPIError::UnexpectedError,
    }
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
