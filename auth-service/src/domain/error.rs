#[derive(Debug)]
pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials(String),
    IncorrectCredentials,
    MissingToken,
    InvalidToken,
    UnexpectedError,
}
