use serde::Serialize;
use utoipa::ToSchema;

use crate::core::config::get_config;

#[derive(Serialize, ToSchema)]
pub struct P2P {
  pub enabled: bool,
  pub tenant_id: i64,
  pub ws_uri: String,
  pub api_uri: String,
  pub id: String,
  pub password: String,
}

impl Default for P2P {
  fn default() -> Self {
    let config = get_config();
    Self {
      enabled: config.p2p.enabled,
      tenant_id: config.p2p.tenant_id,
      ws_uri: config.p2p.ws_uri.clone(),
      api_uri: config.p2p.api_uri.clone(),
      id: config.p2p.id.clone(),
      password: config.p2p.password.clone(),
    }
  }
}
