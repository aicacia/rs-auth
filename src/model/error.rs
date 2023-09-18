use std::{
  collections::HashMap,
  fmt::{self},
};

use actix_web::ResponseError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use validator::{ValidationError, ValidationErrorsKind};

const GLOBAL_KEY: &str = "global";
const INTERNAL_ERROR: &str = "internal_error";

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Message {
  key: String,
  args: HashMap<String, Value>,
}

impl<'a> From<&'a ValidationError> for Message {
  fn from(error: &'a ValidationError) -> Self {
    Self {
      key: error.code.to_string(),
      args: error
        .params
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect(),
    }
  }
}

impl<'a> From<&'a str> for Message {
  fn from(key: &'a str) -> Self {
    Self {
      key: key.to_owned(),
      args: HashMap::default(),
    }
  }
}

impl From<String> for Message {
  fn from(key: String) -> Self {
    Self {
      key: key,
      args: HashMap::default(),
    }
  }
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Messages {
  errors: Vec<Message>,
  warnings: Vec<Message>,
}

impl Messages {
  pub fn error(&mut self, msg: impl Into<Message>) -> &mut Self {
    self.errors.push(msg.into());
    self
  }
  pub fn warning(&mut self, msg: impl Into<Message>) -> &mut Self {
    self.warnings.push(msg.into());
    self
  }
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Errors {
  messages: HashMap<String, Messages>,
}

impl fmt::Display for Errors {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    match serde_json::to_string(self) {
      Ok(json) => write!(f, "{}", json),
      Err(err) => {
        log::error!("Failed to format error response: {}", err);
        Err(fmt::Error)
      }
    }
  }
}

impl<T> From<T> for Errors
where
  T: Into<Message>,
{
  fn from(msg: T) -> Self {
    let mut new = Self::default();
    new.error(GLOBAL_KEY, msg);
    new
  }
}

impl ResponseError for Errors {
  fn status_code(&self) -> actix_web::http::StatusCode {
    actix_web::http::StatusCode::BAD_REQUEST
  }
}

impl Errors {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn internal_error() -> Self {
    Self::from(INTERNAL_ERROR)
  }

  pub fn from_validation_error(err: actix_web_validator::Error) -> Self {
    let mut new = Self::default();
    match err {
      actix_web_validator::Error::Validate(validation_errors) => {
        for (name, error) in validation_errors.errors() {
          match error {
            ValidationErrorsKind::Struct(errors) => {
              // TODO: handle struct errors
              log::info!("Struct name: {}, errors: {:?}", name, errors);
            }
            ValidationErrorsKind::List(errors) => {
              // TODO: handle list errors
              log::info!("List name: {}, errors: {:?}", name, errors);
            }
            ValidationErrorsKind::Field(errors) => {
              for error in errors {
                new.error(*name, error);
              }
            }
          }
        }
      }
      actix_web_validator::Error::Deserialize(err) => {
        new.global_error(format!("{}", err));
      }
      actix_web_validator::Error::JsonPayloadError(err) => {
        new.global_error(format!("{}", err));
      }
      actix_web_validator::Error::UrlEncodedError(err) => {
        new.global_error(format!("{}", err));
      }
      actix_web_validator::Error::QsError(err) => {
        new.global_error(format!("{}", err));
      }
    }
    new
  }

  pub fn error(&mut self, name: impl Into<String>, msg: impl Into<Message>) -> &mut Self {
    self
      .messages
      .entry(name.into())
      .or_insert_with(Default::default)
      .error(msg);
    self
  }
  pub fn warning(&mut self, name: impl Into<String>, msg: impl Into<Message>) -> &mut Self {
    self
      .messages
      .entry(name.into())
      .or_insert_with(Default::default)
      .warning(msg);
    self
  }

  pub fn global_error(&mut self, msg: impl Into<Message>) -> &mut Self {
    self.error(GLOBAL_KEY, msg)
  }
  pub fn global_warning(&mut self, msg: impl Into<Message>) -> &mut Self {
    self.warning(GLOBAL_KEY, msg)
  }
}
