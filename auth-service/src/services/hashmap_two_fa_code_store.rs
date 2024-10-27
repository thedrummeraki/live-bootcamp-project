use std::collections::HashMap;

use crate::domain::{
    data_stores::twofa::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    user::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        let key = email.clone();
        let value = (login_attempt_id.clone(), code.clone());

        self.codes.insert(key, value);

        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        let key = email.clone();

        self.codes
            .remove(&key)
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)
            .and(Ok(()))
    }
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        self.codes
            .get(email)
            .cloned()
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adds_code_to_store() {
        let email = Email::default();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let mut store = HashmapTwoFACodeStore::default();
        let result = store
            .add_code(email.to_owned(), login_attempt_id, code)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_removes_code_from_store() {
        let email = Email::default();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let mut store = HashmapTwoFACodeStore::default();
        assert!(store.remove_code(&email).await.is_err());

        let _ = store
            .add_code(email.to_owned(), login_attempt_id, code)
            .await;

        let result = store.remove_code(&email).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_gets_code_from_store() {
        let email = Email::default();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let mut store = HashmapTwoFACodeStore::default();
        assert!(store.get_code(&email).await.is_err());

        let _ = store
            .add_code(email.to_owned(), login_attempt_id, code)
            .await;

        let result = store.get_code(&email).await;
        assert!(result.is_ok());
    }
}
