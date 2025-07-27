use actix_cors::Cors;
use actix_web::http::header;

pub fn create_cors_middleware() -> Cors {
    Cors::default()
        .allowed_origin_fn(|origin, _req_head| {
            // In production, replace with specific origins
            origin.as_bytes().starts_with(b"http://localhost") ||
            origin.as_bytes().starts_with(b"https://localhost") ||
            origin.as_bytes().starts_with(b"http://127.0.0.1") ||
            origin.as_bytes().starts_with(b"https://yourdomain.com")
        })
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .expose_headers(vec![header::CONTENT_DISPOSITION])
        .max_age(3600)
        .supports_credentials()
}