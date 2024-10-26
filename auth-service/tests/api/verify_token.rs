use auth_service::{domain::user::Email, utils::auth::generate_auth_cookie};
use serde_json::json;

use crate::helpers::{ResponseExt, TestApp};

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
