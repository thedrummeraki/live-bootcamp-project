use crate::routes::SignupRequest;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(signup_request: SignupRequest) -> Self {
        Self {
            email: signup_request.email,
            password: signup_request.password,
            requires_2fa: signup_request.requires_2fa,
        }
    }
}
