use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::repository::user_totp::UserTOTPRow;

#[derive(Serialize, ToSchema)]
pub struct UserTOTP {
  pub algorithm: String,
  pub digits: i64,
  pub step: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub secret: Option<String>,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl From<UserTOTPRow> for UserTOTP {
  fn from(row: UserTOTPRow) -> Self {
    Self {
      algorithm: row.algorithm,
      digits: row.digits,
      step: row.step,
      secret: Some(row.secret),
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateTOTPRequest {
  #[schema(example = "SHA1")]
  pub algorithm: Option<String>,
  #[schema(example = "6")]
  pub digits: Option<i64>,
  #[schema(example = "30")]
  pub step: Option<i64>,
}
