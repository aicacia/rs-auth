use axum::{extract::State, response::IntoResponse, routing::put, Router};
use http::StatusCode;
use utoipa::OpenApi;

use crate::{
  core::error::{Errors, INTERNAL_ERROR, NOT_FOUND_ERROR},
  middleware::{json::Json, user_authorization::UserAuthorization},
  model::current_user::UpdateUserConfigRequest,
  repository::user_config::{update_user_config, UserConfigUpdate},
};

use super::RouterState;

#[derive(OpenApi)]
#[openapi(paths(update_current_user_config))]
pub struct ApiDoc;

#[utoipa::path(
  put,
  path = "current-user/config",
  tags = ["current-user"],
  request_body = UpdateUserConfigRequest,
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
pub async fn update_current_user_config(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  Json(payload): Json<UpdateUserConfigRequest>,
) -> impl IntoResponse {
  match update_user_config(
    &state.pool,
    user.id,
    UserConfigUpdate {
      mfa_type: payload.mfa_type.as_ref().map(ToString::to_string),
    },
  )
  .await
  {
    Ok(_) => {}
    Err(e) => {
      if e.to_string().to_lowercase().contains("no mfa type") {
        return Errors::bad_request()
          .with_error("mfa-type", NOT_FOUND_ERROR)
          .into_response();
      }
      log::error!("Error updating user config: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/current-user/config", put(update_current_user_config))
    .with_state(state)
}
