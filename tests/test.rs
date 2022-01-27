use std::env;

use thumbtaro::{hello, thumb, original};
use actix_web::{test, App, web};
use dotenv;

fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
      web::scope("")
        .service(hello)
        .service(thumb)
        .service(original)
  );
}

#[actix_rt::test]
async fn test_hello() {
  let mut app = test::init_service(App::new().configure(config)).await;

  let req = test::TestRequest::with_uri("/").to_request();
  let resp = test::call_service(&mut app, req).await;

  assert!(resp.status().is_success());

  let body = test::read_body(resp).await;
  assert_eq!(std::str::from_utf8(body.as_ref()).unwrap(), "hello");
}

#[actix_rt::test]
async fn test_original_not_found() {
  dotenv::from_filename(".env.test").ok();
  let mut app = test::init_service(App::new().configure(config)).await;

  let req = test::TestRequest::with_uri("/orig/not_found").to_request();
  let resp = test::call_service(&mut app, req).await;
  let status = resp.status();

  assert_eq!(status.as_str(), "404");
}
