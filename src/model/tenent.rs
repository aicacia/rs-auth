use std::{fmt, str::FromStr};

use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{core::encryption::random_bytes, repository::tenent::TenentRow};

use super::tenent_oauth2_provider::TenentOAuth2Provider;

#[derive(Serialize, ToSchema)]
pub struct Tenent {
  pub id: i64,
  pub client_id: uuid::Uuid,
  pub issuer: String,
  pub audience: Option<String>,
  pub algorithm: String,
  pub public_key: Option<String>,
  pub private_key: Option<String>,
  pub expires_in_seconds: i64,
  pub refresh_expires_in_seconds: i64,
  pub oauth2_providers: Vec<TenentOAuth2Provider>,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl From<TenentRow> for Tenent {
  fn from(row: TenentRow) -> Self {
    Self {
      id: row.id,
      client_id: uuid::Uuid::from_str(&row.client_id).unwrap_or_default(),
      issuer: row.issuer,
      audience: row.audience,
      algorithm: row.algorithm,
      public_key: row.public_key,
      private_key: None,
      expires_in_seconds: row.expires_in_seconds,
      refresh_expires_in_seconds: row.refresh_expires_in_seconds,
      oauth2_providers: Vec::new(),
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    }
  }
}

/// jsonwebtoken::Algorithm
#[derive(Deserialize, ToSchema, Default)]
pub enum Algorithm {
  /// HMAC using SHA-256
  #[default]
  HS256,
  /// HMAC using SHA-384
  HS384,
  /// HMAC using SHA-512
  HS512,

  /// ECDSA using SHA-256
  ES256,
  /// ECDSA using SHA-384
  ES384,

  /// RSASSA-PKCS1-v1_5 using SHA-256
  RS256,
  /// RSASSA-PKCS1-v1_5 using SHA-384
  RS384,
  /// RSASSA-PKCS1-v1_5 using SHA-512
  RS512,

  /// RSASSA-PSS using SHA-256
  PS256,
  /// RSASSA-PSS using SHA-384
  PS384,
  /// RSASSA-PSS using SHA-512
  PS512,

  /// Edwards-curve Digital Signature Algorithm (EdDSA)
  EdDSA,
}

impl Algorithm {
  pub fn keys(
    &self,
    public_key: Option<String>,
    private_key: Option<String>,
  ) -> (Option<String>, String) {
    match self {
      Algorithm::HS256 => (
        public_key,
        private_key.unwrap_or_else(|| BASE64_STANDARD.encode(random_bytes(256))),
      ),
      Algorithm::HS384 => (
        public_key,
        private_key.unwrap_or_else(|| BASE64_STANDARD.encode(random_bytes(384))),
      ),
      Algorithm::HS512 => (
        public_key,
        private_key.unwrap_or_else(|| BASE64_STANDARD.encode(random_bytes(512))),
      ),
      _ => (
        public_key,
        private_key.unwrap_or_else(|| BASE64_STANDARD.encode(random_bytes(256))),
      ),
    }
  }
}

impl fmt::Display for Algorithm {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Algorithm::HS256 => write!(f, "HS256"),
      Algorithm::HS384 => write!(f, "HS384"),
      Algorithm::HS512 => write!(f, "HS512"),
      Algorithm::ES256 => write!(f, "ES256"),
      Algorithm::ES384 => write!(f, "ES384"),
      Algorithm::RS256 => write!(f, "RS256"),
      Algorithm::RS384 => write!(f, "RS384"),
      Algorithm::RS512 => write!(f, "RS512"),
      Algorithm::PS256 => write!(f, "PS256"),
      Algorithm::PS384 => write!(f, "PS384"),
      Algorithm::PS512 => write!(f, "PS512"),
      Algorithm::EdDSA => write!(f, "EdDSA"),
    }
  }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateTenent {
  pub client_id: Option<uuid::Uuid>,
  #[schema(example = "Example")]
  pub issuer: String,
  #[schema(example = "https://example.com")]
  pub audience: String,
  #[schema(example = "HS256")]
  pub algorithm: Option<Algorithm>,
  pub public_key: Option<String>,
  pub private_key: Option<String>,
  #[schema(example = "86400")]
  pub expires_in_seconds: Option<i64>,
  #[schema(example = "604800")]
  pub refresh_expires_in_seconds: Option<i64>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateTenent {
  pub client_id: Option<uuid::Uuid>,
  #[schema(example = "Example")]
  pub issuer: Option<String>,
  #[schema(example = "example.com")]
  pub audience: Option<String>,
  #[schema(example = "HS256")]
  pub algorithm: Option<Algorithm>,
  pub public_key: Option<String>,
  pub private_key: Option<String>,
  #[schema(example = "86400")]
  pub expires_in_seconds: Option<i64>,
  #[schema(example = "604800")]
  pub refresh_expires_in_seconds: Option<i64>,
}

#[derive(Deserialize, IntoParams)]
pub struct TenentQuery {
  pub show_private_key: Option<bool>,
}
