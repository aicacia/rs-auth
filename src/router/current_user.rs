use crate::{
  core::error::Errors, middleware::user_authorization::UserAuthorization,
  model::current_user::CurrentUser,
};

use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use utoipa::OpenApi;

use super::RouterState;

#[derive(OpenApi)]
#[openapi(
  paths(
    current_user,
  ),
  components(
    schemas(
      CurrentUser,
    )
  ),
  tags(
    (name = "current-user", description = "Current user endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  get,
  path = "current-user",
  responses(
    (status = 200, content_type = "application/json", body = CurrentUser),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("UserAuthorization" = [])
  )
)]
pub async fn current_user(
  State(_state): State<RouterState>,
  UserAuthorization(user): UserAuthorization,
) -> impl IntoResponse {
  Json(CurrentUser::from(user)).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/current-user", get(current_user))
    .with_state(state)
}
