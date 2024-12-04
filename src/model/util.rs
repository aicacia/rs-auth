use build_time::build_time_utc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Health {
  pub db: bool,
}

impl Health {
  pub fn is_healthy(&self) -> bool {
    self.db
  }
}

#[derive(Serialize, Deserialize, ToSchema)]
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

pub const DEFAULT_LIMIT: usize = 20;

#[derive(Deserialize, IntoParams)]
pub struct OffsetAndLimit {
  pub offset: Option<usize>,
  pub limit: Option<usize>,
}

#[derive(Serialize, ToSchema)]
pub struct Pagination<T> {
  pub has_more: bool,
  pub items: Vec<T>,
}
