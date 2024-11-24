use std::{sync::RwLock, time::Duration};

use crate::{
  core::{config::get_config, error::Errors},
  model::oauth2::{
    oauth2_authorize_url, oauth2_create_basic_client, oauth2_profile, OAuth2CallbackQuery,
    OAuth2Query, OAuth2State,
  },
};

use axum::{
  extract::{Path, Query},
  response::IntoResponse,
  routing::get,
  Json, Router,
};
use expiringmap::ExpiringMap;
use http::{header::LOCATION, HeaderValue, StatusCode};
use oauth2::TokenResponse;
use utoipa::OpenApi;

use super::RouterState;

lazy_static! {
  static ref PKCE_CODE_VERIFIERS: RwLock<ExpiringMap<String, oauth2::PkceCodeVerifier>> =
    RwLock::new(ExpiringMap::new());
}

#[derive(OpenApi)]
#[openapi(
  paths(
    oauth2,
  ),
  components(
    schemas()
  ),
  tags(
    (name = "oauth2", description = "OAuth2 endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  get,
  path = "oauth2/{provider}",
  tags = ["oauth2"],
  params(
    ("provider" = String, Path, description = "OAuth2 provider", example = "google"),
    OAuth2Query,
  ),
  responses(
    (status = 302),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  )
)]
pub async fn oauth2(
  Path(provider): Path<String>,
  Query(OAuth2Query { register }): Query<OAuth2Query>,
) -> impl IntoResponse {
  let config = get_config();
  let (url, csrf_token, pkce_code_verifier) = match match provider.as_str() {
    "google" => oauth2_authorize_url(
      &config.oauth2.google,
      &provider,
      register.unwrap_or_default(),
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
        csrf_token.clone(),
        pkce_code_verifier,
        Duration::from_secs(config.oauth2.code_timeout_in_seconds),
      );
    }
    Err(e) => {
      log::error!("Error aquiring PKCE verifier map: {}", e);
      return Errors::internal_error().into_response();
    }
  }

  log::info!("Redirecting to OAuth2 provider: {}", url.as_str());
  (
    StatusCode::FOUND,
    [(
      LOCATION,
      HeaderValue::try_from(url.as_str()).expect("URI isn't a valid header value"),
    )],
  )
    .into_response()
}

#[utoipa::path(
  get,
  path = "oauth2/{provider}/callback",
  tags = ["oauth2"],
  params(
    ("provider" = String, Path, description = "OAuth2 provider", example = "google"),
    OAuth2CallbackQuery,
  ),
  responses(
    (status = 302),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  )
)]
pub async fn oauth2_callback(
  Path(provider): Path<String>,
  Query(OAuth2CallbackQuery { state, code }): Query<OAuth2CallbackQuery>,
) -> impl IntoResponse {
  let config = get_config();
  let client = match match provider.as_str() {
    "google" => oauth2_create_basic_client(&config.oauth2.google, &provider),
    _ => {
      log::error!("Unknown OAuth2 provider: {}", provider);
      return Errors::internal_error().into_response();
    }
  } {
    Ok(client) => client,
    Err(e) => {
      log::error!("Error parsing OAuth2 config: {}", e);
      return Errors::internal_error().into_response();
    }
  };

  let oauth2_state = match serde_json::from_str::<OAuth2State>(&state) {
    Ok(s) => s,
    Err(e) => {
      log::error!("Error parsing state \"{state}\": {e}");
      return Errors::internal_error().into_response();
    }
  };

  let pkce_code_verifier = match PKCE_CODE_VERIFIERS.write() {
    Ok(mut map) => match map.remove_entry(&oauth2_state.csrf_token) {
      Some((_, pkce_code_verifier)) => pkce_code_verifier,
      None => {
        log::error!("No PKCE code verifier found for CSRF token");
        return Errors::unauthorized().into_response();
      }
    },
    Err(e) => {
      log::error!("Error aquiring PKCE verifier map: {}", e);
      return Errors::internal_error().into_response();
    }
  };

  let code = oauth2::AuthorizationCode::new(code);

  let token_response = match client
    .exchange_code(code)
    .set_pkce_verifier(pkce_code_verifier)
    .request_async(oauth2::reqwest::async_http_client)
    .await
  {
    Ok(token_response) => token_response,
    Err(e) => {
      log::error!("Error exchanging code for token: {}", e);
      return Errors::internal_error().into_response();
    }
  };

  let openid_profile = match oauth2_profile(&provider, token_response.access_token().secret()).await
  {
    Ok(p) => p,
    Err(e) => {
      log::error!("Error getting OAuth2 profile: {}", e);
      return Errors::internal_error().into_response();
    }
  };

  Json(openid_profile).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/oauth2/{provider}", get(oauth2))
    .route("/oauth2/{provider}/callback", get(oauth2_callback))
    .with_state(state)
}
