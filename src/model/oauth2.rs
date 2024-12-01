use serde::{Deserialize, Serialize};
use std::{io, str::FromStr};
use utoipa::IntoParams;

use crate::{
  core::config::{OAuth2Config, get_config},
  middleware::{claims::tenent_encoding_key, openid_claims::OpenIdProfile},
  repository::tenent::TenentRow,
};

#[derive(Deserialize, IntoParams)]
pub struct OAuth2Query {
  pub register: Option<bool>,
}

#[derive(Deserialize, IntoParams)]
pub struct OAuth2CallbackQuery {
  pub state: String,
  pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct OAuth2State {
  pub exp: i64,
  pub tenent_id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_id: Option<i64>,
  pub register: bool,
}

impl OAuth2State {
  pub fn new(tenent_id: i64, register: bool, user_id: Option<i64>) -> Self {
    Self {
      exp: chrono::Utc::now().timestamp() + (get_config().oauth2.code_timeout_in_seconds as i64),
      tenent_id,
      register: register,
      user_id: user_id,
    }
  }

  fn encode(&self, tenent: &TenentRow) -> Result<String, jsonwebtoken::errors::Error> {
    let algorithm = jsonwebtoken::Algorithm::from_str(&tenent.algorithm)?;

    let mut header = jsonwebtoken::Header::new(algorithm);
    header.kid = Some(tenent.id.to_string());

    let key = tenent_encoding_key(tenent, algorithm)?;

    jsonwebtoken::encode(&header, self, &key)
  }
}

pub fn oauth2_create_basic_client(
  oauth2_config: &OAuth2Config,
  provider: &str,
) -> Result<oauth2::basic::BasicClient, oauth2::url::ParseError> {
  let client = oauth2::basic::BasicClient::new(
    oauth2::ClientId::new(oauth2_config.client_id.clone()),
    Some(oauth2::ClientSecret::new(
      oauth2_config.client_secret.clone(),
    )),
    oauth2::AuthUrl::new(oauth2_config.auth_url.clone())?,
    Some(oauth2::TokenUrl::new(oauth2_config.token_url.clone())?),
  )
  .set_redirect_uri(oauth2::RedirectUrl::new(
    oauth2_config
      .redirect_url
      .clone()
      .unwrap_or_else(|| format!("{}/oauth2/{provider}/callback", &get_config().server.url)),
  )?);
  Ok(client)
}

pub fn oauth2_authorize_url(
  oauth2_config: &OAuth2Config,
  tenent: &TenentRow,
  provider: &str,
  register: bool,
  user_id: Option<i64>,
) -> Result<(oauth2::url::Url, String, oauth2::PkceCodeVerifier), io::Error> {
  let client = match oauth2_create_basic_client(oauth2_config, provider) {
    Ok(client) => client,
    Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
  };

  let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

  let oauth2_state = OAuth2State::new(tenent.id, register, user_id);
  let oauth2_state_token = match oauth2_state.encode(tenent) {
    Ok(t) => t,
    Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
  };

  let csrf_token = oauth2_state_token.clone();
  let (url, _) = client
    .authorize_url(move || oauth2::CsrfToken::new(csrf_token))
    .add_scopes(
      oauth2_config
        .scopes
        .clone()
        .into_iter()
        .map(oauth2::Scope::new),
    )
    .set_pkce_challenge(pkce_code_challenge)
    .url();

  Ok((url, oauth2_state_token, pkce_code_verifier))
}

async fn oauth2_google_profile(
  google_profile_url: &str,
  access_token: &str,
) -> Result<OpenIdProfile, reqwest::Error> {
  reqwest::Client::new()
    .get(google_profile_url)
    .bearer_auth(access_token)
    .send()
    .await?
    .json::<OpenIdProfile>()
    .await
}

pub async fn oauth2_profile<TR, TT>(
  provider: &str,
  token_response: TR,
) -> Result<OpenIdProfile, io::Error>
where
  TR: oauth2::TokenResponse<TT>,
  TT: oauth2::TokenType,
{
  match provider {
    "google" => {
      oauth2_google_profile(
        "https://www.googleapis.com/oauth2/v3/userinfo",
        token_response.access_token().secret(),
      )
      .await
    }
    _ => {
      return Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Unknown provider",
      ));
    }
  }
  .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
