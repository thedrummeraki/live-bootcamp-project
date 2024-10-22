use crate::domain::{data_stores::user::UserStoreError, error::AuthAPIError};

pub fn map_user_store_error_to_api_error(user_error: UserStoreError) -> AuthAPIError {
    match user_error {
        UserStoreError::InvalidCredentials(details) => AuthAPIError::InvalidCredentials(details),
        UserStoreError::UserNotFound => AuthAPIError::IncorrectCredentials,
        UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
        _ => AuthAPIError::UnexpectedError,
    }
}
