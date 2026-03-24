use serde::Deserialize;

pub const DEFAULT_CONFIG_PATH: &str = "config/settings.toml";

#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub mongo_uri: String,
    pub mongo_db: String,
    pub cors_allow_origin: String,
    pub max_payload_bytes: usize,
    pub allowed_hosts: Vec<String>,
    pub expose_detailed_errors: bool,
    pub enable_hsts: bool,
}

#[derive(Deserialize)]
struct FileConfig {
    server: ServerConfig,
    mongodb: MongoConfig,
    cors: CorsConfig,
    security: Option<SecurityConfig>,
}

#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: Option<usize>,
}

#[derive(Deserialize)]
struct MongoConfig {
    uri: String,
    db: String,
}

#[derive(Deserialize)]
struct CorsConfig {
    allow_origin: String,
}

#[derive(Deserialize, Default)]
struct SecurityConfig {
    max_payload_bytes: Option<usize>,
    allowed_hosts: Option<Vec<String>>,
    expose_detailed_errors: Option<bool>,
    enable_hsts: Option<bool>,
}

impl AppConfig {
    pub fn load() -> std::io::Result<Self> {
        let config_path = std::env::var("CONFIG_PATH")
            .unwrap_or_else(|_| DEFAULT_CONFIG_PATH.to_string());
        Self::from_file(&config_path)
    }

    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let raw = std::fs::read_to_string(path)?;
        let parsed: FileConfig = toml::from_str(&raw)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
        let security = parsed.security.unwrap_or_default();

        Ok(Self {
            host: parsed.server.host,
            port: parsed.server.port,
            workers: parsed.server.workers.unwrap_or_else(default_workers),
            mongo_uri: parsed.mongodb.uri,
            mongo_db: parsed.mongodb.db,
            cors_allow_origin: parsed.cors.allow_origin,
            max_payload_bytes: security.max_payload_bytes.unwrap_or(1_048_576),
            allowed_hosts: security
                .allowed_hosts
                .unwrap_or_else(|| vec!["127.0.0.1".to_string(), "localhost".to_string()]),
            expose_detailed_errors: security.expose_detailed_errors.unwrap_or(false),
            enable_hsts: security.enable_hsts.unwrap_or(false),
        })
    }
}

fn default_workers() -> usize {
    std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1)
}

