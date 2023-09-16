use actix_web::test;

use auth::{
  app::create_app,
  model::auth::{SignInWithPasswordRequest, SignUpWithPasswordRequest},
  service::application::set_application_config,
};
use sqlx::{Pool, Postgres};

#[sqlx::test(migrations = "./migrations")]
async fn test_sign_in_with_password(pool: Pool<Postgres>) -> sqlx::Result<()> {
  let app = test::init_service(create_app(&pool)).await;

  let body = SignInWithPasswordRequest {
    application_id: 1,
    username_or_email: "admin".to_owned(),
    password: "password".to_owned(),
  };
  let req = test::TestRequest::post()
    .uri("/auth/sign-in/password")
    .set_json(body)
    .to_request();
  let res = test::call_and_read_body(&app, req).await;

  assert!(!res.is_empty());

  Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn test_sign_up_with_password(pool: Pool<Postgres>) -> sqlx::Result<()> {
  let app = test::init_service(create_app(&pool)).await;

  set_application_config(
    &pool,
    1,
    "disable_public_signup",
    serde_json::Value::Bool(false),
  )
  .await;

  let body = SignUpWithPasswordRequest {
    application_id: 1,
    username: "test".to_owned(),
    email: None,
    password: "password".to_owned(),
    password_confirmation: "password".to_owned(),
  };
  let req = test::TestRequest::post()
    .uri("/auth/sign-up/password")
    .set_json(body)
    .to_request();
  let res = test::call_and_read_body(&app, req).await;

  assert!(!res.is_empty());

  Ok(())
}
