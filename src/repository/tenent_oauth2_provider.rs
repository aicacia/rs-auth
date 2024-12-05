#[derive(sqlx::FromRow)]
pub struct TenentOAuth2ProviderRow {
  pub id: i64,
  pub tenent_id: i64,
  pub provider: String,
  pub active: i64,
  pub client_id: String,
  pub client_secret: String,
  pub auth_url: String,
  pub token_url: String,
  pub scope: Option<String>,
  pub redirect_uri: Option<String>,
  pub created_at: i64,
  pub updated_at: i64,
}

impl TenentOAuth2ProviderRow {
  pub fn is_active(&self) -> bool {
    self.active != 0
  }
}

pub async fn get_tenent_oauth2_provider(
  pool: &sqlx::AnyPool,
  tenent_id: i64,
  provider: &str,
) -> sqlx::Result<Option<TenentOAuth2ProviderRow>> {
  sqlx::query_as(r#"SELECT toap.* FROM tenent_oauth2_providers toap WHERE toap.tenent_id = $1 AND toap.provider = $2 LIMIT 1;"#)
    .bind(tenent_id)
    .bind(provider)
    .fetch_optional(pool)
    .await
}

pub async fn get_tenent_oauth2_providers(
  pool: &sqlx::AnyPool,
  tenent_id: i64,
) -> sqlx::Result<Vec<TenentOAuth2ProviderRow>> {
  sqlx::query_as(r#"SELECT toap.* FROM tenent_oauth2_providers toap WHERE toap.tenent_id = $1;"#)
    .bind(tenent_id)
    .fetch_all(pool)
    .await
}
