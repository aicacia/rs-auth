use crate::{
  core::error::{Errors, InternalError, ALREADY_EXISTS_ERROR, INTERNAL_ERROR, NOT_ALLOWED_ERROR},
  middleware::{openid_claims::SCOPE_OPENID, tenant_id::TenantId, validated_json::ValidatedJson},
  model::{
    register::RegisterUser,
    token::{Token, TOKEN_ISSUED_TYPE_REGISTER},
  },
  repository::{
    self,
    user::{create_user_with_password, CreateUserWithPassword},
  },
};

use axum::{extract::State, response::IntoResponse};
use http::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::{token::create_user_token, RouterState};

pub const REGISTER_TAG: &str = "register";

#[utoipa::path(
  post,
  path = "/register",
  tags = [REGISTER_TAG],
  request_body = RegisterUser,
  responses(
    (status = 201, content_type = "application/json", body = Token),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("TenantUUID" = [])
  )
)]
pub async fn register_user(
  State(state): State<RouterState>,
  TenantId(tenant): TenantId,
  ValidatedJson(payload): ValidatedJson<RegisterUser>,
) -> impl IntoResponse {
  if !state.config.user.register_enabled {
    return InternalError::from(StatusCode::FORBIDDEN)
      .with_application_error(NOT_ALLOWED_ERROR)
      .into_response();
  }
  match repository::user::get_user_by_username(
    &state.pool,
    tenant.application_id,
    &payload.username,
  )
  .await
  {
    Ok(Some(_user)) => {
      return InternalError::from(StatusCode::BAD_REQUEST)
        .with_error("username", ALREADY_EXISTS_ERROR)
        .into_response();
    }
    Ok(None) => {}
    Err(e) => {
      log::error!("error getting user: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }
  let new_user = match create_user_with_password(
    &state.pool,
    state.config.as_ref(),
    tenant.application_id,
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
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  create_user_token(
    &state.pool,
    tenant,
    new_user,
    Some(SCOPE_OPENID.to_owned()),
    Some(TOKEN_ISSUED_TYPE_REGISTER.to_owned()),
    true,
  )
  .await
  .into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(register_user))
    .with_state(state)
}
