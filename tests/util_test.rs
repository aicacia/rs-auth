use actix_web::test;

use auth::{
  app::create_app,
  model::util::{Health, Version},
};
use sqlx::{Pool, Postgres};

#[sqlx::test(migrations = "./migrations")]
async fn test_health(pool: Pool<Postgres>) -> sqlx::Result<()> {
  let app = test::init_service(create_app(&pool)).await;

  let req = test::TestRequest::get().uri("/health").to_request();
  let res: Health = test::call_and_read_body_json(&app, req).await;

  assert_eq!(res.is_healthy(), true);

  Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn test_version(pool: Pool<Postgres>) -> sqlx::Result<()> {
  let app = test::init_service(create_app(&pool)).await;

  let req = test::TestRequest::get().uri("/version").to_request();
  let res: Version = test::call_and_read_body_json(&app, req).await;

  assert_eq!(res.version, env!("CARGO_PKG_VERSION"));

  Ok(())
}
