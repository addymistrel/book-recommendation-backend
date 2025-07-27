use actix_web::{web, HttpResponse, Result as ActixResult};
use validator::Validate;

use crate::application::{
    dtos::auth_dtos::{RegisterUserRequest, LoginRequest},
    use_cases::auth::{register_user::RegisterUserUseCase, authenticate_user::AuthenticateUserUseCase},
};

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterUserRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "User already exists")
    ),
    tag = "Authentication"
)]
pub async fn register(
    req: web::Json<RegisterUserRequest>,
    register_use_case: web::Data<RegisterUserUseCase<UserRepository>>,
) -> ActixResult<HttpResponse> {
    // Validate input
    if let Err(validation_errors) = req.validate() {
        return Ok(HttpResponse::BadRequest().json(validation_errors));
    }

    match register_use_case.execute(req.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(e) => {
            tracing::error!("Registration failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Registration failed"))
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Authentication"
)]
pub async fn login(
    req: web::Json<LoginRequest>,
    auth_use_case: web::Data<AuthenticateUserUseCase<UserRepository>>,
) -> ActixResult<HttpResponse> {
    match auth_use_case.execute(req.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::Unauthorized().json("Invalid credentials")),
    }
}