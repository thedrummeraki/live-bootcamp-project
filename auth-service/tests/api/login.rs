use serde_json::json;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let malformed_body = json!({});
    let response = app.post_login(&malformed_body).await;

    assert_eq!(response.status().as_u16(), 422);
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
        assert_eq!(response.status().as_u16(), 400);
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

    assert_eq!(response.status().as_u16(), 401);
}
