use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::domain::entities::book::{Book, BookRating};

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn save(&self, book: Book) -> Result<Book>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Book>>;
    async fn find_by_title(&self, title: &str) -> Result<Vec<Book>>;
    async fn find_by_author(&self, author: &str) -> Result<Vec<Book>>;
    async fn find_by_genre(&self, genre: &str) -> Result<Vec<Book>>;
    async fn find_all(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<Book>>;
    async fn update(&self, book: Book) -> Result<Book>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    
    // Rating operations
    async fn save_rating(&self, rating: BookRating) -> Result<BookRating>;
    async fn get_ratings_for_book(&self, book_id: Uuid) -> Result<Vec<BookRating>>;
    async fn get_user_rating(&self, book_id: Uuid, user_id: Uuid) -> Result<Option<BookRating>>;
    async fn update_book_rating_stats(&self, book_id: Uuid) -> Result<()>;
    async fn get_user_ratings(&self, user_id: Uuid) -> Result<Vec<BookRating>>;
}