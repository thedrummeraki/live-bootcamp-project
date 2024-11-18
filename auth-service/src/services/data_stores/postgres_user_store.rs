use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use sqlx::PgPool;

use crate::domain::{
    data_stores::user::{UserStore, UserStoreError, UserStoreResult},
    user::{Email, Password, User},
};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> UserStoreResult<()> {
        let email = user.email.to_owned();
        if self.get_user(email.clone()).await.is_ok() {
            return Err(UserStoreError::UserAlreadyExists);
        }

        let password_hash = compute_password_hash(user.password.to_owned().as_ref().to_string())
            .await
            .expect("could not hash password!");

        sqlx::query!(
            "INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)",
            email.as_ref(),
            password_hash,
            user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)
        .map(|_| ())
    }
    async fn get_user(&self, email: Email) -> UserStoreResult<User> {
        let maybe_user_record =
            sqlx::query!("SELECT * FROM users WHERE email = $1", email.as_ref())
                .fetch_optional(&self.pool)
                .await
                .map_err(|_| UserStoreError::UnexpectedError)?;

        let user_record = maybe_user_record.ok_or(UserStoreError::UserNotFound)?;

        Ok(User {
            email: Email::parse(user_record.email).map_err(|_| UserStoreError::UnexpectedError)?,
            password: Password::parse(user_record.password_hash)
                .map_err(|_| UserStoreError::UnexpectedError)?,
            requires_2fa: user_record.requires_2fa,
        })
    }
    async fn validate_user(&self, email: Email, password: Password) -> UserStoreResult<()> {
        let user = self.get_user(email).await?;

        verify_password_hash(
            user.password.as_ref().to_string(),
            password.as_ref().to_string(),
        )
        .await
        .map_err(|_| UserStoreError::InvalidCredentials("Invalid password".into()))
        .map(|_| ())
    }
}

// Helper function to verify if a given password matches an expected hash
async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let result = tokio::task::spawn_blocking(move || {
        let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&expected_password_hash)?;

        Argon2::default()
            .verify_password(password_candidate.as_bytes(), &expected_password_hash)
            .map_err(|e| e.into())
    })
    .await;

    result?
}

// Helper function to hash passwords before persisting them in the database.
async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let result = tokio::task::spawn_blocking(move || {
        let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None)?,
        )
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

        Ok(password_hash)
    })
    .await;

    result?
}
