use anyhow::Result;
use base64::engine::{general_purpose::STANDARD_NO_PAD, Engine};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rand::{distributions::Alphanumeric, Rng};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::encryption::generate_salt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
  pub exp: usize,
  pub iat: usize,
  pub iss: String,
  pub nonce: String,
  pub sub: i32,
  pub app: i32,
}

impl Claims {
  pub fn new(
    app: i32,
    sub: i32,
    now_in_seconds: usize,
    expires_in_seconds: usize,
    iss: &str,
  ) -> Self {
    Self {
      exp: now_in_seconds + expires_in_seconds,
      iat: now_in_seconds,
      iss: iss.to_owned(),
      nonce: rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect(),
      sub,
      app,
    }
  }
}

pub fn parse_jwt<T>(jwt: &str, secret: &str) -> Result<TokenData<T>>
where
  T: DeserializeOwned,
{
  let token_data = decode(
    jwt,
    &DecodingKey::from_secret(secret.as_bytes()),
    &Validation::default(),
  )?;
  Ok(token_data)
}

pub fn parse_jwt_no_validation<T>(jwt: &str) -> Result<T>
where
  T: DeserializeOwned,
{
  let mut parts = jwt.rsplitn(3, '.');
  match (parts.next(), parts.next(), parts.next()) {
    (Some(_header), Some(payload), Some(_signature)) => {
      let json = STANDARD_NO_PAD.decode(payload)?;
      let claims = serde_json::from_slice(&json)?;
      Ok(claims)
    }
    _ => {
      Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken).into())
    }
  }
}

pub fn encode_jwt<T>(claims: &T, secret: &str) -> Result<String>
where
  T: Serialize,
{
  let jwt = encode(
    &Header::default(),
    claims,
    &EncodingKey::from_secret(secret.as_bytes()),
  )?;
  Ok(jwt)
}

pub fn gen_jwt_secret() -> String {
  STANDARD_NO_PAD.encode(generate_salt(&mut [0u8; 256]))
}
