use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

/// Port for recommendation-related external operations
#[async_trait]
pub trait RecommendationPort: Send + Sync {
    async fn train_model(&self, user_data: Vec<UserTrainingData>) -> Result<String>;
    async fn get_model_version(&self) -> Result<String>;
    async fn validate_model(&self, model_path: &str) -> Result<bool>;
    async fn log_recommendation_event(&self, event: RecommendationEvent) -> Result<()>;
}

#[derive(Debug)]
pub struct UserTrainingData {
    pub user_id: Uuid,
    pub book_interactions: Vec<BookInteraction>,
    pub preferences: Vec<String>,
}

#[derive(Debug)]
pub struct BookInteraction {
    pub book_id: Uuid,
    pub interaction_type: InteractionType,
    pub rating: Option<f64>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum InteractionType {
    View,
    Rate,
    Purchase,
    AddToWishlist,
    Share,
}

#[derive(Debug)]
pub struct RecommendationEvent {
    pub user_id: Uuid,
    pub book_id: Uuid,
    pub event_type: String,
    pub recommendation_id: Option<Uuid>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}