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
  model::user::{
    ServiceAccountCreateUserPhoneNumber, ServiceAccountUpdateUserPhoneNumber, UserPhoneNumber,
  },
  repository,
};

use super::{user::USER_TAG, RouterState};

#[utoipa::path(
  post,
  path = "/users/{user_id}/phone_numbers",
  tags = [USER_TAG],
  request_body = ServiceAccountCreateUserPhoneNumber,
  params(
    ("user_id" = i64, Path, description = "User id")
  ),
  responses(
    (status = 201, content_type = "application/json", body = UserPhoneNumber),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 409, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn create_user_phone_number(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(user_id): Path<i64>,
  ValidatedJson(payload): ValidatedJson<ServiceAccountCreateUserPhoneNumber>,
) -> impl IntoResponse {
  let phone_number = match repository::user_phone_number::create_user_phone_number(
    &state.pool,
    user_id,
    repository::user_phone_number::CreateUserPhoneNumber {
      phone_number: payload.phone_number,
      primary: payload.primary,
      verified: payload.verified,
    },
  )
  .await
  {
    Ok(phone_number) => phone_number,
    Err(e) => {
      if e.to_string().to_lowercase().contains("unique constraint") {
        return Errors::from(StatusCode::CONFLICT)
          .with_error("phone_number", ALREADY_EXISTS_ERROR)
          .into_response();
      }
      log::error!("Error creating user's phone number: {e}");
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (
    StatusCode::CREATED,
    axum::Json(UserPhoneNumber::from(phone_number)),
  )
    .into_response()
}

#[utoipa::path(
  put,
  path = "/users/{user_id}/phone-numbers/{phone_number_id}",
  tags = [USER_TAG],
  request_body = ServiceAccountUpdateUserPhoneNumber,
  params(
    ("user_id" = i64, Path, description = "User id"),
    ("phone_number_id" = i64, Path, description = "PhoneNumber id"),
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
pub async fn update_user_phone_number(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path((user_id, phone_number_id)): Path<(i64, i64)>,
  ValidatedJson(payload): ValidatedJson<ServiceAccountUpdateUserPhoneNumber>,
) -> impl IntoResponse {
  match repository::user_phone_number::update_user_phone_number(
    &state.pool,
    user_id,
    phone_number_id,
    repository::user_phone_number::UpdateUserPhoneNumber {
      primary: payload.primary,
      verified: payload.verified,
    },
  )
  .await
  {
    Ok(Some(_)) => {}
    Ok(None) => {
      return Errors::not_found()
        .with_error("phone-number", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("Error updating user's phone number: {e}");
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (StatusCode::NO_CONTENT, ()).into_response()
}

#[utoipa::path(
  delete,
  path = "/users/{user_id}/phone-numbers/{phone_number_id}",
  tags = [USER_TAG],
  params(
    ("user_id" = i64, Path, description = "User id"),
    ("phone_number_id" = i64, Path, description = "PhoneNumber id"),
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
pub async fn delete_user_phone_number(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path((user_id, phone_number_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
  match repository::user_phone_number::delete_user_phone_number(
    &state.pool,
    user_id,
    phone_number_id,
  )
  .await
  {
    Ok(Some(_)) => {}
    Ok(None) => {
      return Errors::not_found()
        .with_error("phone-number", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("Error deleting user's phone number: {e}");
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
      create_user_phone_number,
      update_user_phone_number,
      delete_user_phone_number
    ))
    .with_state(state)
}
