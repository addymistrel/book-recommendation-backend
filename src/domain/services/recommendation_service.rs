use anyhow::Result;
use uuid::Uuid;
use std::collections::HashMap;

use crate::domain::entities::{
    book::Book,
    user::User,
    recommendation::{Recommendation, UserPreference},
};

/// Domain service for recommendation business logic
pub struct RecommendationService;

impl RecommendationService {
    pub fn new() -> Self {
        Self
    }

    /// Calculate user preference scores based on reading history
    pub fn calculate_preference_scores(
        &self,
        user_ratings: &[(Book, f64)], // (book, rating) pairs
    ) -> HashMap<String, f64> {
        let mut genre_scores: HashMap<String, Vec<f64>> = HashMap::new();

        // Collect ratings for each genre
        for (book, rating) in user_ratings {
            for genre in &book.genre {
                genre_scores
                    .entry(genre.clone())
                    .or_insert_with(Vec::new)
                    .push(*rating);
            }
        }

        // Calculate average scores
        genre_scores
            .into_iter()
            .map(|(genre, ratings)| {
                let average = ratings.iter().sum::<f64>() / ratings.len() as f64;
                // Normalize to 0.0 - 1.0 scale (assuming ratings are 1.0 - 5.0)
                let normalized = (average - 1.0) / 4.0;
                (genre, normalized.max(0.0).min(1.0))
            })
            .collect()
    }

    /// Filter books based on user preferences and reading history
    pub fn filter_candidate_books(
        &self,
        all_books: Vec<Book>,
        user: &User,
        read_book_ids: &[Uuid],
        min_rating: f64,
    ) -> Vec<Book> {
        all_books
            .into_iter()
            .filter(|book| {
                // Not already read
                !read_book_ids.contains(&book.id) &&
                // Meets minimum rating threshold
                book.average_rating >= min_rating &&
                // Has overlapping genres with user preferences
                book.genre.iter().any(|genre| user.preferences.contains(genre))
            })
            .collect()
    }

    /// Score a book for a specific user
    pub fn score_book_for_user(
        &self,
        book: &Book,
        user_preferences: &HashMap<String, f64>,
        user_avg_rating: f64,
    ) -> f64 {
        let mut score = 0.0;
        let mut weight_sum = 0.0;

        // Genre preference score (weight: 0.6)
        let genre_weight = 0.6;
        let genre_score = book.genre
            .iter()
            .filter_map(|genre| user_preferences.get(genre))
            .fold(0.0, |acc, &pref_score| acc.max(pref_score));
        
        score += genre_score * genre_weight;
        weight_sum += genre_weight;

        // Book rating score (weight: 0.3)
        let rating_weight = 0.3;
        let rating_score = (book.average_rating - 1.0) / 4.0; // Normalize to 0-1
        score += rating_score * rating_weight;
        weight_sum += rating_weight;

        // Popularity score (weight: 0.1)
        let popularity_weight = 0.1;
        let popularity_score = (book.ratings_count.min(1000) as f64) / 1000.0; // Normalize to 0-1
        score += popularity_score * popularity_weight;
        weight_sum += popularity_weight;

        // Normalize final score
        if weight_sum > 0.0 {
            score / weight_sum
        } else {
            0.0
        }
    }

    /// Update user preferences based on new rating
    pub fn update_user_preferences(
        &self,
        current_preferences: HashMap<String, f64>,
        book: &Book,
        rating: f64,
        learning_rate: f64,
    ) -> HashMap<String, f64> {
        let mut updated_preferences = current_preferences;
        let normalized_rating = (rating - 3.0) / 2.0; // Convert 1-5 to -1 to 1

        for genre in &book.genre {
            let current_score = updated_preferences.get(genre).copied().unwrap_or(0.5);
            let adjustment = normalized_rating * learning_rate;
            let new_score = (current_score + adjustment).max(0.0).min(1.0);
            updated_preferences.insert(genre.clone(), new_score);
        }

        updated_preferences
    }

    /// Generate explanation for recommendation
    pub fn generate_recommendation_explanation(
        &self,
        book: &Book,
        user_preferences: &HashMap<String, f64>,
        score: f64,
    ) -> String {
        let top_matching_genres: Vec<String> = book.genre
            .iter()
            .filter_map(|genre| {
                user_preferences.get(genre).map(|&pref_score| (genre.clone(), pref_score))
            })
            .filter(|(_, pref_score)| *pref_score > 0.6)
            .map(|(genre, _)| genre)
            .take(2)
            .collect();

        if !top_matching_genres.is_empty() {
            format!(
                "You might like this {} book because you enjoy {} ({}% match)",
                book.genre.join("/"),
                top_matching_genres.join(" and "),
                (score * 100.0) as i32
            )
        } else if book.average_rating > 4.5 {
            format!(
                "Highly rated {} book ({}★) - {}% match based on your preferences",
                book.genre.join("/"),
                book.average_rating,
                (score * 100.0) as i32
            )
        } else {
            format!(
                "Recommended {} book based on your reading patterns ({}% match)",
                book.genre.join("/"),
                (score * 100.0) as i32
            )
        }
    }

    /// Validate recommendation parameters
    pub fn validate_recommendation_request(
        &self,
        user_id: Uuid,
        limit: Option<usize>,
    ) -> Result<usize> {
        if user_id.is_nil() {
            return Err(anyhow::anyhow!("Invalid user ID"));
        }

        let validated_limit = match limit {
            Some(l) if l > 100 => 100, // Cap at 100 recommendations
            Some(l) if l == 0 => return Err(anyhow::anyhow!("Limit must be greater than 0")),
            Some(l) => l,
            None => 10, // Default limit
        };

        Ok(validated_limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_book(title: &str, genres: Vec<&str>, rating: f64) -> Book {
        Book {
            id: Uuid::new_v4(),
            title: title.to_string(),
            author: "Test Author".to_string(),
            isbn: None,
            description: "Test description".to_string(),
            genre: genres.into_iter().map(String::from).collect(),
            tags: vec![],
            publication_year: 2023,
            publisher: "Test Publisher".to_string(),
            language: "English".to_string(),
            page_count: 300,
            cover_image_url: None,
            average_rating: rating,
            ratings_count: 100,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: Uuid::new_v4(),
        }
    }

    #[test]
    fn test_calculate_preference_scores() {
        let service = RecommendationService::new();
        
        let book1 = create_test_book("Book 1", vec!["Fiction", "Romance"], 4.0);
        let book2 = create_test_book("Book 2", vec!["Fiction", "Mystery"], 5.0);
        let book3 = create_test_book("Book 3", vec!["Romance"], 3.0);

        let user_ratings = vec![
            (book1, 4.0),
            (book2, 5.0),
            (book3, 3.0),
        ];

        let scores = service.calculate_preference_scores(&user_ratings);

        // Fiction: (4.0 + 5.0) / 2 = 4.5 -> normalized: (4.5 - 1.0) / 4.0 = 0.875
        assert!((scores["Fiction"] - 0.875).abs() < 0.01);
        
        // Romance: (4.0 + 3.0) / 2 = 3.5 -> normalized: (3.5 - 1.0) / 4.0 = 0.625
        assert!((scores["Romance"] - 0.625).abs() < 0.01);
        
        // Mystery: 5.0 -> normalized: (5.0 - 1.0) / 4.0 = 1.0
        assert!((scores["Mystery"] - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_score_book_for_user() {
        let service = RecommendationService::new();
        
        let book = create_test_book("Test Book", vec!["Fiction"], 4.5);
        
        let mut user_preferences = HashMap::new();
        user_preferences.insert("Fiction".to_string(), 0.8);
        
        let score = service.score_book_for_user(&book, &user_preferences, 4.0);
        
        // Should be a weighted average of genre preference (0.8), rating score, and popularity
        assert!(score > 0.0 && score <= 1.0);
    }
}