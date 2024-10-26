use auth_service::{
    domain::user::Email,
    utils::{auth::generate_auth_cookie, constants::JWT_COOKIE_NAME},
};
use reqwest::Url;

use crate::helpers::{get_random_email, ResponseExt, TestApp};

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    let response = app.post_logout().await;

    assert_eq!(response.status_code(), 400)
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status_code(), 401)
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;
    let random_email = Email::parse(get_random_email()).unwrap();
    let cookie = generate_auth_cookie(&random_email).unwrap();
    app.cookie_jar.add_cookie_str(
        &cookie.to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_eq!(response.status_code(), 200);

    let token = cookie.value().to_string();
    let banned_token_state = app.banned_token_store.read().await.verify(&token);
    assert!(banned_token_state.exists());
    assert_eq!(banned_token_state.email().unwrap(), random_email)
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let cookie = generate_auth_cookie(&Email::parse(random_email).unwrap()).unwrap();
    app.cookie_jar.add_cookie_str(
        &cookie.to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    app.post_logout().await;
    let response = app.post_logout().await;

    assert_eq!(response.status_code(), 400)
}
