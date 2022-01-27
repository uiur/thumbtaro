use actix_web::{HttpServer, App};
use thumbtaro::*;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    HttpServer::new(|| {
        App::new()
            .service(original)
            .service(thumb)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
