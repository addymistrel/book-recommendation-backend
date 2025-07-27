use actix_web::{web, HttpResponse, Result as ActixResult, HttpRequest};
use uuid::Uuid;
use validator::Validate;

use crate::{
    application::{
        dtos::recommendation_dtos::{GetRecommendationsRequest, UpdatePreferencesRequest},
        use_cases::recommendations::get_recommendations::GetRecommendationsUseCase,
    },
    domain::{
        repositories::{user_repository::UserRepository, book_repository::BookRepository},
        services::auth_service::Claims,
    },
};

#[utoipa::path(
    get,
    path = "/api/recommendations",
    params(
        ("limit" = Option<usize>, Query, description = "Number of recommendations to return"),
        ("include_reasons" = Option<bool>, Query, description = "Include recommendation reasons")
    ),
    responses(
        (status = 200, description = "Recommendations retrieved successfully", body = RecommendationResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Recommendations",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_recommendations(
    req: HttpRequest,
    query: web::Query<GetRecommendationsRequest>,
    recommendation_use_case: web::Data<GetRecommendationsUseCase<UserRepository, BookRepository>>,
) -> ActixResult<HttpResponse> {
    // Extract user from JWT claims
    let claims = req.extensions().get::<Claims>().unwrap();
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    // Validate input
    if let Err(validation_errors) = query.validate() {
        return Ok(HttpResponse::BadRequest().json(validation_errors));
    }

    match recommendation_use_case.execute(user_id, query.limit).await {
        Ok(recommendations) => Ok(HttpResponse::Ok().json(recommendations)),
        Err(e) => {
            tracing::error!("Failed to get recommendations: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to get recommendations"))
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/recommendations/preferences",
    request_body = UpdatePreferencesRequest,
    responses(
        (status = 200, description = "Preferences updated successfully"),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Recommendations",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_preferences(
    req: HttpRequest,
    preferences_req: web::Json<UpdatePreferencesRequest>,
    user_repository: web::Data<dyn UserRepository>,
) -> ActixResult<HttpResponse> {
    // Extract user from JWT claims
    let claims = req.extensions().get::<Claims>().unwrap();
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    // Validate input
    if let Err(validation_errors) = preferences_req.validate() {
        return Ok(HttpResponse::BadRequest().json(validation_errors));
    }

    match user_repository.update_preferences(user_id, preferences_req.preferences.clone()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Preferences updated successfully"
        }))),
        Err(e) => {
            tracing::error!("Failed to update preferences: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to update preferences"))
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/recommendations/{recommendation_id}/click",
    params(
        ("recommendation_id" = Uuid, Path, description = "Recommendation ID")
    ),
    responses(
        (status = 200, description = "Click recorded successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Recommendation not found")
    ),
    tag = "Recommendations",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn record_recommendation_click(
    req: HttpRequest,
    path: web::Path<Uuid>,
    recommendation_repository: web::Data<dyn crate::domain::repositories::recommendation_repository::RecommendationRepository>,
) -> ActixResult<HttpResponse> {
    let recommendation_id = path.into_inner();
    
    // Extract user from JWT claims (for logging purposes)
    let _claims = req.extensions().get::<Claims>().unwrap();

    match recommendation_repository.mark_recommendation_clicked(recommendation_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Click recorded successfully"
        }))),
        Err(e) => {
            tracing::error!("Failed to record click: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to record click"))
        }
    }
}