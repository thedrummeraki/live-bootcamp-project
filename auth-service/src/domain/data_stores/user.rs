use crate::domain::user::User;

#[derive(Debug, PartialEq, Clone)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

pub type UserStoreResult<T> = Result<T, UserStoreError>;

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    fn add_user(&mut self, user: User) -> UserStoreResult<()>;
    fn get_user(&self, email: &str) -> UserStoreResult<User>;
    fn validate_user(&self, email: &str, password: &str) -> UserStoreResult<()>;
}