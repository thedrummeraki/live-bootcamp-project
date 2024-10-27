use serde::Serialize;

use super::user::Email;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum EmailClientError {}

pub type EmailClientResult<T> = Result<T, EmailClientError>;

#[async_trait::async_trait]
pub trait EmailClient: Send + Sync {
    async fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> EmailClientResult<()>;
}
