use async_trait::async_trait;
use anyhow::Result;

/// Port for book-related external operations
#[async_trait]
pub trait BookPort: Send + Sync {
    async fn fetch_book_metadata(&self, isbn: &str) -> Result<Option<ExternalBookData>>;
    async fn validate_isbn(&self, isbn: &str) -> Result<bool>;
    async fn compress_image(&self, image_data: Vec<u8>) -> Result<Vec<u8>>;
    async fn generate_thumbnail(&self, image_url: &str) -> Result<String>;
}

#[derive(Debug)]
pub struct ExternalBookData {
    pub title: String,
    pub author: String,
    pub description: String,
    pub publisher: String,
    pub publication_year: i32,
    pub page_count: i32,
    pub genre: Vec<String>,
    pub cover_image_url: Option<String>,
}