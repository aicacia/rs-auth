use crate::core::database::run_transaction;

use super::user::from_users_query;

#[derive(sqlx::FromRow)]
pub struct UserMFATypeRow {
  pub user_id: i64,
  pub r#type: String,
}

pub(crate) async fn get_user_mfa_types_by_user_id_internal(
  transaction: &mut sqlx::Transaction<'_, sqlx::Any>,
  application_id: i64,
  user_id: i64,
) -> sqlx::Result<Vec<UserMFATypeRow>> {
  sqlx::query_as(
    r#"SELECT ut.user_id, 'totp' as type 
      FROM user_totps ut 
      JOIN users u ON u.id = ut.user_id 
      WHERE u.application_id = $1 AND u.id = $2
      UNION
      SELECT ue.user_id, 'email' as type 
      FROM user_emails ue 
      JOIN users u ON u.id = ue.user_id 
      WHERE u.application_id = $1 AND u.id = $2 AND ue."verified" = 1
      UNION
      SELECT upn.user_id, 'text' as type 
      FROM user_phone_numbers upn 
      JOIN users u ON u.id = upn.user_id 
      WHERE u.application_id = $1 AND u.id = $2 AND upn."verified" = 1;"#,
  )
  .bind(application_id)
  .bind(user_id)
  .fetch_all(&mut **transaction)
  .await
}

pub async fn get_user_mfa_types_by_user_id(
  pool: &sqlx::AnyPool,
  application_id: i64,
  user_id: i64,
) -> sqlx::Result<Vec<UserMFATypeRow>> {
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      get_user_mfa_types_by_user_id_internal(transaction, application_id, user_id).await
    })
  })
  .await
}

pub async fn get_users_mfa_types(
  pool: &sqlx::AnyPool,
  application_id: i64,
  limit: Option<usize>,
  offset: Option<usize>,
) -> sqlx::Result<Vec<UserMFATypeRow>> {
  let mut qb = sqlx::QueryBuilder::new(
    r#"SELECT ut.user_id, 'totp' as type 
    FROM user_totps ut 
    JOIN users u ON u.id = ut.user_id 
    WHERE ut.user_id IN (SELECT u.id"#,
  );
  from_users_query(&mut qb, application_id, limit, offset);
  qb.push(")");
  qb.push(" UNION ");
  qb.push(
    r#"SELECT ue.user_id, 'email' as type 
    FROM user_emails ue 
    JOIN users u ON u.id = ue.user_id 
    WHERE ue."verified" = 1 AND ue.user_id IN (SELECT u.id"#,
  );
  from_users_query(&mut qb, application_id, limit, offset);
  qb.push(")");
  qb.push(" UNION ");
  qb.push(
    r#"SELECT upn.user_id, 'text' as type 
    FROM user_phone_numbers upn
    JOIN users u ON u.id = upn.user_id 
    WHERE upn."verified" = 1 AND upn.user_id IN (SELECT u.id"#,
  );
  from_users_query(&mut qb, application_id, limit, offset);
  qb.push(")");
  qb.build_query_as().fetch_all(pool).await
}
