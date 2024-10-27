use super::data_stores::user::UserStoreError;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse<S: AsRef<str>>(value: S) -> Result<Self, UserStoreError> {
        let str: &str = value.as_ref();
        if str.is_empty() || !str.contains("@") {
            return Err(UserStoreError::InvalidCredentials(
                "Invalid email address".into(),
            ));
        }

        Ok(Self(str.into()))
    }
}

#[cfg(test)]
impl Default for Email {
    fn default() -> Self {
        Self::parse("email@email.com").unwrap()
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn parse<S: AsRef<str>>(value: S) -> Result<Self, UserStoreError> {
        let str: &str = value.as_ref();
        if str.len() < 8 {
            return Err(UserStoreError::InvalidCredentials(
                "Password too short (must have 8 chars or more)".into(),
            ));
        }

        Ok(Self(str.into()))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Email, Password};

    #[test]
    fn should_return_email_ok_when_properly_parsed() {
        let results = [
            Email::parse("some@value.com"),
            Email::parse("some.other@value.com"),
            Email::parse("some-value123@value.com"),
        ];

        assert!(results.iter().all(|r| r.is_ok()))
    }

    #[test]
    fn should_return_email_err_when_not_properly_parsed() {
        let results = [Email::parse("some]value.com"), Email::parse("")];

        assert!(results.iter().all(|r| r.is_err()))
    }

    #[test]
    fn should_return_password_ok_when_properly_parsed() {
        let results = [
            Password::parse("password"),
            Password::parse("some long passphrase that should also work"),
        ];

        assert!(results.iter().all(|r| r.is_ok()))
    }

    #[test]
    fn should_return_password_err_when_not_properly_parsed() {
        let results = [
            Password::parse(""),
            Password::parse("1"),
            Password::parse("12"),
            Password::parse("123"),
            Password::parse("1234"),
            Password::parse("12345"),
            Password::parse("123456"),
            Password::parse("1234567"),
        ];

        assert!(results.iter().all(|r| r.is_err()))
    }
}
