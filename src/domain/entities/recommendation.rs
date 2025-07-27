use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Recommendation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: Uuid,
    pub score: f64, // Recommendation confidence score
    pub reason: String, // Why this book was recommended
    pub algorithm_version: String, // Which ML model version was used
    pub created_at: DateTime<Utc>,
    pub is_clicked: bool, // User interaction tracking
    pub is_purchased: bool, // Conversion tracking
}

impl Recommendation {
    pub fn new(
        user_id: Uuid,
        book_id: Uuid,
        score: f64,
        reason: String,
        algorithm_version: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            book_id,
            score,
            reason,
            algorithm_version,
            created_at: Utc::now(),
            is_clicked: false,
            is_purchased: false,
        }
    }

    pub fn mark_clicked(&mut self) {
        self.is_clicked = true;
    }

    pub fn mark_purchased(&mut self) {
        self.is_purchased = true;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub genre: String,
    pub preference_score: f64, // How much user likes this genre (0.0 to 1.0)
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReadingSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub pages_read: i32,
    pub session_duration_minutes: Option<i32>,
}