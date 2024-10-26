use auth_service::{
    app_state::AppState,
    services::{
        hashmap_banned_token_store::HashmapBannedTokenStore, hashmap_user_store::HashmapUserStore,
    },
    utils::{constants::prod, ThreadSafe},
    Application,
};

#[tokio::main]
async fn main() {
    let app_state = AppState::default()
        .user_store(HashmapUserStore::thread_safe())
        .banned_token_store(HashmapBannedTokenStore::thread_safe());

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
