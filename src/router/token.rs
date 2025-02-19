use crate::{
  core::{
    config::Config,
    error::{Errors, InternalError, INTERNAL_ERROR, INVALID_ERROR, NOT_FOUND_ERROR},
  },
  middleware::{
    claims::{
      parse_jwt, BasicClaims, Claims, TOKEN_SUB_TYPE_SERVICE_ACCOUNT, TOKEN_SUB_TYPE_USER,
      TOKEN_TYPE_AUTHORIZATION_CODE, TOKEN_TYPE_BEARER, TOKEN_TYPE_ID, TOKEN_TYPE_MFA_TOTP_PREFIX,
      TOKEN_TYPE_REFRESH, TOKEN_TYPE_RESET_PASSWORD,
    },
    json::Json,
    openid_claims::{
      has_address_scope, has_email_scope, has_phone_scope, has_profile_scope, parse_scopes,
      OpenIdClaims,
    },
    tenant_id::TenantId,
  },
  model::token::{
    Token, TokenRequest, TOKEN_ISSUED_TYPE_AUTHORIZATION_CODE, TOKEN_ISSUED_TYPE_PASSWORD,
    TOKEN_ISSUED_TYPE_REFRESH_TOKEN, TOKEN_ISSUED_TYPE_SERVICE_ACCOUNT,
  },
  repository::{
    service_account::{get_service_account_by_client_id, ServiceAccountRow},
    tenant::TenantRow,
    user::{get_user_by_id, get_user_by_username_or_primary_email, UserRow},
    user_config::get_user_config_by_user_id,
    user_email::get_user_emails_by_user_id,
    user_info::get_user_info_by_user_id,
    user_password::get_user_active_password_by_user_id,
    user_phone_number::get_user_phone_numbers_by_user_id,
  },
};

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use sqlx::AnyPool;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::RouterState;

pub const TOKEN_TAG: &str = "token";

#[utoipa::path(
  post,
  path = "/token",
  tags = [TOKEN_TAG],
  request_body = TokenRequest,
  responses(
    (status = 201, content_type = "application/json", body = Token),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 403, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("TenantUUID" = [])
  )
)]
pub async fn token(
  State(state): State<RouterState>,
  TenantId(tenant): TenantId,
  Json(payload): Json<TokenRequest>,
) -> impl IntoResponse {
  match payload {
    TokenRequest::Password {
      username,
      password,
      scope,
    } => password_request(
      &state.pool,
      &state.config,
      tenant,
      username,
      password,
      scope,
    )
    .await
    .into_response(),
    TokenRequest::RefreshToken { refresh_token } => {
      refresh_token_request(&state.pool, tenant, refresh_token)
        .await
        .into_response()
    }
    TokenRequest::ServiceAccount {
      client_id,
      client_secret,
    } => service_account_request(&state.pool, tenant, client_id, client_secret)
      .await
      .into_response(),
    TokenRequest::AuthorizationCode { code, scope } => {
      authorization_code_request(&state.pool, tenant, code, scope)
        .await
        .into_response()
    }
  }
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(token))
    .with_state(state)
}

async fn password_request(
  pool: &AnyPool,
  config: &Config,
  tenant: TenantRow,
  username: String,
  password: String,
  scope: Option<String>,
) -> impl IntoResponse {
  let user =
    match get_user_by_username_or_primary_email(pool, tenant.application_id, &username).await {
      Ok(Some(user)) => user,
      Ok(None) => {
        return InternalError::from(StatusCode::UNAUTHORIZED)
          .with_error("credentials", INVALID_ERROR)
          .into_response()
      }
      Err(e) => {
        log::error!("error fetching user from database: {}", e);
        return InternalError::from(StatusCode::UNAUTHORIZED)
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    };
  let user_password = match get_user_active_password_by_user_id(pool, user.id).await {
    Ok(Some(user_password)) => user_password,
    Ok(None) => {
      return InternalError::from(StatusCode::FORBIDDEN)
        .with_error("authentication-types", "password")
        .into_response();
    }
    Err(e) => {
      log::error!("error fetching user password from database: {}", e);
      return InternalError::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  match user_password.verify(&password) {
    Ok(true) => {}
    Ok(false) => {
      return InternalError::from(StatusCode::UNAUTHORIZED)
        .with_error("credentials", INVALID_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error verifying user password: {}", e);
      return InternalError::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }
  let expire_days = config.password.expire_days;
  if expire_days > 0 {
    let expire_time = user_password.created_at + ((expire_days as i64) * 24 * 60 * 60);
    if chrono::Utc::now().timestamp() >= expire_time {
      return create_reset_password_token(
        pool,
        tenant,
        user,
        scope,
        Some(TOKEN_ISSUED_TYPE_PASSWORD.to_owned()),
      )
      .await
      .into_response();
    }
  }
  create_user_token(
    pool,
    tenant,
    user,
    scope,
    Some(TOKEN_ISSUED_TYPE_PASSWORD.to_owned()),
    false,
  )
  .await
  .into_response()
}

async fn refresh_token_request(
  pool: &AnyPool,
  tenant: TenantRow,
  token_request: String,
) -> impl IntoResponse {
  let jwt = match parse_jwt::<BasicClaims>(&token_request, &tenant) {
    Ok(claims) => claims,
    Err(e) => {
      log::error!("error decoding jwt: {}", e);
      return InternalError::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  if jwt.claims.r#type != TOKEN_TYPE_REFRESH {
    log::error!("invalid token type: {}", jwt.claims.r#type);
    return InternalError::from(StatusCode::UNAUTHORIZED)
      .with_application_error(INTERNAL_ERROR)
      .into_response();
  }
  let user = match get_user_by_id(pool, jwt.claims.app, jwt.claims.sub).await {
    Ok(Some(user)) => user,
    Ok(None) => {
      return InternalError::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error fetching user from database: {}", e);
      return InternalError::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let scope = jwt.claims.scopes.join(" ");
  create_user_token(
    pool,
    tenant,
    user,
    if scope.is_empty() { None } else { Some(scope) },
    Some(TOKEN_ISSUED_TYPE_REFRESH_TOKEN.to_owned()),
    false,
  )
  .await
  .into_response()
}

async fn authorization_code_request(
  pool: &AnyPool,
  tenant: TenantRow,
  code: String,
  scope: Option<String>,
) -> impl IntoResponse {
  let jwt = match parse_jwt::<BasicClaims>(&code, &tenant) {
    Ok(claims) => claims,
    Err(e) => {
      log::error!("error decoding jwt: {}", e);
      return InternalError::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  if jwt.claims.r#type != TOKEN_TYPE_AUTHORIZATION_CODE {
    log::error!("invalid token type: {}", jwt.claims.r#type);
    return InternalError::from(StatusCode::UNAUTHORIZED)
      .with_application_error(INTERNAL_ERROR)
      .into_response();
  }
  if jwt.claims.sub_type != TOKEN_SUB_TYPE_USER {
    log::error!("invalid token sub_type: {}", jwt.claims.sub_type);
    return InternalError::from(StatusCode::UNAUTHORIZED)
      .with_application_error(INTERNAL_ERROR)
      .into_response();
  }
  let user = match get_user_by_id(pool, jwt.claims.app, jwt.claims.sub).await {
    Ok(Some(user)) => user,
    Ok(None) => {
      log::error!("user not found: {}", jwt.claims.sub);
      return InternalError::from(StatusCode::UNAUTHORIZED).into_response();
    }
    Err(e) => {
      log::error!("error fetching user from database: {}", e);
      return InternalError::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let scope = scope.unwrap_or_else(|| jwt.claims.scopes.join(" "));
  create_user_token(
    pool,
    tenant,
    user,
    if scope.is_empty() { None } else { Some(scope) },
    Some(TOKEN_ISSUED_TYPE_AUTHORIZATION_CODE.to_owned()),
    true,
  )
  .await
  .into_response()
}

async fn service_account_request(
  pool: &AnyPool,
  tenant: TenantRow,
  client_id: uuid::Uuid,
  client_secret: uuid::Uuid,
) -> impl IntoResponse {
  let service_account = match get_service_account_by_client_id(pool, &client_id.to_string()).await {
    Ok(Some(service_account)) => service_account,
    Ok(None) => return InternalError::from(StatusCode::UNAUTHORIZED).into_response(),
    Err(e) => {
      log::error!("error fetching service account from database: {}", e);
      return InternalError::from(StatusCode::UNAUTHORIZED)
        .with_error("client_id", NOT_FOUND_ERROR)
        .into_response();
    }
  };
  match service_account.verify(&client_secret.to_string()) {
    Ok(true) => create_service_token_token(
      pool,
      tenant,
      service_account,
      Some(TOKEN_ISSUED_TYPE_SERVICE_ACCOUNT.to_owned()),
    )
    .await
    .into_response(),
    Ok(false) => InternalError::from(StatusCode::UNAUTHORIZED).into_response(),
    Err(e) => {
      log::error!("error verifying user password: {}", e);
      InternalError::from(StatusCode::UNAUTHORIZED)
        .with_error("client_secret", INVALID_ERROR)
        .into_response()
    }
  }
}

async fn create_service_token_token(
  _pool: &AnyPool,
  tenant: TenantRow,
  service_account: ServiceAccountRow,
  issued_token_type: Option<String>,
) -> impl IntoResponse {
  let now = chrono::Utc::now();

  let claims = BasicClaims {
    r#type: TOKEN_TYPE_BEARER.to_owned(),
    app: tenant.application_id,
    sub_type: TOKEN_SUB_TYPE_SERVICE_ACCOUNT.to_owned(),
    sub: service_account.id,
    iat: now.timestamp(),
    nbf: now.timestamp(),
    exp: now.timestamp() + tenant.expires_in_seconds,
    iss: tenant.issuer.clone(),
    aud: tenant.audience.clone(),
    scopes: Vec::with_capacity(0),
  };

  let access_token = match claims.encode(&tenant) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding JWT: {}", e);
      return InternalError::from(StatusCode::INTERNAL_SERVER_ERROR)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  let mut refresh_claims = claims.clone();
  refresh_claims.r#type = TOKEN_TYPE_REFRESH.to_owned();
  refresh_claims.exp = refresh_claims.iat + tenant.refresh_expires_in_seconds;
  let refresh_token = match claims.encode(&tenant) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding jwt: {}", e);
      return InternalError::from(StatusCode::INTERNAL_SERVER_ERROR)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  (
    StatusCode::CREATED,
    axum::Json(Token {
      access_token,
      token_type: claims.r#type,
      issued_token_type,
      expires_in: tenant.expires_in_seconds,
      scope: None,
      refresh_token: Some(refresh_token),
      refresh_token_expires_in: Some(tenant.refresh_expires_in_seconds),
      id_token: None,
    }),
  )
    .into_response()
}

pub(crate) async fn create_user_token(
  pool: &AnyPool,
  tenant: TenantRow,
  user: UserRow,
  scope: Option<String>,
  issued_token_type: Option<String>,
  mfa_validated: bool,
) -> impl IntoResponse {
  if !mfa_validated {
    match get_user_config_by_user_id(pool, user.id).await {
      Ok(Some(config)) => {
        if let Some(mfa_type) = config.mfa_type.as_ref() {
          if mfa_type != "none" {
            return create_mfa_token(
              pool,
              tenant,
              user,
              scope,
              issued_token_type,
              format!("{TOKEN_TYPE_MFA_TOTP_PREFIX}{mfa_type}"),
            )
            .await
            .into_response();
          }
        }
      }
      Ok(None) => {}
      Err(e) => {
        log::error!("error fetching user totp from database: {}", e);
        return InternalError::from(StatusCode::INTERNAL_SERVER_ERROR)
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    }
  }
  let now = chrono::Utc::now();
  let scopes = parse_scopes(scope.as_ref().map(String::as_str));

  let claims = BasicClaims {
    r#type: TOKEN_TYPE_BEARER.to_owned(),
    app: tenant.application_id,
    sub_type: TOKEN_SUB_TYPE_USER.to_owned(),
    sub: user.id,
    iat: now.timestamp(),
    nbf: now.timestamp(),
    exp: now.timestamp() + tenant.expires_in_seconds,
    iss: tenant.issuer.clone(),
    aud: tenant.audience.clone(),
    scopes: scopes.clone(),
  };

  let access_token = match claims.encode(&tenant) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding jwt: {}", e);
      return InternalError::from(StatusCode::INTERNAL_SERVER_ERROR)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  let mut refresh_claims = claims.clone();
  refresh_claims.r#type = TOKEN_TYPE_REFRESH.to_owned();
  refresh_claims.exp = refresh_claims.iat + tenant.refresh_expires_in_seconds;
  let refresh_token = match refresh_claims.encode(&tenant) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding jwt: {}", e);
      return InternalError::from(StatusCode::INTERNAL_SERVER_ERROR)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  let mut id_token = None;
  let show_address = has_address_scope(&scopes);
  let show_profile = has_profile_scope(&scopes);
  let show_email = has_email_scope(&scopes);
  let show_phone = has_phone_scope(&scopes);
  if show_address || show_profile || show_email || show_phone {
    let mut id_claims = OpenIdClaims {
      claims: claims.clone(),
      ..Default::default()
    };
    if show_address || show_profile {
      let user_info = match get_user_info_by_user_id(pool, user.id).await {
        Ok(Some(user_info)) => user_info,
        Ok(None) => {
          log::error!("user info not found for user: {}", user.id);
          return InternalError::from(StatusCode::INTERNAL_SERVER_ERROR)
            .with_application_error(INTERNAL_ERROR)
            .into_response();
        }
        Err(e) => {
          log::error!("error fetching user info from database: {}", e);
          return InternalError::from(StatusCode::INTERNAL_SERVER_ERROR)
            .with_application_error(INTERNAL_ERROR)
            .into_response();
        }
      };
      if show_address {
        id_claims.profile.address = user_info.address;
      }
      if show_profile {
        id_claims.profile.name = user_info.name;
        id_claims.profile.given_name = user_info.given_name;
        id_claims.profile.family_name = user_info.family_name;
        id_claims.profile.middle_name = user_info.middle_name;
        id_claims.profile.nickname = user_info.nickname;
        id_claims.profile.preferred_username = Some(user.username.clone());
        id_claims.profile.profile_picture = user_info.profile_picture;
        id_claims.profile.website = user_info.website;
        id_claims.profile.gender = user_info.gender;
        id_claims.profile.birthdate = user_info
          .birthdate
          .map(|birthdate| DateTime::<Utc>::from_timestamp(birthdate, 0).unwrap_or_default());
        id_claims.profile.zone_info = user_info.zone_info;
        id_claims.profile.locale = user_info.locale;
      }
    }
    if show_email {
      match get_user_emails_by_user_id(pool, user.id).await {
        Ok(emails) => {
          if let Some(email) = emails.iter().find(|email| email.is_primary()) {
            id_claims.profile.email_verified = Some(email.is_verified());
            id_claims.profile.email = Some(email.email.clone());
          } else if let Some(email) = emails.iter().find(|email| email.is_verified()) {
            id_claims.profile.email_verified = Some(email.is_verified());
            id_claims.profile.email = Some(email.email.clone());
          } else if let Some(email) = emails.first() {
            id_claims.profile.email_verified = Some(email.is_verified());
            id_claims.profile.email = Some(email.email.clone());
          }
        }
        Err(e) => {
          log::error!("error getting user primary email: {}", e);
          return InternalError::internal_error()
            .with_application_error(INTERNAL_ERROR)
            .into_response();
        }
      }
    }
    if show_phone {
      match get_user_phone_numbers_by_user_id(pool, user.id).await {
        Ok(phone_numbers) => {
          if let Some(phone_number) = phone_numbers
            .iter()
            .find(|phone_number| phone_number.is_primary())
          {
            id_claims.profile.phone_number_verified = Some(phone_number.is_verified());
            id_claims.profile.phone_number = Some(phone_number.phone_number.clone());
          } else if let Some(phone_number) = phone_numbers
            .iter()
            .find(|phone_number| phone_number.is_verified())
          {
            id_claims.profile.phone_number_verified = Some(phone_number.is_verified());
            id_claims.profile.phone_number = Some(phone_number.phone_number.clone());
          } else if let Some(phone_number) = phone_numbers.first() {
            id_claims.profile.phone_number_verified = Some(phone_number.is_verified());
            id_claims.profile.phone_number = Some(phone_number.phone_number.clone());
          }
        }
        Err(e) => {
          log::error!("error getting user primary phone number: {}", e);
          return InternalError::internal_error()
            .with_application_error(INTERNAL_ERROR)
            .into_response();
        }
      }
    }
    id_claims.claims.r#type = TOKEN_TYPE_ID.to_owned();
    id_token = match id_claims.encode(&tenant) {
      Ok(token) => Some(token),
      Err(e) => {
        log::error!("error encoding jwt: {}", e);
        return InternalError::from(StatusCode::INTERNAL_SERVER_ERROR)
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    };
  }

  (
    StatusCode::CREATED,
    axum::Json(Token {
      access_token,
      token_type: claims.r#type,
      issued_token_type,
      expires_in: tenant.expires_in_seconds,
      scope,
      refresh_token: Some(refresh_token),
      refresh_token_expires_in: Some(tenant.refresh_expires_in_seconds),
      id_token: id_token,
    }),
  )
    .into_response()
}

pub(crate) async fn create_reset_password_token(
  _pool: &AnyPool,
  tenant: TenantRow,
  user: UserRow,
  scope: Option<String>,
  issued_token_type: Option<String>,
) -> impl IntoResponse {
  let now = chrono::Utc::now();
  let scopes = parse_scopes(scope.as_ref().map(String::as_str));

  let claims = BasicClaims {
    r#type: TOKEN_TYPE_RESET_PASSWORD.to_owned(),
    app: tenant.application_id,
    sub_type: TOKEN_SUB_TYPE_USER.to_owned(),
    sub: user.id,
    iat: now.timestamp(),
    nbf: now.timestamp(),
    exp: now.timestamp() + tenant.expires_in_seconds,
    iss: tenant.issuer.clone(),
    aud: tenant.audience.clone(),
    scopes: scopes.clone(),
  };

  let access_token = match claims.encode(&tenant) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding jwt: {}", e);
      return InternalError::from(StatusCode::INTERNAL_SERVER_ERROR)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  (
    StatusCode::CREATED,
    axum::Json(Token {
      access_token,
      token_type: claims.r#type,
      issued_token_type,
      expires_in: tenant.expires_in_seconds,
      scope,
      refresh_token: None,
      refresh_token_expires_in: None,
      id_token: None,
    }),
  )
    .into_response()
}

async fn create_mfa_token(
  _pool: &AnyPool,
  tenant: TenantRow,
  user: UserRow,
  scope: Option<String>,
  issued_token_type: Option<String>,
  mfa_token_type: String,
) -> impl IntoResponse {
  let now = chrono::Utc::now();
  let scopes = parse_scopes(scope.as_ref().map(String::as_str));

  let claims = BasicClaims {
    r#type: mfa_token_type,
    app: tenant.application_id,
    sub_type: TOKEN_SUB_TYPE_USER.to_owned(),
    sub: user.id,
    iat: now.timestamp(),
    nbf: now.timestamp(),
    exp: now.timestamp() + tenant.expires_in_seconds,
    iss: tenant.issuer.clone(),
    aud: tenant.audience.clone(),
    scopes: scopes.clone(),
  };

  let access_token = match claims.encode(&tenant) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding jwt: {}", e);
      return InternalError::from(StatusCode::INTERNAL_SERVER_ERROR)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  (
    StatusCode::CREATED,
    axum::Json(Token {
      access_token,
      token_type: claims.r#type,
      issued_token_type,
      expires_in: tenant.expires_in_seconds,
      scope,
      refresh_token: None,
      refresh_token_expires_in: None,
      id_token: None,
    }),
  )
    .into_response()
}
