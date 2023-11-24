use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct SignInWithPasswordRequest {
  pub application_id: Uuid,
  #[validate(length(min = 1), custom = "validate_no_whitespace")]
  pub username_or_email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct SignUpWithPasswordRequest {
  pub application_id: Uuid,
  #[validate(length(min = 1), custom = "validate_no_whitespace")]
  pub username: String,
  #[validate(email)]
  pub email: Option<String>,
  #[validate(length(min = 1, max = 255))]
  pub password: String,
  #[validate(
    length(min = 1, max = 255),
    must_match(other = "password"),
    custom = "validate_no_whitespace"
  )]
  pub password_confirmation: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct ResetPasswordRequest {
  pub application_id: Uuid,
  pub reset_password_token: Uuid,
  #[validate(length(min = 1, max = 255))]
  pub password: String,
  #[validate(length(min = 1, max = 255), must_match(other = "password"))]
  pub password_confirmation: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct RequestResetPasswordRequest {
  pub application_id: Uuid,
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

pub fn validate_no_whitespace(username: &str) -> Result<(), ValidationError> {
  if username.chars().filter(|c| !c.is_whitespace()).count() != username.len() {
    return Err(ValidationError::new("whitespace"));
  }
  Ok(())
}
