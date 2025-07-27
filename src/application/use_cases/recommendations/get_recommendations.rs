use anyhow::Result;
use uuid::Uuid;

use crate::{
    domain::repositories::{
        user_repository::UserRepository,
        book_repository::BookRepository,
    },
    infrastructure::external::ml_model::{MLModelService, RecommendationInput},
    application::dtos::recommendation_dtos::RecommendationResponse,
};

pub struct GetRecommendationsUseCase<UR: UserRepository, BR: BookRepository> {
    user_repository: UR,
    book_repository: BR,
    ml_service: MLModelService,
}

impl<UR: UserRepository, BR: BookRepository> GetRecommendationsUseCase<UR, BR> {
    pub fn new(
        user_repository: UR,
        book_repository: BR,
        ml_service: MLModelService,
    ) -> Self {
        Self {
            user_repository,
            book_repository,
            ml_service,
        }
    }

    pub async fn execute(&self, user_id: Uuid, limit: Option<usize>) -> Result<RecommendationResponse> {
        // Get user profile and preferences
        let user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Get user's reading history and ratings
        let user_ratings = self.book_repository.get_user_ratings(user_id).await?;
        let reading_history: Vec<Uuid> = user_ratings.iter().map(|r| r.book_id).collect();
        let ratings: Vec<f64> = user_ratings.iter().map(|r| r.rating).collect();

        // Prepare ML model input
        let ml_input = RecommendationInput {
            user_id,
            user_preferences: user.preferences,
            reading_history,
            ratings,
        };

        // Get recommendations from ML model
        let ml_output = self.ml_service.get_recommendations(ml_input)?;

        // Fetch book details for recommended books
        let mut recommended_books = Vec::new();
        let limit = limit.unwrap_or(10).min(ml_output.book_ids.len());
        
        for i in 0..limit {
            if let Some(book) = self.book_repository.find_by_id(ml_output.book_ids[i]).await? {
                recommended_books.push(crate::application::dtos::book_dtos::BookResponse {
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
                });
            }
        }

        Ok(RecommendationResponse {
            recommendations: recommended_books,
            confidence: ml_output.confidence,
            total_count: ml_output.book_ids.len(),
        })
    }
}