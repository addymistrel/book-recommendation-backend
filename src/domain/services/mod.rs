//! Domain services module
//! 
//! Contains domain services that implement business logic
//! that doesn't naturally fit within a single entity.

pub mod auth_service;
pub mod book_service;
pub mod recommendation_service;

pub use auth_service::*;
pub use book_service::*;
pub use recommendation_service::*;