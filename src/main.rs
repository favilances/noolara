mod config;
mod handlers;
mod server;

use crate::config::AppConfig;
use crate::server::{run_server, SERVER_CODE};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env();
    println!(
        r#"
 _   _             _                 
| \ | | ___   ___ | | __ _ _ __ __ _ 
|  \| |/ _ \ / _ \| |/ _` | '__/ _` |
| |\  | (_) | (_) | | (_| | | | (_| |
|_| \_|\___/ \___/|_|\__,_|_|  \__,_|

    Server {SERVER_CODE}
    Listening on {}:{}
"#,
        config.host, config.port
    );
    run_server(config).await
}