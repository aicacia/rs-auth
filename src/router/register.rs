use std::collections::HashMap;

use crate::{
  core::error::{Errors, INVALID, REQUEST_BODY},
  middleware::service_account_authorization::ServiceAccountAuthorization,
  model::{current_user::CurrentUser, register::RegisterUser, user::User},
  repository::user::{create_user_with_password, CreateUserWithPassword},
};

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use http::StatusCode;
use serde_json::json;
use utoipa::OpenApi;

use super::RouterState;

#[derive(OpenApi)]
#[openapi(
  paths(
    register,
  ),
  components(
    schemas(
      CurrentUser,
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
  request_body = RegisterUser,
  responses(
    (status = 201, content_type = "application/json", body = User),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("ServiceAccountAuthorization" = [])
  )
)]
pub async fn register(
  State(state): State<RouterState>,
  ServiceAccountAuthorization(_service_account): ServiceAccountAuthorization,
  Json(payload): Json<RegisterUser>,
) -> impl IntoResponse {
  if payload.password != payload.password_confirmation {
    return Errors::bad_request()
      .with_error(
        "password_confirmation",
        (
          INVALID,
          HashMap::from([("in".to_owned(), json!(REQUEST_BODY))]),
        ),
      )
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
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
  };
  Json(CurrentUser::from(new_user)).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/register", post(register))
    .with_state(state)
}
