use actix_web::{test, web, App};
use surrealdb::{engine::local::Mem, Surreal};
use std::sync::Arc;

use book_recommendation_backend::{
    config::database::Database,
    di::container::AppModule,
    presentation::routes::configure_routes,
};

pub async fn create_test_app() -> impl actix_web::dev::Service<
    actix_web::dev::ServiceRequest,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    // Create in-memory database for testing
    let db: Surreal<surrealdb::engine::local::Db> = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    // Set up test data
    setup_test_data(&db).await;

    // Create DI container with test implementations
    let container = create_test_container(db).await;

    test::init_service(
        App::new()
            .app_data(web::Data::new(container))
            .configure(configure_routes)
    ).await
}

async fn setup_test_data(db: &Surreal<surrealdb::engine::local::Db>) {
    // Create test schema
    db.query(include_str!("../../migrations/001_initial_schema.surql"))
        .await
        .unwrap();
}

async fn create_test_container(db: Surreal<surrealdb::engine::local::Db>) -> AppModule {
    // Create test container with in-memory database
    AppModule::builder()
        .with_component_override::<dyn UserRepository>(Box::new(
            UserRepositoryImpl::new(db.clone())
        ))
        .with_component_override::<dyn BookRepository>(Box::new(
            BookRepositoryImpl::new(db.clone())  
        ))
        .build()
}

pub fn create_test_user() -> crate::application::dtos::auth_dtos::RegisterUserRequest {
    crate::application::dtos::auth_dtos::RegisterUserRequest {
        email: "test@example.com".to_string(),
        username: "testuser".to_string(),
        password: "password123".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
    }
}

pub fn create_test_book() -> crate::application::dtos::book_dtos::CreateBookRequest {
    crate::application::dtos::book_dtos::CreateBookRequest {
        title: "Test Book".to_string(),
        author: "Test Author".to_string(),
        isbn: Some("9781234567890".to_string()),
        description: "This is a test book for unit testing purposes.".to_string(),
        genre: vec!["Fiction".to_string(), "Test".to_string()],
        tags: vec!["test".to_string(), "sample".to_string()],
        publication_year: 2023,
        publisher: "Test Publisher".to_string(),
        language: "English".to_string(),
        page_count: 300,
    }
}