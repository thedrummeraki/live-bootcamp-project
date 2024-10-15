use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq, Clone)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

type HashmapUserStoreResult<T> = Result<T, UserStoreError>;

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> HashmapUserStoreResult<()> {
        let email = user.clone().email;
        if self.get_user(&email).is_ok() {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(email, user);
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> HashmapUserStoreResult<User> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &str, password: &str) -> HashmapUserStoreResult<()> {
        let user = self.get_user(email)?;
        if user.password != password {
            return Err(UserStoreError::InvalidCredentials);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "some@email.com".into(),
            password: "password".into(),
            requires_2fa: true,
        };
        let other_user = user.clone();

        let result = store.add_user(user);
        assert!(result.is_ok());

        let result = store.add_user(other_user);
        assert_eq!(result.err().unwrap(), UserStoreError::UserAlreadyExists)
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "some@email.com".into(),
            password: "password".into(),
            requires_2fa: true,
        };

        store.add_user(user.clone()).unwrap();
        assert_eq!(user, store.get_user("some@email.com").unwrap());
        assert_eq!(
            UserStoreError::UserNotFound,
            store.get_user("unknown@email.com").unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "some@email.com".into(),
            password: "password".into(),
            requires_2fa: true,
        };
        store.add_user(user.clone()).unwrap();
        let result = store.validate_user("some@email.com", "password");
        assert!(result.is_ok());

        let error = store
            .validate_user("invalid@email.com", "password")
            .unwrap_err();
        assert_eq!(error, UserStoreError::UserNotFound);

        let error = store
            .validate_user("some@email.com", "invalid")
            .unwrap_err();
        assert_eq!(error, UserStoreError::InvalidCredentials);
    }
}