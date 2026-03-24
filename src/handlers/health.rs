use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{bson::doc, Client};
use serde::Serialize;

#[derive(Serialize)]
struct PingResponse {
    status: &'static str,
}

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(PingResponse { status: "ok" })
}

#[get("/ra")]
pub async fn ra() -> impl Responder {
    HttpResponse::Ok().json(PingResponse {
        status: "ra is watching",
    })
}

#[get("/mongo-ping")]
pub async fn mongo_ping(
    mongo_client: web::Data<Client>,
    config: web::Data<crate::config::AppConfig>,
) -> impl Responder {
    match mongo_client
        .database(&config.mongo_db)
        .run_command(doc! { "ping": 1 })
        .await
    {
        Ok(_) => HttpResponse::Ok().json(PingResponse { status: "mongo_ok" }),
        Err(err) => {
            if config.expose_detailed_errors {
                HttpResponse::InternalServerError().body(format!("mongo_error: {}", err))
            } else {
                HttpResponse::InternalServerError().body("mongo_error")
            }
        }
    }
}

