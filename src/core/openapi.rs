use utoipa::{
  openapi::{
    security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
    Server,
  },
  Modify,
};

use super::config::get_config;

pub const AUTHORIZATION_HEADER: &str = "Authorization";
pub const TENENT_ID_HEADER: &str = "Tenant-ID";

pub struct SecurityAddon;

impl Modify for SecurityAddon {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    let components = openapi.components.as_mut().unwrap();
    components.add_security_scheme(
      "Authorization",
      SecurityScheme::Http(
        HttpBuilder::new()
          .scheme(HttpAuthScheme::Bearer)
          .bearer_format("JWT")
          .build(),
      ),
    );
    components.add_security_scheme(
      "TenantUUID",
      SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(TENENT_ID_HEADER))),
    );
  }
}

pub struct ServersAddon;

impl Modify for ServersAddon {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    let config = get_config();
    openapi
      .servers
      .get_or_insert(Vec::default())
      .push(Server::new(config.server.url.clone()));
  }
}
