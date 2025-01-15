use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum MFARequest {
  #[serde(rename = "totp")]
  #[schema(title = "MFARequestTOTP")]
  TOTP { code: String },
  #[serde(rename = "service-account")]
  #[schema(title = "MFARequestServiceAccount")]
  ServiceAccount { code: String },
}
