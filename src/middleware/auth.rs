use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::web;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};
use redis::Commands;
use serde_json::json;

use crate::models::user::TokenClaims;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret = match req.app_data::<web::Data<String>>() {
        Some(secret) => secret.get_ref().clone(),
        None => {
            log::error!("JWT_SECRET not found in app_data");
            return Err((
                actix_web::error::ErrorInternalServerError(json!({
                    "error": "Internal server error"
                })),
                req,
            ));
        }
    };
    
    let redis_client = match req.app_data::<web::Data<redis::Client>>() {
        Some(client) => client.get_ref().clone(),
        None => {
            log::error!("Redis client not found in app_data");
            return Err((
                actix_web::error::ErrorInternalServerError(json!({
                    "error": "Internal server error"
                })),
                req,
            ));
        }
    };

    let token = credentials.token();
    log::debug!("Validating token: {}", token);
    
    let token_data = match decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(data) => {
            log::debug!("Token decoded successfully for user: {}", data.claims.sub);
            data
        },
        Err(e) => {
            log::error!("Token decode error: {}", e);
            return Err((
                actix_web::error::ErrorUnauthorized(json!({
                    "error": "Invalid token"
                })),
                req,
            ));
        }
    };

    let mut conn = match redis_client.get_connection() {
        Ok(conn) => {
            log::debug!("Redis connection established");
            conn
        },
        Err(e) => {
            log::error!("Redis connection error: {}", e);
            return Err((
                actix_web::error::ErrorUnauthorized(json!({
                    "error": "Redis connection error"
                })),
                req,
            ));
        }
    };

    // Obtener la sesión del usuario
    let user_session_key = format!("user_session:{}", token_data.claims.sub);
    log::debug!("Looking for user session with key: {}", user_session_key);
    
    let session_id: Option<String> = match conn.get(&user_session_key) {
        Ok(session) => {
            log::debug!("Found session ID: {:?}", session);
            session
        },
        Err(e) => {
            log::error!("Redis error getting user session: {}", e);
            return Err((
                actix_web::error::ErrorUnauthorized(json!({
                    "error": "Redis error"
                })),
                req,
            ));
        }
    };

    match session_id {
        Some(session_id) => {
            // Verificar la sesión
            let session_key = format!("session:{}", session_id);
            log::debug!("Looking for session data with key: {}", session_key);
            
            let session_data: Option<String> = match conn.get(&session_key) {
                Ok(data) => {
                    log::debug!("Found session data");
                    data
                },
                Err(e) => {
                    log::error!("Redis error getting session data: {}", e);
                    return Err((
                        actix_web::error::ErrorUnauthorized(json!({
                            "error": "Redis error"
                        })),
                        req,
                    ));
                }
            };

            match session_data {
                Some(data) => {
                    let session: serde_json::Value = match serde_json::from_str(&data) {
                        Ok(session) => {
                            log::debug!("Session data parsed successfully");
                            session
                        },
                        Err(e) => {
                            log::error!("Error parsing session data: {}", e);
                            return Err((
                                actix_web::error::ErrorUnauthorized(json!({
                                    "error": "Invalid session data"
                                })),
                                req,
                            ));
                        }
                    };

                    let session_token = session["token"].as_str();
                    log::debug!("Comparing tokens - Session: {:?}, Request: {}", session_token, token);
                    
                    if session_token == Some(token) {
                        log::debug!("Token validation successful");
                        log::debug!("Inserting user ID into extensions: {}", token_data.claims.sub);
                        req.extensions_mut().insert(token_data.claims.sub as i64);
                        Ok(req)
                    } else {
                        log::error!("Token mismatch");
                        Err((
                            actix_web::error::ErrorUnauthorized(json!({
                                "error": "Invalid or expired session"
                            })),
                            req,
                        ))
                    }
                }
                None => {
                    log::error!("Session data not found");
                    Err((
                        actix_web::error::ErrorUnauthorized(json!({
                            "error": "Session not found"
                        })),
                        req,
                    ))
                },
            }
        }
        None => {
            log::error!("No active session found for user");
            Err((
                actix_web::error::ErrorUnauthorized(json!({
                    "error": "No active session"
                })),
                req,
            ))
        },
    }
} 