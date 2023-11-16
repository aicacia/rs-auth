use super::auth::validate_username;
use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use chrono::{DateTime, Utc};
use futures::future::{err, ok};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use super::error::Errors;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct UserRow {
  pub id: i32,
  pub username: String,
  pub email_id: Option<i32>,
  pub encrypted_password: String,
  pub reset_password_token: Option<Uuid>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl FromRequest for UserRow {
  type Error = actix_web::Error;
  type Future = futures::future::Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    match req.extensions().get::<UserRow>() {
      Some(user) => ok(user.clone()),
      None => {
        let mut errors = Errors::new();
        errors.error("user", "invalid");
        err(actix_web::error::ErrorUnauthorized(errors))
      }
    }
  }
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct EmailRow {
  pub id: i32,
  pub user_id: i32,
  pub email: String,
  pub confirmed: bool,
  pub confirmation_token: Option<Uuid>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Email {
  pub id: i32,
  pub email: String,
  pub confirmed: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<EmailRow> for Email {
  fn from(email: EmailRow) -> Self {
    Self {
      id: email.id,
      email: email.email,
      confirmed: email.confirmed,
      created_at: email.created_at,
      updated_at: email.updated_at,
    }
  }
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
  pub id: i32,
  pub permissions: Vec<String>,
  pub username: String,
  pub email: Option<Email>,
  pub emails: Vec<Email>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<(UserRow, Vec<EmailRow>, Vec<String>)> for User {
  fn from((user, emails, permissions): (UserRow, Vec<EmailRow>, Vec<String>)) -> Self {
    let mut email: Option<EmailRow> = None;
    let mut emails = emails.clone();

    if let Some(index) = emails.iter().position(|e| user.email_id == Some(e.id)) {
      email = Some(emails.remove(index));
    }

    Self {
      id: user.id,
      permissions,
      username: user.username,
      email: email.map(Email::from),
      emails: emails.into_iter().map(Email::from).collect(),
      created_at: user.created_at,
      updated_at: user.updated_at,
    }
  }
}

#[derive(Deserialize)]
pub struct PaginationQuery {
  pub page: Option<i64>,
  pub page_size: Option<i64>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PaginationUser {
  pub has_more: bool,
  pub data: Vec<User>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct ResetUserPasswordRequest {
  #[validate(length(min = 1, max = 255))]
  pub password: String,
  #[validate(length(min = 1, max = 255))]
  pub password_confirmation: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct ChangeUsernameRequest {
  #[validate(length(min = 1), custom = "validate_username")]
  pub username: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct CreateUserEmailRequest {
  #[validate(email)]
  pub email: String,
}
