//! Book Recommendation Backend Library
//! 
//! This library provides a complete book recommendation system
//! built with clean architecture principles.

pub mod config;
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
pub mod di;

pub use config::*;
pub use domain::*;
pub use application::*;
pub use infrastructure::*;
pub use presentation::*;
pub use di::*;