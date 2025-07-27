use anyhow::Result;

use crate::domain::repositories::book_repository::BookRepository;
use crate::application::dtos::book_dtos::{BookSearchQuery, BookResponse};

pub struct GetBooksUseCase<R: BookRepository> {
    book_repository: R,
}

impl<R: BookRepository> GetBooksUseCase<R> {
    pub fn new(book_repository: R) -> Self {
        Self { book_repository }
    }

    pub async fn execute(&self, query: BookSearchQuery) -> Result<Vec<BookResponse>> {
        let books = if let Some(title) = &query.title {
            self.book_repository.find_by_title(title).await?
        } else if let Some(author) = &query.author {
            self.book_repository.find_by_author(author).await?
        } else if let Some(genre) = &query.genre {
            self.book_repository.find_by_genre(genre).await?
        } else {
            self.book_repository.find_all(query.limit, query.offset).await?
        };

        let book_responses = books
            .into_iter()
            .map(|book| BookResponse {
                id: book.id.to_string(),
                title: book.title,
                author: book.author,
                isbn: book.isbn,
                description: book.description,
                genre: book.genre,
                tags: book.tags,
                publication_year: book.publication_year,
                publisher: book.publisher,
                language: book.language,
                page_count: book.page_count,
                cover_image_url: book.cover_image_url,
                average_rating: book.average_rating,
                ratings_count: book.ratings_count,
                created_at: book.created_at.to_rfc3339(),
            })
            .collect();

        Ok(book_responses)
    }
}