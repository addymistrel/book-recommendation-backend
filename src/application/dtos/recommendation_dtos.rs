use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::application::dtos::book_dtos::BookResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct RecommendationResponse {
    pub recommendations: Vec<BookResponse>,
    pub confidence: f64,
    pub total_count: usize,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GetRecommendationsRequest {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<usize>,
    
    pub include_reasons: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecommendationItem {
    pub book: BookResponse,
    pub score: f64,
    pub reason: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DetailedRecommendationResponse {
    pub recommendations: Vec<RecommendationItem>,
    pub confidence: f64,
    pub total_count: usize,
    pub algorithm_version: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePreferencesRequest {
    #[validate(length(min = 1, max = 20))]
    pub preferences: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserPreferenceResponse {
    pub genre: String,
    pub score: f64,
    pub last_updated: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecommendationAnalyticsResponse {
    pub click_rate: f64,
    pub conversion_rate: f64,
    pub popular_genres: Vec<GenrePopularity>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GenrePopularity {
    pub genre: String,
    pub count: i64,
    pub percentage: f64,
}