use anyhow::Result;
use uuid::Uuid;

use crate::domain::repositories::book_repository::BookRepository;
use crate::infrastructure::external::cloudinary::CloudinaryService;
use crate::domain::errors::domain_error::DomainError;

pub struct UploadBookImageUseCase<R: BookRepository> {
    book_repository: R,
    cloudinary_service: CloudinaryService,
}

impl<R: BookRepository> UploadBookImageUseCase<R> {
    pub fn new(book_repository: R, cloudinary_service: CloudinaryService) -> Self {
        Self {
            book_repository,
            cloudinary_service,
        }
    }

    pub async fn execute(
        &self,
        book_id: Uuid,
        user_id: Uuid,
        image_data: Vec<u8>,
        filename: &str,
    ) -> Result<String> {
        // Check if book exists and user has permission to upload
        let mut book = self.book_repository
            .find_by_id(book_id)
            .await?
            .ok_or(DomainError::BookNotFound)?;

        // Check if user created the book (basic authorization)
        if book.created_by != user_id {
            return Err(DomainError::Unauthorized.into());
        }

        // Validate image (basic validation)
        if image_data.is_empty() {
            return Err(DomainError::InvalidInput("Empty image data".to_string()).into());
        }

        // Upload to Cloudinary
        let upload_result = self.cloudinary_service
            .upload_image(image_data, filename, "book-covers")
            .await
            .map_err(|e| DomainError::ExternalServiceError(e.to_string()))?;

        // Update book with image URL
        book.cover_image_url = Some(upload_result.secure_url.clone());
        book.updated_at = chrono::Utc::now();

        self.book_repository.update(book).await?;

        Ok(upload_result.secure_url)
    }
}