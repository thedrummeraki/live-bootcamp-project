use crate::domain::user::Email;

#[derive(Debug, Clone, PartialEq)]
pub enum BannedTokenStoreError {
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum BannedTokenState {
    #[default]
    Absent, // token doesn't exist, was not banned
    Exists(Email), // token was banned, as it exists in the store
}

impl BannedTokenState {
    pub fn email(&self) -> Option<Email> {
        match self {
            Self::Absent => None,
            BannedTokenState::Exists(email) => Some(email.to_owned()),
        }
    }

    pub fn exists(&self) -> bool {
        self.email().is_some()
    }

    pub fn is_absent(&self) -> bool {
        self.email().is_none()
    }
}

pub trait BannedTokenStore: Send + Sync {
    fn add(&mut self, email: &Email, token: &str);
    fn verify(&self, token: &str) -> BannedTokenState;
}
