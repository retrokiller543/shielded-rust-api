mod caesar;
mod error;
mod config;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use log::debug;

const HOST_ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 6968;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let addr = format!("{}:{}", HOST_ADDRESS, PORT);

    debug!("Listening on: {}", addr);

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .service(caesar::caesar_encrypt)
            .service(caesar::caesar_decrypt)
            .service(config::config)
    })
        .bind(addr)?
        .run()
        .await
}