use auth_service::{
    domain::{
        data_stores::twofa::LoginAttemptId,
        user::{Email, Password},
    },
    routes::TwoFactorAuthResponse,
};
use serde_json::json;

use crate::helpers::{get_random_email, ResponseExt, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let response = app.post_verify_2fa(&"malformed body").await;

    assert_eq!(response.status_code(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let invalid_body = json!({"email": "bademail", "loginAttemptId": LoginAttemptId::default().as_ref(), "2FACode": "123456"});
    let response = app.post_verify_2fa(&invalid_body).await;
    assert_eq!(response.status_code(), 400);

    let invalid_body = json!({"email": "good@email.com", "loginAttemptId": "bad login attempt", "2FACode": "123456"});
    let response = app.post_verify_2fa(&invalid_body).await;
    assert_eq!(response.status_code(), 400);

    let invalid_body = json!({"email": "good@email.com", "loginAttemptId": LoginAttemptId::default().as_ref(), "2FACode": "badcode"});
    let response = app.post_verify_2fa(&invalid_body).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let email = Email::parse(get_random_email()).unwrap();
    let body = json!({"email": email.as_ref(), "loginAttemptId": LoginAttemptId::default().as_ref(), "2FACode": "123456"});
    let response = app.post_verify_2fa(&body).await;

    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let app = TestApp::new().await;

    let email = Email::parse(get_random_email()).unwrap();
    let password = Password::parse("password").unwrap();

    app.post_signup(
        &json!({"email": email.as_ref(), "password": password.as_ref(), "requires2FA": true}),
    )
    .await;

    let login_body = json!({"email": email.as_ref(), "password": password.as_ref()});
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status_code(), 206);

    let response_body = response.json::<TwoFactorAuthResponse>().await.unwrap();
    let TwoFactorAuthResponse {
        login_attempt_id, ..
    } = response_body;

    let (_, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .unwrap();

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status_code(), 206);

    let verify_2fa_body = json!({"email": email.as_ref(), "loginAttemptId": login_attempt_id, "2FACode": two_fa_code.as_ref()});

    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    // Make sure to assert the auth cookie gets set
    let app = TestApp::new().await;

    let email = Email::parse(get_random_email()).unwrap();
    let password = Password::parse("password").unwrap();

    app.post_signup(
        &json!({"email": email.as_ref(), "password": password.as_ref(), "requires2FA": true}),
    )
    .await;

    let login_body = json!({"email": email.as_ref(), "password": password.as_ref()});
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status_code(), 206);

    let response_body = response.json::<TwoFactorAuthResponse>().await.unwrap();
    let TwoFactorAuthResponse {
        login_attempt_id, ..
    } = response_body;

    let (_, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .unwrap();

    let verify_2fa_body = json!({"email": email.as_ref(), "loginAttemptId": login_attempt_id, "2FACode": two_fa_code.as_ref()});

    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status_code(), 200);

    let auth_cookie = response.get_auth_cookie().expect("No auth cookie found");
    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    // Make sure to assert the auth cookie gets set
    let app = TestApp::new().await;

    let email = Email::parse(get_random_email()).unwrap();
    let password = Password::parse("password").unwrap();

    app.post_signup(
        &json!({"email": email.as_ref(), "password": password.as_ref(), "requires2FA": true}),
    )
    .await;

    let login_body = json!({"email": email.as_ref(), "password": password.as_ref()});
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status_code(), 206);

    let response_body = response.json::<TwoFactorAuthResponse>().await.unwrap();
    let TwoFactorAuthResponse {
        login_attempt_id, ..
    } = response_body;

    let (_, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .unwrap();

    let verify_2fa_body = json!({"email": email.as_ref(), "loginAttemptId": login_attempt_id, "2FACode": two_fa_code.as_ref()});

    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status_code(), 200);

    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status_code(), 401);
}
