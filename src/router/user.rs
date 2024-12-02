use std::collections::HashMap;

use crate::{
  core::error::{Errors, NOT_FOUND_ERROR},
  middleware::{
    json::Json, service_account_authorization::ServiceAccountAuthorization,
    validated_json::ValidatedJson,
  },
  model::{
    token::Token,
    user::{CreateUser, User, UserResetPassword},
    util::{DEFAULT_LIMIT, OffsetAndLimit, Pagination},
  },
  repository::{
    self,
    tenent::get_tenent_by_id,
    user::{get_user_by_id, get_users},
    user_email::{UserEmailRow, get_users_emails},
    user_info::get_users_infos,
    user_oauth2_provider::{UserOAuth2ProviderRow, get_users_oauth2_providers},
    user_phone_number::{UserPhoneNumberRow, get_users_phone_numbers},
  },
};

use axum::{
  Router,
  extract::{Query, State},
  response::IntoResponse,
  routing::{get, post},
};
use http::StatusCode;
use utoipa::OpenApi;

use super::{RouterState, token::create_reset_password_token};

#[derive(OpenApi)]
#[openapi(
  paths(
    users,
    create_user,
    create_user_reset_password_token,
  ),
  components(
    schemas(
      CreateUser,
      UserResetPassword,
    )
  ),
  tags(
    (name = "user", description = "User endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  get,
  path = "users",
  tags = ["user"],
  params(
    OffsetAndLimit,
  ),
  responses(
    (status = 200, content_type = "application/json", body = Pagination<User>),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn users(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Query(query): Query<OffsetAndLimit>,
) -> impl IntoResponse {
  let limit = query.limit.unwrap_or(DEFAULT_LIMIT);
  let offset = query.offset.unwrap_or(0);
  let (rows, users_emails, users_phone_numbers, users_oauth2_providers, users_infos) = match tokio::try_join!(
    get_users(&state.pool, limit, offset),
    get_users_emails(&state.pool, limit, offset),
    get_users_phone_numbers(&state.pool, limit, offset),
    get_users_oauth2_providers(&state.pool, limit, offset),
    get_users_infos(&state.pool, limit, offset)
  ) {
    Ok(results) => results,
    Err(e) => {
      log::error!("error getting users: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR).into_response();
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
  let mut users_infos_by_id: HashMap<i64, Vec<repository::user_info::UserInfoRow>> = users_infos
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
      for user_info in users_infos_by_id.remove(&user.id).unwrap_or_default() {
        user.info = user_info.into();
      }
      user
    })
    .collect::<Vec<User>>();

  axum::Json(Pagination {
    has_more: users.len() == limit,
    items: users,
  })
  .into_response()
}

#[utoipa::path(
  post,
  path = "users",
  tags = ["user"],
  request_body = CreateUser,
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
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  ValidatedJson(payload): ValidatedJson<CreateUser>,
) -> impl IntoResponse {
  let new_user = match repository::user::create_user(&state.pool, repository::user::CreateUser {
    username: payload.username,
    active: payload.active,
    user_info: Default::default(),
  })
  .await
  {
    Ok(user) => user,
    Err(e) => {
      log::error!("error creating user: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
  };
  (StatusCode::CREATED, axum::Json(User::from(new_user))).into_response()
}

#[utoipa::path(
  post,
  path = "users/reset-password",
  tags = ["user"],
  request_body = UserResetPassword,
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
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Json(payload): Json<UserResetPassword>,
) -> impl IntoResponse {
  let (user, tenent) = match tokio::try_join!(
    get_user_by_id(&state.pool, payload.user_id),
    get_tenent_by_id(&state.pool, payload.tenent_id)
  ) {
    Ok((Some(user), Some(tenent))) => (user, tenent),
    Ok((user, tenent)) => {
      let mut errors = Errors::not_found();
      if user.is_none() {
        errors.error("user_id", NOT_FOUND_ERROR);
      }
      if tenent.is_none() {
        errors.error("tenent_id", NOT_FOUND_ERROR);
      }
      return errors.into_response();
    }
    Err(e) => {
      log::error!("error getting user/tenent: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
  };

  create_reset_password_token(&state.pool, tenent, user, payload.scope, None)
    .await
    .into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/users", get(users))
    .route("/users", post(create_user))
    .route(
      "/users/reset-password",
      post(create_user_reset_password_token),
    )
    .with_state(state)
}
