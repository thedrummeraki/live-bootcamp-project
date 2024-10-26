use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    domain::error::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    match validate_token_from_cookie_jar(jar.clone()).await {
        Err(error) => (jar, Err(error)),
        Ok(cookie) => {
            let jar = jar.remove(cookie);
            (jar, Ok(StatusCode::OK.into_response()))
        }
    }
}

async fn validate_token_from_cookie_jar(jar: CookieJar) -> Result<Cookie<'static>, AuthAPIError> {
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;
    let token = cookie.value().to_owned();

    validate_token(&token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    Ok(cookie.to_owned())
}
