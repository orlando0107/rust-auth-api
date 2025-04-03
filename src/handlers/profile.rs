use actix_web::{web, HttpResponse, Responder, HttpMessage};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde_json::json;
use sqlx::PgPool;

use crate::middleware::auth::validator;
use crate::models::user::User;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);
    
    cfg.service(
        web::scope("/profile")
            .wrap(auth)
            .route("", web::get().to(get_profile)),
    );
}

#[utoipa::path(
    get,
    path = "/profile",
    responses(
        (status = 200, description = "Profile retrieved successfully", body = User),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "profile"
)]
pub async fn get_profile(
    req: actix_web::HttpRequest,
    pool: web::Data<PgPool>,
) -> impl Responder {
    log::debug!("Getting profile for request");
    log::debug!("Request extensions: {:?}", req.extensions().get::<i64>());
    
    let user_id = match req.extensions().get::<i64>() {
        Some(id) => {
            log::debug!("Found user ID in extensions: {}", id);
            *id
        },
        None => {
            log::error!("User ID not found in request extensions");
            return HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            }));
        }
    };

    log::debug!("Querying database for user with ID: {}", user_id);
    
    let user = match sqlx::query_as::<_, User>(
        "SELECT id, email, password, name FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(&**pool)
    .await {
        Ok(user) => {
            log::debug!("Database query completed");
            user
        },
        Err(e) => {
            log::error!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "error": "Database error"
            }));
        }
    };

    match user {
        Some(user) => {
            log::debug!("User found: {:?}", user);
            HttpResponse::Ok().json(json!({
                "id": user.id,
                "email": user.email,
                "name": user.name
            }))
        },
        None => {
            log::error!("User not found with ID: {}", user_id);
            HttpResponse::NotFound().json(json!({
                "error": "User not found"
            }))
        },
    }
} 