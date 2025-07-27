use anyhow::Result;
use uuid::Uuid;

use crate::domain::{
    entities::book::Book,
    repositories::book_repository::BookRepository,
};
use crate::application::dtos::book_dtos::{CreateBookRequest, BookResponse};

pub struct CreateBookUseCase<R: BookRepository> {
    book_repository: R,
}

impl<R: BookRepository> CreateBookUseCase<R> {
    pub fn new(book_repository: R) -> Self {
        Self { book_repository }
    }

    pub async fn execute(&self, request: CreateBookRequest, created_by: Uuid) -> Result<BookResponse> {
        let book = Book::new(
            request.title,
            request.author,
            request.description,
            request.genre,
            request.publication_year,
            request.publisher,
            request.language,
            request.page_count,
            created_by,
        );

        let saved_book = self.book_repository.save(book).await?;

        Ok(BookResponse {
            id: saved_book.id.to_string(),
            title: saved_book.title,
            author: saved_book.author,
            isbn: saved_book.isbn,
            description: saved_book.description,
            genre: saved_book.genre,
            tags: saved_book.tags,
            publication_year: saved_book.publication_year,
            publisher: saved_book.publisher,
            language: saved_book.language,
            page_count: saved_book.page_count,
            cover_image_url: saved_book.cover_image_url,
            average_rating: saved_book.average_rating,
            ratings_count: saved_book.ratings_count,
            created_at: saved_book.created_at.to_rfc3339(),
        })
    }
}