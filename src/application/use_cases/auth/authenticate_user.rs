use anyhow::Result;

use crate::domain::{
    repositories::user_repository::UserRepository,
    services::auth_service::AuthService,
    errors::domain_error::DomainError,
};
use crate::application::dtos::auth_dtos::{LoginRequest, AuthResponse, UserDto};

pub struct AuthenticateUserUseCase<R: UserRepository> {
    user_repository: R,
    auth_service: AuthService,
}

impl<R: UserRepository> AuthenticateUserUseCase<R> {
    pub fn new(user_repository: R, auth_service: AuthService) -> Self {
        Self {
            user_repository,
            auth_service,
        }
    }

    pub async fn execute(&self, request: LoginRequest) -> Result<AuthResponse> {
        // Find user by email
        let user = self.user_repository
            .find_by_email(&request.email)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        // Check if user is active
        if !user.is_active {
            return Err(DomainError::InvalidCredentials.into());
        }

        // Verify password
        if !self.auth_service.verify_password(&request.password, &user.password_hash)? {
            return Err(DomainError::InvalidCredentials.into());
        }

        // Generate JWT token
        let token = self.auth_service.generate_jwt(user.id, &user.username)?;

        Ok(AuthResponse {
            token,
            user: UserDto {
                id: user.id.to_string(),
                email: user.email,
                username: user.username,
                first_name: user.first_name,
                last_name: user.last_name,
            },
        })
    }
}