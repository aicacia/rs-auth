use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct SignInWithPasswordRequest {
  pub application_id: i32,
  pub username_or_email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct SignUpWithPasswordRequest {
  pub application_id: i32,
  pub username: String,
  #[validate(email)]
  pub email: Option<String>,
  #[validate(length(min = 1, max = 255))]
  pub password: String,
  #[validate(length(min = 1, max = 255), must_match(other = "password"))]
  pub password_confirmation: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct ResetPasswordRequest {
  pub application_id: i32,
  pub reset_password_token: Uuid,
  #[validate(length(min = 1, max = 255))]
  pub password: String,
  #[validate(length(min = 1, max = 255), must_match(other = "password"))]
  pub password_confirmation: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct RequestResetPasswordRequest {
  pub application_id: i32,
  #[validate(email)]
  pub email: String,
}
