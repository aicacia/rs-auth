use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct SignInWithPasswordRequest {
  pub username_or_email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct SignUpWithPasswordRequest {
  pub username: String,
  #[validate(email)]
  pub email: Option<String>,
  #[validate(length(min = 1, max = 256))]
  pub password: String,
  #[validate(length(min = 1, max = 256))]
  pub password_confirmation: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct ResetPasswrodRequest {
  pub reset_password_token: Uuid,
  #[validate(length(min = 1, max = 256))]
  pub password: String,
  #[validate(length(min = 1, max = 256))]
  pub password_confirmation: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct RequestResetPasswrodRequest {
  #[validate(email)]
  pub email: String,
}
