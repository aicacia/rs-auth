use crate::{
  core::error::Errors,
  middleware::validated_json::ValidatedJson,
  model::{register::RegisterUser, user::User},
  repository::user::{CreateUserWithPassword, create_user_with_password},
};

use axum::{Router, extract::State, response::IntoResponse, routing::post};
use http::StatusCode;
use utoipa::OpenApi;

use super::RouterState;

#[derive(OpenApi)]
#[openapi(
  paths(
    register,
  ),
  components(
    schemas(
      User,
      RegisterUser,
    )
  ),
  tags(
    (name = "register", description = "Register endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  post,
  path = "register",
  tags = ["register"],
  request_body = RegisterUser,
  responses(
    (status = 201, content_type = "application/json", body = User),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("TenentId" = [])
  )
)]
pub async fn register(
  State(state): State<RouterState>,
  ValidatedJson(payload): ValidatedJson<RegisterUser>,
) -> impl IntoResponse {
  let new_user = match create_user_with_password(&state.pool, CreateUserWithPassword {
    username: payload.username,
    password: payload.password,
  })
  .await
  {
    Ok(user) => user,
    Err(e) => {
      log::error!("error creating user: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
  };
  axum::Json(User::from(new_user)).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/register", post(register))
    .with_state(state)
}
