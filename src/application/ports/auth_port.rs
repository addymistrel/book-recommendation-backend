use async_trait::async_trait;
use anyhow::Result;

/// Port for authentication-related external operations
#[async_trait]
pub trait AuthPort: Send + Sync {
    async fn send_welcome_email(&self, email: &str, username: &str) -> Result<()>;
    async fn send_password_reset_email(&self, email: &str, reset_token: &str) -> Result<()>;
    async fn validate_email_domain(&self, email: &str) -> Result<bool>;
    async fn check_password_breach(&self, password: &str) -> Result<bool>;
}