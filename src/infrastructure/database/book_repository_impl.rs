use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::domain::{
    entities::book::{Book, BookRating},
    repositories::book_repository::BookRepository,
};

pub struct BookRepositoryImpl {
    db: Surreal<Client>,
}

impl BookRepositoryImpl {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn save(&self, book: Book) -> Result<Book> {
        let created: Option<Book> = self
            .db
            .create(("books", book.id.to_string()))
            .content(&book)
            .await?;
        
        Ok(created.unwrap())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Book>> {
        let book: Option<Book> = self
            .db
            .select(("books", id.to_string()))
            .await?;
        
        Ok(book)
    }

    async fn find_by_title(&self, title: &str) -> Result<Vec<Book>> {
        let mut result = self
            .db
            .query("SELECT * FROM books WHERE string::lowercase(title) CONTAINS string::lowercase($title)")
            .bind(("title", title))
            .await?;
        
        let books: Vec<Book> = result.take(0)?;
        Ok(books)
    }

    async fn find_by_author(&self, author: &str) -> Result<Vec<Book>> {
        let mut result = self
            .db
            .query("SELECT * FROM books WHERE string::lowercase(author) CONTAINS string::lowercase($author)")
            .bind(("author", author))
            .await?;
        
        let books: Vec<Book> = result.take(0)?;
        Ok(books)
    }

    async fn find_by_genre(&self, genre: &str) -> Result<Vec<Book>> {
        let mut result = self
            .db
            .query("SELECT * FROM books WHERE $genre IN genre")
            .bind(("genre", genre))
            .await?;
        
        let books: Vec<Book> = result.take(0)?;
        Ok(books)
    }

    async fn find_all(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<Book>> {
        let limit = limit.unwrap_or(10);
        let offset = offset.unwrap_or(0);
        
        let mut result = self
            .db
            .query("SELECT * FROM books ORDER BY created_at DESC LIMIT $limit START $offset")
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await?;
        
        let books: Vec<Book> = result.take(0)?;
        Ok(books)
    }

    async fn update(&self, book: Book) -> Result<Book> {
        let updated: Option<Book> = self
            .db
            .update(("books", book.id.to_string()))
            .content(&book)
            .await?;
        
        Ok(updated.unwrap())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let _: Option<Book> = self
            .db
            .delete(("books", id.to_string()))
            .await?;
        
        Ok(())
    }

    async fn save_rating(&self, rating: BookRating) -> Result<BookRating> {
        let created: Option<BookRating> = self
            .db
            .create(("book_ratings", rating.id.to_string()))
            .content(&rating)
            .await?;
        
        Ok(created.unwrap())
    }

    async fn get_ratings_for_book(&self, book_id: Uuid) -> Result<Vec<BookRating>> {
        let mut result = self
            .db
            .query("SELECT * FROM book_ratings WHERE book_id = $book_id ORDER BY created_at DESC")
            .bind(("book_id", book_id))
            .await?;
        
        let ratings: Vec<BookRating> = result.take(0)?;
        Ok(ratings)
    }

    async fn get_user_rating(&self, book_id: Uuid, user_id: Uuid) -> Result<Option<BookRating>> {
        let mut result = self
            .db
            .query("SELECT * FROM book_ratings WHERE book_id = $book_id AND user_id = $user_id")
            .bind(("book_id", book_id))
            .bind(("user_id", user_id))
            .await?;
        
        let ratings: Vec<BookRating> = result.take(0)?;
        Ok(ratings.into_iter().next())
    }

    async fn update_book_rating_stats(&self, book_id: Uuid) -> Result<()> {
        // Calculate new average rating and count
        let mut result = self
            .db
            .query("
                LET $ratings = (SELECT rating FROM book_ratings WHERE book_id = $book_id);
                LET $avg_rating = math::mean($ratings.rating);
                LET $count = array::len($ratings);
                UPDATE books SET 
                    average_rating = $avg_rating,
                    ratings_count = $count,
                    updated_at = time::now()
                WHERE id = $book_id;
            ")
            .bind(("book_id", book_id))
            .await?;
        
        let _: Option<surrealdb::sql::Value> = result.take(2)?;
        Ok(())
    }

    async fn get_user_ratings(&self, user_id: Uuid) -> Result<Vec<BookRating>> {
        let mut result = self
            .db
            .query("SELECT * FROM book_ratings WHERE user_id = $user_id ORDER BY created_at DESC")
            .bind(("user_id", user_id))
            .await?;
        
        let ratings: Vec<BookRating> = result.take(0)?;
        Ok(ratings)
    }
}