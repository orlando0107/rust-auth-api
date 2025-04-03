use actix_web::{web, HttpResponse, Responder, HttpMessage};
use actix_web_httpauth::middleware::HttpAuthentication;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use redis::Commands;
use serde_json::json;
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};
use validator::Validate;
use uuid::Uuid;

use crate::models::user::{LoginUser, NewUser, TokenClaims, User};
use crate::middleware::auth::validator;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);
    
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout).wrap(auth)),
    );
}

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = NewUser,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
async fn register(
    user: web::Json<NewUser>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    if let Err(errors) = user.0.validate() {
        return HttpResponse::BadRequest().json(json!({ "errors": errors }));
    }

    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            log::error!("Error hashing password: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            }));
        }
    };
    
    let result = match sqlx::query_as::<_, (i64,)>(
        "INSERT INTO users (email, password, name) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(&user.email)
    .bind(&hashed_password)
    .bind(&user.name)
    .fetch_one(&**pool)
    .await {
        Ok(record) => record,
        Err(e) => {
            log::error!("Error en la base de datos: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Database error: {:?}", e)
            }));
        }
    };

    HttpResponse::Created().json(json!({
        "id": result.0,
        "email": user.email,
        "name": user.name
    }))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginUser,
    responses(
        (status = 200, description = "Login successful", body = TokenResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
pub async fn login(
    credentials: web::Json<LoginUser>,
    pool: web::Data<PgPool>,
    redis_client: web::Data<redis::Client>,
    jwt_secret: web::Data<String>,
) -> impl Responder {
    let user = match sqlx::query_as::<_, User>(
        "SELECT id, email, password, name FROM users WHERE email = $1",
    )
    .bind(&credentials.email)
    .fetch_optional(&**pool)
    .await {
        Ok(user) => user,
        Err(e) => {
            log::error!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "error": "Database error"
            }));
        }
    };

    match user {
        Some(user) => {
            if verify(&credentials.password, &user.password).unwrap_or(false) {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                
                let session_id = Uuid::new_v4().to_string();
                let claims = TokenClaims {
                    sub: user.id,
                    exp: now + 3600, // 1 hora
                };

                let token = match encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(jwt_secret.as_bytes()),
                ) {
                    Ok(token) => token,
                    Err(e) => {
                        log::error!("Error generating token: {}", e);
                        return HttpResponse::InternalServerError().json(json!({
                            "error": "Error generating token"
                        }));
                    }
                };

                // Almacenar sesión en Redis
                let mut conn = match redis_client.get_connection() {
                    Ok(conn) => conn,
                    Err(e) => {
                        log::error!("Redis connection error: {}", e);
                        return HttpResponse::InternalServerError().json(json!({
                            "error": "Redis connection error"
                        }));
                    }
                };

                // Almacenar información de la sesión
                let session_data = json!({
                    "user_id": user.id,
                    "email": user.email,
                    "name": user.name,
                    "token": token.clone(),
                    "created_at": now,
                    "expires_at": now + 3600
                }).to_string();

                if let Err(e) = conn.set_ex::<String, String, ()>(
                    format!("session:{}", session_id),
                    session_data,
                    3600
                ) {
                    log::error!("Redis error: {}", e);
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Redis error"
                    }));
                }

                // Almacenar la relación usuario-sesión
                if let Err(e) = conn.set_ex::<String, String, ()>(
                    format!("user_session:{}", user.id),
                    session_id.clone(),
                    3600
                ) {
                    log::error!("Redis error: {}", e);
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Redis error"
                    }));
                }

                HttpResponse::Ok().json(json!({
                    "session_id": session_id,
                    "token": token,
                    "user": {
                        "id": user.id,
                        "email": user.email,
                        "name": user.name
                    }
                }))
            } else {
                HttpResponse::Unauthorized().json(json!({
                    "error": "Invalid credentials"
                }))
            }
        }
        None => HttpResponse::Unauthorized().json(json!({
            "error": "Invalid credentials"
        })),
    }
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
pub async fn logout(
    req: actix_web::HttpRequest,
    redis_client: web::Data<redis::Client>,
) -> impl Responder {
    let user_id = match req.extensions().get::<i64>() {
        Some(id) => *id,
        None => {
            log::error!("User ID not found in request extensions");
            return HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            }));
        }
    };

    let mut conn = match redis_client.get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Redis connection error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "error": "Redis connection error"
            }));
        }
    };

    // Obtener la sesión del usuario
    let session_id: Option<String> = match conn.get(format!("user_session:{}", user_id)) {
        Ok(session) => session,
        Err(e) => {
            log::error!("Redis error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "error": "Redis error"
            }));
        }
    };

    if let Some(session_id) = session_id {
        // Eliminar la sesión
        if let Err(e) = conn.del::<String, ()>(format!("session:{}", session_id)) {
            log::error!("Redis error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "error": "Redis error"
            }));
        }

        // Eliminar la relación usuario-sesión
        if let Err(e) = conn.del::<String, ()>(format!("user_session:{}", user_id)) {
            log::error!("Redis error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "error": "Redis error"
            }));
        }
    }
    
    HttpResponse::Ok().json(json!({
        "message": "Successfully logged out"
    }))
} 