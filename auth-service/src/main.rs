use auth_service::{
    app_state::AppState,
    get_postgres_pool,
    services::{
        hashmap_banned_token_store::HashmapBannedTokenStore,
        hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore,
        mock_email_client::MockEmailClient,
    },
    utils::{
        constants::{prod, DATABASE_URL},
        ThreadSafe,
    },
    Application,
};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let app_state = AppState::default()
        .user_store(HashmapUserStore::thread_safe())
        .banned_token_store(HashmapBannedTokenStore::thread_safe())
        .two_fa_code_store(HashmapTwoFACodeStore::thread_safe())
        .email_client(MockEmailClient::thread_safe());

    let pg_pool = configure_postgresql().await;

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
