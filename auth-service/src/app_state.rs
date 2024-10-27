use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{
        data_stores::{token::BannedTokenStore, twofa::TwoFACodeStore, user::UserStore},
        EmailClient,
    },
    services::{
        hashmap_banned_token_store::HashmapBannedTokenStore,
        hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore,
        mock_email_client::MockEmailClient,
    },
    utils::ThreadSafe,
};

pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore>>;
pub type UserStoreType = Arc<RwLock<dyn UserStore>>;
pub type TwoFACodeStoreType = Arc<RwLock<dyn TwoFACodeStore>>;
pub type EmailClientType = Arc<RwLock<dyn EmailClient>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub email_client: EmailClientType,
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

    pub fn two_fa_code_store(mut self, two_fa_code_store: TwoFACodeStoreType) -> Self {
        self.two_fa_code_store = two_fa_code_store;
        self
    }

    pub fn email_client(mut self, email_client: EmailClientType) -> Self {
        self.email_client = email_client;
        self
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            user_store: HashmapUserStore::thread_safe(),
            banned_token_store: HashmapBannedTokenStore::thread_safe(),
            two_fa_code_store: HashmapTwoFACodeStore::thread_safe(),
            email_client: MockEmailClient::thread_safe(),
        }
    }
}
