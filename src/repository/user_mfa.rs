#[derive(sqlx::FromRow)]
pub struct UserMFATypeRow {
  pub user_id: i64,
  pub kind: String,
}

pub async fn get_user_mfa_types_by_user_id(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Vec<UserMFATypeRow>> {
  sqlx::query_as(
    r#"SELECT ut.user_id, 'totp' as kind 
      FROM user_totps ut 
      JOIN users u ON u.id = ut.user_id 
      WHERE u.id = $1
      LIMIT 1;"#,
  )
  .bind(user_id)
  .fetch_all(pool)
  .await
}

pub async fn get_users_mfa_types(
  pool: &sqlx::AnyPool,
  limit: usize,
  offset: usize,
) -> sqlx::Result<Vec<UserMFATypeRow>> {
  sqlx::query_as(
    r#"SELECT ut.user_id, 'totp' as kind 
    FROM user_totps ut 
    JOIN users u ON u.id = ut.user_id 
    WHERE ut.user_id in (SELECT u.id FROM users u LIMIT $1 OFFSET $2);"#,
  )
  .bind(limit as i64)
  .bind(offset as i64)
  .fetch_all(pool)
  .await
}
