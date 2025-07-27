use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::domain::entities::recommendation::{Recommendation, UserPreference, ReadingSession};

#[async_trait]
pub trait RecommendationRepository: Send + Sync {
    // Recommendation operations
    async fn save_recommendation(&self, recommendation: Recommendation) -> Result<Recommendation>;
    async fn get_recommendations_for_user(&self, user_id: Uuid, limit: Option<usize>) -> Result<Vec<Recommendation>>;
    async fn mark_recommendation_clicked(&self, recommendation_id: Uuid) -> Result<()>;
    async fn mark_recommendation_purchased(&self, recommendation_id: Uuid) -> Result<()>;
    async fn delete_old_recommendations(&self, user_id: Uuid, days: i32) -> Result<()>;
    
    // User preference operations
    async fn save_user_preference(&self, preference: UserPreference) -> Result<UserPreference>;
    async fn get_user_preferences(&self, user_id: Uuid) -> Result<Vec<UserPreference>>;
    async fn update_user_preference(&self, preference: UserPreference) -> Result<UserPreference>;
    async fn delete_user_preference(&self, user_id: Uuid, genre: &str) -> Result<()>;
    
    // Reading session operations
    async fn start_reading_session(&self, session: ReadingSession) -> Result<ReadingSession>;
    async fn end_reading_session(&self, session_id: Uuid, pages_read: i32, duration_minutes: i32) -> Result<()>;
    async fn get_user_reading_sessions(&self, user_id: Uuid, limit: Option<usize>) -> Result<Vec<ReadingSession>>;
    
    // Analytics
    async fn get_recommendation_click_rate(&self, user_id: Uuid) -> Result<f64>;
    async fn get_popular_genres(&self, limit: Option<usize>) -> Result<Vec<(String, i64)>>;
}