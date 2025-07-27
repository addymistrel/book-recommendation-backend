//! Configuration module for the application
//! 
//! This module handles all configuration-related functionality
//! including database setup, settings management, and environment variables.

pub mod settings;
pub mod database;

pub use settings::*;
pub use database::*;