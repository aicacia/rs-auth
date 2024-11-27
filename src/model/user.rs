use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::repository::{
  user::UserRow, user_email::UserEmailRow, user_oauth2_provider::UserOAuth2ProviderRow,
  user_phone_number::UserPhoneNumberRow,
};

#[derive(Serialize, ToSchema)]
pub struct User {
  pub id: i64,
  pub username: String,
  pub active: bool,
  pub email: Option<UserEmail>,
  pub emails: Vec<UserEmail>,
  pub phone_number: Option<UserPhoneNumber>,
  pub phone_numbers: Vec<UserPhoneNumber>,
  pub oauth2_providers: Vec<UserOAuth2Provider>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<UserRow> for User {
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
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Serialize, ToSchema)]
pub struct UserEmail {
  pub id: i64,
  pub primary: bool,
  pub verified: bool,
  pub email: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<UserEmailRow> for UserEmail {
  fn from(row: UserEmailRow) -> Self {
    Self {
      id: row.id,
      primary: row.primary != 0,
      verified: row.verified != 0,
      email: row.email,
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Serialize, ToSchema)]
pub struct UserPhoneNumber {
  pub id: i64,
  pub primary: bool,
  pub verified: bool,
  pub phone_number: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<UserPhoneNumberRow> for UserPhoneNumber {
  fn from(row: UserPhoneNumberRow) -> Self {
    Self {
      id: row.id,
      primary: row.primary != 0,
      verified: row.verified != 0,
      phone_number: row.phone_number,
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Serialize, ToSchema)]
pub struct UserOAuth2Provider {
  pub id: i64,
  pub provider: String,
  pub email: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<UserOAuth2ProviderRow> for UserOAuth2Provider {
  fn from(row: UserOAuth2ProviderRow) -> Self {
    Self {
      id: row.id,
      provider: row.provider,
      email: row.email,
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct CreateUser {
  pub username: String,
  pub active: bool,
}
