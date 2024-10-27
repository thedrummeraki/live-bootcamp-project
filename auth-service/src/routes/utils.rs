use crate::domain::{data_stores::user::UserStoreError, error::AuthAPIError};

pub fn map_user_store_error_to_api_error(user_error: UserStoreError) -> AuthAPIError {
    match user_error {
        UserStoreError::InvalidCredentials(details) => AuthAPIError::InvalidCredentials(details),
        UserStoreError::UserNotFound => AuthAPIError::IncorrectCredentials,
        UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
        _ => AuthAPIError::UnexpectedError,
    }
}

pub fn map_string_error_to_api_error(str_error: String) -> AuthAPIError {
    println!("[ERROR] Unexpected generic error. Details: {str_error}");
    AuthAPIError::UnexpectedError
}

pub fn map_string_error_to_bad_input_error(str_error: String) -> AuthAPIError {
    println!("[ERROR] Unexpected param. Details: {str_error}");
    AuthAPIError::BadInput(str_error)
}
