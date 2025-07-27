//! Data Transfer Objects module
//! 
//! Contains all DTOs used for communication between layers
//! and external API contracts.

pub mod auth_dtos;
pub mod book_dtos;
pub mod recommendation_dtos;

pub use auth_dtos::*;
pub use book_dtos::*;
pub use recommendation_dtos::*;