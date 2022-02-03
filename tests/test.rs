use std::env;

use actix_web::{test, web, App};
use dotenv;
use thumbtaro::{hello, original, thumb};

fn setup() {
    dotenv::from_filename(".env.test").ok();
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(hello)
            .service(thumb)
            .service(original),
    );
}

#[actix_rt::test]
async fn test_original_not_found() {
    setup();
    let mut app = test::init_service(App::new().configure(config)).await;

    let req = test::TestRequest::with_uri("/orig/not_found").to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();

    assert_eq!(status.as_str(), "404");
}

const IMAGE_PATH: &str = "food/banana.png";

#[actix_rt::test]
async fn test_original_found() {
    setup();
    let mut app = test::init_service(App::new().configure(config)).await;

    let req = test::TestRequest::with_uri(&format!("/orig/{}", IMAGE_PATH)).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();

    assert_eq!(status.as_str(), "200");
}

#[actix_rt::test]
async fn test_thumb_found() {
    setup();
    let mut app = test::init_service(App::new().configure(config)).await;

    let req = test::TestRequest::with_uri(&format!("/thumb/400x400/{}", IMAGE_PATH)).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();

    assert_eq!(status.as_str(), "200");
}
