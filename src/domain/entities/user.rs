use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub preferences: Vec<String>, // Book genres/categories
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub preferences: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        email: String,
        username: String,
        password_hash: String,
        first_name: String,
        last_name: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            username,
            password_hash,
            first_name,
            last_name,
            preferences: Vec::new(),
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }

    pub fn to_profile(&self) -> UserProfile {
        UserProfile {
            id: self.id,
            email: self.email.clone(),
            username: self.username.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            preferences: self.preferences.clone(),
            created_at: self.created_at,
        }
    }
}