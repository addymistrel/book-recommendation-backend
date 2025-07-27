use anyhow::Result;
use uuid::Uuid;

use crate::domain::{
    entities::user::User,
    repositories::user_repository::UserRepository,
    services::auth_service::AuthService,
};
use crate::application::dtos::auth_dtos::{RegisterUserRequest, AuthResponse, UserDto};
use crate::domain::errors::domain_error::DomainError;

pub struct RegisterUserUseCase<R: UserRepository> {
    user_repository: R,
    auth_service: AuthService,
}

impl<R: UserRepository> RegisterUserUseCase<R> {
    pub fn new(user_repository: R, auth_service: AuthService) -> Self {
        Self {
            user_repository,
            auth_service,
        }
    }

    pub async fn execute(&self, request: RegisterUserRequest) -> Result<AuthResponse> {
        // Check if user exists
        if self.user_repository.find_by_email(&request.email).await?.is_some() {
            return Err(DomainError::UserAlreadyExists.into());
        }

        if self.user_repository.find_by_username(&request.username).await?.is_some() {
            return Err(DomainError::UsernameAlreadyTaken.into());
        }

        // Hash password
        let password_hash = self.auth_service.hash_password(&request.password)?;

        // Create user
        let user = User::new(
            request.email,
            request.username.clone(),
            password_hash,
            request.first_name,
            request.last_name,
        );

        // Save user
        let saved_user = self.user_repository.save(user).await?;

        // Generate JWT
        let token = self.auth_service.generate_jwt(saved_user.id, &saved_user.username)?;

        Ok(AuthResponse {
            token,
            user: UserDto {
                id: saved_user.id.to_string(),
                email: saved_user.email,
                username: saved_user.username,
                first_name: saved_user.first_name,
                last_name: saved_user.last_name,
            },
        })
    }
}