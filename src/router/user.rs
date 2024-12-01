use crate::{
  core::error::Errors,
  middleware::{
    service_account_authorization::ServiceAccountAuthorization, validated_json::ValidatedJson,
  },
  model::user::{CreateUser, User},
  repository::{self, user::get_users},
};

use axum::{
  Router,
  extract::State,
  response::IntoResponse,
  routing::{get, post},
};
use http::StatusCode;
use utoipa::OpenApi;

use super::RouterState;

#[derive(OpenApi)]
#[openapi(
  paths(
    users,
    create_user,
  ),
  components(
    schemas(
      CreateUser,
      User,
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
  responses(
    (status = 200, content_type = "application/json", body = Vec<User>),
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
) -> impl IntoResponse {
  let user_rows = match get_users(&state.pool).await {
    Ok(users) => users,
    Err(e) => {
      log::error!("error getting users: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
  };
  let users: Vec<User> = user_rows.into_iter().map(Into::into).collect();
  axum::Json(users).into_response()
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

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/users", get(users))
    .route("/users", post(create_user))
    .with_state(state)
}
