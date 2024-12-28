use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
  pub callback_url: String,
  pub redirect_url: String,
  pub scope: String,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl From<TenentOAuth2ProviderRow> for TenentOAuth2Provider {
  fn from(row: TenentOAuth2ProviderRow) -> Self {
    let active = row.is_active();
    let callback_url = row.callback_url();
    Self {
      id: row.id,
      tenent_id: row.tenent_id,
      provider: row.provider,
      active,
      client_id: row.client_id,
      client_secret: row.client_secret,
      auth_url: row.auth_url,
      token_url: row.token_url,
      callback_url,
      redirect_url: row.redirect_url,
      scope: row.scope,
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateTenentOAuth2Provider {
  pub provider: String,
  pub client_id: String,
  pub client_secret: String,
  pub auth_url: Option<String>,
  pub token_url: Option<String>,
  pub callback_url: Option<String>,
  pub redirect_url: String,
  pub scope: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateTenentOAuth2Provider {
  pub client_id: Option<String>,
  pub client_secret: Option<String>,
  pub active: Option<bool>,
  pub auth_url: Option<String>,
  pub token_url: Option<String>,
  pub callback_url: Option<String>,
  pub redirect_url: Option<String>,
  pub scope: Option<String>,
}