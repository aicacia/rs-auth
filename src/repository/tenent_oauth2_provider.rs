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
  pub callback_url: Option<String>,
  pub redirect_url: String,
  pub scope: String,
  pub updated_at: i64,
  pub created_at: i64,
}

impl TenentOAuth2ProviderRow {
  pub fn is_active(&self) -> bool {
    self.active != 0
  }

  pub fn callback_url(&self) -> String {
    self.callback_url.clone().unwrap_or_else(|| {
      format!(
        "{}/oauth2/{}/callback",
        &get_config().server.url,
        &self.provider
      )
    })
  }

  pub fn basic_client(&self) -> Result<oauth2::basic::BasicClient, oauth2::url::ParseError> {
    let client = oauth2::basic::BasicClient::new(
      oauth2::ClientId::new(self.client_id.to_owned()),
      Some(oauth2::ClientSecret::new(self.client_secret.to_owned())),
      oauth2::AuthUrl::new(self.auth_url.to_owned())?,
      Some(oauth2::TokenUrl::new(self.token_url.to_owned())?),
    )
    .set_redirect_uri(oauth2::RedirectUrl::new(self.callback_url())?);
    Ok(client)
  }
}

pub async fn get_active_tenent_oauth2_provider(
  pool: &sqlx::AnyPool,
  tenent_id: i64,
  provider: &str,
) -> sqlx::Result<Option<TenentOAuth2ProviderRow>> {
  sqlx::query_as(
    r#"SELECT toap.* 
    FROM tenent_oauth2_providers toap 
    WHERE toap.active = 1 AND toap.tenent_id = $1 AND toap.provider = $2 
    LIMIT 1;"#,
  )
  .bind(tenent_id)
  .bind(provider)
  .fetch_optional(pool)
  .await
}

pub async fn get_tenent_oauth2_providers(
  pool: &sqlx::AnyPool,
  tenent_id: i64,
) -> sqlx::Result<Vec<TenentOAuth2ProviderRow>> {
  sqlx::query_as(
    r#"SELECT toap.* 
      FROM tenent_oauth2_providers toap 
      WHERE toap.tenent_id = $1;"#,
  )
  .bind(tenent_id)
  .fetch_all(pool)
  .await
}

pub async fn get_tenents_oauth2_providers(
  pool: &sqlx::AnyPool,
  limit: usize,
  offset: usize,
) -> sqlx::Result<Vec<TenentOAuth2ProviderRow>> {
  sqlx::query_as(
    r#"SELECT toap.* 
    FROM tenent_oauth2_providers toap 
    WHERE toap.tenent_id IN (SELECT t.id FROM tenents t LIMIT $1 OFFSET $2);"#,
  )
  .bind(limit as i64)
  .bind(offset as i64)
  .fetch_all(pool)
  .await
}

#[derive(Default)]
pub struct CreateTenentOAuth2Provider {
  pub provider: String,
  pub client_id: String,
  pub client_secret: String,
  pub auth_url: String,
  pub token_url: String,
  pub callback_url: Option<String>,
  pub redirect_url: String,
  pub scope: String,
}

impl CreateTenentOAuth2Provider {
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

pub async fn create_tenent_oauth2_provider(
  pool: &sqlx::AnyPool,
  tenent_id: i64,
  params: CreateTenentOAuth2Provider,
) -> sqlx::Result<TenentOAuth2ProviderRow> {
  sqlx::query_as(
    r#"INSERT INTO tenent_oauth2_providers 
      (tenent_id, provider, client_id, client_secret, auth_url, token_url, callback_url, redirect_url, scope) 
      VALUES 
      ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
      RETURNING *;"#,
  )
  .bind(tenent_id)
  .bind(params.provider)
  .bind(params.client_id)
  .bind(params.client_secret)
  .bind(params.auth_url)
  .bind(params.token_url)
  .bind(params.callback_url)
  .bind(params.redirect_url)
  .bind(params.scope)
  .fetch_one(pool)
  .await
}

#[derive(Default)]
pub struct UpdateTenentOAuth2Provider {
  pub client_id: Option<String>,
  pub client_secret: Option<String>,
  pub active: Option<i64>,
  pub auth_url: Option<String>,
  pub token_url: Option<String>,
  pub redirect_url: Option<String>,
  pub callback_url: Option<String>,
  pub scope: Option<String>,
}

pub async fn update_tenent_oauth2_provider(
  pool: &sqlx::AnyPool,
  tenent_id: i64,
  tenent_oauht2_provider_id: i64,
  params: UpdateTenentOAuth2Provider,
) -> sqlx::Result<Option<TenentOAuth2ProviderRow>> {
  sqlx::query_as(
    r#"UPDATE tenent_oauth2_providers SET
      client_id = COALESCE($3, client_id),
      client_secret = COALESCE($4, client_secret),
      active = COALESCE($5, active),
      auth_url = COALESCE($6, auth_url),
      token_url = COALESCE($7, token_url),
      callback_url = COALESCE($8, callback_url),
      redirect_url = COALESCE($9, redirect_url),
      scope = COALESCE($10, scope),
      updated_at = $11
    WHERE tenent_id = $1 AND id = $2
    RETURNING *;"#,
  )
  .bind(tenent_id)
  .bind(tenent_oauht2_provider_id)
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

pub async fn delete_tenent_oauth2_provider(
  pool: &sqlx::AnyPool,
  tenent_id: i64,
  id: i64,
) -> sqlx::Result<Option<TenentOAuth2ProviderRow>> {
  sqlx::query_as(
    r#"DELETE FROM tenent_oauth2_providers WHERE tenent_id = $1 AND id = $2 RETURNING *;"#,
  )
  .bind(tenent_id)
  .bind(id)
  .fetch_optional(pool)
  .await
}
