use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, ToSchema)]
pub struct RegisterUser {
  pub username: String,
  #[validate(must_match(other = "password_confirmation"))]
  pub password: String,
  pub password_confirmation: String,
}
