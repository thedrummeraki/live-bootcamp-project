use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{
        data_stores::user::UserStoreError,
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

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(login_request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // let email = Email::parse(login_request.email);
    let cloned_jar = jar.clone();
    let user_validation_result = validate_login_request(state, login_request)
        .await
        .map(|email| match generate_auth_cookie(&email) {
            Ok(auth_cookie) => {
                let updated_jar = cloned_jar.add(auth_cookie);
                updated_jar
            }
            Err(_) => cloned_jar,
        });

    match user_validation_result {
        Err(e) => (jar, Err(e)),
        Ok(jar) => (jar, Ok(StatusCode::OK.into_response())),
    }
}

async fn validate_login_request(
    state: AppState,
    login_request: LoginRequest,
) -> Result<Email, AuthAPIError> {
    let LoginRequest { email, password } = login_request;

    let email = Email::parse(email).map_err(map_user_store_error_to_api_error)?;
    let password = Password::parse(password).map_err(map_user_store_error_to_api_error)?;

    let user_store = state.user_store.write().await;

    user_store
        .validate_user(email.to_owned(), password)
        .map_err(map_user_store_error_to_api_error)?;

    if let Err(error) = user_store.get_user(email.to_owned()) {
        match error {
            UserStoreError::InvalidCredentials(_) => {
                return Err(AuthAPIError::IncorrectCredentials)
            }
            UserStoreError::UserNotFound => return Err(AuthAPIError::IncorrectCredentials),
            _ => return Err(AuthAPIError::UnexpectedError),
        };
    }

    Ok(email)
}
