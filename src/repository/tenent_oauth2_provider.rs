use crate::core::config::get_config;

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
  pub redirect_url: Option<String>,
  pub created_at: i64,
  pub updated_at: i64,
}

impl TenentOAuth2ProviderRow {
  pub fn is_active(&self) -> bool {
    self.active != 0
  }

  pub fn basic_client(&self) -> Result<oauth2::basic::BasicClient, oauth2::url::ParseError> {
    let client = oauth2::basic::BasicClient::new(
      oauth2::ClientId::new(self.client_id.clone()),
      Some(oauth2::ClientSecret::new(self.client_secret.clone())),
      oauth2::AuthUrl::new(self.auth_url.clone())?,
      Some(oauth2::TokenUrl::new(self.token_url.clone())?),
    )
    .set_redirect_uri(oauth2::RedirectUrl::new(
      self.redirect_url.clone().unwrap_or_else(|| {
        format!(
          "{}/oauth2/{}/callback",
          &get_config().server.url,
          &self.provider,
        )
      }),
    )?);
    Ok(client)
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
