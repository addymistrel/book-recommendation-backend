use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateBookRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    
    #[validate(length(min = 1, max = 200))]
    pub author: String,
    
    pub isbn: Option<String>,
    
    #[validate(length(min = 10, max = 5000))]
    pub description: String,
    
    #[validate(length(min = 1))]
    pub genre: Vec<String>,
    
    pub tags: Vec<String>,
    
    #[validate(range(min = 1000, max = 2024))]
    pub publication_year: i32,
    
    #[validate(length(min = 1, max = 200))]
    pub publisher: String,
    
    #[validate(length(min = 1, max = 50))]
    pub language: String,
    
    #[validate(range(min = 1, max = 10000))]
    pub page_count: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BookResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub description: String,
    pub genre: Vec<String>,
    pub tags: Vec<String>,
    pub publication_year: i32,
    pub publisher: String,
    pub language: String,
    pub page_count: i32,
    pub cover_image_url: Option<String>,
    pub average_rating: f64,
    pub ratings_count: i32,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BookRatingRequest {
    #[validate(range(min = 1.0, max = 5.0))]
    pub rating: f64,
    
    #[validate(length(max = 1000))]
    pub review: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BookRatingResponse {
    pub id: String,
    pub book_id: String,
    pub user_id: String,
    pub rating: f64,
    pub review: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct BookSearchQuery {
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}