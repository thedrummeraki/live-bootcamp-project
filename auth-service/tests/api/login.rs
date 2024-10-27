use auth_service::{
    domain::{data_stores::twofa::LoginAttemptId, user::Email},
    routes::TwoFactorAuthResponse,
};
use serde_json::json;

use crate::helpers::{get_random_email, ResponseExt, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let malformed_body = json!({});
    let response = app.post_login(&malformed_body).await;

    assert_eq!(response.status_code(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;

    let bodies: &[serde_json::Value] = &[
        // serde_json::Value::String("".into()),
        json!({"email": "valid@email.com", "password": "pass"}),
        json!({"email": "invalid", "password": "password"}),
    ];

    for body_ref in bodies {
        let response = app.post_login(body_ref).await;
        assert_eq!(response.status_code(), 400);
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;

    let response = app
        .post_login(&json!({"email": "unknown@email.com", "password": "password"}))
        .await;

    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status_code(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status_code(), 200);

    let auth_cookie = response.get_auth_cookie().expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status_code(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status_code(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    assert_eq!(response_body.message, "2FA required".to_owned());

    let two_fa_store = app.two_fa_code_store.read().await;
    let (stored_login_attempt_id, _) = two_fa_store
        .get_code(&Email::parse(random_email).unwrap())
        .await
        .expect("Code for email not properly stored!");

    let returned_login_attempt_id = LoginAttemptId::parse(response_body.login_attempt_id)
        .expect("Code for email not properly formated!");

    assert_eq!(stored_login_attempt_id, returned_login_attempt_id)
}
