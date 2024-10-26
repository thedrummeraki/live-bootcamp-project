use crate::helpers::{ResponseExt, TestApp};

#[tokio::test]
async fn verify_2fa_should_return_200() {
    let app = TestApp::new().await;

    let response = app.post_verify_2fa().await;

    assert_eq!(response.status_code(), 200);
}
