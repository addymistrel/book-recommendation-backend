//! Ports module for hexagonal architecture
//! 
//! Defines the interfaces that the application layer expects
//! from external systems (adapters).

pub mod auth_port;
pub mod book_port;
pub mod recommendation_port;

pub use auth_port::*;
pub use book_port::*;
pub use recommendation_port::*;