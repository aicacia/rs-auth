use std::{collections::HashMap, sync::RwLock, time::Duration};

use crate::{
  core::{
    config::get_config,
    error::{Errors, REQUIRED_ERROR},
  },
  middleware::{
    claims::{
      parse_jwt, parse_jwt_no_validation, BasicClaims, Claims, TOKEN_SUB_TYPE_USER,
      TOKEN_TYPE_AUTHORIZATION_CODE,
    },
    tenent_id::TenentId,
  },
  model::oauth2::{
    oauth2_authorize_url, oauth2_create_basic_client, oauth2_profile, OAuth2CallbackQuery,
    OAuth2Query, OAuth2State,
  },
  repository::{
    tenent::get_tenent_by_id,
    user::{create_user_with_oauth2, CreateUserWithOAuth2},
    user_info::UserInfoUpdate,
    user_oauth2_provider::get_user_by_oauth2_provider_and_email,
  },
};

use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
  routing::get,
  Router,
};
use expiringmap::ExpiringMap;
use http::{header::LOCATION, HeaderValue, StatusCode};
use oauth2::TokenResponse;
use reqwest::Url;
use serde_json::json;
use utoipa::OpenApi;

use super::RouterState;

lazy_static! {
  pub(crate) static ref PKCE_CODE_VERIFIERS: RwLock<ExpiringMap<String, oauth2::PkceCodeVerifier>> =
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
    (status = 200, content_type = "text/plain", body = String),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("TenentId" = [])
  )
)]
pub async fn oauth2(
  Path(provider): Path<String>,
  TenentId(tenent): TenentId,
  Query(OAuth2Query { register }): Query<OAuth2Query>,
) -> impl IntoResponse {
  let config = get_config();
  let (url, oauth2_state_token, pkce_code_verifier) = match match provider.as_str() {
    "google" => oauth2_authorize_url(
      &config.oauth2.google,
      &tenent,
      &provider,
      register.unwrap_or_default(),
      None,
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
  State(state): State<RouterState>,
  Path(provider): Path<String>,
  Query(OAuth2CallbackQuery {
    state: oauth2_state_token,
    code,
  }): Query<OAuth2CallbackQuery>,
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

  let pkce_code_verifier = match PKCE_CODE_VERIFIERS.write() {
    Ok(mut map) => match map.remove_entry(&oauth2_state_token) {
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

  let maybe_invalid_token = match parse_jwt_no_validation::<OAuth2State>(&oauth2_state_token) {
    Ok(token) => token,
    Err(e) => {
      log::error!("Error parsing OAuth2 state: {}", e);
      return Errors::internal_error().into_response();
    }
  };
  let tenent_id = match maybe_invalid_token
    .header
    .kid
    .as_ref()
    .map(String::as_str)
    .map(str::parse::<i64>)
  {
    Some(Ok(tenent_id)) => tenent_id,
    Some(Err(e)) => {
      log::error!("Error parsing tenent id: {}", e);
      return Errors::internal_error().into_response();
    }
    None => {
      log::error!("No tenent id found in OAuth2 state");
      return Errors::internal_error().into_response();
    }
  };
  let tenent = match get_tenent_by_id(&state.pool, tenent_id).await {
    Ok(Some(tenent)) => tenent,
    Ok(None) => {
      log::error!("Tenent not found");
      return Errors::unauthorized().into_response();
    }
    Err(e) => {
      log::error!("Error getting tenent from OAuth2 state: {}", e);
      return Errors::internal_error().into_response();
    }
  };

  let token: jsonwebtoken::TokenData<OAuth2State> =
    match parse_jwt::<OAuth2State>(&oauth2_state_token, &tenent) {
      Ok(token) => token,
      Err(e) => {
        log::error!("Error parsing OAuth2 state: {}", e);
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

  let email = match openid_profile.email {
    Some(email) => email,
    None => {
      log::error!("No email found in openid profile");
      return Errors::unauthorized()
        .with_error(
          REQUIRED_ERROR,
          (
            "email",
            HashMap::from([("in".to_owned(), json!("oauth2-account"))]),
          ),
        )
        .into_response();
    }
  };

  let user = if token.claims.register {
    match create_user_with_oauth2(
      &state.pool,
      CreateUserWithOAuth2 {
        active: true,
        provider: provider,
        email: email,
        email_verified: openid_profile.email_verified.unwrap_or(false),
        user_info: UserInfoUpdate {
          name: openid_profile.name,
          given_name: openid_profile.given_name,
          family_name: openid_profile.family_name,
          middle_name: openid_profile.middle_name,
          nickname: openid_profile.nickname,
          profile_picture: openid_profile.profile_picture,
          website: openid_profile.website,
          gender: openid_profile.gender,
          birthdate: openid_profile.birthdate,
          zone_info: openid_profile.zone_info,
          locale: openid_profile.locale,
          address: openid_profile.address,
        },
      },
    )
    .await
    {
      Ok(user) => user,
      Err(e) => {
        log::error!("Error creating user with OAuth2 provider: {}", e);
        return Errors::internal_error().into_response();
      }
    }
  } else {
    match get_user_by_oauth2_provider_and_email(&state.pool, &provider, &email).await {
      Ok(Some(user)) => user,
      Ok(None) => return Errors::unauthorized().into_response(),
      Err(e) => {
        log::error!("Error fetching user by OAuth2 provider: {}", e);
        return Errors::internal_error().into_response();
      }
    }
  };

  let now = chrono::Utc::now();
  let claims = BasicClaims {
    kind: TOKEN_TYPE_AUTHORIZATION_CODE.to_owned(),
    app: tenent.id,
    sub_kind: TOKEN_SUB_TYPE_USER.to_owned(),
    sub: user.id,
    iat: now.timestamp(),
    nbf: now.timestamp(),
    exp: now.timestamp() + tenent.expires_in_seconds,
    iss: tenent.issuer.clone(),
    aud: tenent.audience.clone(),
    scopes: Vec::with_capacity(0),
  };

  let authorization_code = match claims.encode(&tenent) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding jwt: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
  };

  let mut url = match Url::parse(&config.oauth2.redirect_url) {
    Ok(url) => url,
    Err(e) => {
      log::error!("Error parsing redirect URL: {}", e);
      return Errors::internal_error().into_response();
    }
  };
  url.set_query(Some(&format!("authorization_code={authorization_code}")));
  let url_header = match HeaderValue::try_from(url.as_str()) {
    Ok(url_header) => url_header,
    Err(e) => {
      log::error!("Error converting url to header value URL: {}", e);
      return Errors::internal_error().into_response();
    }
  };
  (StatusCode::FOUND, [(LOCATION, url_header)]).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/oauth2/{provider}", get(oauth2))
    .route("/oauth2/{provider}/callback", get(oauth2_callback))
    .with_state(state)
}
