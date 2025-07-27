//! Repository traits module
//! 
//! Defines the interfaces for data access without specifying implementation details.
//! This follows the dependency inversion principle.

pub mod user_repository;
pub mod book_repository;
pub mod recommendation_repository;

pub use user_repository::*;
pub use book_repository::*;
pub use recommendation_repository::*;