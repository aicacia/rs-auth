use serde::Deserialize;
use serde_json::{Map, Value};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct JWTRequest {
  pub tenent_id: i64,
  pub claims: Map<String, Value>,
}
