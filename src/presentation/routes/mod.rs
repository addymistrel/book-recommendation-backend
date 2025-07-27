//! Routes module

pub mod auth_routes;
pub mod book_routes;
pub mod recommendation_routes;

pub use auth_routes::*;
pub use book_routes::*;
pub use recommendation_routes::*;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth_routes::configure)
       .configure(book_routes::configure)
       .configure(recommendation_routes::configure);
}