use auth_service::{domain::user::Email, utils::auth::generate_auth_cookie};
use serde_json::json;

use crate::helpers::{get_random_email, ResponseExt, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let response = app.post_verify_token(&"invalidtoken").await;

    assert_eq!(response.status_code(), 422)
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;
    let email = Email::parse("valid@email.com").unwrap();
    let cookie = generate_auth_cookie(&email).unwrap();
    let token = cookie.value().to_owned();

    let response = app.post_verify_token(&json!({"token": token})).await;

    assert_eq!(response.status_code(), 200)
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    let response = app
        .post_verify_token(&json!({"token": "invalidtoken"}))
        .await;

    assert_eq!(response.status_code(), 401)
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;
    let email = Email::parse(get_random_email()).unwrap();

    let signup_body =
        json!({"email": email.as_ref(), "password": "password", "requires2FA": false});

    app.post_signup(&signup_body).await;

    let login_body = json!({"email": email.as_ref(), "password": "password"});
    let response = app.post_login(&login_body).await;

    // get the token from the login response, logout...
    let token = response.get_auth_cookie().unwrap().value().to_owned();
    app.post_logout().await;

    // then ensure that the token is no longer valid.
    let response = app.post_verify_token(&json!({"token": token})).await;
    assert_eq!(response.status_code(), 401)
}
