use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;

pub async fn create_pool(database_url: &str) -> Pool<Postgres> {
    match PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await {
            Ok(pool) => {
                log::info!("Conexión a la base de datos establecida correctamente");
                pool
            },
            Err(e) => {
                log::error!("Error al conectar con la base de datos: {}", e);
                panic!("Error al conectar con la base de datos: {}", e);
            }
        }
} 