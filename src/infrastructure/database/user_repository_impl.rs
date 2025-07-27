use anyhow::Result;
use async_trait::async_trait;
use surrealdb::{Surreal, engine::remote::ws::Client};
use uuid::Uuid;

use crate::domain::{entities::user::User, repositories::user_repository::UserRepository};

pub struct UserRepositoryImpl {
    db: Surreal<Client>,
}

impl UserRepositoryImpl {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: User) -> Result<User> {
        let created: Option<User> = self
            .db
            .create(("users", user.id.to_string()))
            .content(&user)
            .await?;

        Ok(created.unwrap())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user: Option<User> = self.db.select(("users", id.to_string())).await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let mut result = self
            .db
            .query("SELECT * FROM users WHERE email = $email")
            .bind(("email", email))
            .await?;

        let users: Vec<User> = result.take(0)?;
        Ok(users.into_iter().next())
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let mut result = self
            .db
            .query("SELECT * FROM users WHERE username = $username")
            .bind(("username", username))
            .await?;

        let users: Vec<User> = result.take(0)?;
        Ok(users.into_iter().next())
    }

    async fn update(&self, user: User) -> Result<User> {
        let updated: Option<User> = self
            .db
            .update(("users", user.id.to_string()))
            .content(&user)
            .await?;

        Ok(updated.unwrap())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let _: Option<User> = self.db.delete(("users", id.to_string())).await?;

        Ok(())
    }

    async fn find_all(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<User>> {
        let mut query = "SELECT * FROM users".to_string();

        if let Some(limit_val) = limit {
            query.push_str(&format!(" LIMIT {}", limit_val));

            if let Some(offset_val) = offset {
                query.push_str(&format!(" START {}", offset_val));
            }
        }

        let mut result = self.db.query(&query).await?;
        let users: Vec<User> = result.take(0)?;

        Ok(users)
    }

    async fn update_preferences(&self, user_id: Uuid, preferences: Vec<String>) -> Result<()> {
        let _: Option<User> = self
            .db
            .update(("users", user_id.to_string()))
            .merge(surrealdb::sql::Object::from([(
                "preferences".to_string(),
                preferences.into(),
            )]))
            .await?;

        Ok(())
    }

    async fn deactivate_user(&self, id: Uuid) -> Result<()> {
        let _: Option<User> = self
            .db
            .update(("users", id.to_string()))
            .merge(surrealdb::sql::Object::from([(
                "is_active".to_string(),
                false.into(),
            )]))
            .await?;

        Ok(())
    }

    async fn activate_user(&self, id: Uuid) -> Result<()> {
        let _: Option<User> = self
            .db
            .update(("users", id.to_string()))
            .merge(surrealdb::sql::Object::from([(
                "is_active".to_string(),
                true.into(),
            )]))
            .await?;

        Ok(())
    }
}
