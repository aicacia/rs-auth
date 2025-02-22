use super::user::{from_users_query, UserRow};

#[derive(sqlx::FromRow)]
pub struct UserOAuth2ProviderRow {
  pub id: i64,
  pub user_id: i64,
  pub tenant_oauth2_provider_id: i64,
  pub provider: String,
  pub email: String,
  pub updated_at: i64,
  pub created_at: i64,
}

pub async fn get_users_oauth2_providers(
  pool: &sqlx::AnyPool,
  application_id: i64,
  limit: Option<usize>,
  offset: Option<usize>,
) -> sqlx::Result<Vec<UserOAuth2ProviderRow>> {
  let mut qb = sqlx::QueryBuilder::new(
    r#"SELECT uop.*, toap.provider
    FROM user_oauth2_providers uop 
    JOIN tenant_oauth2_providers toap ON toap.id = uop.tenant_oauth2_provider_id 
    WHERE toap.active = 1 AND uop.user_id IN (SELECT u.id"#,
  );
  from_users_query(&mut qb, application_id, limit, offset);
  qb.push(")");
  qb.build_query_as().fetch_all(pool).await
}

pub async fn get_user_by_oauth2_provider_and_email(
  pool: &sqlx::AnyPool,
  tenant_oauth2_provider_id: i64,
  email: &str,
) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    JOIN user_oauth2_providers uop ON u.id = uop.user_id
    WHERE uop.tenant_oauth2_provider_id = $1 AND uop.email = $2;"#,
  )
  .bind(tenant_oauth2_provider_id)
  .bind(email)
  .fetch_optional(pool)
  .await
}

pub async fn get_user_oauth2_providers_by_user_id(
  pool: &sqlx::AnyPool,
  application_id: i64,
  user_id: i64,
) -> sqlx::Result<Vec<UserOAuth2ProviderRow>> {
  sqlx::query_as(
    r#"SELECT uop.*, toap.provider
    FROM user_oauth2_providers uop
    JOIN tenant_oauth2_providers toap ON toap.id = uop.tenant_oauth2_provider_id 
    JOIN users u ON u.id = uop.user_id
    WHERE u.application_id = $1 AND toap.active = 1 AND uop.user_id = $2;"#,
  )
  .bind(application_id)
  .bind(user_id)
  .fetch_all(pool)
  .await
}

pub async fn create_user_oauth2_provider_and_email(
  pool: &sqlx::AnyPool,
  user_id: i64,
  tenant_oauth2_provider_id: i64,
  email: &str,
  provider: &str,
) -> sqlx::Result<UserOAuth2ProviderRow> {
  sqlx::query_as(
    r#"INSERT INTO user_oauth2_providers 
          ("user_id", "tenant_oauth2_provider_id", "email")
          VALUES
          ($1, $2, $3)
          RETURNING *, $4 as provider;"#,
  )
  .bind(user_id)
  .bind(tenant_oauth2_provider_id)
  .bind(email)
  .bind(provider)
  .fetch_one(pool)
  .await
}
