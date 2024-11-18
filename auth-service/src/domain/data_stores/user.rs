use crate::domain::user::{Email, Password, User};

#[derive(Debug, PartialEq, Clone)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials(String),
    UnexpectedError,
}

pub type UserStoreResult<T> = Result<T, UserStoreError>;

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> UserStoreResult<()>;
    async fn get_user(&self, email: Email) -> UserStoreResult<User>;
    async fn validate_user(&self, email: Email, password: Password) -> UserStoreResult<()>;
}
