use actix_web::{post, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub status: &'static str,
}

#[post("/register")]
pub async fn register(
    pool: web::Data<MySqlPool>,
    body: web::Json<RegisterRequest>,
) -> impl Responder {
    let payload = body.into_inner();

    if let Err(msg) = validate_credentials(&payload.email, &payload.password) {
        return HttpResponse::BadRequest().body(msg);
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match Argon2::default().hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return HttpResponse::InternalServerError().body("hash_error"),
    };

    match sqlx::query("INSERT INTO users (email, password_hash) VALUES (?, ?)")
        .bind(&payload.email)
        .bind(&password_hash)
        .execute(&**pool)
        .await
    {
        Ok(_) => HttpResponse::Created().json(RegisterResponse { status: "created" }),
        Err(err) => {
            if let sqlx::Error::Database(db_err) = &err {
                let is_duplicate = db_err.code().as_deref() == Some("1062")
                    || db_err.code().as_deref() == Some("23000");

                if is_duplicate {
                    return HttpResponse::Conflict().body("user_exists");
                }
            }

            eprintln!("register_error: {err}");
            HttpResponse::InternalServerError().body("db_error")
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub status: &'static str,
}

#[post("/login")]
pub async fn login(pool: web::Data<MySqlPool>, body: web::Json<LoginRequest>) -> impl Responder {
    let payload = body.into_inner();

    let row = match sqlx::query("SELECT password_hash FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&**pool)
        .await
    {
        Ok(row) => row,
        Err(_) => return HttpResponse::InternalServerError().body("db_error"),
    };

    let Some(row) = row else {
        return HttpResponse::Unauthorized().body("invalid_credentials");
    };

    let stored_hash: String = match row.try_get("password_hash") {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("db_error"),
    };

    let parsed_hash = match PasswordHash::new(&stored_hash) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("hash_error"),
    };

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return HttpResponse::Unauthorized().body("invalid_credentials");
    }

    HttpResponse::Ok().json(LoginResponse { status: "ok" })
}

fn validate_credentials(email: &str, password: &str) -> Result<(), &'static str> {
    if email.trim().is_empty() {
        return Err("email_required");
    }

    if password.len() < 8 {
        return Err("password_too_short");
    }

    Ok(())
}
