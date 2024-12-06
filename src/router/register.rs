use crate::{
  core::{
    config::get_config,
    error::{Errors, INTERNAL_ERROR, NOT_ALLOWED_ERROR},
  },
  middleware::{openid_claims::SCOPE_OPENID, tenent_id::TenentId, validated_json::ValidatedJson},
  model::{register::RegisterUser, token::TOKEN_ISSUED_TYPE_REGISTER, user::User},
  repository::user::{create_user_with_password, CreateUserWithPassword},
};

use axum::{extract::State, response::IntoResponse, routing::post, Router};
use http::StatusCode;
use utoipa::OpenApi;

use super::{token::create_user_token, RouterState};

#[derive(OpenApi)]
#[openapi(
  paths(
    register,
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
    ("TenentUUID" = [])
  )
)]
pub async fn register(
  State(state): State<RouterState>,
  TenentId(tenent): TenentId,
  ValidatedJson(payload): ValidatedJson<RegisterUser>,
) -> impl IntoResponse {
  if !get_config().user.register_enabled {
    return Errors::from(StatusCode::FORBIDDEN)
      .with_application_error(NOT_ALLOWED_ERROR)
      .into_response();
  }
  let new_user = match create_user_with_password(
    &state.pool,
    CreateUserWithPassword {
      username: payload.username,
      password: payload.password,
    },
  )
  .await
  {
    Ok(user) => user,
    Err(e) => {
      log::error!("error creating user: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  create_user_token(
    &state.pool,
    tenent,
    new_user,
    Some(SCOPE_OPENID.to_owned()),
    Some(TOKEN_ISSUED_TYPE_REGISTER.to_owned()),
    true,
  )
  .await
  .into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/register", post(register))
    .with_state(state)
}
