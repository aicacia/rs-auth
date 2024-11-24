use build_time::build_time_utc;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Health {
  pub db: bool,
}

impl Health {
  pub fn is_healthy(&self) -> bool {
    self.db
  }
}

#[derive(Serialize, ToSchema)]
pub struct Version {
  pub version: String,
  pub build: DateTime<Utc>,
}

impl Default for Version {
  fn default() -> Self {
    Version {
      version: env!("CARGO_PKG_VERSION").to_string(),
      build: DateTime::parse_from_rfc3339(build_time_utc!())
        .expect("invalid build time")
        .with_timezone(&Utc),
    }
  }
}
