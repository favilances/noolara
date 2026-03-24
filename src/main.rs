mod config;
mod handlers;
mod server;

use crate::config::AppConfig;
use crate::server::{run_server, SERVER_CODE};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = AppConfig::load()?;
    println!(
        r#"
 _   _             _                 
| \ | | ___   ___ | | __ _ _ __ __ _ 
|  \| |/ _ \ / _ \| |/ _` | '__/ _` |
| |\  | (_) | (_) | | (_| | | | (_| |
|_| \_|\___/ \___/|_|\__,_|_|  \__,_|

    Server {SERVER_CODE}
    Listening on {}:{}
    Max payload {} bytes
"#,
        config.host, config.port, config.max_payload_bytes
    );
    run_server(config).await
}