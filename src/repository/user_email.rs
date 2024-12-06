use crate::core::database::run_transaction;

use super::user::UserRow;

#[derive(sqlx::FromRow)]
pub struct UserEmailRow {
  pub id: i64,
  pub user_id: i64,
  pub primary: i32,
  pub verified: i32,
  pub email: String,
  pub updated_at: i64,
  pub created_at: i64,
}

impl UserEmailRow {
  pub fn is_verified(&self) -> bool {
    self.verified != 0
  }
  pub fn is_primary(&self) -> bool {
    self.primary != 0
  }
}

pub async fn get_users_emails(
  pool: &sqlx::AnyPool,
  limit: usize,
  offset: usize,
) -> sqlx::Result<Vec<UserEmailRow>> {
  sqlx::query_as(r#"SELECT ue.* FROM user_emails ue WHERE ue.user_id in (SELECT u.id FROM users u LIMIT $1 OFFSET $2);"#)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
}

pub async fn get_user_by_email(pool: &sqlx::AnyPool, email: &str) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    JOIN user_emails ue ON u.id = ue.user_id
    WHERE ue.email = $1;"#,
  )
  .bind(email)
  .fetch_optional(pool)
  .await
}

pub async fn get_user_primary_email(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Option<UserEmailRow>> {
  sqlx::query_as(
    r#"SELECT ue.*
    FROM user_emails ue
    WHERE ue.user_id = $1 AND ue."primary" = TRUE 
    LIMIT 1;"#,
  )
  .bind(user_id)
  .fetch_optional(pool)
  .await
}

pub async fn get_user_emails_by_user_id(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Vec<UserEmailRow>> {
  sqlx::query_as(
    r#"SELECT ue.*
    FROM user_emails ue
    WHERE ue.user_id = $1;"#,
  )
  .bind(user_id)
  .fetch_all(pool)
  .await
}

#[derive(Default)]
pub struct CreateUserEmail {
  pub email: String,
  pub primary: Option<bool>,
  pub verified: Option<bool>,
}

pub async fn create_user_email(
  pool: &sqlx::AnyPool,
  user_id: i64,
  params: CreateUserEmail,
) -> sqlx::Result<UserEmailRow> {
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let email: UserEmailRow = sqlx::query_as(
        r#"INSERT INTO user_emails ("user_id", "email", "primary", "verified")
        VALUES ($1, $2, $3, $4)
        RETURNING *;"#,
      )
      .bind(user_id)
      .bind(params.email)
      .bind(params.primary.unwrap_or(false))
      .bind(params.verified.unwrap_or(false))
      .fetch_one(&mut **transaction)
      .await?;

      if email.is_primary() {
        sqlx::query(r#"UPDATE user_emails SET "primary" = FALSE WHERE user_id=$1 AND id != $2;"#)
          .bind(user_id)
          .bind(email.id)
          .execute(&mut **transaction)
          .await?;
      }

      Ok(email)
    })
  })
  .await
}

#[derive(Default)]
pub struct UpdateUserEmail {
  pub primary: Option<bool>,
  pub verified: Option<bool>,
}

pub async fn update_user_email(
  pool: &sqlx::AnyPool,
  user_id: i64,
  email_id: i64,
  params: UpdateUserEmail,
) -> sqlx::Result<Option<UserEmailRow>> {
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let email: Option<UserEmailRow> = sqlx::query_as(
        r#"UPDATE user_emails SET 
          "primary" = COALESCE($3, "primary"),
          "verified" = COALESCE($4, "verified")
        WHERE user_id = $1 AND id = $2
        RETURNING *;"#,
      )
      .bind(user_id)
      .bind(email_id)
      .bind(params.primary)
      .bind(params.verified)
      .fetch_optional(&mut **transaction)
      .await?;

      if email
        .as_ref()
        .map(UserEmailRow::is_primary)
        .unwrap_or(false)
      {
        sqlx::query(r#"UPDATE user_emails SET "primary" = FALSE WHERE user_id=$1 AND id != $2;"#)
          .bind(user_id)
          .bind(email_id)
          .execute(&mut **transaction)
          .await?;
      }

      Ok(email)
    })
  })
  .await
}

pub async fn set_user_email_as_primary(
  pool: &sqlx::AnyPool,
  user_id: i64,
  email_id: i64,
) -> sqlx::Result<UserEmailRow> {
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let email: UserEmailRow =
        sqlx::query_as(r#"UPDATE user_emails SET "primary" = TRUE WHERE "verified" = TRUE AND user_id=$1 AND id = $2 RETURNING *;"#)
          .bind(user_id)
          .bind(email_id)
          .fetch_one(&mut **transaction)
          .await?;

      sqlx::query(r#"UPDATE user_emails SET "primary" = FALSE WHERE user_id=$1 AND id != $2;"#)
        .bind(user_id)
        .bind(email_id)
        .execute(&mut **transaction)
        .await?;

      Ok(email)
    })
  })
  .await
}

pub async fn delete_user_email(
  pool: &sqlx::AnyPool,
  user_id: i64,
  email_id: i64,
) -> sqlx::Result<Option<UserEmailRow>> {
  sqlx::query_as(
    r#"DELETE FROM user_emails
    WHERE user_id = $1 AND id = $2 
    RETURNING *;"#,
  )
  .bind(user_id)
  .bind(email_id)
  .fetch_optional(pool)
  .await
}
