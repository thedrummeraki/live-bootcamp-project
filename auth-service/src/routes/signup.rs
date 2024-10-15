use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::user::User};

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let user = User::new(request);
    let mut user_store = state.user_store.write().await;
    user_store.add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "User created successfully!".into(),
    });

    (StatusCode::CREATED, response)
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
