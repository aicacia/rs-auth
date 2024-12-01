use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Validate, Deserialize, ToSchema)]
pub struct ResetPasswordRequest {
  pub current_password: String,
  #[validate(length(min = 6), must_match(other = "password_confirmation"))]
  pub password: String,
  #[validate(length(min = 6))]
  pub password_confirmation: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserInfoRequest {
  pub name: Option<String>,
  pub given_name: Option<String>,
  pub family_name: Option<String>,
  pub middle_name: Option<String>,
  pub nickname: Option<String>,
  pub profile_picture: Option<String>,
  pub website: Option<String>,
  pub gender: Option<String>,
  pub birthdate: Option<DateTime<Utc>>,
  pub zone_info: Option<String>,
  pub locale: Option<String>,
  pub address: Option<String>,
}
