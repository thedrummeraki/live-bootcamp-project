use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{token::BannedTokenStore, user::UserStore},
    services::{
        hashmap_banned_token_store::HashmapBannedTokenStore, hashmap_user_store::HashmapUserStore,
    },
    utils::ThreadSafe,
};

pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore>>;
pub type UserStoreType = Arc<RwLock<dyn UserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
}

impl AppState {
    pub fn user_store(mut self, user_store: UserStoreType) -> Self {
        self.user_store = user_store;
        self
    }

    pub fn banned_token_store(mut self, banned_token_store: BannedTokenStoreType) -> Self {
        self.banned_token_store = banned_token_store;
        self
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            user_store: HashmapUserStore::thread_safe(),
            banned_token_store: HashmapBannedTokenStore::thread_safe(),
        }
    }
}
