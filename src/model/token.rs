use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const TOKEN_ISSUED_TYPE_PASSWORD: &str = "password";
pub const TOKEN_ISSUED_TYPE_REFRESH_TOKEN: &str = "refresh-token";
pub const TOKEN_ISSUED_TYPE_AUTHORIZATION_CODE: &str = "authorization-code";
pub const TOKEN_ISSUED_TYPE_SERVICE_ACCOUNT: &str = "service-account";
pub const TOKEN_ISSUED_TYPE_REGISTER: &str = "register";
pub const TOKEN_ISSUED_TYPE_MFA: &str = "mfa";

#[derive(Serialize, ToSchema)]
pub struct Token {
  pub access_token: String,
  pub token_type: String,
  pub issued_token_type: String,
  pub expires_in: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub scope: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub refresh_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub refresh_token_expires_in: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id_token: Option<String>,
}

#[derive(Deserialize, ToSchema)]
#[serde(tag = "grant_type")]
pub enum TokenRequest {
  #[serde(rename = "password")]
  Password {
    username: String,
    password: String,
    scope: Option<String>,
  },
  #[serde(rename = "refresh_token")]
  RefreshToken { refresh_token: String },
  #[serde(rename = "service_account")]
  ServiceAccount {
    client_id: uuid::Uuid,
    secret: uuid::Uuid,
  },
  #[serde(rename = "authorization_code")]
  AuthorizationCode { code: String },
}
