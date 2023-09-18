use anyhow::Result;
use base64::engine::{general_purpose::STANDARD, Engine};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

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

  pub fn parse(jwt: &str, secret: &str) -> Result<TokenData<Self>> {
    let token_data = decode::<Self>(
      jwt,
      &DecodingKey::from_secret(secret.as_bytes()),
      &Validation::default(),
    )?;
    Ok(token_data)
  }

  pub fn parse_no_validation(jwt: &str) -> Result<Self> {
    let mut parts = jwt.rsplitn(3, '.');
    match (parts.next(), parts.next(), parts.next()) {
      (Some(_header), Some(payload), Some(_signature)) => {
        let json = STANDARD.decode(payload)?;
        let claims = serde_json::from_slice(&json)?;
        Ok(claims)
      }
      _ => {
        Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken).into())
      }
    }
  }

  pub fn encode(&self, secret: &str) -> Result<String> {
    let jwt = encode(
      &Header::default(),
      self,
      &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(jwt)
  }
}
