use utoipa::{
  openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
  Modify,
};

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
