use rand::seq::SliceRandom;
use uuid::Uuid;

use crate::domain::user::Email;

#[async_trait::async_trait]
pub trait TwoFACodeStore: Send + Sync {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        Uuid::parse_str(&id)
            .and(Ok(Self(id)))
            .map_err(|e| format!("Invalid login attempt ID. Expected a valid UUID. Details: {e:?}"))
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self::parse(Uuid::new_v4().to_string())
            .expect("Could not parse default UUID for LoginAttemptId")
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        let digits: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() == 6 {
            Ok(Self(code))
        } else {
            Err(format!(
                "Invalid 2FA code. Expected a 6 digit string. Got: \"{code}\""
            ))
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<i32> = (0..10).collect();
        nums.shuffle(&mut rng);

        Self(nums.iter().take(6).map(|num| num.to_string()).collect())
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
