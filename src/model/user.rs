use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use chrono::{DateTime, Utc};
use futures::future::{err, ok};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email_id: Option<i32>,
  pub encrypted_password: String,
  pub reset_password_token: Option<Uuid>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl FromRequest for User {
  type Error = actix_web::Error;
  type Future = futures::future::Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    match req.extensions().get::<User>() {
      Some(user) => ok(user.clone()),
      None => err(actix_web::error::ErrorUnauthorized("invalid_user")),
    }
  }
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Email {
  pub id: i32,
  pub user_id: i32,
  pub email: String,
  pub confirmed: bool,
  pub confirmation_token: Option<Uuid>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct EmailResponse {
  pub id: i32,
  pub email: String,
  pub confirmed: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<Email> for EmailResponse {
  fn from(email: Email) -> Self {
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
pub struct UserResponse {
  pub id: i32,
  pub username: String,
  pub email: Option<EmailResponse>,
  pub emails: Vec<EmailResponse>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<(User, Vec<Email>)> for UserResponse {
  fn from((user, emails): (User, Vec<Email>)) -> Self {
    let mut email: Option<Email> = None;
    let mut emails = emails.clone();

    if let Some(index) = emails.iter().position(|e| user.email_id == Some(e.id)) {
      email = Some(emails.remove(index));
    }

    Self {
      id: user.id,
      username: user.username,
      email: email.map(EmailResponse::from),
      emails: emails.into_iter().map(EmailResponse::from).collect(),
      created_at: user.created_at,
      updated_at: user.updated_at,
    }
  }
}
