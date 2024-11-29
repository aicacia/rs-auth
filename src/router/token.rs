use crate::{
  core::error::{Errors, INTERNAL_ERROR},
  middleware::{
    claims::{
      BasicClaims, Claims, TOKEN_SUB_TYPE_SERVICE_ACCOUNT, TOKEN_SUB_TYPE_USER,
      TOKEN_TYPE_AUTHORIZATION_CODE, TOKEN_TYPE_BEARER, TOKEN_TYPE_ID, TOKEN_TYPE_REFRESH,
      parse_jwt,
    },
    json::Json,
    openid_claims::{
      OpenIdClaims, has_address_scope, has_email_scope, has_phone_scope, has_profile_scope,
      parse_scopes,
    },
    tenent_id::TenentId,
  },
  model::token::{
    TOKEN_ISSUED_TYPE_PASSWORD, TOKEN_ISSUED_TYPE_REFRESH_TOKEN, TOKEN_ISSUED_TYPE_SERVICE_ACCOUNT,
    Token, TokenRequest,
  },
  repository::{
    service_account::{ServiceAccountRow, get_service_account_by_client_id},
    tenent::TenentRow,
    user::{UserRow, get_user_by_id, get_user_by_username},
    user_info::get_user_info_by_user_id,
    user_password::get_active_user_password_by_user_id,
  },
};

use axum::{Router, extract::State, response::IntoResponse, routing::post};
use http::StatusCode;
use sqlx::AnyPool;
use utoipa::OpenApi;

use super::RouterState;

#[derive(OpenApi)]
#[openapi(
  paths(
    token,
  ),
  components(
    schemas(
      Token,
      TokenRequest,
    )
  ),
  tags(
    (name = "token", description = "Token endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  post,
  path = "token",
  tags = ["token"],
  request_body = TokenRequest,
  responses(
    (status = 201, content_type = "application/json", body = Token),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("TenentId" = [])
  )
)]
pub async fn token(
  State(state): State<RouterState>,
  TenentId(tenent): TenentId,
  Json(payload): Json<TokenRequest>,
) -> impl IntoResponse {
  match payload {
    TokenRequest::Password {
      username,
      password,
      scope,
    } => password_request(&state.pool, tenent, username, password, scope)
      .await
      .into_response(),
    TokenRequest::RefreshToken { refresh_token } => {
      refresh_token_request(&state.pool, tenent, refresh_token)
        .await
        .into_response()
    }
    TokenRequest::ServiceAccount { client_id, secret } => {
      service_account_request(&state.pool, tenent, client_id, secret)
        .await
        .into_response()
    }
    TokenRequest::AuthorizationCode { code } => {
      authorization_code_request(&state.pool, tenent, code)
        .await
        .into_response()
    }
  }
}

pub fn create_router(state: RouterState) -> Router {
  Router::new().route("/token", post(token)).with_state(state)
}

async fn password_request(
  pool: &AnyPool,
  tenent: TenentRow,
  username: String,
  password: String,
  scope: Option<String>,
) -> impl IntoResponse {
  let user = match get_user_by_username(pool, &username).await {
    Ok(Some(user)) => user,
    Ok(None) => return Errors::from(StatusCode::UNAUTHORIZED).into_response(),
    Err(e) => {
      log::error!("error fetching user from database: {}", e);
      return Errors::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  match get_active_user_password_by_user_id(pool, user.id).await {
    Ok(Some(user_password)) => match user_password.verify(&password) {
      Ok(true) => create_user_token(pool, CreateUserToken {
        tenent,
        user,
        scope,
        issued_token_type: TOKEN_ISSUED_TYPE_PASSWORD.to_owned(),
      })
      .await
      .into_response(),
      Ok(false) => Errors::from(StatusCode::UNAUTHORIZED).into_response(),
      Err(e) => {
        log::error!("error verifying user password: {}", e);
        Errors::from(StatusCode::UNAUTHORIZED)
          .with_application_error(INTERNAL_ERROR)
          .into_response()
      }
    },
    Ok(None) => Errors::from(StatusCode::FORBIDDEN)
      .with_error("user-config", "password")
      .into_response(),
    Err(e) => {
      log::error!("error fetching user password from database: {}", e);
      Errors::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response()
    }
  }
}

async fn refresh_token_request(
  pool: &AnyPool,
  tenent: TenentRow,
  token_request: String,
) -> impl IntoResponse {
  let jwt = match parse_jwt::<BasicClaims>(&token_request, &tenent) {
    Ok(claims) => claims,
    Err(e) => {
      log::error!("error decoding jwt: {}", e);
      return Errors::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  if jwt.claims.kind != TOKEN_TYPE_REFRESH {
    log::error!("invalid token type: {}", jwt.claims.kind);
    return Errors::from(StatusCode::UNAUTHORIZED)
      .with_application_error(INTERNAL_ERROR)
      .into_response();
  }
  let user = match get_user_by_id(pool, jwt.claims.sub).await {
    Ok(Some(user)) => user,
    Ok(None) => {
      return Errors::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error fetching user from database: {}", e);
      return Errors::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let scope = jwt.claims.scopes.join(" ");
  create_user_token(pool, CreateUserToken {
    tenent,
    user,
    scope: if scope.is_empty() { None } else { Some(scope) },
    issued_token_type: TOKEN_ISSUED_TYPE_REFRESH_TOKEN.to_owned(),
  })
  .await
  .into_response()
}

async fn authorization_code_request(
  pool: &AnyPool,
  tenent: TenentRow,
  code: String,
) -> impl IntoResponse {
  let jwt = match parse_jwt::<BasicClaims>(&code, &tenent) {
    Ok(claims) => claims,
    Err(e) => {
      log::error!("error decoding jwt: {}", e);
      return Errors::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  if jwt.claims.kind != TOKEN_TYPE_AUTHORIZATION_CODE {
    log::error!("invalid token type: {}", jwt.claims.kind);
    return Errors::from(StatusCode::UNAUTHORIZED)
      .with_application_error(INTERNAL_ERROR)
      .into_response();
  }
  let user = match get_user_by_id(pool, jwt.claims.sub).await {
    Ok(Some(user)) => user,
    Ok(None) => return Errors::from(StatusCode::UNAUTHORIZED).into_response(),
    Err(e) => {
      log::error!("error fetching user from database: {}", e);
      return Errors::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let scope = jwt.claims.scopes.join(" ");
  create_user_token(pool, CreateUserToken {
    tenent,
    user,
    scope: if scope.is_empty() { None } else { Some(scope) },
    issued_token_type: TOKEN_ISSUED_TYPE_REFRESH_TOKEN.to_owned(),
  })
  .await
  .into_response()
}

async fn service_account_request(
  pool: &AnyPool,
  tenent: TenentRow,
  client_id: uuid::Uuid,
  secret: uuid::Uuid,
) -> impl IntoResponse {
  let service_account = match get_service_account_by_client_id(pool, &client_id.to_string()).await {
    Ok(Some(service_account)) => service_account,
    Ok(None) => return Errors::from(StatusCode::UNAUTHORIZED).into_response(),
    Err(e) => {
      log::error!("error fetching service account from database: {}", e);
      return Errors::from(StatusCode::UNAUTHORIZED).into_response();
    }
  };
  match service_account.verify(&secret.to_string()) {
    Ok(true) => create_service_token_token(pool, CreateServiceAccountToken {
      tenent,
      service_account,
      issued_token_type: TOKEN_ISSUED_TYPE_SERVICE_ACCOUNT.to_owned(),
    })
    .await
    .into_response(),
    Ok(false) => Errors::from(StatusCode::UNAUTHORIZED).into_response(),
    Err(e) => {
      log::error!("error verifying user password: {}", e);
      Errors::from(StatusCode::UNAUTHORIZED)
        .with_application_error(INTERNAL_ERROR)
        .into_response()
    }
  }
}

struct CreateServiceAccountToken {
  tenent: TenentRow,
  service_account: ServiceAccountRow,
  issued_token_type: String,
}

async fn create_service_token_token(
  _pool: &AnyPool,
  CreateServiceAccountToken {
    tenent,
    service_account,
    issued_token_type,
  }: CreateServiceAccountToken,
) -> impl IntoResponse {
  let now = chrono::Utc::now();

  let claims = BasicClaims {
    kind: TOKEN_TYPE_BEARER.to_owned(),
    app: tenent.id,
    sub_kind: TOKEN_SUB_TYPE_SERVICE_ACCOUNT.to_owned(),
    sub: service_account.id,
    iat: now.timestamp(),
    nbf: now.timestamp(),
    exp: now.timestamp() + tenent.expires_in_seconds,
    iss: tenent.issuer.clone(),
    aud: tenent.audience.clone(),
    scopes: Vec::with_capacity(0),
  };

  let access_token = match claims.encode(&tenent) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding jwt: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  let mut refresh_claims = claims.clone();
  refresh_claims.kind = TOKEN_TYPE_REFRESH.to_owned();
  refresh_claims.exp = refresh_claims.iat + tenent.refresh_expires_in_seconds;
  let refresh_token = match claims.encode(&tenent) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding jwt: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  (
    StatusCode::CREATED,
    axum::Json(Token {
      access_token,
      token_type: claims.kind,
      issued_token_type,
      expires_in: tenent.expires_in_seconds,
      scope: None,
      refresh_token: Some(refresh_token),
      refresh_token_expires_in: Some(tenent.refresh_expires_in_seconds),
      id_token: None,
    }),
  )
    .into_response()
}

struct CreateUserToken {
  tenent: TenentRow,
  user: UserRow,
  scope: Option<String>,
  issued_token_type: String,
}

async fn create_user_token(
  pool: &AnyPool,
  CreateUserToken {
    tenent,
    user,
    scope,
    issued_token_type,
  }: CreateUserToken,
) -> impl IntoResponse {
  let now = chrono::Utc::now();
  let scopes = parse_scopes(scope.as_ref().map(String::as_str));

  let claims = BasicClaims {
    kind: TOKEN_TYPE_BEARER.to_owned(),
    app: tenent.id,
    sub_kind: TOKEN_SUB_TYPE_USER.to_owned(),
    sub: user.id,
    iat: now.timestamp(),
    nbf: now.timestamp(),
    exp: now.timestamp() + tenent.expires_in_seconds,
    iss: tenent.issuer.clone(),
    aud: tenent.audience.clone(),
    scopes: scopes.clone(),
  };

  let access_token = match claims.encode(&tenent) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding jwt: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  let mut refresh_claims = claims.clone();
  refresh_claims.kind = TOKEN_TYPE_REFRESH.to_owned();
  refresh_claims.exp = refresh_claims.iat + tenent.refresh_expires_in_seconds;
  let refresh_token = match claims.encode(&tenent) {
    Ok(token) => token,
    Err(e) => {
      log::error!("error encoding jwt: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR)
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  let mut id_token = None;
  let show_address = has_profile_scope(&scopes);
  let show_profile = has_address_scope(&scopes);
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
          return Errors::from(StatusCode::INTERNAL_SERVER_ERROR)
            .with_application_error(INTERNAL_ERROR)
            .into_response();
        }
        Err(e) => {
          log::error!("error fetching user info from database: {}", e);
          return Errors::from(StatusCode::INTERNAL_SERVER_ERROR)
            .with_application_error(INTERNAL_ERROR)
            .into_response();
        }
      };
      if show_address {
        id_claims.address = user_info.address;
      }
      if show_profile {
        id_claims.name = user_info.name;
        id_claims.given_name = user_info.given_name;
        id_claims.family_name = user_info.family_name;
        id_claims.middle_name = user_info.middle_name;
        id_claims.nickname = user_info.nickname;
        id_claims.preferred_username = Some(user.username.clone());
        id_claims.profile_picture = user_info.profile_picture;
        id_claims.website = user_info.website;
        id_claims.gender = user_info.gender;
        id_claims.birthdate = user_info.birthdate;
        id_claims.zone_info = user_info.zone_info;
        id_claims.locale = user_info.locale;
      }
    }
    if show_email {
      // TODO: get email
      id_claims.email = None;
      id_claims.email_verified = None;
    }
    if show_phone {
      // TODO: get phone number
      id_claims.phone_number = None;
      id_claims.phone_number_verified = None;
    }
    id_claims.claims.kind = TOKEN_TYPE_ID.to_owned();
    id_token = match id_claims.encode(&tenent) {
      Ok(token) => Some(token),
      Err(e) => {
        log::error!("error encoding jwt: {}", e);
        return Errors::from(StatusCode::INTERNAL_SERVER_ERROR)
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    };
  }

  (
    StatusCode::CREATED,
    axum::Json(Token {
      access_token,
      token_type: claims.kind,
      issued_token_type,
      expires_in: tenent.expires_in_seconds,
      scope,
      refresh_token: Some(refresh_token),
      refresh_token_expires_in: Some(tenent.refresh_expires_in_seconds),
      id_token: id_token,
    }),
  )
    .into_response()
}
