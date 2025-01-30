use crate::core::database::run_transaction;

#[derive(sqlx::FromRow)]
pub struct UserMFATypeRow {
  pub user_id: i64,
  pub r#type: String,
}

pub(crate) async fn get_user_mfa_types_by_user_id_internal(
  transaction: &mut sqlx::Transaction<'_, sqlx::Any>,
  user_id: i64,
) -> sqlx::Result<Vec<UserMFATypeRow>> {
  sqlx::query_as(
    r#"SELECT ut.user_id, 'totp' as type 
      FROM user_totps ut 
      JOIN users u ON u.id = ut.user_id 
      WHERE u.id = $1
      UNION
      SELECT ue.user_id, 'email' as type 
      FROM user_emails ue 
      JOIN users u ON u.id = ue.user_id 
      WHERE u.id = $1 AND ue."verified" = 1
      UNION
      SELECT upn.user_id, 'text' as type 
      FROM user_phone_numbers upn 
      JOIN users u ON u.id = upn.user_id 
      WHERE u.id = $1 AND upn."verified" = 1;"#,
  )
  .bind(user_id)
  .fetch_all(&mut **transaction)
  .await
}

pub async fn get_user_mfa_types_by_user_id(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Vec<UserMFATypeRow>> {
  run_transaction(pool, |transaction| {
    Box::pin(async move { get_user_mfa_types_by_user_id_internal(transaction, user_id).await })
  })
  .await
}

pub async fn get_users_mfa_types(
  pool: &sqlx::AnyPool,
  limit: usize,
  offset: usize,
) -> sqlx::Result<Vec<UserMFATypeRow>> {
  sqlx::query_as(
    r#"SELECT ut.user_id, 'totp' as type 
    FROM user_totps ut 
    JOIN users u ON u.id = ut.user_id 
    WHERE ut.user_id in (SELECT u.id FROM users u LIMIT $1 OFFSET $2)
    UNION
    SELECT ue.user_id, 'email' as type 
    FROM user_emails ue 
    JOIN users u ON u.id = ue.user_id 
    WHERE ue."verified" = 1 AND ue.user_id in (SELECT u.id FROM users u LIMIT $1 OFFSET $2)UNION
    SELECT upn.user_id, 'text' as type 
    FROM user_phone_numbers upn
    JOIN users u ON u.id = upn.user_id 
    WHERE upn."verified" = 1 AND upn.user_id in (SELECT u.id FROM users u LIMIT $1 OFFSET $2);"#,
  )
  .bind(limit as i64)
  .bind(offset as i64)
  .fetch_all(pool)
  .await
}
