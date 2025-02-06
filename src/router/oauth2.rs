use std::{sync::RwLock, time::Duration};

use crate::{
  core::error::{
    Errors, InternalError, ALREADY_EXISTS_ERROR, INTERNAL_ERROR, INVALID_ERROR, NOT_ALLOWED_ERROR,
    NOT_FOUND_ERROR, PARSE_ERROR, REQUIRED_ERROR,
  },
  middleware::{
    claims::{
      parse_jwt, parse_jwt_no_validation, BasicClaims, Claims, TOKEN_SUB_TYPE_USER,
      TOKEN_TYPE_AUTHORIZATION_CODE,
    },
    openid_claims::{parse_scopes, SCOPE_ADDRESS, SCOPE_EMAIL, SCOPE_PHONE, SCOPE_PROFILE},
    tenant_id::TenantId,
  },
  model::oauth2::{
    oauth2_authorize_url, oauth2_profile, OAuth2CallbackQuery, OAuth2Query, OAuth2State,
  },
  repository::{
    tenant::get_tenant_by_id,
    tenant_oauth2_provider::get_active_tenant_oauth2_provider,
    user::{create_user_with_oauth2, get_user_by_id, CreateUserWithOAuth2},
    user_info::UserInfoUpdate,
    user_oauth2_provider::{
      create_user_oauth2_provider_and_email, get_user_by_oauth2_provider_and_email,
    },
  },
};

use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
};
use chrono::DateTime;
use expiringmap::ExpiringMap;
use http::{header::LOCATION, HeaderValue, StatusCode};
use reqwest::Url;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::RouterState;

lazy_static! {
  pub(crate) static ref PKCE_CODE_VERIFIERS: RwLock<ExpiringMap<String, oauth2::PkceCodeVerifier>> =
    RwLock::new(ExpiringMap::new());
}

pub const OAUTH2_TAG: &str = "oauth2";

#[utoipa::path(
  post,
  path = "/oauth2/{provider}",
  tags = [OAUTH2_TAG],
  params(
    ("provider" = String, Path, description = "OAuth2 provider", example = "google"),
    OAuth2Query,
  ),
  responses(
    (status = 200, content_type = "text/plain", body = String),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("TenantUUID" = [])
  )
)]
pub async fn create_oauth2_url(
  State(state): State<RouterState>,
  Path(provider): Path<String>,
  TenantId(tenant): TenantId,
  Query(OAuth2Query {
    register,
    state: custom_state,
  }): Query<OAuth2Query>,
) -> impl IntoResponse {
  if register.unwrap_or_default() && !state.config.oauth2.register_enabled {
    return InternalError::internal_error()
      .with_error("oauth2-provider", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let tenant_oauth2_provider =
    match get_active_tenant_oauth2_provider(&state.pool, tenant.id, &provider).await {
      Ok(Some(tenant_oauth2_provider)) => tenant_oauth2_provider,
      Ok(None) => {
        log::error!("Unknown OAuth2 provider: {}", provider);
        return InternalError::internal_error()
          .with_error("oauth2-provider", NOT_FOUND_ERROR)
          .into_response();
      }
      Err(e) => {
        log::error!("Error getting tenant oauth2 provider: {}", e);
        return InternalError::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    };
  let basic_client = match tenant_oauth2_provider.basic_client(state.config.as_ref()) {
    Ok(client) => client,
    Err(e) => {
      log::error!("Error getting basic client: {}", e);
      return InternalError::internal_error()
        .with_error("oauth2-provider", INVALID_ERROR)
        .into_response();
    }
  };
  let (url, csrf_token, pkce_code_verifier) = match oauth2_authorize_url(
    state.config.as_ref(),
    &basic_client,
    &tenant,
    register.unwrap_or(false),
    custom_state,
    None,
    parse_scopes(Some(tenant_oauth2_provider.scope.as_str()))
      .into_iter()
      .map(oauth2::Scope::new),
  ) {
    Ok(tuple) => tuple,
    Err(e) => {
      log::error!("Error parsing OAuth2 provider: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  match PKCE_CODE_VERIFIERS.write() {
    Ok(mut map) => {
      map.insert(
        csrf_token.secret().to_owned(),
        pkce_code_verifier,
        Duration::from_secs(state.config.oauth2.code_timeout_in_seconds),
      );
    }
    Err(e) => {
      log::error!("Error aquiring PKCE verifier map: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  url.as_str().to_owned().into_response()
}

#[utoipa::path(
  get,
  path = "/oauth2/{provider}/callback",
  tags = [OAUTH2_TAG],
  params(
    ("provider" = String, Path, description = "OAuth2 provider", example = "google"),
    OAuth2CallbackQuery,
  ),
  responses(
    (status = 302),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 403, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  )
)]
pub async fn oauth2_callback(
  State(state): State<RouterState>,
  Path(provider): Path<String>,
  Query(OAuth2CallbackQuery {
    state: oauth2_state_token_string,
    code,
    ..
  }): Query<OAuth2CallbackQuery>,
) -> impl IntoResponse {
  let maybe_invalid_oauth2_state_token =
    match parse_jwt_no_validation::<OAuth2State>(&oauth2_state_token_string) {
      Ok(token) => token,
      Err(e) => {
        log::error!("Error parsing OAuth2 state: {}", e);
        return InternalError::internal_error()
          .with_error("oauth2-state-token", INVALID_ERROR)
          .into_response();
      }
    };
  let tenant_id = match maybe_invalid_oauth2_state_token
    .header
    .kid
    .as_ref()
    .map(String::as_str)
    .map(str::parse::<i64>)
  {
    Some(Ok(tenant_id)) => tenant_id,
    Some(Err(e)) => {
      log::error!("Error parsing tenant id: {}", e);
      return InternalError::internal_error()
        .with_error("oauth2-state-token", PARSE_ERROR)
        .into_response();
    }
    None => {
      log::error!("No tenant id found in OAuth2 state");
      return InternalError::internal_error()
        .with_error("oauth2-state-token", INVALID_ERROR)
        .into_response();
    }
  };

  let tenant_oauth2_provider =
    match get_active_tenant_oauth2_provider(&state.pool, tenant_id, &provider).await {
      Ok(Some(tenant_oauth2_provider)) => tenant_oauth2_provider,
      Ok(None) => {
        log::error!("Unknown OAuth2 provider: {}", provider);
        return InternalError::internal_error()
          .with_error("oauth2-provider", NOT_FOUND_ERROR)
          .into_response();
      }
      Err(e) => {
        log::error!("Error getting tenant oauth2 provider: {}", e);
        return InternalError::internal_error()
          .with_error("oauth2-provider", INTERNAL_ERROR)
          .into_response();
      }
    };
  let mut errors = InternalError::bad_request();
  let redirect_url = match Url::parse(&tenant_oauth2_provider.redirect_url) {
    Ok(url) => url.to_owned(),
    Err(e) => {
      log::error!(
        "Error parsing redirect url from tenant oauth2 provider: {}",
        e
      );
      return InternalError::internal_error()
        .with_error("redirect-url", PARSE_ERROR)
        .into_response();
    }
  };

  let tenant = match get_tenant_by_id(&state.pool, tenant_id).await {
    Ok(Some(tenant)) => tenant,
    Ok(None) => {
      log::error!("Tenant not found");
      errors.error("oauth2-state-token", INVALID_ERROR);
      return redirect_with_query(redirect_url, None, None, Some(errors)).into_response();
    }
    Err(e) => {
      log::error!("Error getting tenant from OAuth2 state: {}", e);
      errors.error("oauth2-state-token", INVALID_ERROR);
      return redirect_with_query(redirect_url, None, None, Some(errors)).into_response();
    }
  };

  let oauth2_state_token: jsonwebtoken::TokenData<OAuth2State> =
    match parse_jwt::<OAuth2State>(&oauth2_state_token_string, &tenant) {
      Ok(token) => token,
      Err(e) => {
        log::error!("Error parsing OAuth2 state: {}", e);
        errors.status(StatusCode::INTERNAL_SERVER_ERROR);
        errors.error("oauth2-state-token", PARSE_ERROR);
        return redirect_with_query(redirect_url, None, None, Some(errors)).into_response();
      }
    };

  let basic_client = match tenant_oauth2_provider.basic_client(state.config.as_ref()) {
    Ok(client) => client,
    Err(e) => {
      log::error!("Error getting basic client: {}", e);
      errors.error("oauth2-provider", INVALID_ERROR);
      return redirect_with_query(
        redirect_url,
        None,
        oauth2_state_token.claims.custom_state,
        Some(errors),
      )
      .into_response();
    }
  };

  let pkce_code_verifier = match PKCE_CODE_VERIFIERS.write() {
    Ok(mut map) => match map.remove_entry(&oauth2_state_token_string) {
      Some((_, pkce_code_verifier)) => pkce_code_verifier,
      None => {
        log::error!("No PKCE code verifier found for CSRF token");
        errors.status(StatusCode::INTERNAL_SERVER_ERROR);
        errors.error("pkce-code-verifier", INTERNAL_ERROR);
        return redirect_with_query(
          redirect_url,
          None,
          oauth2_state_token.claims.custom_state,
          Some(errors),
        )
        .into_response();
      }
    },
    Err(e) => {
      log::error!("Error aquiring PKCE verifier map: {}", e);
      errors.status(StatusCode::INTERNAL_SERVER_ERROR);
      errors.error("pkce-code-verifier", INTERNAL_ERROR);
      return redirect_with_query(
        redirect_url,
        None,
        oauth2_state_token.claims.custom_state,
        Some(errors),
      )
      .into_response();
    }
  };

  let token_response = match basic_client
    .exchange_code(oauth2::AuthorizationCode::new(code))
    .set_pkce_verifier(pkce_code_verifier)
    .request_async(oauth2::reqwest::async_http_client)
    .await
  {
    Ok(token_response) => token_response,
    Err(e) => {
      log::error!("Error exchanging code for token: {}", e);
      errors.status(StatusCode::INTERNAL_SERVER_ERROR);
      errors.error("oauth2-code-exchange", INTERNAL_ERROR);
      return redirect_with_query(
        redirect_url,
        None,
        oauth2_state_token.claims.custom_state,
        Some(errors),
      )
      .into_response();
    }
  };

  let openid_profile = match oauth2_profile(&tenant_oauth2_provider, token_response).await {
    Ok(p) => p,
    Err(e) => {
      log::error!("Error getting OAuth2 profile: {}", e);
      errors.status(StatusCode::INTERNAL_SERVER_ERROR);
      errors.error("oauth2-provider-profile", INVALID_ERROR);
      return redirect_with_query(
        redirect_url,
        None,
        oauth2_state_token.claims.custom_state,
        Some(errors),
      )
      .into_response();
    }
  };

  let email = match openid_profile.email {
    Some(email) => email,
    None => {
      log::error!("No email found in openid profile");
      errors.error("email", REQUIRED_ERROR);
      return redirect_with_query(
        redirect_url,
        None,
        oauth2_state_token.claims.custom_state,
        Some(errors),
      )
      .into_response();
    }
  };

  let mut scopes = Vec::new();
  scopes.push(SCOPE_EMAIL.to_owned());
  if openid_profile.phone_number.is_some() {
    scopes.push(SCOPE_PHONE.to_owned());
  }
  if openid_profile.address.is_some() {
    scopes.push(SCOPE_ADDRESS.to_owned());
  }
  if openid_profile.name.is_some() {
    scopes.push(SCOPE_PROFILE.to_owned());
  }

  let user = if oauth2_state_token.claims.register {
    match create_user_with_oauth2(
      &state.pool,
      CreateUserWithOAuth2 {
        active: true,
        tenant_oauth2_provider_id: tenant_oauth2_provider.id,
        email: email,
        email_verified: openid_profile.email_verified.unwrap_or(false),
        phone_number: openid_profile.phone_number,
        phone_number_verified: openid_profile.phone_number_verified.unwrap_or(false),
        user_info: UserInfoUpdate {
          name: openid_profile.name,
          given_name: openid_profile.given_name,
          family_name: openid_profile.family_name,
          middle_name: openid_profile.middle_name,
          nickname: openid_profile.nickname,
          profile_picture: openid_profile.profile_picture,
          website: openid_profile.website,
          gender: openid_profile.gender,
          birthdate: openid_profile.birthdate.as_ref().map(DateTime::timestamp),
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
        if e.to_string().to_lowercase().contains("unique constraint") {
          errors.status(StatusCode::CONFLICT);
          errors.error("user", ALREADY_EXISTS_ERROR);
          return redirect_with_query(
            redirect_url,
            None,
            oauth2_state_token.claims.custom_state,
            Some(errors),
          )
          .into_response();
        }
        log::error!("Error creating user with OAuth2 provider: {}", e);
        errors.status(StatusCode::INTERNAL_SERVER_ERROR);
        errors.error("oauth2-provider", INTERNAL_ERROR);
        return redirect_with_query(
          redirect_url,
          None,
          oauth2_state_token.claims.custom_state,
          Some(errors),
        )
        .into_response();
      }
    }
  } else if let Some(user_id) = oauth2_state_token.claims.user_id {
    let user = match get_user_by_id(&state.pool, user_id).await {
      Ok(Some(user)) => user,
      Ok(None) => {
        errors.error("user", NOT_FOUND_ERROR);
        return redirect_with_query(
          redirect_url,
          None,
          oauth2_state_token.claims.custom_state,
          Some(errors),
        )
        .into_response();
      }
      Err(e) => {
        if e.to_string().to_lowercase().contains("unique constraint") {
          return InternalError::from(StatusCode::CONFLICT)
            .with_error("oauth2-provider", ALREADY_EXISTS_ERROR)
            .into_response();
        }
        log::error!("Error fetching user by ID: {}", e);
        errors.error("oauth2-provider", REQUIRED_ERROR);
        return redirect_with_query(
          redirect_url,
          None,
          oauth2_state_token.claims.custom_state,
          Some(errors),
        )
        .into_response();
      }
    };

    match create_user_oauth2_provider_and_email(
      &state.pool,
      user.id,
      tenant_oauth2_provider.id,
      &email,
      &tenant_oauth2_provider.provider,
    )
    .await
    {
      Ok(_) => {}
      Err(e) => {
        log::error!("Error creating user OAuth2 provider: {}", e);
        errors.status(StatusCode::INTERNAL_SERVER_ERROR);
        errors.error("oauth2-provider", INTERNAL_ERROR);
        return redirect_with_query(
          redirect_url,
          None,
          oauth2_state_token.claims.custom_state,
          Some(errors),
        )
        .into_response();
      }
    }

    user
  } else {
    match get_user_by_oauth2_provider_and_email(&state.pool, tenant_oauth2_provider.id, &email)
      .await
    {
      Ok(Some(user)) => user,
      Ok(None) => {
        errors.status(StatusCode::NOT_FOUND);
        errors.error("oauth2-provider", NOT_FOUND_ERROR);
        return redirect_with_query(
          redirect_url,
          None,
          oauth2_state_token.claims.custom_state,
          Some(errors),
        )
        .into_response();
      }
      Err(e) => {
        log::error!("Error fetching user by OAuth2 provider: {}", e);
        errors.status(StatusCode::FORBIDDEN);
        errors.error("oauth2-provider", NOT_ALLOWED_ERROR);
        return redirect_with_query(
          redirect_url,
          None,
          oauth2_state_token.claims.custom_state,
          Some(errors),
        )
        .into_response();
      }
    }
  };

  let now = chrono::Utc::now();
  let claims = BasicClaims {
    r#type: TOKEN_TYPE_AUTHORIZATION_CODE.to_owned(),
    app: tenant.id,
    sub_type: TOKEN_SUB_TYPE_USER.to_owned(),
    sub: user.id,
    iat: now.timestamp(),
    nbf: now.timestamp(),
    exp: now.timestamp() + tenant.expires_in_seconds,
    iss: tenant.issuer.clone(),
    aud: tenant.audience.clone(),
    scopes,
  };

  let authorization_code = match claims.encode(&tenant) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding jwt: {}", e);
      errors.status(StatusCode::INTERNAL_SERVER_ERROR);
      errors.error("authorization-code", PARSE_ERROR);
      return redirect_with_query(
        redirect_url,
        None,
        oauth2_state_token.claims.custom_state,
        Some(errors),
      )
      .into_response();
    }
  };
  redirect_with_query(
    redirect_url,
    Some(authorization_code),
    oauth2_state_token.claims.custom_state,
    None,
  )
  .into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(create_oauth2_url))
    .routes(routes!(oauth2_callback))
    .with_state(state)
}

fn redirect_with_query(
  mut redirect_url: Url,
  authorization_code: Option<String>,
  state: Option<String>,
  error: Option<InternalError>,
) -> impl IntoResponse {
  let mut query = form_urlencoded::Serializer::new(String::new());
  if let Some(error) = error {
    let errors = match serde_json::to_string(&error.errors()) {
      Ok(errors) => errors,
      Err(e) => {
        log::error!("Error serializing errors: {}", e);
        return InternalError::internal_error()
          .with_error("redirect-url", INTERNAL_ERROR)
          .into_response();
      }
    };
    query.append_pair("errors", &errors);
  }
  if let Some(authorization_code) = authorization_code {
    query.append_pair("authorization-code", &authorization_code);
  }
  if let Some(state) = state {
    if !state.is_empty() {
      query.append_pair("state", &state);
    }
  }
  redirect_url.set_query(Some(&query.finish()));
  redirect(redirect_url).into_response()
}

fn redirect(redirect_url: Url) -> impl IntoResponse {
  let url_header = match HeaderValue::try_from(redirect_url.as_str()) {
    Ok(url_header) => url_header,
    Err(e) => {
      log::error!("Error converting url to header value URL: {}", e);
      return InternalError::internal_error()
        .with_error("redirect_url", INVALID_ERROR)
        .into_response();
    }
  };
  (StatusCode::FOUND, [(LOCATION, url_header)]).into_response()
}
