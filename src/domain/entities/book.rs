use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Book {
    pub id: Uuid,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid, // User who added the book
}

impl Book {
    pub fn new(
        title: String,
        author: String,
        description: String,
        genre: Vec<String>,
        publication_year: i32,
        publisher: String,
        language: String,
        page_count: i32,
        created_by: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            author,
            isbn: None,
            description,
            genre,
            tags: Vec::new(),
            publication_year,
            publisher,
            language,
            page_count,
            cover_image_url: None,
            average_rating: 0.0,
            ratings_count: 0,
            created_at: now,
            updated_at: now,
            created_by,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BookRating {
    pub id: Uuid,
    pub book_id: Uuid,
    pub user_id: Uuid,
    pub rating: f64, // 1.0 to 5.0
    pub review: Option<String>,
    pub created_at: DateTime<Utc>,
}