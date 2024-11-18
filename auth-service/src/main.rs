use std::sync::Arc;

use auth_service::{
    app_state::AppState,
    get_postgres_pool,
    services::{
        data_stores::postgres_user_store::PostgresUserStore,
        hashmap_banned_token_store::HashmapBannedTokenStore,
        hashmap_two_fa_code_store::HashmapTwoFACodeStore, mock_email_client::MockEmailClient,
    },
    utils::{
        constants::{prod, DATABASE_URL},
        ThreadSafe,
    },
    Application,
};
use sqlx::PgPool;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let pool = configure_postgresql().await;

    let app_state = AppState::default()
        .user_store(Arc::new(RwLock::new(PostgresUserStore::new(pool))))
        .banned_token_store(HashmapBannedTokenStore::thread_safe())
        .two_fa_code_store(HashmapTwoFACodeStore::thread_safe())
        .email_client(MockEmailClient::thread_safe());

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn configure_postgresql() -> PgPool {
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create PG connection pool!");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    println!("Migrations successfully ran.");

    pg_pool
}
