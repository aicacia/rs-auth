use crate::{
  core::error::Errors,
  middleware::{
    service_account_authorization::ServiceAccountAuthorization, validated_json::ValidatedJson,
  },
  model::user::{CreateUser, User},
  repository,
};

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use http::StatusCode;
use utoipa::OpenApi;

use super::RouterState;

#[derive(OpenApi)]
#[openapi(
  paths(
    create_user,
  ),
  components(
    schemas(
      CreateUser,
      User,
    )
  ),
  tags(
    (name = "user", description = "User endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  post,
  path = "user",
  tags = ["user"],
  request_body = CreateUser,
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
pub async fn create_user(
  State(state): State<RouterState>,
  ServiceAccountAuthorization(_service_account, _tenent): ServiceAccountAuthorization,
  ValidatedJson(payload): ValidatedJson<CreateUser>,
) -> impl IntoResponse {
  let new_user = match repository::user::create_user(
    &state.pool,
    repository::user::CreateUser {
      username: payload.username,
      active: payload.active,
      user_info: Default::default(),
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
  Json(User::from(new_user)).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/user", post(create_user))
    .with_state(state)
}
