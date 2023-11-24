use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AuthorizeQuery {
  pub client_id: String,
  pub redirect_uri: String,
  pub response_type: String,
  pub scope: Vec<String>,
}
