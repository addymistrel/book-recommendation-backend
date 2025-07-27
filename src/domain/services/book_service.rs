use anyhow::Result;
use uuid::Uuid;

use crate::domain::entities::book::{Book, BookRating};

/// Domain service for book-related business logic
pub struct BookService;

impl BookService {
    pub fn new() -> Self {
        Self
    }

    /// Calculate new average rating when a rating is added
    pub fn calculate_new_average_rating(
        current_average: f64,
        current_count: i32,
        new_rating: f64,
    ) -> f64 {
        if current_count == 0 {
            return new_rating;
        }
        
        let total_score = current_average * current_count as f64;
        let new_total = total_score + new_rating;
        let new_count = current_count + 1;
        
        new_total / new_count as f64
    }

    /// Update book rating statistics
    pub fn update_book_rating_stats(&self, book: &mut Book, new_rating: f64) {
        book.average_rating = Self::calculate_new_average_rating(
            book.average_rating,
            book.ratings_count,
            new_rating,
        );
        book.ratings_count += 1;
        book.updated_at = chrono::Utc::now();
    }

    /// Validate book data before saving
    pub fn validate_book(&self, book: &Book) -> Result<()> {
        if book.title.trim().is_empty() {
            return Err(anyhow::anyhow!("Book title cannot be empty"));
        }

        if book.author.trim().is_empty() {
            return Err(anyhow::anyhow!("Book author cannot be empty"));
        }

        if book.publication_year < 1000 || book.publication_year > 2024 {
            return Err(anyhow::anyhow!("Invalid publication year"));
        }

        if book.page_count <= 0 {
            return Err(anyhow::anyhow!("Page count must be positive"));
        }

        Ok(())
    }

    /// Check if user can rate a book (hasn't rated it before)
    pub fn can_user_rate_book(&self, existing_rating: Option<BookRating>) -> bool {
        existing_rating.is_none()
    }

    /// Generate book recommendation reason
    pub fn generate_recommendation_reason(
        book: &Book,
        user_preferences: &[String],
        score: f64,
    ) -> String {
        let matching_genres: Vec<&String> = book.genre
            .iter()
            .filter(|genre| user_preferences.contains(genre))
            .collect();

        if !matching_genres.is_empty() {
            format!(
                "Recommended because you like {} books (confidence: {:.1}%)",
                matching_genres.join(", "),
                score * 100.0
            )
        } else if book.average_rating > 4.0 {
            format!(
                "Highly rated book ({}★) that might interest you (confidence: {:.1}%)",
                book.average_rating,
                score * 100.0
            )
        } else {
            format!(
                "Discovered based on your reading patterns (confidence: {:.1}%)",
                score * 100.0
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_new_average_rating() {
        // Test first rating
        let average = BookService::calculate_new_average_rating(0.0, 0, 4.5);
        assert_eq!(average, 4.5);

        // Test adding second rating
        let average = BookService::calculate_new_average_rating(4.5, 1, 3.5);
        assert_eq!(average, 4.0);

        // Test adding third rating
        let average = BookService::calculate_new_average_rating(4.0, 2, 5.0);
        assert!((average - 4.333333333333333).abs() < f64::EPSILON);
    }
}