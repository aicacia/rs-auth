use std::time::Duration;

use crate::{
  core::{config::get_config, error::Errors},
  middleware::user_authorization::UserAuthorization,
  model::{current_user::CurrentUser, oauth2::oauth2_authorize_url},
};

use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use utoipa::OpenApi;

use super::{oauth2::PKCE_CODE_VERIFIERS, RouterState};

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
  tags = ["current-user"],
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
  UserAuthorization(user, _tenent): UserAuthorization,
) -> impl IntoResponse {
  Json(CurrentUser::from(user)).into_response()
}

#[utoipa::path(
  get,
  path = "current-user/oauth2/{provider}",
  tags = ["current-user", "oauth2"],
  params(
    ("provider" = String, Path, description = "OAuth2 provider", example = "google"),
  ),
  responses(
    (status = 200, content_type = "text/plain", body = String),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("UserAuthorization" = [])
  )
)]
pub async fn oauth2(
  Path(provider): Path<String>,
  UserAuthorization(user, tenent): UserAuthorization,
) -> impl IntoResponse {
  let config = get_config();
  let (url, oauth2_state_token, pkce_code_verifier) = match match provider.as_str() {
    "google" => oauth2_authorize_url(
      &config.oauth2.google,
      &tenent,
      &provider,
      false,
      Some(user.id),
    ),
    _ => {
      log::error!("Unknown OAuth2 provider: {}", provider);
      return Errors::internal_error().into_response();
    }
  } {
    Ok(tuple) => tuple,
    Err(e) => {
      log::error!("Error parsing OAuth2 config: {}", e);
      return Errors::internal_error().into_response();
    }
  };

  match PKCE_CODE_VERIFIERS.write() {
    Ok(mut map) => {
      map.insert(
        oauth2_state_token.clone(),
        pkce_code_verifier,
        Duration::from_secs(config.oauth2.code_timeout_in_seconds),
      );
    }
    Err(e) => {
      log::error!("Error aquiring PKCE verifier map: {}", e);
      return Errors::internal_error().into_response();
    }
  }

  url.as_str().to_owned().into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/current-user", get(current_user))
    .with_state(state)
}
