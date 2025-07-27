use actix_web::web;

use crate::presentation::controllers::recommendation_controller::{
    get_recommendations, update_preferences, record_recommendation_click
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/recommendations")
            .route("", web::get().to(get_recommendations))
            .route("/preferences", web::put().to(update_preferences))
            .route("/{recommendation_id}/click", web::post().to(record_recommendation_click))
    );
}