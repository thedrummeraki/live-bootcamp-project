use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{
        data_stores::user::UserStoreError,
        error::AuthAPIError,
        user::{Email, Password},
    },
};

use super::utils::map_user_store_error_to_api_error;

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // let email = Email::parse(login_request.email);
    let LoginRequest { email, password } = login_request;
    let email = Email::parse(email).map_err(map_user_store_error_to_api_error)?;
    let password = Password::parse(password).map_err(map_user_store_error_to_api_error)?;

    let user_store = state.user_store.read().await;

    user_store
        .validate_user(email.to_owned(), password)
        .map_err(map_user_store_error_to_api_error)?;

    if let Err(error) = user_store.get_user(email) {
        match error {
            UserStoreError::InvalidCredentials(_) => {
                return Err(AuthAPIError::IncorrectCredentials)
            }
            UserStoreError::UserNotFound => return Err(AuthAPIError::IncorrectCredentials),
            _ => return Err(AuthAPIError::UnexpectedError),
        }
    }

    Ok(StatusCode::OK.into_response())
}
