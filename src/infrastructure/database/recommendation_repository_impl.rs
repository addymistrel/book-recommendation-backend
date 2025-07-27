use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::domain::{
    entities::recommendation::{Recommendation, UserPreference, ReadingSession},
    repositories::recommendation_repository::RecommendationRepository,
};

pub struct RecommendationRepositoryImpl {
    db: Surreal<Client>,
}

impl RecommendationRepositoryImpl {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RecommendationRepository for RecommendationRepositoryImpl {
    async fn save_recommendation(&self, recommendation: Recommendation) -> Result<Recommendation> {
        let created: Option<Recommendation> = self
            .db
            .create(("recommendations", recommendation.id.to_string()))
            .content(&recommendation)
            .await?;
        
        Ok(created.unwrap())
    }

    async fn get_recommendations_for_user(&self, user_id: Uuid, limit: Option<usize>) -> Result<Vec<Recommendation>> {
        let limit = limit.unwrap_or(10);
        
        let mut result = self
            .db
            .query("SELECT * FROM recommendations WHERE user_id = $user_id ORDER BY score DESC, created_at DESC LIMIT $limit")
            .bind(("user_id", user_id))
            .bind(("limit", limit))
            .await?;
        
        let recommendations: Vec<Recommendation> = result.take(0)?;
        Ok(recommendations)
    }

    async fn mark_recommendation_clicked(&self, recommendation_id: Uuid) -> Result<()> {
        let _: Option<Recommendation> = self
            .db
            .update(("recommendations", recommendation_id.to_string()))
            .patch(surrealdb::opt::PatchOp::replace("/is_clicked", true))
            .await?;
        
        Ok(())
    }

    async fn mark_recommendation_purchased(&self, recommendation_id: Uuid) -> Result<()> {
        let _: Option<Recommendation> = self
            .db
            .update(("recommendations", recommendation_id.to_string()))
            .patch(surrealdb::opt::PatchOp::replace("/is_purchased", true))
            .await?;
        
        Ok(())
    }

    async fn delete_old_recommendations(&self, user_id: Uuid, days: i32) -> Result<()> {
        let mut result = self
            .db
            .query("DELETE FROM recommendations WHERE user_id = $user_id AND created_at < (time::now() - duration::from::days($days))")
            .bind(("user_id", user_id))
            .bind(("days", days))
            .await?;
        
        let _: Option<surrealdb::sql::Value> = result.take(0)?;
        Ok(())
    }

    async fn save_user_preference(&self, preference: UserPreference) -> Result<UserPreference> {
        let created: Option<UserPreference> = self
            .db
            .create(("user_preferences", preference.id.to_string()))
            .content(&preference)
            .await?;
        
        Ok(created.unwrap())
    }

    async fn get_user_preferences(&self, user_id: Uuid) -> Result<Vec<UserPreference>> {
        let mut result = self
            .db
            .query("SELECT * FROM user_preferences WHERE user_id = $user_id ORDER BY preference_score DESC")
            .bind(("user_id", user_id))
            .await?;
        
        let preferences: Vec<UserPreference> = result.take(0)?;
        Ok(preferences)
    }

    async fn update_user_preference(&self, preference: UserPreference) -> Result<UserPreference> {
        let updated: Option<UserPreference> = self
            .db
            .update(("user_preferences", preference.id.to_string()))
            .content(&preference)
            .await?;
        
        Ok(updated.unwrap())
    }

    async fn delete_user_preference(&self, user_id: Uuid, genre: &str) -> Result<()> {
        let mut result = self
            .db
            .query("DELETE FROM user_preferences WHERE user_id = $user_id AND genre = $genre")
            .bind(("user_id", user_id))
            .bind(("genre", genre))
            .await?;
        
        let _: Option<surrealdb::sql::Value> = result.take(0)?;
        Ok(())
    }

    async fn start_reading_session(&self, session: ReadingSession) -> Result<ReadingSession> {
        let created: Option<ReadingSession> = self
            .db
            .create(("reading_sessions", session.id.to_string()))
            .content(&session)
            .await?;
        
        Ok(created.unwrap())
    }

    async fn end_reading_session(&self, session_id: Uuid, pages_read: i32, duration_minutes: i32) -> Result<()> {
        let _: Option<ReadingSession> = self
            .db
            .update(("reading_sessions", session_id.to_string()))
            .patch(surrealdb::opt::PatchOp::replace("/end_time", chrono::Utc::now()))
            .patch(surrealdb::opt::PatchOp::replace("/pages_read", pages_read))
            .patch(surrealdb::opt::PatchOp::replace("/session_duration_minutes", duration_minutes))
            .await?;
        
        Ok(())
    }

    async fn get_user_reading_sessions(&self, user_id: Uuid, limit: Option<usize>) -> Result<Vec<ReadingSession>> {
        let limit = limit.unwrap_or(10);
        
        let mut result = self
            .db
            .query("SELECT * FROM reading_sessions WHERE user_id = $user_id ORDER BY start_time DESC LIMIT $limit")
            .bind(("user_id", user_id))
            .bind(("limit", limit))
            .await?;
        
        let sessions: Vec<ReadingSession> = result.take(0)?;
        Ok(sessions)
    }

    async fn get_recommendation_click_rate(&self, user_id: Uuid) -> Result<f64> {
        let mut result = self
            .db
            .query("
                LET $total = (SELECT COUNT() FROM recommendations WHERE user_id = $user_id GROUP ALL).count;
                LET $clicked = (SELECT COUNT() FROM recommendations WHERE user_id = $user_id AND is_clicked = true GROUP ALL).count;
                RETURN math::round(($clicked / $total) * 100, 2);
            ")
            .bind(("user_id", user_id))
            .await?;
        
        let click_rate: Option<f64> = result.take(0)?;
        Ok(click_rate.unwrap_or(0.0))
    }

    async fn get_popular_genres(&self, limit: Option<usize>) -> Result<Vec<(String, i64)>> {
        let limit = limit.unwrap_or(10);
        
        let mut result = self
            .db
            .query("
                SELECT genre, count() AS popularity 
                FROM (SELECT genre FROM books)
                GROUP BY genre
                ORDER BY popularity DESC
                LIMIT $limit
            ")
            .bind(("limit", limit))
            .await?;
        
        #[derive(serde::Deserialize)]
        struct GenreCount {
            genre: String,
            popularity: i64,
        }
        
        let genre_counts: Vec<GenreCount> = result.take(0)?;
        Ok(genre_counts.into_iter().map(|gc| (gc.genre, gc.popularity)).collect())
    }
}