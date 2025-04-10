use crate::core::config::Config;

use super::tenant::from_tenants_query;

pub type TenantOAuth2Client = oauth2::Client<
  oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
  oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
  oauth2::StandardTokenIntrospectionResponse<
    oauth2::EmptyExtraTokenFields,
    oauth2::basic::BasicTokenType,
  >,
  oauth2::StandardRevocableToken,
  oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
  oauth2::EndpointSet,
  oauth2::EndpointNotSet,
  oauth2::EndpointNotSet,
  oauth2::EndpointNotSet,
  oauth2::EndpointSet,
>;

#[derive(sqlx::FromRow)]
pub struct TenantOAuth2ProviderRow {
  pub id: i64,
  pub tenant_id: i64,
  pub provider: String,
  pub active: i64,
  pub client_id: String,
  pub client_secret: String,
  pub auth_url: String,
  pub token_url: String,
  pub callback_url: Option<String>,
  pub redirect_url: String,
  pub scope: String,
  pub updated_at: i64,
  pub created_at: i64,
}

impl TenantOAuth2ProviderRow {
  pub fn is_active(&self) -> bool {
    self.active != 0
  }

  pub fn callback_url(&self, config: &Config) -> String {
    self
      .callback_url
      .clone()
      .unwrap_or_else(|| format!("{}/oauth2/{}/callback", &config.server.url, &self.provider))
  }

  pub fn basic_client(
    &self,
    config: &Config,
  ) -> Result<TenantOAuth2Client, oauth2::url::ParseError> {
    let client = oauth2::basic::BasicClient::new(oauth2::ClientId::new(self.client_id.to_owned()))
      .set_client_secret(oauth2::ClientSecret::new(self.client_secret.to_owned()))
      .set_auth_uri(oauth2::AuthUrl::new(self.auth_url.to_owned())?)
      .set_token_uri(oauth2::TokenUrl::new(self.token_url.to_owned())?)
      .set_redirect_uri(oauth2::RedirectUrl::new(self.callback_url(config))?);
    Ok(client)
  }
}

pub async fn get_active_tenant_oauth2_provider(
  pool: &sqlx::AnyPool,
  application_id: i64,
  tenant_id: i64,
  provider: &str,
) -> sqlx::Result<Option<TenantOAuth2ProviderRow>> {
  sqlx::query_as(
    r#"SELECT toap.* 
    FROM tenant_oauth2_providers toap
    JOIN tenants t ON t.id = toap.tenant_id 
    WHERE t.application_id = $1 AND toap.active = 1 AND toap.tenant_id = $2 AND toap.provider = $3 
    LIMIT 1;"#,
  )
  .bind(application_id)
  .bind(tenant_id)
  .bind(provider)
  .fetch_optional(pool)
  .await
}

pub async fn get_tenant_oauth2_providers(
  pool: &sqlx::AnyPool,
  tenant_id: i64,
) -> sqlx::Result<Vec<TenantOAuth2ProviderRow>> {
  sqlx::query_as(
    r#"SELECT toap.* 
      FROM tenant_oauth2_providers toap 
      WHERE toap.tenant_id = $1;"#,
  )
  .bind(tenant_id)
  .fetch_all(pool)
  .await
}

pub async fn get_tenants_oauth2_providers(
  pool: &sqlx::AnyPool,
  application_id: i64,
  limit: Option<usize>,
  offset: Option<usize>,
) -> sqlx::Result<Vec<TenantOAuth2ProviderRow>> {
  let mut qb = sqlx::QueryBuilder::new(
    "SELECT toap.* FROM tenant_oauth2_providers toap WHERE toap.tenant_id IN (SELECT t.id",
  );
  from_tenants_query(&mut qb, application_id, limit, offset);
  qb.push(")");
  qb.build_query_as().fetch_all(pool).await
}

#[derive(Default)]
pub struct CreateTenantOAuth2Provider {
  pub provider: String,
  pub client_id: String,
  pub client_secret: String,
  pub active: i64,
  pub auth_url: String,
  pub token_url: String,
  pub callback_url: Option<String>,
  pub redirect_url: String,
  pub scope: String,
}

impl CreateTenantOAuth2Provider {
  pub fn new(provider: &str) -> Option<Self> {
    match provider {
      "google" => Some(Self::google()),
      "facebook" => Some(Self::facebook()),
      "github" => Some(Self::github()),
      "microsoft" => Some(Self::microsoft()),
      "x" => Some(Self::x()),
      _ => None,
    }
  }
  pub fn google() -> Self {
    Self {
      provider: "google".to_owned(),
      auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_owned(),
      token_url: "https://www.googleapis.com/oauth2/v3/token".to_owned(),
      scope: "https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/userinfo.profile".to_owned(),
      ..Default::default()
    }
  }
  pub fn facebook() -> Self {
    Self {
      provider: "facebook".to_owned(),
      auth_url: "https://www.facebook.com/v21.0/dialog/oauth".to_owned(),
      token_url: "https://graph.facebook.com/v21.0/oauth/access_token".to_owned(),
      scope: "public_profile email".to_owned(),
      ..Default::default()
    }
  }
  pub fn github() -> Self {
    Self {
      provider: "github".to_owned(),
      auth_url: "https://github.com/login/oauth/authorize".to_owned(),
      token_url: "https://github.com/login/oauth/access_token".to_owned(),
      scope: "public_repo user.email".to_owned(),
      ..Default::default()
    }
  }
  pub fn microsoft() -> Self {
    Self {
      provider: "microsoft".to_owned(),
      auth_url: "https://microsoft.com/login/oauth/authorize".to_owned(),
      token_url: "https://microsoft.com/login/oauth/access_token".to_owned(),
      scope: "public_profile email".to_owned(),
      ..Default::default()
    }
  }
  pub fn x() -> Self {
    Self {
      provider: "x".to_owned(),
      auth_url: "https://x.com/login/oauth/authorize".to_owned(),
      token_url: "https://x.com/login/oauth/access_token".to_owned(),
      scope: "public_profile email".to_owned(),
      ..Default::default()
    }
  }
}

pub async fn create_tenant_oauth2_provider(
  pool: &sqlx::AnyPool,
  tenant_id: i64,
  params: CreateTenantOAuth2Provider,
) -> sqlx::Result<TenantOAuth2ProviderRow> {
  sqlx::query_as(
    r#"INSERT INTO tenant_oauth2_providers 
      (tenant_id, provider, client_id, client_secret, active, auth_url, token_url, callback_url, redirect_url, scope) 
      VALUES 
      ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
      RETURNING *;"#,
  )
  .bind(tenant_id)
  .bind(params.provider)
  .bind(params.client_id)
  .bind(params.client_secret)
  .bind(params.active)
  .bind(params.auth_url)
  .bind(params.token_url)
  .bind(params.callback_url)
  .bind(params.redirect_url)
  .bind(params.scope)
  .fetch_one(pool)
  .await
}

#[derive(Default)]
pub struct UpdateTenantOAuth2Provider {
  pub client_id: Option<String>,
  pub client_secret: Option<String>,
  pub active: Option<i64>,
  pub auth_url: Option<String>,
  pub token_url: Option<String>,
  pub redirect_url: Option<String>,
  pub callback_url: Option<String>,
  pub scope: Option<String>,
}

pub async fn update_tenant_oauth2_provider(
  pool: &sqlx::AnyPool,
  tenant_id: i64,
  tenant_oauht2_provider_id: i64,
  params: UpdateTenantOAuth2Provider,
) -> sqlx::Result<Option<TenantOAuth2ProviderRow>> {
  sqlx::query_as(
    r#"UPDATE tenant_oauth2_providers SET
      client_id = COALESCE($3, client_id),
      client_secret = COALESCE($4, client_secret),
      active = COALESCE($5, active),
      auth_url = COALESCE($6, auth_url),
      token_url = COALESCE($7, token_url),
      callback_url = COALESCE($8, callback_url),
      redirect_url = COALESCE($9, redirect_url),
      scope = COALESCE($10, scope),
      updated_at = $11
    WHERE tenant_id = $1 AND id = $2
    RETURNING *;"#,
  )
  .bind(tenant_id)
  .bind(tenant_oauht2_provider_id)
  .bind(params.client_id)
  .bind(params.client_secret)
  .bind(params.active)
  .bind(params.auth_url)
  .bind(params.token_url)
  .bind(params.callback_url)
  .bind(params.redirect_url)
  .bind(params.scope)
  .bind(chrono::Utc::now().timestamp())
  .fetch_optional(pool)
  .await
}

pub async fn delete_tenant_oauth2_provider(
  pool: &sqlx::AnyPool,
  tenant_id: i64,
  id: i64,
) -> sqlx::Result<Option<TenantOAuth2ProviderRow>> {
  sqlx::query_as(
    r#"DELETE FROM tenant_oauth2_providers WHERE tenant_id = $1 AND id = $2 RETURNING *;"#,
  )
  .bind(tenant_id)
  .bind(id)
  .fetch_optional(pool)
  .await
}
