use actix_web::{http::header, web, App, HttpServer};
use actix_files::Files;
use actix_cors::Cors;
use actix_web::middleware::{DefaultHeaders, Logger};
use mongodb::Client;

use crate::config::AppConfig;
use crate::handlers;

pub const SERVER_CODE: &str = "20";

pub async fn run_server(config: AppConfig) -> std::io::Result<()> {
    let bind_address = format!("{}:{}", config.host, config.port);
    let workers = config.workers;
    let shared_config = config.clone();
    let allowed_hosts = config.allowed_hosts.clone();
    let mongo_client = Client::with_uri_str(&config.mongo_uri)
        .await
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    let mongo_client = web::Data::new(mongo_client);
    let max_payload = config.max_payload_bytes;
    let host_allowlist = web::Data::new(allowed_hosts);

    HttpServer::new(move || {
        let cors = if shared_config.cors_allow_origin == "*" {
            Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                .max_age(3600)
        } else {
            Cors::default()
                .allowed_origin(&shared_config.cors_allow_origin)
                .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
                .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                .max_age(3600)
                .supports_credentials()
        };

        let mut headers = DefaultHeaders::new()
            .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
            .add((header::X_FRAME_OPTIONS, "DENY"))
            .add((header::REFERRER_POLICY, "no-referrer"))
            .add((header::HeaderName::from_static("x-permitted-cross-domain-policies"), "none"))
            .add((
                header::HeaderName::from_static("permissions-policy"),
                "geolocation=(), camera=(), microphone=()",
            ));

        if shared_config.enable_hsts {
            headers = headers.add((
                header::STRICT_TRANSPORT_SECURITY,
                "max-age=63072000; includeSubDomains; preload",
            ));
        }

        App::new()
            .wrap(Logger::default())
            .wrap(headers)
            .wrap(cors)
            .app_data(web::Data::new(shared_config.clone()))
            .app_data(mongo_client.clone())
            .app_data(host_allowlist.clone())
            .app_data(web::JsonConfig::default().limit(max_payload))
            .app_data(web::PayloadConfig::new(max_payload))
            .wrap_fn(|req, srv| {
                let maybe_allowed = req.app_data::<web::Data<Vec<String>>>().cloned();
                let host = req
                    .connection_info()
                    .host()
                    .split(':')
                    .next()
                    .unwrap_or_default()
                    .to_string();

                async move {
                    if let Some(allowed_hosts) = maybe_allowed {
                        let allow_all = allowed_hosts.is_empty();
                        let is_allowed = allowed_hosts
                            .iter()
                            .any(|allowed| allowed.eq_ignore_ascii_case(&host));

                        if !allow_all && !is_allowed {
                            let response = actix_web::HttpResponse::Forbidden()
                                .body("host_not_allowed");
                            return Ok(req.into_response(response));
                        }
                    }

                    srv.call(req).await
                }
            })
            .service(
                web::scope("/api")
                    .service(handlers::health::ping)
                    .service(handlers::health::mongo_ping),
            )
            .service(Files::new("/test-web", "test/web").index_file("index.html"))
            .service(handlers::health::ra)
    })
    .workers(workers)
    .bind(bind_address)?
    .run()
    .await
}
