use anyhow::Result;
use async_trait::async_trait;
use shaku::{Component, Interface, module, provide};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    application::use_cases::auth::{
        authenticate_user::AuthenticateUserUseCase, register_user::RegisterUserUseCase,
    },
    config::database::Database,
    domain::{
        entities::user::User,
        repositories::{book_repository::BookRepository, user_repository::UserRepository},
        services::auth_service::AuthService,
    },
    infrastructure::{
        database::{
            book_repository_impl::BookRepositoryImpl, user_repository_impl::UserRepositoryImpl,
        },
        external::cloudinary::CloudinaryService,
    },
};

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositoryComponent {
    #[shaku(inject)]
    db: Arc<Database>,
}

impl UserRepositoryComponent {
    fn get_impl(&self) -> UserRepositoryImpl {
        UserRepositoryImpl::new(self.db.get_connection())
    }
}

#[async_trait]
impl UserRepository for UserRepositoryComponent {
    async fn save(&self, user: User) -> Result<User> {
        self.get_impl().save(user).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        self.get_impl().find_by_id(id).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        self.get_impl().find_by_email(email).await
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        self.get_impl().find_by_username(username).await
    }

    async fn update(&self, user: User) -> Result<User> {
        self.get_impl().update(user).await
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        self.get_impl().delete(id).await
    }

    async fn find_all(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<User>> {
        self.get_impl().find_all(limit, offset).await
    }

    async fn update_preferences(&self, user_id: Uuid, preferences: Vec<String>) -> Result<()> {
        self.get_impl()
            .update_preferences(user_id, preferences)
            .await
    }

    async fn deactivate_user(&self, id: Uuid) -> Result<()> {
        self.get_impl().deactivate_user(id).await
    }

    async fn activate_user(&self, id: Uuid) -> Result<()> {
        self.get_impl().activate_user(id).await
    }
}

// You'll need to create a similar component for BookRepository
#[derive(Component)]
#[shaku(interface = BookRepository)]
pub struct BookRepositoryComponent {
    #[shaku(inject)]
    db: Arc<Database>,
}

impl BookRepositoryComponent {
    fn get_impl(&self) -> BookRepositoryImpl {
        BookRepositoryImpl::new(self.db.get_connection())
    }
}

// Implement BookRepository for BookRepositoryComponent here...

module! {
    pub AppModule {
        components = [UserRepositoryComponent, BookRepositoryComponent],
        providers = [
            AuthServiceProvider,
            CloudinaryServiceProvider,
            RegisterUserUseCaseProvider,
        ]
    }
}

#[provide]
impl AuthServiceProvider for AppModule {
    type Provide = AuthService;

    fn provide(_: &AppModule) -> Result<AuthService, Box<dyn std::error::Error + Send + Sync>> {
        Ok(AuthService::new(
            std::env::var("JWT_SECRET")?,
            24, // 24 hours
        ))
    }
}
