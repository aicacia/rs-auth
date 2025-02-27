use crate::core::database::run_transaction;

use super::user::{from_users_query, UserRow};

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
  application_id: i64,
  limit: Option<usize>,
  offset: Option<usize>,
) -> sqlx::Result<Vec<UserEmailRow>> {
  let mut qb =
    sqlx::QueryBuilder::new("SELECT ue.* FROM user_emails ue WHERE ue.user_id IN (SELECT u.id");
  from_users_query(&mut qb, application_id, limit, offset);
  qb.push(")");
  qb.build_query_as().fetch_all(pool).await
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

pub async fn get_user_emails_by_user_id(
  pool: &sqlx::AnyPool,
  application_id: i64,
  user_id: i64,
) -> sqlx::Result<Vec<UserEmailRow>> {
  sqlx::query_as(
    r#"SELECT ue.*
    FROM user_emails ue
    JOIN users u ON u.id = ue.user_id
    WHERE u.application_id = $1 AND ue.user_id = $2;"#,
  )
  .bind(application_id)
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
        sqlx::query(
          r#"UPDATE user_emails SET 
            "primary" = 0,
            "updated_at" = $3
            WHERE user_id=$1 AND id != $2;"#,
        )
        .bind(user_id)
        .bind(email.id)
        .bind(chrono::Utc::now().timestamp())
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
      let now = chrono::Utc::now().timestamp();
      let email: Option<UserEmailRow> = sqlx::query_as(
        r#"UPDATE user_emails SET 
          primary = COALESCE($3, primary),
          verified = COALESCE($4, verified),
          updated_at = $5
        WHERE user_id = $1 AND id = $2
        RETURNING *;"#,
      )
      .bind(user_id)
      .bind(email_id)
      .bind(params.primary)
      .bind(params.verified)
      .bind(now)
      .fetch_optional(&mut **transaction)
      .await?;

      if let Some(email) = email.as_ref() {
        if email.is_primary() {
          sqlx::query(
            r#"UPDATE user_emails SET 
          "primary" = 0, 
          "updated_at" = $3 
          WHERE user_id = $1 AND id != $2;"#,
          )
          .bind(user_id)
          .bind(email_id)
          .bind(now)
          .execute(&mut **transaction)
          .await?;
        }
        sqlx::query(
          r#"UPDATE user_configs SET
        mfa_type = NULL,
        updated_at = $2
        WHERE user_id = $1 AND mfa_type = 'email' AND NOT EXISTS(
          SELECT ue.id 
          FROM user_emails ue 
          JOIN users u ON u.id = ue.user_id
          WHERE u.user_id = $1 AND ue.verified = 1
        )
        RETURNING *;"#,
        )
        .bind(user_id)
        .bind(chrono::Utc::now().timestamp())
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
      let now = chrono::Utc::now().timestamp();
      let email: UserEmailRow = sqlx::query_as(
        r#"UPDATE user_emails SET 
        "primary" = 1,
        "updated_at" = $3 
        WHERE "verified" = 1 AND user_id=$1 AND id = $2 
        RETURNING *;"#,
      )
      .bind(user_id)
      .bind(email_id)
      .bind(now)
      .fetch_one(&mut **transaction)
      .await?;

      sqlx::query(
        r#"UPDATE user_emails SET
        "primary" = 0,
        "updated_at" = $3 
        WHERE user_id=$1 AND id != $2;"#,
      )
      .bind(user_id)
      .bind(email_id)
      .bind(now)
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
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let email: Option<UserEmailRow> = sqlx::query_as(
        r#"DELETE FROM user_emails
        WHERE user_id = $1 AND id = $2
        RETURNING *;"#,
      )
      .bind(user_id)
      .bind(email_id)
      .fetch_optional(&mut **transaction)
      .await?;

      if let Some(email) = email.as_ref() {
        if email.is_primary() && email.is_verified() {
          sqlx::query(
            r#"UPDATE user_configs SET
            mfa_type = NULL,
            updated_at = $2
            WHERE user_id = $1 AND mfa_type = 'email'
            RETURNING *;"#,
          )
          .bind(user_id)
          .bind(chrono::Utc::now().timestamp())
          .execute(&mut **transaction)
          .await?;
        }
      }

      Ok(email)
    })
  })
  .await
}
