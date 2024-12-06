use super::user::UserRow;

#[derive(sqlx::FromRow)]
pub struct UserOAuth2ProviderRow {
  pub id: i64,
  pub user_id: i64,
  pub tenent_oauth2_provider_id: i64,
  pub email: String,
  pub updated_at: i64,
  pub created_at: i64,
}

pub async fn get_users_oauth2_providers(
  pool: &sqlx::AnyPool,
  limit: usize,
  offset: usize,
) -> sqlx::Result<Vec<UserOAuth2ProviderRow>> {
  sqlx::query_as(r#"SELECT uop.* FROM user_oauth2_providers uop WHERE uop.user_id in (SELECT u.id FROM users u LIMIT $1 OFFSET $2);"#)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
}

pub async fn get_user_by_oauth2_provider_and_email(
  pool: &sqlx::AnyPool,
  tenent_oauth2_provider_id: i64,
  email: &str,
) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    JOIN user_oauth2_providers uop ON u.id = uop.user_id
    WHERE uop.tenent_oauth2_provider_id = $1 AND uop.email = $2;"#,
  )
  .bind(tenent_oauth2_provider_id)
  .bind(email)
  .fetch_optional(pool)
  .await
}

pub async fn get_user_oauth2_providers_by_user_id(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Vec<UserOAuth2ProviderRow>> {
  sqlx::query_as(
    r#"SELECT uop.*
    FROM user_oauth2_providers uop
    WHERE uop.user_id = $1;"#,
  )
  .bind(user_id)
  .fetch_all(pool)
  .await
}

pub async fn create_user_oauth2_provider_and_email(
  pool: &sqlx::AnyPool,
  user_id: i64,
  tenent_oauth2_provider_id: i64,
  email: &str,
) -> sqlx::Result<UserOAuth2ProviderRow> {
  sqlx::query_as(
    r#"INSERT INTO user_oauth2_providers 
          ("user_id", "tenent_oauth2_provider_id", "email")
          VALUES
          ($1, $2, $3)
          RETURNING *;"#,
  )
  .bind(user_id)
  .bind(tenent_oauth2_provider_id)
  .bind(email)
  .fetch_one(pool)
  .await
}
