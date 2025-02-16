use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::repository::application::ApplicationRow;

use super::util::Pagination;

#[derive(Serialize, ToSchema)]
pub struct Application {
  pub id: i64,
  pub name: String,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl From<ApplicationRow> for Application {
  fn from(row: ApplicationRow) -> Self {
    Self {
      id: row.id,
      name: row.name,
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    }
  }
}

pub type ApplicationPagination = Pagination<Application>;

#[derive(Deserialize, ToSchema)]
pub struct CreateApplication {
  pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateApplication {
  pub name: Option<String>,
}
