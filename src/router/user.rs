use std::collections::HashMap;

use crate::{
  core::error::{
    Errors, InternalError, ALREADY_EXISTS_ERROR, INTERNAL_ERROR, NOT_ALLOWED_ERROR, NOT_FOUND_ERROR,
  },
  middleware::{
    json::Json, service_account_authorization::ServiceAccountAuthorization,
    validated_json::ValidatedJson,
  },
  model::{
    current_user::UpdateUserInfoRequest,
    token::Token,
    user::{CreateUser, UpdateUser, User, UserInfo, UserPagination, UserResetPassword},
    util::{ApplicationId, OffsetAndLimit},
  },
  repository::{
    self,
    tenant::get_tenant_by_id,
    user::get_users,
    user_config::{get_users_configs, UserConfigRow},
    user_email::{get_user_emails_by_user_id, get_users_emails, UserEmailRow},
    user_info::{get_user_info_by_user_id, get_users_infos, UserInfoRow, UserInfoUpdate},
    user_mfa::{get_user_mfa_types_by_user_id, get_users_mfa_types, UserMFATypeRow},
    user_oauth2_provider::{
      get_user_oauth2_providers_by_user_id, get_users_oauth2_providers, UserOAuth2ProviderRow,
    },
    user_phone_number::{
      get_user_phone_numbers_by_user_id, get_users_phone_numbers, UserPhoneNumberRow,
    },
  },
};

use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
};
use chrono::DateTime;
use http::StatusCode;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::{token, RouterState};

pub const USER_TAG: &str = "user";

#[derive(OpenApi)]
#[openapi(
  paths(
    all_users,
    get_user_by_id,
    create_user,
    create_user_reset_password_token,
  ),
  tags(
    (name = USER_TAG, description = "User's endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  get,
  path = "/users",
  tags = [USER_TAG],
  params(
    OffsetAndLimit,
    ApplicationId,
  ),
  responses(
    (status = 200, content_type = "application/json", body = UserPagination),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn all_users(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Query(application_id): Query<ApplicationId>,
  Query(offset_and_limit): Query<OffsetAndLimit>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("view-users", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let (
    rows,
    users_emails,
    users_phone_numbers,
    users_oauth2_providers,
    users_configs,
    users_infos,
    users_mfa_types,
  ) = match tokio::try_join!(
    get_users(
      &state.pool,
      application_id,
      offset_and_limit.limit,
      offset_and_limit.offset
    ),
    get_users_emails(
      &state.pool,
      application_id,
      offset_and_limit.limit,
      offset_and_limit.offset
    ),
    get_users_phone_numbers(
      &state.pool,
      application_id,
      offset_and_limit.limit,
      offset_and_limit.offset
    ),
    get_users_oauth2_providers(
      &state.pool,
      application_id,
      offset_and_limit.limit,
      offset_and_limit.offset
    ),
    get_users_configs(
      &state.pool,
      application_id,
      offset_and_limit.limit,
      offset_and_limit.offset
    ),
    get_users_infos(
      &state.pool,
      application_id,
      offset_and_limit.limit,
      offset_and_limit.offset
    ),
    get_users_mfa_types(
      &state.pool,
      application_id,
      offset_and_limit.limit,
      offset_and_limit.offset
    )
  ) {
    Ok(results) => results,
    Err(e) => {
      log::error!("error getting users: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let mut users_emails_by_id: HashMap<i64, Vec<UserEmailRow>> =
    users_emails
      .into_iter()
      .fold(HashMap::new(), |mut acc, row| {
        acc.entry(row.user_id).or_default().push(row);
        acc
      });
  let mut users_phone_numbers_by_id: HashMap<i64, Vec<UserPhoneNumberRow>> = users_phone_numbers
    .into_iter()
    .fold(HashMap::new(), |mut acc, row| {
      acc.entry(row.user_id).or_default().push(row);
      acc
    });
  let mut users_oauth2_providers_by_id: HashMap<i64, Vec<UserOAuth2ProviderRow>> =
    users_oauth2_providers
      .into_iter()
      .fold(HashMap::new(), |mut acc, row| {
        acc.entry(row.user_id).or_default().push(row);
        acc
      });
  let mut users_configs_by_id: HashMap<i64, UserConfigRow> =
    users_configs
      .into_iter()
      .fold(HashMap::new(), |mut acc, row| {
        acc.insert(row.user_id, row);
        acc
      });
  let mut users_infos_by_id: HashMap<i64, UserInfoRow> =
    users_infos
      .into_iter()
      .fold(HashMap::new(), |mut acc, row| {
        acc.insert(row.user_id, row);
        acc
      });
  let mut users_mfa_types_by_id: HashMap<i64, Vec<UserMFATypeRow>> = users_mfa_types
    .into_iter()
    .fold(HashMap::new(), |mut acc, row| {
      acc.entry(row.user_id).or_default().push(row);
      acc
    });

  let users = rows
    .into_iter()
    .map(|row| {
      let mut user = User::from(row);
      for email in users_emails_by_id.remove(&user.id).unwrap_or_default() {
        if email.is_primary() {
          user.email = Some(email.into());
        } else {
          user.emails.push(email.into());
        }
      }
      for phone_number in users_phone_numbers_by_id
        .remove(&user.id)
        .unwrap_or_default()
      {
        if phone_number.is_primary() {
          user.phone_number = Some(phone_number.into());
        } else {
          user.phone_numbers.push(phone_number.into());
        }
      }
      for oauth2_provider in users_oauth2_providers_by_id
        .remove(&user.id)
        .unwrap_or_default()
      {
        user.oauth2_providers.push(oauth2_provider.into());
      }
      if let Some(user_config) = users_configs_by_id.remove(&user.id) {
        user.config = Some(user_config.into());
      }
      if let Some(user_info) = users_infos_by_id.remove(&user.id) {
        user.info = user_info.into();
      }
      for mfa_type in users_mfa_types_by_id.remove(&user.id).unwrap_or_default() {
        user.mfa_types.push(mfa_type.into());
      }
      user
    })
    .collect::<Vec<User>>();

  axum::Json(UserPagination {
    has_more: if let Some(limit) = offset_and_limit.limit {
      limit == users.len()
    } else {
      false
    },
    items: users,
  })
  .into_response()
}

#[utoipa::path(
  get,
  path = "/users/{user_id}",
  tags = [USER_TAG],
  params(
    ("user_id" = i64, Path, description = "User id"),
    ApplicationId,
  ),
  responses(
    (status = 200, content_type = "application/json", body = User),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn get_user_by_id(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(user_id): Path<i64>,
  Query(application_id): Query<ApplicationId>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("view-user", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let (
    row_optional,
    user_emails,
    user_phone_numbers,
    user_oauth2_providers,
    user_info_row_optional,
    user_mfa_types,
  ) = match tokio::try_join!(
    repository::user::get_user_by_id(&state.pool, application_id, user_id),
    get_user_emails_by_user_id(&state.pool, application_id, user_id),
    get_user_phone_numbers_by_user_id(&state.pool, application_id, user_id),
    get_user_oauth2_providers_by_user_id(&state.pool, application_id, user_id),
    get_user_info_by_user_id(&state.pool, application_id, user_id),
    get_user_mfa_types_by_user_id(&state.pool, application_id, user_id)
  ) {
    Ok(results) => results,
    Err(e) => {
      log::error!("error getting users: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let row = match row_optional {
    Some(row) => row,
    None => {
      return InternalError::not_found()
        .with_error("user", NOT_FOUND_ERROR)
        .into_response()
    }
  };

  let mut user = User::from(row);
  for email in user_emails {
    if email.is_primary() {
      user.email = Some(email.into());
    } else {
      user.emails.push(email.into());
    }
  }
  for phone_number in user_phone_numbers {
    if phone_number.is_primary() {
      user.phone_number = Some(phone_number.into());
    } else {
      user.phone_numbers.push(phone_number.into());
    }
  }
  for oauth2_provider in user_oauth2_providers {
    user.oauth2_providers.push(oauth2_provider.into());
  }
  if let Some(user_info_row) = user_info_row_optional {
    user.info = user_info_row.into();
  }
  for mfa_type in user_mfa_types {
    user.mfa_types.push(mfa_type.into());
  }

  axum::Json(user).into_response()
}

#[utoipa::path(
  post,
  path = "/users",
  tags = [USER_TAG],
  request_body = CreateUser,
  params(
    ApplicationId,
  ),
  responses(
    (status = 201, content_type = "application/json", body = User),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn create_user(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Query(application_id): Query<ApplicationId>,
  ValidatedJson(payload): ValidatedJson<CreateUser>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("create-user", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let new_user = match repository::user::create_user(
    &state.pool,
    application_id,
    repository::user::CreateUser {
      username: payload.username,
      active: payload.active,
      user_info: Default::default(),
    },
  )
  .await
  {
    Ok(user) => user,
    Err(e) => {
      if e.to_string().to_lowercase().contains("unique constraint") {
        return InternalError::from(StatusCode::BAD_REQUEST)
          .with_error("username", ALREADY_EXISTS_ERROR)
          .into_response();
      }
      log::error!("error creating user: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (StatusCode::CREATED, axum::Json(User::from(new_user))).into_response()
}

#[utoipa::path(
  put,
  path = "/users/{user_id}",
  tags = [USER_TAG],
  request_body = UpdateUser,
  params(
    ApplicationId,
  ),
  responses(
    (status = 200, content_type = "application/json", body = User),
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
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(user_id): Path<i64>,
  Query(application_id): Query<ApplicationId>,
  ValidatedJson(payload): ValidatedJson<UpdateUser>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("update-user", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let user_row = match repository::user::update_user(
    &state.pool,
    application_id,
    user_id,
    repository::user::UpdateUser {
      username: payload.username,
      active: None,
    },
  )
  .await
  {
    Ok(Some(user_row)) => user_row,
    Ok(None) => {
      return InternalError::not_found()
        .with_error("user", NOT_FOUND_ERROR)
        .into_response()
    }
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
  };

  let (
    user_emails,
    user_phone_numbers,
    user_oauth2_providers,
    user_info_row_optional,
    user_mfa_types,
  ) = match tokio::try_join!(
    get_user_emails_by_user_id(&state.pool, application_id, user_id),
    get_user_phone_numbers_by_user_id(&state.pool, application_id, user_id),
    get_user_oauth2_providers_by_user_id(&state.pool, application_id, user_id),
    get_user_info_by_user_id(&state.pool, application_id, user_id),
    get_user_mfa_types_by_user_id(&state.pool, application_id, user_id)
  ) {
    Ok(results) => results,
    Err(e) => {
      log::error!("error getting users: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  let mut user = User::from(user_row);
  for email in user_emails {
    if email.is_primary() {
      user.email = Some(email.into());
    } else {
      user.emails.push(email.into());
    }
  }
  for phone_number in user_phone_numbers {
    if phone_number.is_primary() {
      user.phone_number = Some(phone_number.into());
    } else {
      user.phone_numbers.push(phone_number.into());
    }
  }
  for oauth2_provider in user_oauth2_providers {
    user.oauth2_providers.push(oauth2_provider.into());
  }
  if let Some(user_info_row) = user_info_row_optional {
    user.info = user_info_row.into();
  }
  for mfa_type in user_mfa_types {
    user.mfa_types.push(mfa_type.into());
  }

  axum::Json(user).into_response()
}

#[utoipa::path(
  put,
  path = "/users/{user_id}/info",
  tags = [USER_TAG],
  request_body = UpdateUserInfoRequest,
  params(
    ApplicationId,
  ),
  responses(
    (status = 200, content_type = "application/json", body = UserInfo),
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
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(user_id): Path<i64>,
  Query(application_id): Query<ApplicationId>,
  Json(payload): Json<UpdateUserInfoRequest>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("update-user", NOT_ALLOWED_ERROR)
      .into_response();
  }
  match repository::user::get_user_by_id(&state.pool, application_id, user_id).await {
    Ok(Some(_)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error("user", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error getting user: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }
  let info = match repository::user_info::update_user_info(
    &state.pool,
    user_id,
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
    Ok(info) => info,
    Err(e) => {
      log::error!("error updating user info: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  axum::Json(UserInfo::from(info)).into_response()
}

#[utoipa::path(
  post,
  path = "/users/{user_id}/reset-password",
  tags = [USER_TAG],
  request_body = UserResetPassword,
  params(
    ("user_id" = i64, Path, description = "User id"),
    ApplicationId,
  ),
  responses(
    (status = 201, content_type = "application/json", body = Token),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn create_user_reset_password_token(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account,
    tenant,
    ..
  }: ServiceAccountAuthorization,
  Path(user_id): Path<i64>,
  Query(application_id): Query<ApplicationId>,
  Json(payload): Json<UserResetPassword>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("update-user", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let (user, tenant) = match tokio::try_join!(
    repository::user::get_user_by_id(&state.pool, application_id, user_id),
    get_tenant_by_id(
      &state.pool,
      application_id,
      payload.tenant_id.unwrap_or(tenant.id)
    )
  ) {
    Ok((Some(user), Some(tenant))) => (user, tenant),
    Ok((user, tenant)) => {
      let mut errors = InternalError::not_found();
      if user.is_none() {
        errors.error("user_id", NOT_FOUND_ERROR);
      }
      if tenant.is_none() {
        errors.error("tenant_id", NOT_FOUND_ERROR);
      }
      return errors.into_response();
    }
    Err(e) => {
      log::error!("error getting user/tenant: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  token::create_reset_password_token(&state.pool, tenant, user, payload.scope, None)
    .await
    .into_response()
}

#[utoipa::path(
  delete,
  path = "/users/{user_id}",
  tags = [USER_TAG],
  params(
    ("user_id" = i64, Path, description = "User id"),
    ApplicationId,
  ),
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
pub async fn delete_user(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(user_id): Path<i64>,
  Query(application_id): Query<ApplicationId>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("update-user", NOT_ALLOWED_ERROR)
      .into_response();
  }
  match repository::user::delete_user(&state.pool, application_id, user_id).await {
    Ok(Some(..)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error("user", NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error getting user/tenant: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }
  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(all_users))
    .routes(routes!(get_user_by_id))
    .routes(routes!(create_user))
    .routes(routes!(create_user_reset_password_token))
    .routes(routes!(update_user))
    .routes(routes!(update_user_info))
    .routes(routes!(delete_user))
    .with_state(state)
}
