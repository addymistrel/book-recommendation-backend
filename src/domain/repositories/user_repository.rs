use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::domain::entities::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> Result<User>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn update(&self, user: User) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn find_all(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<User>>;
    async fn update_preferences(&self, user_id: Uuid, preferences: Vec<String>) -> Result<()>;
    async fn deactivate_user(&self, id: Uuid) -> Result<()>;
    async fn activate_user(&self, id: Uuid) -> Result<()>;
}