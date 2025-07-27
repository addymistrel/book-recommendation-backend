//! Use cases module
//! 
//! Contains all application use cases that represent the application's
//! business operations and workflows.

pub mod auth;
pub mod books;
pub mod recommendations;

pub use auth::*;
pub use books::*;
pub use recommendations::*;