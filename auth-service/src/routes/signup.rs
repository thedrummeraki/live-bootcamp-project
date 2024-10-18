use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{error::AuthAPIError, user::User},
    services::hashmap_user_store::UserStoreError,
};

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let user = User::new(request).ok_or(AuthAPIError::InvalidCredentials)?;
    let mut user_store = state.user_store.write().await;
    if let Err(error) = user_store.add_user(user) {
        match error {
            UserStoreError::UserAlreadyExists => return Err(AuthAPIError::UserAlreadyExists),
            UserStoreError::InvalidCredentials => return Err(AuthAPIError::InvalidCredentials),
            _ => return Err(AuthAPIError::UnexpectedError),
        }
    }

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
