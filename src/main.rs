use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod handlers;
mod models;
mod services;
mod middleware;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth::register,
        handlers::auth::login,
        handlers::auth::logout,
        handlers::profile::get_profile
    ),
    components(
        schemas(models::user::User, models::user::NewUser, models::user::LoginUser)
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "profile", description = "User profile endpoints")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    log::debug!("Connecting to Redis at: {}", redis_url);
    let redis_client = config::redis::create_redis_client(&redis_url);
    
    // Verificar la conexión a Redis
    match redis_client.get_connection() {
        Ok(_) => log::info!("Redis connection test successful"),
        Err(e) => {
            log::error!("Failed to connect to Redis: {}", e);
            panic!("Could not connect to Redis");
        }
    }

    let pool = config::database::create_pool(&database_url).await;

    let app_data = web::Data::new(redis_client.clone());
    let pool_data = web::Data::new(pool.clone());
    let jwt_data = web::Data::new(jwt_secret.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .app_data(app_data.clone())
            .app_data(jwt_data.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .configure(handlers::auth::config)
            .configure(handlers::profile::config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
} 