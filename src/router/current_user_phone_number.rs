use axum::{
  extract::{Path, State},
  response::IntoResponse,
  routing::{delete, post, put},
  Router,
};
use http::StatusCode;
use utoipa::OpenApi;

use crate::{
  core::error::{Errors, ALREADY_EXISTS_ERROR, INTERNAL_ERROR},
  middleware::{user_authorization::UserAuthorization, validated_json::ValidatedJson},
  model::user::{CreateUserPhoneNumber, UserPhoneNumber},
  repository::{
    self,
    user_phone_number::{
      create_user_phone_number, delete_user_phone_number, set_user_phone_number_as_primary,
    },
  },
};

use super::RouterState;

#[derive(OpenApi)]
#[openapi(
  paths(
    create_phone_number,
    set_phone_number_as_primary,
    delete_phone_number,
  ),
  tags(
    (name = "phone-number", description = "Phone Number endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  post,
  path = "current-user/phone-numbers",
  tags = ["current-user", "phone-number"],
  request_body = CreateUserPhoneNumber,
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
pub async fn create_phone_number(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  ValidatedJson(payload): ValidatedJson<CreateUserPhoneNumber>,
) -> impl IntoResponse {
  let phone_number = match create_user_phone_number(
    &state.pool,
    user.id,
    repository::user_phone_number::CreateUserPhoneNumber {
      phone_number: payload.phone_number,
      primary: Some(false),
      verified: Some(false),
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
      log::error!("Error creating user phone number: {e}");
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
  path = "current-user/phone-numbers/{phone_number_id}/set-as-primary",
  tags = ["current-user", "phone-number"],
  params(
    ("phone_number_id" = i64, Path, description = "PhoneNumber ID to set as primary"),
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
pub async fn set_phone_number_as_primary(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  Path(phone_number_id): Path<i64>,
) -> impl IntoResponse {
  match set_user_phone_number_as_primary(&state.pool, user.id, phone_number_id).await {
    Ok(_) => {}
    Err(e) => {
      if e.to_string().to_lowercase().contains("at least one row") {
        return Errors::bad_request()
          .with_error("phone-number", "not-verified")
          .into_response();
      }
      log::error!("Error setting user phone_number={phone_number_id} as primary: {e}");
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (StatusCode::NO_CONTENT, ()).into_response()
}

#[utoipa::path(
  delete,
  path = "current-user/phone-numbers/{phone_number_id}",
  tags = ["current-user", "phone-number"],
  params(
    ("phone_number_id" = i64, Path, description = "PhoneNumber ID to delete"),
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
pub async fn delete_phone_number(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  Path(phone_number_id): Path<i64>,
) -> impl IntoResponse {
  match delete_user_phone_number(&state.pool, user.id, phone_number_id).await {
    Ok(_) => {}
    Err(e) => {
      log::error!("Error deleting user phone_number={phone_number_id}: {e}");
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/current-user/phone-numbers", post(create_phone_number))
    .route(
      "/current-user/phone-numbers/{phone_number_id}/set-as-primary",
      put(set_phone_number_as_primary),
    )
    .route(
      "/current-user/phone-numbers/{phone_number_id}",
      delete(delete_phone_number),
    )
    .with_state(state)
}
