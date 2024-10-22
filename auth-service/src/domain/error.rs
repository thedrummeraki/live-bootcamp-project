pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials(String),
    IncorrectCredentials,
    UnexpectedError,
}
