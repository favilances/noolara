use actix_web::{web, App, HttpServer};
use sqlx::mysql::MySqlPoolOptions;

use crate::config::AppConfig;
use crate::handlers;

pub const SERVER_CODE: &str = "20";

pub async fn run_server(config: AppConfig) -> std::io::Result<()> {
    let bind_address = format!("{}:{}", config.host, config.port);
    let workers = config.workers;
    let shared_config = config.clone();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_dsn)
        .await
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    let pool = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_config.clone()))
            .app_data(pool.clone())
            .service(
                web::scope("/api")
                    .service(handlers::health::ping)
                    .service(handlers::health::db_ping)
                    .service(
                        web::scope("/auth")
                            .service(handlers::auth::register)
                            .service(handlers::auth::login),
                    ),
            )
            .service(handlers::health::ra)
    })
    .workers(workers)
    .bind(bind_address)?
    .run()
    .await
}
