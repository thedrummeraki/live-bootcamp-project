use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    app_state::{AppState, BannedTokenStoreType},
    domain::{error::AuthAPIError, user::Email},
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let banned_token_store = state.banned_token_store.clone();

    match validate_token_from_cookie_jar(jar.clone(), banned_token_store).await {
        Err(error) => (jar, Err(error)),
        Ok((cookie, token, email)) => {
            let jar = jar.remove(cookie);
            let mut banned_token_store = state.banned_token_store.write().await;
            banned_token_store.add(&email, &token);

            (jar, Ok(StatusCode::OK.into_response()))
        }
    }
}

async fn validate_token_from_cookie_jar(
    jar: CookieJar,
    banned_token_store: BannedTokenStoreType,
) -> Result<(Cookie<'static>, String, Email), AuthAPIError> {
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;
    let token = cookie.value().to_owned();

    let claims = validate_token(&token, banned_token_store)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    let email = Email::parse(claims.sub).expect("Could not parse email address from store.");

    Ok((cookie.to_owned(), token, email))
}
