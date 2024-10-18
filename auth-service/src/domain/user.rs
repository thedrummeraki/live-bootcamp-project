use crate::routes::SignupRequest;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(signup_request: SignupRequest) -> Option<Self> {
        let email = signup_request.email.trim().to_owned();
        let password = signup_request.password.to_owned();

        if email.is_empty() || !email.contains("@") || password.len() < 8 {
            return None;
        }

        Some(Self {
            email,
            password,
            requires_2fa: signup_request.requires_2fa,
        })
    }
}
