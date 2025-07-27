use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Username already taken")]
    UsernameAlreadyTaken,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Book not found")]
    BookNotFound,
    
    #[error("Unauthorized access")]
    Unauthorized,
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("External service error: {0}")]
    ExternalServiceError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("ML model error: {0}")]
    MLModelError(String),
}