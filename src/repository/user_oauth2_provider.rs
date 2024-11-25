use super::user::UserRow;

#[derive(sqlx::FromRow)]
pub struct UserOAuth2ProviderRow {
  pub id: i64,
  pub user_id: i64,
  pub provider: String,
  pub email: String,
  pub created_at: i64,
  pub updated_at: i64,
}

pub async fn get_user_by_oauth2_provider_and_email(
  pool: &sqlx::AnyPool,
  provider: &str,
  email: &str,
) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    JOIN user_oauth2_providers uop ON u.id = uop.user_id
    WHERE uop.provider = $1 AND uop.email = $2;"#,
  )
  .bind(provider)
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
