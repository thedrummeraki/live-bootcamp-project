pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials(String),
    UnexpectedError,
}
