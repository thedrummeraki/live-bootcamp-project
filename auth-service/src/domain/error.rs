use crate::utils::auth::GenerateTokenError;

#[derive(Debug)]
pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials(String),
    IncorrectCredentials,
    MissingToken,
    InvalidToken,
    GenerateTokenError(GenerateTokenError),
    BadInput(String),
    UnexpectedError,
}
