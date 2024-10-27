use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{
        data_stores::twofa::{LoginAttemptId, TwoFACode},
        error::AuthAPIError,
        user::{Email, Password},
    },
    utils::auth::generate_auth_cookie,
};

use super::utils::map_user_store_error_to_api_error;

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl LoginRequest {
    pub fn parse_email(&self) -> Result<Email, AuthAPIError> {
        Email::parse(self.email.clone()).map_err(map_user_store_error_to_api_error)
    }

    pub fn parse_password(&self) -> Result<Password, AuthAPIError> {
        Password::parse(self.password.clone()).map_err(map_user_store_error_to_api_error)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(login_request): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let user_store = state.user_store.write().await;

    let email = login_request.parse_email()?;
    let password = login_request.parse_password()?;

    user_store
        .validate_user(email.to_owned(), password)
        .map_err(map_user_store_error_to_api_error)?;

    let user = user_store
        .get_user(email.to_owned())
        .map_err(map_user_store_error_to_api_error)?;

    if user.requires_2fa {
        handle_2fa(&user.email, &state, jar).await
    } else {
        handle_regular(&user.email, jar).await
    }
}

async fn handle_regular(
    email: &Email,
    jar: CookieJar,
) -> Result<(CookieJar, (StatusCode, Json<LoginResponse>)), AuthAPIError> {
    let cookie = generate_auth_cookie(&email).map_err(AuthAPIError::GenerateTokenError)?;
    let jar = jar.add(cookie);

    Ok((jar, (StatusCode::OK, Json(LoginResponse::RegularAuth))))
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> Result<(CookieJar, (StatusCode, Json<LoginResponse>)), AuthAPIError> {
    let mut two_fa_code_store = state.two_fa_code_store.write().await;

    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    two_fa_code_store
        .add_code(email.to_owned(), login_attempt_id.clone(), two_fa_code)
        .await
        .map_err(|e| {
            println!("[ERROR] Unexpected error while trying to store 2FA code: {e:?}");
            AuthAPIError::UnexpectedError
        })?;

    let response = TwoFactorAuthResponse {
        login_attempt_id: login_attempt_id.as_ref().into(),
        message: "2FA required".into(),
    };

    Ok((
        jar,
        (
            StatusCode::PARTIAL_CONTENT,
            Json(LoginResponse::TwoFactorAuth(response)),
        ),
    ))
}
