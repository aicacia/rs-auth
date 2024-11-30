use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::repository::user::UserRow;

use super::user::{UserEmail, UserInfo, UserOAuth2Provider, UserPhoneNumber};

#[derive(Serialize, ToSchema)]
pub struct CurrentUser {
  pub id: i64,
  pub username: String,
  pub active: bool,
  pub email: Option<UserEmail>,
  pub emails: Vec<UserEmail>,
  pub phone_number: Option<UserPhoneNumber>,
  pub phone_numbers: Vec<UserPhoneNumber>,
  pub oauth2_providers: Vec<UserOAuth2Provider>,
  pub info: UserInfo,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<UserRow> for CurrentUser {
  fn from(row: UserRow) -> Self {
    Self {
      id: row.id,
      username: row.username,
      active: row.active != 0,
      email: None,
      emails: Vec::default(),
      phone_number: None,
      phone_numbers: Vec::default(),
      oauth2_providers: Vec::default(),
      info: UserInfo::default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
    }
  }
}

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
  pub birthdate: Option<String>,
  pub zone_info: Option<String>,
  pub locale: Option<String>,
  pub address: Option<String>,
}
