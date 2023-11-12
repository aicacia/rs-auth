use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct SignInWithPasswordRequest {
  pub application_id: i32,
  #[validate(length(min = 1), custom = "validate_username")]
  pub username_or_email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct SignUpWithPasswordRequest {
  pub application_id: i32,
  #[validate(length(min = 1), custom = "validate_username")]
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

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct SignUpMethods {
  pub enabled: bool,
  pub password: bool,
}

impl Default for SignUpMethods {
  fn default() -> Self {
    Self {
      enabled: false,
      password: false,
    }
  }
}

impl SignUpMethods {
  pub fn validate(mut self) -> Self {
    self.enabled = self.password;
    self
  }
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
  if username.trim().len() != username.len() {
    return Err(ValidationError::new("username_whitespace"));
  }
  Ok(())
}
