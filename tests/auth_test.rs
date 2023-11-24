use actix_web::test;

use anyhow::Result;
use auth::{
  app::create_app,
  core::config::get_config,
  model::auth::{SignInWithPasswordRequest, SignUpWithPasswordRequest},
  service::application::update_application_config,
};
use futures::try_join;
use sqlx::{Pool, Postgres};

#[sqlx::test(migrations = "./migrations")]
async fn test_sign_in_with_password(pool: Pool<Postgres>) -> Result<()> {
  let app = test::init_service(create_app(&pool)).await;

  let body = SignInWithPasswordRequest {
    application_id: get_config().admin_application_id,
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
async fn test_sign_up_with_password(pool: Pool<Postgres>) -> Result<()> {
  let app = test::init_service(create_app(&pool)).await;

  try_join!(
    async {
      update_application_config(
        &pool,
        get_config().admin_application_id,
        "signup.enabled",
        &serde_json::Value::Bool(true),
      )
      .await
    },
    async {
      update_application_config(
        &pool,
        get_config().admin_application_id,
        "signup.password",
        &serde_json::Value::Bool(true),
      )
      .await
    },
  )?;

  let body = SignUpWithPasswordRequest {
    application_id: get_config().admin_application_id,
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
