use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::MySqlPool;

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

#[get("/db-ping")]
pub async fn db_ping(pool: web::Data<MySqlPool>) -> impl Responder {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&**pool)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(PingResponse { status: "db_ok" }),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("db_error: {}", err))
        }
    }
}

