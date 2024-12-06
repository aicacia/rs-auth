use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::repository::{
  user::{UserMFATypeRow, UserRow},
  user_email::UserEmailRow,
  user_info::UserInfoRow,
  user_oauth2_provider::UserOAuth2ProviderRow,
  user_phone_number::UserPhoneNumberRow,
};

use super::register::validate_unique_username;

#[derive(Serialize, ToSchema)]
pub struct User {
  pub id: i64,
  pub username: String,
  pub active: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<UserEmail>,
  pub emails: Vec<UserEmail>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone_number: Option<UserPhoneNumber>,
  pub phone_numbers: Vec<UserPhoneNumber>,
  pub oauth2_providers: Vec<UserOAuth2Provider>,
  pub mfa_types: Vec<UserMFAType>,
  pub info: UserInfo,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl From<UserRow> for User {
  fn from(row: UserRow) -> Self {
    Self {
      id: row.id,
      username: row.username,
      active: row.active != 0,
      email: None,
      emails: Vec::new(),
      phone_number: None,
      phone_numbers: Vec::new(),
      oauth2_providers: Vec::new(),
      mfa_types: Vec::new(),
      info: UserInfo::default(),
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Serialize, ToSchema, Default)]
pub struct UserInfo {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub given_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub family_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub middle_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nickname: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub profile_picture: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub website: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gender: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub birthdate: Option<DateTime<Utc>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub zone_info: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub locale: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub address: Option<String>,
}

impl From<UserInfoRow> for UserInfo {
  fn from(row: UserInfoRow) -> Self {
    Self {
      name: row.name,
      given_name: row.given_name,
      family_name: row.family_name,
      middle_name: row.middle_name,
      nickname: row.nickname,
      profile_picture: row.profile_picture,
      website: row.website,
      gender: row.gender,
      birthdate: row
        .birthdate
        .map(|birthdate| DateTime::<Utc>::from_timestamp(birthdate, 0).unwrap_or_default()),
      zone_info: row.zone_info,
      locale: row.locale,
      address: row.address,
    }
  }
}

#[derive(Serialize, ToSchema)]
pub struct UserEmail {
  pub id: i64,
  pub primary: bool,
  pub verified: bool,
  pub email: String,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl From<UserEmailRow> for UserEmail {
  fn from(row: UserEmailRow) -> Self {
    Self {
      id: row.id,
      primary: row.primary != 0,
      verified: row.verified != 0,
      email: row.email,
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Serialize, ToSchema)]
pub struct UserPhoneNumber {
  pub id: i64,
  pub primary: bool,
  pub verified: bool,
  pub phone_number: String,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl From<UserPhoneNumberRow> for UserPhoneNumber {
  fn from(row: UserPhoneNumberRow) -> Self {
    Self {
      id: row.id,
      primary: row.primary != 0,
      verified: row.verified != 0,
      phone_number: row.phone_number,
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Serialize, ToSchema)]
pub struct UserOAuth2Provider {
  pub id: i64,
  pub tenent_oauth2_provider_id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl From<UserOAuth2ProviderRow> for UserOAuth2Provider {
  fn from(row: UserOAuth2ProviderRow) -> Self {
    Self {
      id: row.id,
      tenent_oauth2_provider_id: row.tenent_oauth2_provider_id,
      email: Some(row.email),
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Serialize, ToSchema)]
pub enum UserMFAType {
  #[serde(rename = "totp")]
  TOTP,
  #[serde(rename = "email")]
  Email,
  #[serde(rename = "text")]
  Text,
}

impl From<UserMFATypeRow> for UserMFAType {
  fn from(row: UserMFATypeRow) -> Self {
    match row.kind.as_str() {
      "totp" => Self::TOTP,
      "email" => Self::Email,
      "text" => Self::Text,
      _ => panic!("Unknown MFA type: {}", row.kind),
    }
  }
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct UpdateUsername {
  #[validate(length(min = 1), custom(function = "validate_unique_username"))]
  pub username: Option<String>,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct CreateUser {
  #[validate(length(min = 1), custom(function = "validate_unique_username"))]
  pub username: String,
  pub active: bool,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct UserResetPassword {
  pub tenent_id: i64,
  pub scope: Option<String>,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct CreateUserEmail {
  #[validate(email)]
  pub email: String,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct ServiceAccountCreateUserEmail {
  #[validate(email)]
  pub email: String,
  pub verified: Option<bool>,
  pub primary: Option<bool>,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct ServiceAccountUpdateUserEmail {
  pub verified: Option<bool>,
  pub primary: Option<bool>,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct CreateUserPhoneNumber {
  #[validate(length(min = 7))]
  pub phone_number: String,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct ServiceAccountCreateUserPhoneNumber {
  #[validate(length(min = 7))]
  pub phone_number: String,
  pub verified: Option<bool>,
  pub primary: Option<bool>,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct ServiceAccountUpdateUserPhoneNumber {
  pub verified: Option<bool>,
  pub primary: Option<bool>,
}
