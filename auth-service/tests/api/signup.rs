use auth_service::routes::SignupResponse;
use serde_json::json;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123",
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": "true"
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let body = json!({"email": "email@email.com", "password": "password", "requires2FA": true});
    let expected_response = SignupResponse {
        message: "User created successfully!".into(),
    };

    let response = app.post_signup(&body).await;

    assert_eq!(response.status().as_u16(), 201);
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialilze response body to UserBody"),
        expected_response
    )
}