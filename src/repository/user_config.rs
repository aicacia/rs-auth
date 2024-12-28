use crate::core::database::run_transaction;

use super::user_mfa::get_user_mfa_types_by_user_id_internal;

#[derive(Debug, sqlx::FromRow)]
pub struct UserConfigRow {
  pub user_id: i64,
  pub mfa_type: Option<String>,
  pub updated_at: i64,
  pub created_at: i64,
}

pub async fn get_users_configs(
  pool: &sqlx::AnyPool,
  limit: usize,
  offset: usize,
) -> sqlx::Result<Vec<UserConfigRow>> {
  sqlx::query_as(
    r#"SELECT ui.* 
            FROM user_configs ui 
            WHERE ui.user_id in (SELECT u.id FROM users u LIMIT $1 OFFSET $2);"#,
  )
  .bind(limit as i64)
  .bind(offset as i64)
  .fetch_all(pool)
  .await
}

pub async fn get_user_config_by_user_id(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Option<UserConfigRow>> {
  sqlx::query_as(
    r#"SELECT ui.*
    FROM user_configs ui
    WHERE ui.user_id = $1 
    LIMIT 1;"#,
  )
  .bind(user_id)
  .fetch_optional(pool)
  .await
}

#[derive(Default)]
pub struct UserConfigUpdate {
  pub mfa_type: Option<String>,
}

pub async fn update_user_config(
  pool: &sqlx::AnyPool,
  user_id: i64,
  updates: UserConfigUpdate,
) -> sqlx::Result<UserConfigRow> {
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let mfa_types = get_user_mfa_types_by_user_id_internal(transaction, user_id).await?;

      if let Some(updated_mfa_type) = updates.mfa_type.as_ref() {
        if updated_mfa_type != "none" {
          let mut found = false;
          for mfa_type in &mfa_types {
            if &mfa_type.kind == updated_mfa_type {
              found = true;
              break;
            }
          }
          if !found {
            return Err(sqlx::Error::Protocol(format!(
              "no mfa type {} exists for user",
              updated_mfa_type
            )));
          }
        }
      }

      let user_config = sqlx::query_as(
        r#"UPDATE user_configs SET
        mfa_type = COALESCE($2, mfa_type),
        updated_at = $3
        WHERE user_id = $1
        RETURNING *;"#,
      )
      .bind(user_id)
      .bind(updates.mfa_type)
      .bind(chrono::Utc::now().timestamp())
      .fetch_one(&mut **transaction)
      .await?;

      Ok(user_config)
    })
  })
  .await
}