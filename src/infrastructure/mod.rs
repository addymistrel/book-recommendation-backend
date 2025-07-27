//! Infrastructure layer module
//! 
//! Contains implementations of repositories, external services,
//! and other infrastructure concerns.

pub mod database;
pub mod external;
pub mod middleware;

pub use database::*;
pub use external::*;
pub use middleware::*;