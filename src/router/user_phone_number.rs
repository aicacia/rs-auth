use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
};
use http::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  core::error::{
    Errors, InternalError, ALREADY_EXISTS_ERROR, INTERNAL_ERROR, NOT_ALLOWED_ERROR, NOT_FOUND_ERROR,
  },
  middleware::{
    service_account_authorization::ServiceAccountAuthorization, validated_json::ValidatedJson,
  },
  model::{
    user::{
      ServiceAccountCreateUserPhoneNumber, ServiceAccountUpdateUserPhoneNumber, UserPhoneNumber,
    },
    util::ApplicationId,
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
    ("user_id" = i64, Path, description = "User id"),
    ApplicationId,
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
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(user_id): Path<i64>,
  Query(application_id): Query<ApplicationId>,
  ValidatedJson(payload): ValidatedJson<ServiceAccountCreateUserPhoneNumber>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("create-user-phone-numbers", NOT_ALLOWED_ERROR)
      .into_response();
  }
  match repository::user::get_user_by_id(&state.pool, application_id, user_id).await {
    Ok(Some(..)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error("user", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error getting user: {e}");
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
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
        return InternalError::from(StatusCode::CONFLICT)
          .with_error("phone_number", ALREADY_EXISTS_ERROR)
          .into_response();
      }
      log::error!("error creating user's phone number: {e}");
      return InternalError::internal_error()
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
    ApplicationId,
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
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path((user_id, phone_number_id)): Path<(i64, i64)>,
  Query(application_id): Query<ApplicationId>,
  ValidatedJson(payload): ValidatedJson<ServiceAccountUpdateUserPhoneNumber>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("update-user-phone-numbers", NOT_ALLOWED_ERROR)
      .into_response();
  }
  match repository::user::get_user_by_id(&state.pool, application_id, user_id).await {
    Ok(Some(..)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error("user", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error getting user: {e}");
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
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
      return InternalError::not_found()
        .with_error("phone-number", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error updating user's phone number: {e}");
      return InternalError::internal_error()
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
    ApplicationId,
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
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path((user_id, phone_number_id)): Path<(i64, i64)>,
  Query(application_id): Query<ApplicationId>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("delete-user-phone-numbers", NOT_ALLOWED_ERROR)
      .into_response();
  }
  match repository::user::get_user_by_id(&state.pool, application_id, user_id).await {
    Ok(Some(..)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error("user", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error getting user: {e}");
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  match repository::user_phone_number::delete_user_phone_number(
    &state.pool,
    user_id,
    phone_number_id,
  )
  .await
  {
    Ok(Some(_)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error("phone-number", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error deleting user's phone number: {e}");
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
      create_user_phone_number,
      update_user_phone_number,
      delete_user_phone_number
    ))
    .with_state(state)
}
