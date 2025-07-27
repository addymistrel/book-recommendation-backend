use anyhow::Result;
use uuid::Uuid;

use crate::domain::{
    entities::book::BookRating,
    repositories::book_repository::BookRepository,
    services::book_service::BookService,
    errors::domain_error::DomainError,
};
use crate::application::dtos::book_dtos::{BookRatingRequest, BookRatingResponse};

pub struct RateBookUseCase<R: BookRepository> {
    book_repository: R,
    book_service: BookService,
}

impl<R: BookRepository> RateBookUseCase<R> {
    pub fn new(book_repository: R, book_service: BookService) -> Self {
        Self {
            book_repository,
            book_service,
        }
    }

    pub async fn execute(
        &self,
        book_id: Uuid,
        user_id: Uuid,
        request: BookRatingRequest,
    ) -> Result<BookRatingResponse> {
        // Check if book exists
        let mut book = self.book_repository
            .find_by_id(book_id)
            .await?
            .ok_or(DomainError::BookNotFound)?;

        // Check if user has already rated this book
        let existing_rating = self.book_repository
            .get_user_rating(book_id, user_id)
            .await?;

        if !self.book_service.can_user_rate_book(existing_rating) {
            return Err(DomainError::InvalidInput("User has already rated this book".to_string()).into());
        }

        // Create new rating
        let rating = BookRating {
            id: Uuid::new_v4(),
            book_id,
            user_id,
            rating: request.rating,
            review: request.review,
            created_at: chrono::Utc::now(),
        };

        // Save rating
        let saved_rating = self.book_repository.save_rating(rating).await?;

        // Update book rating statistics
        self.book_service.update_book_rating_stats(&mut book, request.rating);
        self.book_repository.update(book).await?;

        // Update aggregated rating stats
        self.book_repository.update_book_rating_stats(book_id).await?;

        Ok(BookRatingResponse {
            id: saved_rating.id.to_string(),
            book_id: saved_rating.book_id.to_string(),
            user_id: saved_rating.user_id.to_string(),
            rating: saved_rating.rating,
            review: saved_rating.review,
            created_at: saved_rating.created_at.to_rfc3339(),
        })
    }
}