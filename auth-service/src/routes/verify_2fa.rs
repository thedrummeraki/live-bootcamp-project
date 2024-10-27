use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{
        data_stores::twofa::{LoginAttemptId, TwoFACode},
        error::AuthAPIError,
        user::Email,
    },
    utils::auth::generate_auth_cookie,
};

use super::utils::{map_string_error_to_bad_input_error, map_user_store_error_to_api_error};

#[derive(Serialize, Clone, Debug, PartialEq, Deserialize)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_code: String,
}

pub async fn verify_2fa(
    jar: CookieJar,
    State(state): State<AppState>,
    Json(verify_2fa_token): Json<Verify2FARequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email =
        Email::parse(verify_2fa_token.email.clone()).map_err(map_user_store_error_to_api_error)?;
    let login_attempt_id = LoginAttemptId::parse(verify_2fa_token.login_attempt_id.clone())
        .map_err(map_string_error_to_bad_input_error)?;

    let two_fa_code = TwoFACode::parse(verify_2fa_token.two_fa_code.clone())
        .map_err(map_string_error_to_bad_input_error)?;

    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    let code_tuple = two_fa_code_store
        .get_code(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    if code_tuple.0 != login_attempt_id || code_tuple.1 != two_fa_code {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let cookie = generate_auth_cookie(&email).map_err(AuthAPIError::GenerateTokenError)?;
    let jar = jar.add(cookie);

    two_fa_code_store
        .remove_code(&email)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    Ok((jar, StatusCode::OK.into_response()))
}
