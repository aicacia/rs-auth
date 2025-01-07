use axum::{
  extract::{Path, State},
  response::IntoResponse,
};
use http::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  core::error::{Errors, ALREADY_EXISTS_ERROR, INTERNAL_ERROR, NOT_FOUND_ERROR},
  middleware::{
    service_account_authorization::ServiceAccountAuthorization, validated_json::ValidatedJson,
  },
  model::user::{ServiceAccountCreateUserEmail, ServiceAccountUpdateUserEmail, UserEmail},
  repository,
};

use super::{user::USER_TAG, RouterState};

#[utoipa::path(
  post,
  path = "/users/{user_id}/emails",
  tags = [USER_TAG],
  request_body = ServiceAccountCreateUserEmail,
  params(
    ("user_id" = i64, Path, description = "User id")
  ),
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
pub async fn create_user_email(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(user_id): Path<i64>,
  ValidatedJson(payload): ValidatedJson<ServiceAccountCreateUserEmail>,
) -> impl IntoResponse {
  let email = match repository::user_email::create_user_email(
    &state.pool,
    user_id,
    repository::user_email::CreateUserEmail {
      email: payload.email,
      primary: payload.primary,
      verified: payload.verified,
    },
  )
  .await
  {
    Ok(email) => email,
    Err(e) => {
      if e.to_string().to_lowercase().contains("unique constraint") {
        return Errors::from(StatusCode::CONFLICT)
          .with_error("email", ALREADY_EXISTS_ERROR)
          .into_response();
      }
      log::error!("Error creating user's email: {e}");
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (StatusCode::CREATED, axum::Json(UserEmail::from(email))).into_response()
}

#[utoipa::path(
  put,
  path = "/users/{user_id}/emails/{email_id}",
  tags = [USER_TAG],
  request_body = ServiceAccountUpdateUserEmail,
  params(
    ("user_id" = i64, Path, description = "User id"),
    ("email_id" = i64, Path, description = "Email id"),
  ),
  responses(
    (status = 204),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn update_user_email(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path((user_id, email_id)): Path<(i64, i64)>,
  ValidatedJson(payload): ValidatedJson<ServiceAccountUpdateUserEmail>,
) -> impl IntoResponse {
  match repository::user_email::update_user_email(
    &state.pool,
    user_id,
    email_id,
    repository::user_email::UpdateUserEmail {
      primary: payload.primary,
      verified: payload.verified,
    },
  )
  .await
  {
    Ok(Some(_)) => {}
    Ok(None) => {
      return Errors::not_found()
        .with_error("email", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("Error updating user's email: {e}");
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (StatusCode::NO_CONTENT, ()).into_response()
}

#[utoipa::path(
  delete,
  path = "/users/{user_id}/emails/{email_id}",
  tags = [USER_TAG],
  params(
    ("user_id" = i64, Path, description = "User id"),
    ("email_id" = i64, Path, description = "Email id"),
  ),
  responses(
    (status = 204),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn delete_user_email(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path((user_id, email_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
  match repository::user_email::delete_user_email(&state.pool, user_id, email_id).await {
    Ok(Some(_)) => {}
    Ok(None) => {
      return Errors::not_found()
        .with_error("email", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("Error deleting user's email: {e}");
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(
      create_user_email,
      update_user_email,
      delete_user_email
    ))
    .with_state(state)
}
