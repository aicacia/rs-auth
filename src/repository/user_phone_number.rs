use crate::core::database::run_transaction;

use super::user::{from_users_query, UserRow};

#[derive(sqlx::FromRow)]
pub struct UserPhoneNumberRow {
  pub id: i64,
  pub user_id: i64,
  pub primary: i32,
  pub verified: i32,
  pub phone_number: String,
  pub updated_at: i64,
  pub created_at: i64,
}

impl UserPhoneNumberRow {
  pub fn is_verified(&self) -> bool {
    self.verified != 0
  }
  pub fn is_primary(&self) -> bool {
    self.primary != 0
  }
}

pub async fn get_users_phone_numbers(
  pool: &sqlx::AnyPool,
  application_id: i64,
  limit: Option<usize>,
  offset: Option<usize>,
) -> sqlx::Result<Vec<UserPhoneNumberRow>> {
  let mut qb = sqlx::QueryBuilder::new(
    r#"SELECT upn.* FROM user_phone_numbers upn WHERE upn.user_id IN (SELECT u.id"#,
  );
  from_users_query(&mut qb, application_id, limit, offset);
  qb.push(")");
  qb.build_query_as().fetch_all(pool).await
}

pub async fn get_user_by_phone_number(
  pool: &sqlx::AnyPool,
  phone_number: &str,
) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    JOIN user_phone_numbers upn ON u.id = upn.user_id
    WHERE upn.phone_number = $1;"#,
  )
  .bind(phone_number)
  .fetch_optional(pool)
  .await
}

pub async fn get_user_phone_numbers_by_user_id(
  pool: &sqlx::AnyPool,
  application_id: i64,
  user_id: i64,
) -> sqlx::Result<Vec<UserPhoneNumberRow>> {
  sqlx::query_as(
    r#"SELECT upn.*
    FROM user_phone_numbers upn
    JOIN users u ON u.id = upn.user_id
    WHERE u.application_id = $1 AND upn.user_id = $2;"#,
  )
  .bind(application_id)
  .bind(user_id)
  .fetch_all(pool)
  .await
}

#[derive(Default)]
pub struct CreateUserPhoneNumber {
  pub phone_number: String,
  pub primary: Option<bool>,
  pub verified: Option<bool>,
}

pub async fn create_user_phone_number(
  pool: &sqlx::AnyPool,
  user_id: i64,
  params: CreateUserPhoneNumber,
) -> sqlx::Result<UserPhoneNumberRow> {
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let phone_number: UserPhoneNumberRow = sqlx::query_as(
        r#"INSERT INTO user_phone_numbers ("user_id", "phone_number", "primary", "verified")
        VALUES ($1, $2, $3, $4)
        RETURNING *;"#,
      )
      .bind(user_id)
      .bind(params.phone_number)
      .bind(params.primary.unwrap_or(false))
      .bind(params.verified.unwrap_or(false))
      .fetch_one(&mut **transaction)
      .await?;

      if phone_number.is_primary() {
        sqlx::query(
          r#"UPDATE user_phone_numbers SET 
            "primary" = 0,
            "updated_at" = $3
            WHERE user_id=$1 AND id != $2;"#,
        )
        .bind(user_id)
        .bind(phone_number.id)
        .bind(chrono::Utc::now().timestamp())
        .execute(&mut **transaction)
        .await?;
      }

      Ok(phone_number)
    })
  })
  .await
}

#[derive(Default)]
pub struct UpdateUserPhoneNumber {
  pub primary: Option<bool>,
  pub verified: Option<bool>,
}

pub async fn update_user_phone_number(
  pool: &sqlx::AnyPool,
  user_id: i64,
  phone_number_id: i64,
  params: UpdateUserPhoneNumber,
) -> sqlx::Result<Option<UserPhoneNumberRow>> {
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let now = chrono::Utc::now().timestamp();
      let phone_number: Option<UserPhoneNumberRow> = sqlx::query_as(
        r#"UPDATE user_phone_numbers SET 
          "primary" = COALESCE($3, "primary"),
          "verified" = COALESCE($4, "verified"),
          "updated_at" = $5
        WHERE user_id = $1 AND id = $2
        RETURNING *;"#,
      )
      .bind(user_id)
      .bind(phone_number_id)
      .bind(params.primary)
      .bind(params.verified)
      .bind(now)
      .fetch_optional(&mut **transaction)
      .await?;

      if let Some(phone_number) = phone_number.as_ref() {
        if phone_number.is_primary() {
          sqlx::query(
            r#"UPDATE user_phone_numbers SET 
            "primary" = 0, 
            "updated_at" = $3 
            WHERE user_id = $1 AND id != $2;"#,
          )
          .bind(user_id)
          .bind(phone_number_id)
          .bind(now)
          .execute(&mut **transaction)
          .await?;
        }
        sqlx::query(
          r#"UPDATE user_configs SET
          mfa_type = NULL,
          updated_at = $2
          WHERE user_id = $1 AND mfa_type = 'text' AND NOT EXISTS(
            SELECT upn.id 
            FROM user_phone_numbers upn 
            JOIN users u ON u.id = upn.user_id
            WHERE u.user_id = $1 AND upn.verified = 1
          )
          RETURNING *;"#,
        )
        .bind(user_id)
        .bind(chrono::Utc::now().timestamp())
        .execute(&mut **transaction)
        .await?;
      }

      Ok(phone_number)
    })
  })
  .await
}

pub async fn set_user_phone_number_as_primary(
  pool: &sqlx::AnyPool,
  user_id: i64,
  phone_number_id: i64,
) -> sqlx::Result<UserPhoneNumberRow> {
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let now = chrono::Utc::now().timestamp();
      let phone_number: UserPhoneNumberRow = sqlx::query_as(
        r#"UPDATE user_phone_numbers SET 
        "primary" = 1,
        "updated_at" = $3 
        WHERE "verified" = 1 AND user_id=$1 AND id = $2 
        RETURNING *;"#,
      )
      .bind(user_id)
      .bind(phone_number_id)
      .bind(now)
      .fetch_one(&mut **transaction)
      .await?;

      sqlx::query(
        r#"UPDATE user_phone_numbers SET
        "primary" = 0,
        "updated_at" = $3 
        WHERE user_id=$1 AND id != $2;"#,
      )
      .bind(user_id)
      .bind(phone_number_id)
      .bind(now)
      .execute(&mut **transaction)
      .await?;

      Ok(phone_number)
    })
  })
  .await
}

pub async fn delete_user_phone_number(
  pool: &sqlx::AnyPool,
  user_id: i64,
  phone_number_id: i64,
) -> sqlx::Result<Option<UserPhoneNumberRow>> {
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let phone_number: Option<UserPhoneNumberRow> = sqlx::query_as(
        r#"DELETE FROM user_phone_numbers
        WHERE user_id = $1 AND id = $2
        RETURNING *;"#,
      )
      .bind(user_id)
      .bind(phone_number_id)
      .fetch_optional(&mut **transaction)
      .await?;

      if let Some(phone_number) = phone_number.as_ref() {
        if phone_number.is_primary() && phone_number.is_verified() {
          sqlx::query(
            r#"UPDATE user_configs SET
            mfa_type = NULL,
            updated_at = $2
            WHERE user_id = $1 AND mfa_type = 'text'
            RETURNING *;"#,
          )
          .bind(user_id)
          .bind(chrono::Utc::now().timestamp())
          .execute(&mut **transaction)
          .await?;
        }
      }

      Ok(phone_number)
    })
  })
  .await
}
