use crate::helpers::{ResponseExt, TestApp};

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
