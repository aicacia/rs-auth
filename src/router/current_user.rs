use std::time::Duration;

use crate::{
  core::{
    config::get_config,
    error::{Errors, INTERNAL_ERROR},
  },
  middleware::user_authorization::UserAuthorization,
  model::{current_user::CurrentUser, oauth2::oauth2_authorize_url},
  repository::{
    user_email::get_user_emails_by_user_id,
    user_oauth2_provider::get_user_oauth2_providers_by_user_id,
    user_phone_number::get_user_phone_numbers_by_user_id,
  },
};

use axum::{
  extract::{Path, State},
  response::IntoResponse,
  routing::get,
  Router,
};
use utoipa::OpenApi;

use super::{oauth2::PKCE_CODE_VERIFIERS, RouterState};

#[derive(OpenApi)]
#[openapi(
  paths(
    current_user,
    add_oauth2_provider,
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
  State(state): State<RouterState>,
  UserAuthorization(user, _tenent): UserAuthorization,
) -> impl IntoResponse {
  let mut current_user = CurrentUser::from(user);

  let emails = match get_user_emails_by_user_id(&state.pool, current_user.id).await {
    Ok(emails) => emails,
    Err(e) => {
      log::error!("Error getting user emails: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  for email in emails {
    if email.is_primary() {
      current_user.email = Some(email.into());
    } else {
      current_user.emails.push(email.into());
    }
  }

  let phone_numbers = match get_user_phone_numbers_by_user_id(&state.pool, current_user.id).await {
    Ok(phone_numbers) => phone_numbers,
    Err(e) => {
      log::error!("Error getting user phone numbers: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  for phone_number in phone_numbers {
    if phone_number.is_primary() {
      current_user.phone_number = Some(phone_number.into());
    } else {
      current_user.phone_numbers.push(phone_number.into());
    }
  }

  let oauth2_providers =
    match get_user_oauth2_providers_by_user_id(&state.pool, current_user.id).await {
      Ok(oauth2_providers) => oauth2_providers,
      Err(e) => {
        log::error!("Error getting user oauth2 providers: {}", e);
        return Errors::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    };
  for oauth2_provider in oauth2_providers {
    current_user.oauth2_providers.push(oauth2_provider.into());
  }

  axum::Json(current_user).into_response()
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
pub async fn add_oauth2_provider(
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
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  } {
    Ok(tuple) => tuple,
    Err(e) => {
      log::error!("Error parsing OAuth2 config: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
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
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  url.as_str().to_owned().into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/current-user/oauth2/{provider}", get(add_oauth2_provider))
    .route("/current-user", get(current_user))
    .with_state(state)
}
