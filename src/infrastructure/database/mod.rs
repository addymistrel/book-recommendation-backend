//! Database implementations module

pub mod user_repository_impl;
pub mod book_repository_impl;
pub mod recommendation_repository_impl;

pub use user_repository_impl::*;
pub use book_repository_impl::*;
pub use recommendation_repository_impl::*;