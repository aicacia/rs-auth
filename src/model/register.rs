use serde::Deserialize;
use tokio::runtime::Handle;
use utoipa::ToSchema;
use validator::Validate;

use crate::{core::database::get_pool, repository::user::get_user_by_username};

#[derive(Validate, Deserialize, ToSchema)]
pub struct RegisterUser {
  #[validate(length(min = 1), custom(function = "validate_unique_username"))]
  pub username: String,
  #[validate(length(min = 6), must_match(other = "password_confirmation"))]
  pub password: String,
  #[validate(length(min = 6))]
  pub password_confirmation: String,
}

fn validate_unique_username(username: &str) -> Result<(), validator::ValidationError> {
  match tokio::task::block_in_place(move || {
    Handle::current().block_on(async move { get_user_by_username(&get_pool(), username).await })
  }) {
    Ok(Some(_)) => Err(validator::ValidationError::new("unique_username")),
    Ok(None) => Ok(()),
    Err(_) => Ok(()),
  }
}
