#[derive(sqlx::FromRow)]
pub struct UserPhoneNumberRow {
  pub id: i64,
  pub user_id: i64,
  pub primary: i32,
  pub verified: i32,
  pub phone_number: String,
  pub created_at: i64,
  pub updated_at: i64,
}

impl UserPhoneNumberRow {
  pub fn is_verified(&self) -> bool {
    self.verified != 0
  }
  pub fn is_primary(&self) -> bool {
    self.primary != 0
  }
}

pub async fn get_user_phone_numbers_by_user_id(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Vec<UserPhoneNumberRow>> {
  sqlx::query_as(
    r#"SELECT upn.*
    FROM user_phone_numbers upn
    WHERE upn.user_id = $1;"#,
  )
  .bind(user_id)
  .fetch_all(pool)
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
  params: UpdateUserPhoneNumber,
) -> sqlx::Result<Option<UserPhoneNumberRow>> {
  sqlx::query_as(
    r#"UPDATE user_phone_numbers SET 
      primary = COALESCE($2, primary),
      verified = COALESCE($3, verified),
    WHERE user_id = $1
    RETURNING *;"#,
  )
  .bind(user_id)
  .bind(params.primary)
  .bind(params.verified)
  .fetch_optional(pool)
  .await
}

pub async fn delete_user_phone_number(pool: &sqlx::AnyPool, user_id: i64) -> sqlx::Result<u64> {
  sqlx::query(
    r#"DELETE FROM user_phone_numbers
    WHERE user_id = $1;"#,
  )
  .bind(user_id)
  .execute(pool)
  .await
  .map(|r| r.rows_affected())
}
