use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
}
