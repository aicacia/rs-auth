use serde::Serialize;
use utoipa::ToSchema;

use crate::core::config::Config;

#[derive(Serialize, ToSchema)]
pub struct P2P {
  pub enabled: bool,
  pub tenant_client_id: uuid::Uuid,
  pub ws_uri: String,
  pub api_uri: String,
  pub id: String,
  pub password: String,
}

impl P2P {
  pub fn new(config: &Config) -> Self {
    Self {
      enabled: config.p2p.enabled,
      tenant_client_id: config.p2p.tenant_client_id.clone(),
      ws_uri: config.p2p.ws_uri.clone(),
      api_uri: config.p2p.api_uri.clone(),
      id: config.p2p.id.clone(),
      password: config.p2p.password.clone(),
    }
  }
}
