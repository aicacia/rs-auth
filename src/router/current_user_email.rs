use axum::{
  extract::{Path, State},
  response::IntoResponse,
};
use http::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  core::error::{Errors, InternalError, ALREADY_EXISTS_ERROR, INTERNAL_ERROR},
  middleware::{user_authorization::UserAuthorization, validated_json::ValidatedJson},
  model::user::{CreateUserEmail, UserEmail},
  repository::{
    self,
    user_email::{create_user_email, delete_user_email, set_user_email_as_primary},
  },
};

use super::{current_user::CURRENT_USER_TAG, RouterState};

#[utoipa::path(
  post,
  path = "/current-user/emails",
  tags = [CURRENT_USER_TAG],
  request_body = CreateUserEmail,
  responses(
    (status = 201, content_type = "application/json", body = UserEmail),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 409, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn create_current_user_email(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  ValidatedJson(payload): ValidatedJson<CreateUserEmail>,
) -> impl IntoResponse {
  let email = match create_user_email(
    &state.pool,
    user.id,
    repository::user_email::CreateUserEmail {
      email: payload.email,
      primary: Some(false),
      verified: Some(false),
    },
  )
  .await
  {
    Ok(email) => email,
    Err(e) => {
      if e.to_string().to_lowercase().contains("unique constraint") {
        return InternalError::from(StatusCode::CONFLICT)
          .with_error("email", ALREADY_EXISTS_ERROR)
          .into_response();
      }
      log::error!("error creating user email: {e}");
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (StatusCode::CREATED, axum::Json(UserEmail::from(email))).into_response()
}

#[utoipa::path(
  put,
  path = "/current-user/emails/{email_id}/set-as-primary",
  tags = [CURRENT_USER_TAG],
  params(
    ("email_id" = i64, Path, description = "Email ID to set as primary"),
  ),
  responses(
    (status = 204),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 409, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn set_current_user_email_as_primary(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  Path(email_id): Path<i64>,
) -> impl IntoResponse {
  match set_user_email_as_primary(&state.pool, user.id, email_id).await {
    Ok(_) => {}
    Err(e) => {
      if e.to_string().to_lowercase().contains("at least one row") {
        return InternalError::bad_request()
          .with_error("email", "not-verified")
          .into_response();
      }
      log::error!("error setting user email={email_id} as primary: {e}");
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (StatusCode::NO_CONTENT, ()).into_response()
}

#[utoipa::path(
  delete,
  path = "/current-user/emails/{email_id}",
  tags = [CURRENT_USER_TAG],
  params(
    ("email_id" = i64, Path, description = "Email ID to delete"),
  ),
  responses(
    (status = 204),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 409, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn delete_current_user_email(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  Path(email_id): Path<i64>,
) -> impl IntoResponse {
  match delete_user_email(&state.pool, user.id, email_id).await {
    Ok(_) => {}
    Err(e) => {
      log::error!("error deleting user email={email_id}: {e}");
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(
      create_current_user_email,
      set_current_user_email_as_primary,
      delete_current_user_email
    ))
    .with_state(state)
}
