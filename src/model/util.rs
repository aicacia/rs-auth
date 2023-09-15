use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub struct Health {
  pub db: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
  pub ok: bool,
}

impl From<Health> for HealthResponse {
  fn from(health: Health) -> Self {
    Self { ok: health.db }
  }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct VersionResponse {
  pub version: String,
}
