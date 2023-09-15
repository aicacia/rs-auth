use anyhow::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::time::{SystemTime, UNIX_EPOCH};

use super::settings::get_setting;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims<T> {
  pub exp: usize,
  pub iat: usize,
  pub iss: String,
  pub nonce: String,
  pub sub: T,
}

impl<T> Claims<T>
where
  T: Serialize + DeserializeOwned,
{
  pub async fn new(pool: &Pool<Postgres>, sub: T) -> Result<Self> {
    let expires_in_seconds = get_setting(pool, "jwt.expires_in_seconds")
      .await
      .as_u64()
      .unwrap_or(3600) as usize;
    let uri = get_setting(pool, "server.uri")
      .await
      .as_str()
      .unwrap_or_default()
      .to_owned();
    let now_in_seconds = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
    Ok(Self {
      exp: now_in_seconds + expires_in_seconds,
      iat: now_in_seconds,
      iss: uri,
      nonce: rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect(),
      sub,
    })
  }

  pub async fn new_encoded(pool: &Pool<Postgres>, sub: T) -> Result<String> {
    let secret = get_setting(pool, "jwt.secret")
      .await
      .as_str()
      .unwrap_or_default()
      .to_owned();
    let claims = Self::new(pool, sub).await?;
    let jwt = encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(jwt)
  }

  pub async fn from_encoded(pool: &Pool<Postgres>, jwt: &str) -> Result<Self> {
    let secret = get_setting(pool, "jwt.secret")
      .await
      .as_str()
      .unwrap_or_default()
      .to_owned();
    let token_data = decode::<Self>(
      jwt,
      &DecodingKey::from_secret(secret.as_bytes()),
      &Validation::default(),
    )?;
    Ok(token_data.claims)
  }
}
