use actix_web::test;

use auth::{
  app::create_app,
  model::util::{HealthResponse, VersionResponse},
};
use sqlx::{Pool, Postgres};

#[sqlx::test(migrations = "./migrations")]
async fn test_health(pool: Pool<Postgres>) -> sqlx::Result<()> {
  let app = test::init_service(create_app(&pool)).await;

  let req = test::TestRequest::get().uri("/health").to_request();
  let res: HealthResponse = test::call_and_read_body_json(&app, req).await;

  assert_eq!(res.ok, true);

  Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn test_version(pool: Pool<Postgres>) -> sqlx::Result<()> {
  let app = test::init_service(create_app(&pool)).await;

  let req = test::TestRequest::get().uri("/version").to_request();
  let res: VersionResponse = test::call_and_read_body_json(&app, req).await;

  assert_eq!(res.version, env!("CARGO_PKG_VERSION"));

  Ok(())
}
