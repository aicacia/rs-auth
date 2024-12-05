use std::{collections::HashMap, time::Duration};

use crate::{
  core::{
    config::get_config,
    error::{Errors, ALREADY_USED_ERROR, INTERNAL_ERROR, INVALID_ERROR},
  },
  middleware::{
    json::Json,
    openid_claims::{has_address_scope, has_email_scope, has_phone_scope, has_profile_scope},
    user_authorization::UserAuthorization,
    validated_json::ValidatedJson,
  },
  model::{
    current_user::{ResetPasswordRequest, UpdateUserInfoRequest},
    oauth2::oauth2_authorize_url,
    user::{UpdateUsername, User, UserOAuth2Provider},
  },
  repository::{
    self,
    user::get_user_mfa_types_by_user_id,
    user_email::get_user_emails_by_user_id,
    user_info::{get_user_info_by_user_id, UserInfoUpdate},
    user_oauth2_provider::get_user_oauth2_providers_by_user_id,
    user_password::{create_user_password, get_user_active_password_by_user_id},
    user_phone_number::get_user_phone_numbers_by_user_id,
  },
};

use axum::{
  extract::{Path, State},
  response::IntoResponse,
  routing::{delete, get, post, put},
  Router,
};
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde_json::json;
use utoipa::OpenApi;

use super::{oauth2::PKCE_CODE_VERIFIERS, RouterState};

#[derive(OpenApi)]
#[openapi(
  paths(
    current_user,
    reset_password,
    add_oauth2_provider,
    update_user_info,
    update_user,
    deactivate_user,
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
    (status = 200, content_type = "application/json", body = User),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn current_user(
  State(state): State<RouterState>,
  UserAuthorization { user, scopes, .. }: UserAuthorization,
) -> impl IntoResponse {
  let mut current_user = User::from(user);

  let show_email = has_email_scope(&scopes);
  if show_email {
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
  }
  if has_phone_scope(&scopes) {
    let phone_numbers = match get_user_phone_numbers_by_user_id(&state.pool, current_user.id).await
    {
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
  for row in oauth2_providers {
    let mut oauth2_provider: UserOAuth2Provider = row.into();
    if !show_email {
      oauth2_provider.email.take();
    }
    current_user.oauth2_providers.push(oauth2_provider);
  }

  let mfa_types = match get_user_mfa_types_by_user_id(&state.pool, current_user.id).await {
    Ok(mfa_types) => mfa_types,
    Err(e) => {
      log::error!("Error getting user MFA types: {}", e);
      return Errors::internal_error()
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
    let maybe_user_info = match get_user_info_by_user_id(&state.pool, current_user.id).await {
      Ok(user_info) => user_info,
      Err(e) => {
        log::error!("Error getting user info: {}", e);
        return Errors::internal_error()
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
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn add_oauth2_provider(
  Path(provider): Path<String>,
  UserAuthorization { user, tenent, .. }: UserAuthorization,
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
    "facebook" => oauth2_authorize_url(
      &config.oauth2.facebook,
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

#[utoipa::path(
  post,
  path = "current-user/reset-password",
  tags = ["current-user"],
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
pub async fn reset_password(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  ValidatedJson(payload): ValidatedJson<ResetPasswordRequest>,
) -> impl IntoResponse {
  match get_user_active_password_by_user_id(&state.pool, user.id).await {
    Ok(Some(user_password)) => match user_password.verify(&payload.current_password) {
      Ok(true) => {}
      Ok(false) => {
        return Errors::bad_request()
          .with_error("current_password", INVALID_ERROR)
          .into_response();
      }
      Err(e) => {
        log::error!("Error verifying user password: {}", e);
        return Errors::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    },
    Ok(None) => {}
    Err(e) => {
      log::error!("Error getting user password: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  match create_user_password(&state.pool, user.id, &payload.password).await {
    Ok(_) => {}
    Err(e) => {
      match &e {
        sqlx::Error::Configuration(e) => {
          if e.to_string().contains("password_already_used") {
            return Errors::bad_request()
              .with_error(
                "password",
                (
                  ALREADY_USED_ERROR,
                  HashMap::from([(
                    "password.history".to_owned(),
                    json!(get_config().password.history),
                  )]),
                ),
              )
              .into_response();
          }
        }
        _ => {}
      }
      log::error!("Error creating user password: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  (StatusCode::NO_CONTENT, ()).into_response()
}

#[utoipa::path(
  put,
  path = "current-user",
  tags = ["current-user"],
  request_body = UpdateUsername,
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
pub async fn update_user(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  ValidatedJson(payload): ValidatedJson<UpdateUsername>,
) -> impl IntoResponse {
  match repository::user::update_user(
    &state.pool,
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
      log::error!("Error updating user: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  (StatusCode::NO_CONTENT, ()).into_response()
}

#[utoipa::path(
  put,
  path = "current-user/info",
  tags = ["current-user", "openid"],
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
pub async fn update_user_info(
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
      log::error!("Error updating user info: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  (StatusCode::NO_CONTENT, ()).into_response()
}

#[utoipa::path(
  delete,
  path = "current-user",
  tags = ["current-user"],
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
pub async fn deactivate_user(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
) -> impl IntoResponse {
  match repository::user::update_user(
    &state.pool,
    user.id,
    repository::user::UpdateUser {
      username: None,
      active: Some(false),
    },
  )
  .await
  {
    Ok(_) => {}
    Err(e) => {
      log::error!("Error deactivate user: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/current-user/oauth2/{provider}", get(add_oauth2_provider))
    .route("/current-user", get(current_user))
    .route("/current-user/reset-password", post(reset_password))
    .route("/current-user/info", put(update_user_info))
    .route("/current-user", put(update_user))
    .route("/current-user", delete(deactivate_user))
    .with_state(state)
}
