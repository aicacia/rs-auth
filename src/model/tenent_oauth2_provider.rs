use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use crate::repository::tenent_oauth2_provider::TenentOAuth2ProviderRow;

#[derive(Serialize, ToSchema)]
pub struct TenentOAuth2Provider {
  pub id: i64,
  pub tenent_id: i64,
  pub provider: String,
  pub active: bool,
  pub client_id: String,
  pub client_secret: String,
  pub auth_url: String,
  pub token_url: String,
  pub scope: Option<String>,
  pub redirect_uri: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<TenentOAuth2ProviderRow> for TenentOAuth2Provider {
  fn from(row: TenentOAuth2ProviderRow) -> Self {
    Self {
      id: row.id,
      tenent_id: row.tenent_id,
      active: row.is_active(),
      provider: row.provider,
      client_id: row.client_id,
      client_secret: row.client_secret,
      auth_url: row.auth_url,
      token_url: row.token_url,
      scope: row.scope,
      redirect_uri: row.redirect_uri,
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
    }
  }
}
