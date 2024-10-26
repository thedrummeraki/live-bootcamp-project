use std::error::Error;

use app_state::AppState;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use domain::error::AuthAPIError;
use reqwest::Method;
use routes::{login, logout, signup, verify_2fa, verify_token};
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message): (StatusCode, String) = match self {
            AuthAPIError::InvalidCredentials(details) => (
                StatusCode::BAD_REQUEST,
                ("Invalid credentials: ".to_owned() + details.as_str()).into(),
            ),
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists".into()),
            AuthAPIError::IncorrectCredentials => (
                StatusCode::UNAUTHORIZED,
                "Access to server limitted or no access granted.".into(),
            ),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing token".into()),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".into()),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error".into())
            }
        };

        let body = Json(ErrorResponse {
            error: error_message.into(),
        });
        (status, body).into_response()
    }
}

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://37.184.163.199".parse()?,
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
