use actix_web::{web, HttpResponse, Result as ActixResult, HttpRequest};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use validator::Validate;
use uuid::Uuid;

use crate::{
    application::{
        dtos::book_dtos::{CreateBookRequest, BookSearchQuery, BookRatingRequest},
        use_cases::books::{
            create_book::CreateBookUseCase,
            get_books::GetBooksUseCase,
            upload_book_image::UploadBookImageUseCase,
            rate_book::RateBookUseCase,
        },
    },
    domain::{
        repositories::book_repository::BookRepository,
        services::auth_service::Claims,
    },
};

#[utoipa::path(
    post,
    path = "/api/books",
    request_body = CreateBookRequest,
    responses(
        (status = 201, description = "Book created successfully", body = BookResponse),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Books",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_book(
    req: HttpRequest,
    book_req: web::Json<CreateBookRequest>,
    create_book_use_case: web::Data<CreateBookUseCase<BookRepository>>,
) -> ActixResult<HttpResponse> {
    // Extract user from JWT claims
    let claims = req.extensions().get::<Claims>().unwrap();
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    // Validate input
    if let Err(validation_errors) = book_req.validate() {
        return Ok(HttpResponse::BadRequest().json(validation_errors));
    }

    match create_book_use_case.execute(book_req.into_inner(), user_id).await {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(e) => {
            tracing::error!("Book creation failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Book creation failed"))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/books",
    params(
        ("title" = Option<String>, Query, description = "Filter by title"),
        ("author" = Option<String>, Query, description = "Filter by author"),
        ("genre" = Option<String>, Query, description = "Filter by genre"),
        ("limit" = Option<usize>, Query, description = "Limit results"),
        ("offset" = Option<usize>, Query, description = "Offset for pagination")
    ),
    responses(
        (status = 200, description = "Books retrieved successfully", body = Vec<BookResponse>)
    ),
    tag = "Books"
)]
pub async fn get_books(
    query: web::Query<BookSearchQuery>,
    get_books_use_case: web::Data<GetBooksUseCase<BookRepository>>,
) -> ActixResult<HttpResponse> {
    match get_books_use_case.execute(query.into_inner()).await {
        Ok(books) => Ok(HttpResponse::Ok().json(books)),
        Err(e) => {
            tracing::error!("Failed to get books: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to retrieve books"))
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/books/{book_id}/upload-image",
    params(
        ("book_id" = Uuid, Path, description = "Book ID")
    ),
    request_body(content = String, description = "Image file", content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Image uploaded successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Book not found")
    ),
    tag = "Books",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn upload_book_image(
    req: HttpRequest,
    path: web::Path<Uuid>,
    mut payload: Multipart,
    upload_use_case: web::Data<UploadBookImageUseCase<BookRepository>>,
) -> ActixResult<HttpResponse> {
    let book_id = path.into_inner();
    
    // Extract user from JWT claims
    let claims = req.extensions().get::<Claims>().unwrap();
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    // Extract file from multipart
    while let Some(mut field) = payload.next().await {
        let field = field?;
        let content_disposition = field.content_disposition();
        
        if let Some(filename) = content_disposition.get_filename() {
            let mut file_data = Vec::new();
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                file_data.extend_from_slice(&data);
            }

            match upload_use_case.execute(book_id, user_id, file_data, filename).await {
                Ok(image_url) => return Ok(HttpResponse::Ok().json(serde_json::json!({
                    "message": "Image uploaded successfully",
                    "image_url": image_url
                }))),
                Err(e) => {
                    tracing::error!("Image upload failed: {:?}", e);
                    return Ok(HttpResponse::InternalServerError().json("Image upload failed"));
                }
            }
        }
    }

    Ok(HttpResponse::BadRequest().json("No file provided"))
}

#[utoipa::path(
    post,
    path = "/api/books/{book_id}/rate",
    params(
        ("book_id" = Uuid, Path, description = "Book ID")
    ),
    request_body = BookRatingRequest,
    responses(
        (status = 201, description = "Rating added successfully", body = BookRatingResponse),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Book not found")
    ),
    tag = "Books",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn rate_book(
    req: HttpRequest,
    path: web::Path<Uuid>,
    rating_req: web::Json<BookRatingRequest>,
    rate_book_use_case: web::Data<RateBookUseCase<BookRepository>>,
) -> ActixResult<HttpResponse> {
    let book_id = path.into_inner();
    
    // Extract user from JWT claims
    let claims = req.extensions().get::<Claims>().unwrap();
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    // Validate input
    if let Err(validation_errors) = rating_req.validate() {
        return Ok(HttpResponse::BadRequest().json(validation_errors));
    }

    match rate_book_use_case.execute(book_id, user_id, rating_req.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(e) => {
            tracing::error!("Rating failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Rating failed"))
        }
    }
}