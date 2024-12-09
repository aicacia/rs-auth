use std::str::FromStr;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::repository::tenent::TenentRow;

pub const TOKEN_TYPE_BEARER: &str = "bearer";
pub const TOKEN_TYPE_REFRESH: &str = "refresh";
pub const TOKEN_TYPE_AUTHORIZATION_CODE: &str = "authorization-code";
pub const TOKEN_TYPE_RESET_PASSWORD: &str = "reset-password";
pub const TOKEN_TYPE_MFA_TOTP_PREFIX: &str = "mfa-";
pub const TOKEN_TYPE_ID: &str = "id";

pub const TOKEN_SUB_TYPE_USER: &str = "user";
pub const TOKEN_SUB_TYPE_SERVICE_ACCOUNT: &str = "service-account";

pub trait Claims: Serialize + DeserializeOwned {
  fn kind(&self) -> &String;
  fn exp(&self) -> i64;
  fn iat(&self) -> i64;
  fn nbf(&self) -> i64;
  fn iss(&self) -> &String;
  fn aud(&self) -> Option<&String>;
  fn sub_kind(&self) -> &String;
  fn sub(&self) -> i64;
  fn app(&self) -> i64;
  fn scopes(&self) -> &[String];

  fn encode(&self, tenent: &TenentRow) -> Result<String, jsonwebtoken::errors::Error> {
    let algorithm = jsonwebtoken::Algorithm::from_str(&tenent.algorithm)?;

    let mut header = jsonwebtoken::Header::new(algorithm);
    header.kid = Some(tenent.id.to_string());

    let key = tenent_encoding_key(tenent, algorithm)?;

    jsonwebtoken::encode(&header, self, &key)
  }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct BasicClaims {
  #[serde(rename = "type")]
  pub kind: String,
  pub exp: i64,
  pub iat: i64,
  pub nbf: i64,
  pub iss: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub aud: Option<String>,
  #[serde(rename = "sub_type")]
  pub sub_kind: String,
  pub sub: i64,
  pub app: i64,
  #[serde(rename = "scope")]
  pub scopes: Vec<String>,
}

impl Claims for BasicClaims {
  fn kind(&self) -> &String {
    &self.kind
  }
  fn exp(&self) -> i64 {
    self.exp
  }
  fn iat(&self) -> i64 {
    self.iat
  }
  fn nbf(&self) -> i64 {
    self.nbf
  }
  fn iss(&self) -> &String {
    &self.iss
  }
  fn aud(&self) -> Option<&String> {
    self.aud.as_ref()
  }
  fn sub_kind(&self) -> &String {
    &self.sub_kind
  }
  fn sub(&self) -> i64 {
    self.sub
  }
  fn app(&self) -> i64 {
    self.app
  }
  fn scopes(&self) -> &[String] {
    &self.scopes
  }
}

pub fn parse_jwt<T>(
  jwt: &str,
  tenent: &TenentRow,
) -> Result<jsonwebtoken::TokenData<T>, jsonwebtoken::errors::Error>
where
  T: DeserializeOwned,
{
  let algorithm = jsonwebtoken::Algorithm::from_str(&tenent.algorithm)?;

  let mut validation = jsonwebtoken::Validation::new(algorithm);
  validation.validate_nbf = true;
  validation.set_issuer(&[&tenent.issuer]);
  if let Some(audience) = &tenent.audience {
    validation.set_audience(&[audience]);
  }

  let key = tenent_decoding_key(tenent, algorithm)?;

  jsonwebtoken::decode(jwt, &key, &validation)
}

pub fn parse_jwt_no_validation<T>(
  jwt: &str,
) -> Result<jsonwebtoken::TokenData<T>, jsonwebtoken::errors::Error>
where
  T: DeserializeOwned,
{
  let mut validation = jsonwebtoken::Validation::default();

  validation.validate_aud = false;
  validation.insecure_disable_signature_validation();

  jsonwebtoken::decode(
    jwt,
    &jsonwebtoken::DecodingKey::from_secret("".as_bytes()),
    &validation,
  )
}

pub fn tenent_decoding_key(
  tenent: &TenentRow,
  algorithm: jsonwebtoken::Algorithm,
) -> Result<jsonwebtoken::DecodingKey, jsonwebtoken::errors::Error> {
  match &algorithm {
    jsonwebtoken::Algorithm::HS256
    | jsonwebtoken::Algorithm::HS384
    | jsonwebtoken::Algorithm::HS512 => Ok(jsonwebtoken::DecodingKey::from_secret(
      tenent.private_key.as_bytes(),
    )),
    _ => {
      return Err(jsonwebtoken::errors::Error::from(
        jsonwebtoken::errors::ErrorKind::InvalidAlgorithm,
      ));
    }
  }
}

pub fn tenent_encoding_key(
  tenent: &TenentRow,
  algorithm: jsonwebtoken::Algorithm,
) -> Result<jsonwebtoken::EncodingKey, jsonwebtoken::errors::Error> {
  match &algorithm {
    jsonwebtoken::Algorithm::HS256
    | jsonwebtoken::Algorithm::HS384
    | jsonwebtoken::Algorithm::HS512 => Ok(jsonwebtoken::EncodingKey::from_secret(
      tenent.private_key.as_bytes(),
    )),
    _ => {
      return Err(jsonwebtoken::errors::Error::from(
        jsonwebtoken::errors::ErrorKind::InvalidAlgorithm,
      ));
    }
  }
}
