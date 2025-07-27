use actix_web::web;

use crate::presentation::controllers::book_controller::{
    create_book, get_books, upload_book_image, rate_book
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/books")
            .route("", web::post().to(create_book))
            .route("", web::get().to(get_books))
            .route("/{book_id}/upload-image", web::post().to(upload_book_image))
            .route("/{book_id}/rate", web::post().to(rate_book))
    );
}