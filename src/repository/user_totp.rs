use std::io;

use totp_rs::Secret;

#[derive(sqlx::FromRow)]
pub struct UserTOTPRow {
  pub user_id: i64,
  pub active: i32,
  pub algorithm: String,
  pub digits: i64,
  pub step: i64,
  pub secret: String,
  pub updated_at: i64,
  pub created_at: i64,
}

impl UserTOTPRow {
  pub fn is_active(&self) -> bool {
    self.active == 1
  }

  pub fn algorithm(&self) -> totp_rs::Algorithm {
    match self.algorithm.as_str() {
      "SHA1" => totp_rs::Algorithm::SHA1,
      "SHA256" => totp_rs::Algorithm::SHA256,
      "SHA512" => totp_rs::Algorithm::SHA512,
      _ => totp_rs::Algorithm::SHA1,
    }
  }

  pub fn totp(&self) -> Result<totp_rs::TOTP, totp_rs::TotpUrlError> {
    totp_rs::TOTP::new(
      self.algorithm(),
      self.digits as usize,
      1,
      self.step as u64,
      Secret::Encoded(self.secret.to_owned())
        .to_bytes()
        .unwrap_or_default(),
    )
  }

  pub fn verify(&self, code: &str) -> Result<bool, io::Error> {
    match self.totp() {
      Ok(totp) => match totp.check_current(code) {
        Ok(valid) => Ok(valid),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
      },
      Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    }
  }
}

pub async fn get_user_totp_by_user_id(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Option<UserTOTPRow>> {
  sqlx::query_as(
    r#"SELECT ut.*
    FROM user_totps ut
    WHERE ut.user_id = $1 AND ut.active = TRUE 
    LIMIT 1;"#,
  )
  .bind(user_id)
  .fetch_optional(pool)
  .await
}

#[derive(Default)]
pub struct CreateUserTOTP {
  pub algorithm: String,
  pub digits: i64,
  pub step: i64,
  pub secret: String,
}

pub async fn create_user_totp(
  pool: &sqlx::AnyPool,
  user_id: i64,
  params: CreateUserTOTP,
) -> sqlx::Result<UserTOTPRow> {
  sqlx::query_as(
    r#"INSERT INTO user_totps (user_id, algorithm, digits, step, secret)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING *;"#,
  )
  .bind(user_id)
  .bind(params.algorithm)
  .bind(params.digits)
  .bind(params.step)
  .bind(params.secret)
  .fetch_one(pool)
  .await
}

#[derive(Default)]
pub struct UpdateUserTOTPRow {
  pub algorithm: Option<String>,
  pub digits: Option<i64>,
  pub step: Option<i64>,
  pub secret: Option<String>,
}

pub async fn update_user_totp(
  pool: &sqlx::AnyPool,
  user_id: i64,
  params: UpdateUserTOTPRow,
) -> sqlx::Result<Option<UserTOTPRow>> {
  sqlx::query_as(
    r#"UPDATE user_totps SET 
      algorithm = COALESCE($2, algorithm),
      digits = COALESCE($3, digits),
      step = COALESCE($4, step),
      secret = COALESCE($5, secret)
    WHERE user_id = $1
    RETURNING *;"#,
  )
  .bind(user_id)
  .bind(params.algorithm)
  .bind(params.digits)
  .bind(params.step)
  .bind(params.secret)
  .fetch_optional(pool)
  .await
}

pub async fn delete_user_totp(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Option<UserTOTPRow>> {
  sqlx::query_as(
    r#"DELETE FROM user_totps
    WHERE user_id = $1
    RETURNING *;"#,
  )
  .bind(user_id)
  .fetch_optional(pool)
  .await
}
