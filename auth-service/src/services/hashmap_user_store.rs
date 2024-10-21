use std::collections::HashMap;

use crate::domain::{
    data_stores::user::{UserStore, UserStoreError, UserStoreResult},
    user::{Email, Password, User},
};

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    fn add_user(&mut self, user: User) -> UserStoreResult<()> {
        let email = user.clone().email;
        if self.get_user(email.to_owned()).is_ok() {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(email, user);
        Ok(())
    }

    fn get_user(&self, email: Email) -> UserStoreResult<User> {
        self.users
            .get(&email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    fn validate_user(&self, email: Email, password: Password) -> UserStoreResult<()> {
        let user = self.get_user(email)?;
        if user.password != password {
            return Err(UserStoreError::InvalidCredentials(
                "Passwords do not match".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user::{Email, Password};

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: Email::parse("some@email.com").unwrap(),
            password: Password::parse("password").unwrap(),
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
        let email = Email::parse("some@email.com").unwrap();
        let user = User {
            email: email.to_owned(),
            password: Password::parse("password").unwrap(),
            requires_2fa: true,
        };

        store.add_user(user.clone()).unwrap();
        assert_eq!(user, store.get_user(email).unwrap());
        assert_eq!(
            UserStoreError::UserNotFound,
            store
                .get_user(Email::parse("unknown@email.com").unwrap())
                .unwrap_err()
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("some@email.com").unwrap();
        let password = Password::parse("password").unwrap();
        let user = User {
            email: email.to_owned(),
            password: password.to_owned(),
            requires_2fa: true,
        };
        store.add_user(user.clone()).unwrap();
        let result = store.validate_user(email.to_owned(), password.to_owned());
        assert!(result.is_ok());

        let error = store
            .validate_user(
                Email::parse("invalid@email.com").unwrap(),
                password.to_owned(),
            )
            .unwrap_err();
        assert_eq!(error, UserStoreError::UserNotFound);
    }
}
