use std::io;

use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::core::{
  config::{get_config, OAuth2Config},
  encryption::random_bytes,
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
  pub tenent_id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub register: Option<bool>,
  pub csrf_token: String,
}

impl OAuth2State {
  pub fn new(tenent_id: i64, register: bool) -> Self {
    Self {
      tenent_id,
      register: Some(register),
      csrf_token: BASE64_STANDARD_NO_PAD.encode(random_bytes(16)),
    }
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
  provider: &str,
  tenent_id: i64,
  register: bool,
) -> Result<(oauth2::url::Url, String, oauth2::PkceCodeVerifier), io::Error> {
  let client = match oauth2_create_basic_client(oauth2_config, provider) {
    Ok(client) => client,
    Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
  };

  let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

  let state = OAuth2State::new(tenent_id, register);
  let state_string = match serde_json::to_string(&state) {
    Ok(state_string) => state_string,
    Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
  };

  let (url, _) = client
    .authorize_url(move || oauth2::CsrfToken::new(state_string))
    .add_scopes(
      oauth2_config
        .scopes
        .clone()
        .into_iter()
        .map(oauth2::Scope::new),
    )
    .set_pkce_challenge(pkce_code_challenge)
    .url();

  Ok((url, state.csrf_token, pkce_code_verifier))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenIdProfile {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub given_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub family_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub middle_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nickname: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub preferred_username: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub profile_picture: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub website: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email_verified: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gender: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub birthdate: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub zone_info: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub locale: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone_number_verified: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub address: Option<String>,
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

pub async fn oauth2_profile(
  provider: &str,
  access_token: &str,
) -> Result<OpenIdProfile, io::Error> {
  match provider {
    "google" => {
      oauth2_google_profile(
        "https://www.googleapis.com/oauth2/v3/userinfo ",
        access_token,
      )
      .await
    }
    _ => {
      return Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Unknown provider",
      ))
    }
  }
  .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
