use std::collections::HashMap;

use crate::domain::{
    data_stores::token::{BannedTokenState, BannedTokenStore},
    user::Email,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HashmapBannedTokenStore {
    // keep a list of tokens per user
    data: HashMap<String, Email>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashmapBannedTokenStore {
    fn add(&mut self, email: &Email, token: &str) {
        self.data.insert(token.to_owned(), email.clone());
    }

    fn verify(&self, token: &str) -> BannedTokenState {
        match self.data.get(token).cloned() {
            None => BannedTokenState::Absent,
            Some(email) => BannedTokenState::Exists(email),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token_adds_user_and_token() {
        let token = "sometoken";
        let email = Email::parse("email@email.com").unwrap();

        let mut store = HashmapBannedTokenStore::default();
        store.add(&email, token);

        let result = store.verify(token);
        assert!(result.email().is_some());
    }

    #[tokio::test]
    async fn test_user_and_token_not_added() {
        let token = "sometoken";

        let store = HashmapBannedTokenStore::default();
        let result = store.verify(token);

        assert_eq!(result, BannedTokenState::Absent);
    }
}
