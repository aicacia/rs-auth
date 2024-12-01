use super::user::UserRow;

#[derive(sqlx::FromRow)]
pub struct UserEmailRow {
  pub id: i64,
  pub user_id: i64,
  pub primary: i32,
  pub verified: i32,
  pub email: String,
  pub created_at: i64,
  pub updated_at: i64,
}

impl UserEmailRow {
  pub fn is_verified(&self) -> bool {
    self.verified != 0
  }
  pub fn is_primary(&self) -> bool {
    self.primary != 0
  }
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
pub struct UpdateUserEmail {
  pub primary: Option<bool>,
  pub verified: Option<bool>,
}

pub async fn update_user_email(
  pool: &sqlx::AnyPool,
  user_id: i64,
  params: UpdateUserEmail,
) -> sqlx::Result<Option<UserEmailRow>> {
  sqlx::query_as(
    r#"UPDATE user_emails SET 
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

pub async fn delete_user_email(pool: &sqlx::AnyPool, user_id: i64) -> sqlx::Result<u64> {
  sqlx::query(
    r#"DELETE FROM user_emails
    WHERE user_id = $1;"#,
  )
  .bind(user_id)
  .execute(pool)
  .await
  .map(|r| r.rows_affected())
}
