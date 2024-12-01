use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::claims::{BasicClaims, Claims};

pub const SCOPE_OPENID: &str = "openid";
pub const SCOPE_PROFILE: &str = "profile";
pub const SCOPE_EMAIL: &str = "email";
pub const SCOPE_PHONE: &str = "phone";
pub const SCOPE_ADDRESS: &str = "address";

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct OpenIdProfile {
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
  pub preferred_username: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub profile_picture: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub website: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email_verified: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gender: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub birthdate: Option<DateTime<Utc>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub zone_info: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub locale: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone_number_verified: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub address: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct OpenIdClaims {
  #[serde(flatten)]
  pub claims: BasicClaims,
  #[serde(flatten)]
  pub profile: OpenIdProfile,
}

unsafe impl Send for OpenIdClaims {}

impl Claims for OpenIdClaims {
  fn kind(&self) -> &String {
    &self.claims.kind
  }
  fn exp(&self) -> i64 {
    self.claims.exp
  }
  fn iat(&self) -> i64 {
    self.claims.iat
  }
  fn nbf(&self) -> i64 {
    self.claims.nbf
  }
  fn iss(&self) -> &String {
    &self.claims.iss
  }
  fn aud(&self) -> &String {
    &self.claims.aud
  }
  fn sub_kind(&self) -> &String {
    &self.claims.sub_kind
  }
  fn sub(&self) -> i64 {
    self.claims.sub
  }
  fn app(&self) -> i64 {
    self.claims.app
  }
  fn scopes(&self) -> &[String] {
    &self.claims.scopes
  }
}

pub fn parse_scopes(scopes: Option<&str>) -> Vec<String> {
  match scopes {
    Some(scopes) => scopes.split(' ').map(|s| s.trim().to_owned()).collect(),
    None => vec![],
  }
}

pub fn has_profile_scope(scopes: &[String]) -> bool {
  scopes.contains(&SCOPE_PROFILE.to_owned()) || has_openid_scope(scopes)
}

pub fn has_email_scope(scopes: &[String]) -> bool {
  scopes.contains(&SCOPE_EMAIL.to_owned()) || has_openid_scope(scopes)
}

pub fn has_phone_scope(scopes: &[String]) -> bool {
  scopes.contains(&SCOPE_PHONE.to_owned()) || has_openid_scope(scopes)
}

pub fn has_address_scope(scopes: &[String]) -> bool {
  scopes.contains(&SCOPE_ADDRESS.to_owned()) || has_openid_scope(scopes)
}

pub fn has_openid_scope(scopes: &[String]) -> bool {
  scopes.contains(&SCOPE_OPENID.to_owned())
}
