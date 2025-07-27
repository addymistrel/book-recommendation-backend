use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod domain;
mod application;
mod infrastructure;
mod presentation;
mod di;

use config::database::initialize_database;
use presentation::routes::{auth_routes, book_routes, recommendation_routes};

#[derive(OpenApi)]
#[openapi(
    paths(
        presentation::controllers::auth_controller::register,
        presentation::controllers::auth_controller::login,
        presentation::controllers::book_controller::create_book,
        presentation::controllers::book_controller::get_books,
        presentation::controllers::recommendation_controller::get_recommendations,
    ),
    components(
        schemas(
            application::dtos::auth_dtos::RegisterUserRequest,
            application::dtos::auth_dtos::LoginRequest,
            application::dtos::auth_dtos::AuthResponse,
            application::dtos::book_dtos::CreateBookRequest,
            domain::entities::user::User,
            domain::entities::book::Book,
        )
    ),
    tags(
        (name = "Authentication", description = "User authentication endpoints"),
        (name = "Books", description = "Book management endpoints"),
        (name = "Recommendations", description = "Book recommendation endpoints")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::init();

    // Initialize database
    let db = initialize_database(
        &env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        &env::var("DATABASE_NAMESPACE").unwrap_or_else(|_| "book_rec".to_string()),
        &env::var("DATABASE_NAME").unwrap_or_else(|_| "main".to_string()),
    )
    .await
    .expect("Failed to initialize database");

    // Create dependency injection container
    let container = di::container::AppModule::builder()
        .with_component_override::<dyn UserRepository>(Box::new(
            infrastructure::database::user_repository_impl::UserRepositoryImpl::new(db.clone())
        ))
        .with_component_override::<dyn BookRepository>(Box::new(
            infrastructure::database::book_repository_impl::BookRepositoryImpl::new(db.clone())
        ))
        .build();

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(container.clone()))
            .wrap(Logger::default())
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(
                web::scope("/api")
                    .configure(auth_routes::configure)
                    .configure(book_routes::configure)
                    .configure(recommendation_routes::configure)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}