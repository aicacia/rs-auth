use std::collections::HashMap;

use crate::{
  core::{
    error::{
      ALREADY_EXISTS_ERROR, ALREADY_USED_ERROR, Errors, INTERNAL_ERROR, INVALID_ERROR,
      InternalError, NOT_FOUND_ERROR,
    },
    openapi::AUTHORIZATION_HEADER,
  },
  middleware::{
    authorization::Authorization,
    claims::{TOKEN_SUB_TYPE_USER, TOKEN_TYPE_BEARER, TOKEN_TYPE_RESET_PASSWORD},
    json::Json,
    openid_claims::{
      has_address_scope, has_email_scope, has_phone_scope, has_profile_scope, parse_scopes,
    },
    user_authorization::UserAuthorization,
    validated_json::ValidatedJson,
  },
  model::{
    current_user::{OAuth2Query, ResetPasswordRequest, UpdateUserInfoRequest},
    oauth2::oauth2_authorize_url,
    user::{UpdateUser, User, UserOAuth2Provider},
  },
  repository::{
    self, kv,
    tenant_oauth2_provider::get_active_tenant_oauth2_provider,
    user_config::get_user_config_by_user_id,
    user_email::get_user_emails_by_user_id,
    user_info::{UserInfoUpdate, get_user_info_by_user_id},
    user_mfa::get_user_mfa_types_by_user_id,
    user_oauth2_provider::get_user_oauth2_providers_by_user_id,
    user_password::{create_user_password, get_user_active_password_by_user_id},
    user_phone_number::get_user_phone_numbers_by_user_id,
  },
};

use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
};
use chrono::{DateTime, Duration, Utc};
use http::StatusCode;
use serde_json::json;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::RouterState;

pub const CURRENT_USER_TAG: &str = "current-user";

#[utoipa::path(
  get,
  path = "/current-user",
  tags = [CURRENT_USER_TAG],
  responses(
    (status = 200, content_type = "application/json", body = User),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn get_current_user(
  State(state): State<RouterState>,
  UserAuthorization { user, scopes, .. }: UserAuthorization,
) -> impl IntoResponse {
  let application_id = user.application_id;
  let mut current_user = User::from(user);

  let show_email = has_email_scope(&scopes);
  if show_email {
    let emails =
      match get_user_emails_by_user_id(&state.pool, application_id, current_user.id).await {
        Ok(emails) => emails,
        Err(e) => {
          log::error!("error getting user emails: {}", e);
          return InternalError::internal_error()
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
  }
  if has_phone_scope(&scopes) {
    let phone_numbers =
      match get_user_phone_numbers_by_user_id(&state.pool, application_id, current_user.id).await {
        Ok(phone_numbers) => phone_numbers,
        Err(e) => {
          log::error!("error getting user phone numbers: {}", e);
          return InternalError::internal_error()
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
  }

  let oauth2_providers = match get_user_oauth2_providers_by_user_id(
    &state.pool,
    application_id,
    current_user.id,
  )
  .await
  {
    Ok(oauth2_providers) => oauth2_providers,
    Err(e) => {
      log::error!("error getting user oauth2 providers: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  for row in oauth2_providers {
    let mut oauth2_provider: UserOAuth2Provider = row.into();
    if !show_email {
      oauth2_provider.email.take();
    }
    current_user.oauth2_providers.push(oauth2_provider);
  }

  let mfa_types =
    match get_user_mfa_types_by_user_id(&state.pool, application_id, current_user.id).await {
      Ok(mfa_types) => mfa_types,
      Err(e) => {
        log::error!("error getting user MFA types: {}", e);
        return InternalError::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    };
  for row in mfa_types {
    current_user.mfa_types.push(row.into());
  }

  let show_profile = has_profile_scope(&scopes);
  let show_address = has_address_scope(&scopes);
  if show_address || show_profile {
    let maybe_user_info =
      match get_user_info_by_user_id(&state.pool, application_id, current_user.id).await {
        Ok(user_info) => user_info,
        Err(e) => {
          log::error!("error getting user info: {}", e);
          return InternalError::internal_error()
            .with_application_error(INTERNAL_ERROR)
            .into_response();
        }
      };
    if let Some(user_info) = maybe_user_info {
      if show_profile {
        current_user.info.name = user_info.name;
        current_user.info.given_name = user_info.given_name;
        current_user.info.family_name = user_info.family_name;
        current_user.info.middle_name = user_info.middle_name;
        current_user.info.nickname = user_info.nickname;
        current_user.info.profile_picture = user_info.profile_picture;
        current_user.info.website = user_info.website;
        current_user.info.gender = user_info.gender;
        current_user.info.birthdate = user_info
          .birthdate
          .map(|birthdate| DateTime::<Utc>::from_timestamp(birthdate, 0).unwrap_or_default());
        current_user.info.zone_info = user_info.zone_info.clone();
        current_user.info.locale = user_info.locale.clone();
      }
      if show_address {
        current_user.info.address = user_info.address;
        current_user.info.zone_info = user_info.zone_info;
        current_user.info.locale = user_info.locale;
      }
    }
  }

  if show_profile {
    let user_config = match get_user_config_by_user_id(&state.pool, current_user.id).await {
      Ok(Some(user_config)) => user_config,
      Ok(None) => {
        log::error!("User config not found for {}", current_user.id);
        return InternalError::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
      Err(e) => {
        log::error!("error getting user config: {}", e);
        return InternalError::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    };
    current_user.config = Some(user_config.into());
  }

  axum::Json(current_user).into_response()
}

#[utoipa::path(
  post,
  path = "/current-user/oauth2/{provider}",
  tags = [CURRENT_USER_TAG],
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
    ("Authorization" = [])
  )
)]
pub async fn create_current_user_add_oauth2_provider_url(
  State(state): State<RouterState>,
  Path(provider): Path<String>,
  Query(OAuth2Query {
    state: custom_state,
  }): Query<OAuth2Query>,
  UserAuthorization { user, tenant, .. }: UserAuthorization,
) -> impl IntoResponse {
  let tenant_oauth2_provider = match get_active_tenant_oauth2_provider(
    &state.pool,
    tenant.application_id,
    tenant.id,
    &provider,
  )
  .await
  {
    Ok(Some(tenant_oauth2_provider)) => tenant_oauth2_provider,
    Ok(None) => {
      log::error!("Unknown OAuth2 provider: {}", provider);
      return InternalError::internal_error()
        .with_error("oauth2-provider", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error getting tenant oauth2 provider: {}", e);
      return InternalError::internal_error()
        .with_error("oauth2-provider", INTERNAL_ERROR)
        .into_response();
    }
  };
  let basic_client = match tenant_oauth2_provider.basic_client(state.config.as_ref()) {
    Ok(client) => client,
    Err(e) => {
      log::error!("error getting basic client: {}", e);
      return InternalError::internal_error()
        .with_error("oauth2-provider", INVALID_ERROR)
        .into_response();
    }
  };
  let (url, csrf_token, pkce_code_verifier) = match oauth2_authorize_url(
    state.config.as_ref(),
    &basic_client,
    &tenant,
    false,
    custom_state,
    Some(user.id),
    parse_scopes(Some(tenant_oauth2_provider.scope.as_str()))
      .into_iter()
      .map(oauth2::Scope::new),
  ) {
    Ok(tuple) => tuple,
    Err(e) => {
      log::error!("error parsing OAuth2 provider: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  if !kv::set(
    &state.pool,
    csrf_token.secret(),
    pkce_code_verifier.secret(),
    Some(Duration::seconds(
      state.config.oauth2.code_timeout_in_seconds as i64,
    )),
  )
  .await
  {
    log::error!("error setting pkce code verifier");
    return InternalError::internal_error()
      .with_application_error(INTERNAL_ERROR)
      .into_response();
  }

  url.as_str().to_owned().into_response()
}

#[utoipa::path(
  post,
  path = "/current-user/reset-password",
  tags = [CURRENT_USER_TAG],
  request_body = ResetPasswordRequest,
  responses(
    (status = 204),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn reset_current_user_password(
  State(state): State<RouterState>,
  Authorization { claims, .. }: Authorization,
  ValidatedJson(payload): ValidatedJson<ResetPasswordRequest>,
) -> impl IntoResponse {
  if (claims.r#type != TOKEN_TYPE_BEARER && claims.r#type != TOKEN_TYPE_RESET_PASSWORD)
    || claims.sub_type != TOKEN_SUB_TYPE_USER
  {
    return InternalError::unauthorized()
      .with_error(AUTHORIZATION_HEADER, "invalid-token-type")
      .into_response();
  }
  let user_id = claims.sub;

  match get_user_active_password_by_user_id(&state.pool, user_id).await {
    Ok(Some(user_password)) => match user_password.verify(&payload.current_password) {
      Ok(true) => {}
      Ok(false) => {
        return InternalError::bad_request()
          .with_error("current_password", INVALID_ERROR)
          .into_response();
      }
      Err(e) => {
        log::error!("error verifying user password: {}", e);
        return InternalError::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    },
    Ok(None) => {}
    Err(e) => {
      log::error!("error getting user password: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  match create_user_password(
    &state.pool,
    state.config.clone(),
    user_id,
    &payload.password,
  )
  .await
  {
    Ok(_) => {}
    Err(e) => {
      match &e {
        sqlx::Error::Configuration(e) => {
          if e.to_string().contains("password_already_used") {
            return InternalError::bad_request()
              .with_error(
                "password",
                (
                  ALREADY_USED_ERROR,
                  HashMap::from([(
                    "password.history".to_owned(),
                    json!(state.config.password.history),
                  )]),
                ),
              )
              .into_response();
          }
        }
        _ => {}
      }
      log::error!("error creating user password: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  (StatusCode::NO_CONTENT, ()).into_response()
}

#[utoipa::path(
  put,
  path = "/current-user",
  tags = [CURRENT_USER_TAG],
  request_body = UpdateUser,
  responses(
    (status = 204),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn update_current_user(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  ValidatedJson(payload): ValidatedJson<UpdateUser>,
) -> impl IntoResponse {
  match repository::user::update_user(
    &state.pool,
    user.application_id,
    user.id,
    repository::user::UpdateUser {
      username: payload.username,
      active: None,
    },
  )
  .await
  {
    Ok(_) => {}
    Err(e) => {
      if e.to_string().to_lowercase().contains("unique constraint") {
        return InternalError::from(StatusCode::BAD_REQUEST)
          .with_error("username", ALREADY_EXISTS_ERROR)
          .into_response();
      }
      log::error!("error updating user: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  (StatusCode::NO_CONTENT, ()).into_response()
}

#[utoipa::path(
  put,
  path = "/current-user/info",
  tags = [CURRENT_USER_TAG],
  request_body = UpdateUserInfoRequest,
  responses(
    (status = 204),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn update_current_user_info(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  Json(payload): Json<UpdateUserInfoRequest>,
) -> impl IntoResponse {
  match repository::user_info::update_user_info(
    &state.pool,
    user.id,
    UserInfoUpdate {
      name: payload.name,
      given_name: payload.given_name,
      family_name: payload.family_name,
      middle_name: payload.middle_name,
      nickname: payload.nickname,
      profile_picture: payload.profile_picture,
      website: payload.website,
      gender: payload.gender,
      birthdate: payload.birthdate.as_ref().map(DateTime::timestamp),
      address: payload.address,
      zone_info: payload.zone_info,
      locale: payload.locale,
    },
  )
  .await
  {
    Ok(_) => {}
    Err(e) => {
      log::error!("error updating user info: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  (StatusCode::NO_CONTENT, ()).into_response()
}

#[utoipa::path(
  delete,
  path = "/current-user",
  tags = [CURRENT_USER_TAG],
  responses(
    (status = 204),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn deactivate_current_user(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
) -> impl IntoResponse {
  match repository::user::update_user(
    &state.pool,
    user.application_id,
    user.id,
    repository::user::UpdateUser {
      username: None,
      active: Some(0),
    },
  )
  .await
  {
    Ok(_) => {}
    Err(e) => {
      log::error!("error deactivate user: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(create_current_user_add_oauth2_provider_url))
    .routes(routes!(get_current_user))
    .routes(routes!(update_current_user_info))
    .routes(routes!(update_current_user))
    .routes(routes!(deactivate_current_user))
    .routes(routes!(reset_current_user_password))
    .with_state(state)
}
