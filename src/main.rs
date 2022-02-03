use std::env;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use thumbtaro::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or("5000".to_string())
        .parse::<u16>()
        .expect("port must be number");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    HttpServer::new(|| App::new().service(original).service(thumb))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
