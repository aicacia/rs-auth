use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum MFARequest {
  #[serde(rename = "totp")]
  TOTP { code: String },
}
