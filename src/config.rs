#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub db_dsn: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(8080);
        let workers = std::env::var("WORKERS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or_else(default_workers);

        let db_dsn = std::env::var("DB_DSN").unwrap_or_else(|_| {
            "SQL_CONNECTION".to_string()
        });

        Self {
            host,
            port,
            workers,
            db_dsn,
        }
    }
}

fn default_workers() -> usize {
    std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1)
}

