use serde::{Deserialize, Serialize};
use std::{io, str::FromStr};
use utoipa::IntoParams;

use crate::{
  core::config::Config,
  middleware::{
    authorization::ApplicationIdTenantId, claims::tenant_encoding_key, openid_claims::OpenIdProfile,
  },
  repository::{tenant::TenantRow, tenant_oauth2_provider::TenantOAuth2ProviderRow},
};

#[derive(Deserialize, IntoParams)]
pub struct OAuth2Query {
  pub register: Option<bool>,
  pub state: Option<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct OAuth2CallbackQuery {
  pub state: String,
  pub code: String,
  pub scope: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OAuth2State {
  pub exp: i64,
  pub application_id: i64,
  pub tenant_id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_id: Option<i64>,
  pub register: bool,
  pub custom_state: Option<String>,
}

impl OAuth2State {
  pub fn new(
    config: &Config,
    application_id: i64,
    tenant_id: i64,
    register: bool,
    custom_state: Option<String>,
    user_id: Option<i64>,
  ) -> Self {
    Self {
      exp: chrono::Utc::now().timestamp() + (config.oauth2.code_timeout_in_seconds as i64),
      application_id,
      tenant_id,
      register,
      user_id,
      custom_state,
    }
  }

  fn encode(&self, tenant: &TenantRow) -> Result<String, jsonwebtoken::errors::Error> {
    let algorithm = jsonwebtoken::Algorithm::from_str(&tenant.algorithm)?;

    let mut header = jsonwebtoken::Header::new(algorithm);
    header.kid = Some(ApplicationIdTenantId::new_kid(
      tenant.application_id,
      tenant.id,
    ));

    let key = tenant_encoding_key(tenant, algorithm)?;

    jsonwebtoken::encode(&header, self, &key)
  }
}

pub fn oauth2_authorize_url<I>(
  config: &Config,
  client: &oauth2::basic::BasicClient,
  tenant: &TenantRow,
  register: bool,
  custom_state: Option<String>,
  user_id: Option<i64>,
  scopes: I,
) -> Result<
  (
    oauth2::url::Url,
    oauth2::CsrfToken,
    oauth2::PkceCodeVerifier,
  ),
  io::Error,
>
where
  I: IntoIterator<Item = oauth2::Scope>,
{
  let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

  let oauth2_state = OAuth2State::new(
    config,
    tenant.application_id,
    tenant.id,
    register,
    custom_state,
    user_id,
  );
  let oauth2_state_token = match oauth2_state.encode(tenant) {
    Ok(t) => t,
    Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidData, err)),
  };

  let oauth2_state_token_csrf_token = oauth2_state_token.clone();
  let (url, csrf_token) = client
    .authorize_url(move || oauth2::CsrfToken::new(oauth2_state_token_csrf_token))
    .add_scopes(scopes)
    .set_pkce_challenge(pkce_code_challenge)
    .url();

  Ok((url, csrf_token, pkce_code_verifier))
}

pub async fn oauth2_profile<TR, TT>(
  tenant_oauth2_provider: &TenantOAuth2ProviderRow,
  token_response: TR,
) -> Result<OpenIdProfile, io::Error>
where
  TR: oauth2::TokenResponse<TT>,
  TT: oauth2::TokenType,
{
  match tenant_oauth2_provider.provider.as_str() {
    "google" => oauth2_google_profile(token_response.access_token().secret()).await,
    "facebook" => oauth2_facebook_profile(token_response.access_token().secret()).await,
    _ => {
      return Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Unknown provider",
      ));
    }
  }
  .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

async fn oauth2_google_profile(access_token: &str) -> Result<OpenIdProfile, reqwest::Error> {
  reqwest::Client::new()
    .get("https://www.googleapis.com/oauth2/v3/userinfo")
    .bearer_auth(access_token)
    .send()
    .await?
    .json::<OpenIdProfile>()
    .await
}

async fn oauth2_facebook_profile(access_token: &str) -> Result<OpenIdProfile, reqwest::Error> {
  reqwest::Client::new()
    .get(format!(
      "https://graph.facebook.com/me?fields=email&access_token={access_token}"
    ))
    .send()
    .await?
    .json::<OpenIdProfile>()
    .await
}
