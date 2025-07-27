//! Domain entities module
//! 
//! Contains all business entities that represent the core concepts
//! of the book recommendation system.

pub mod user;
pub mod book;
pub mod recommendation;

pub use user::*;
pub use book::*;
pub use recommendation::*;