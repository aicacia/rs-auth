use std::{
  collections::HashMap,
  fmt::{self},
};

use actix_web::ResponseError;
use actix_web_validator::error::DeserializeErrors;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

const APPLICATION_KEY: &str = "application";
const INTERNAL_ERROR: &str = "internal_error";
const UNAUTHORIZED: &str = "unauthorized";
const NOT_FOUND: &str = "not_found";

lazy_static! {
  static ref RE_BETWEEN_TICKS: Regex = Regex::new(r"`(.*)`").expect("Failed to compile regex");
}

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
pub struct Messages(Vec<Message>);

impl Messages {
  pub fn error(&mut self, msg: impl Into<Message>) -> &mut Self {
    self.0.push(msg.into());
    self
  }
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Errors(HashMap<String, Messages>);

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
    new.error(APPLICATION_KEY, msg);
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

  pub fn unauthorized() -> Self {
    Self::from(UNAUTHORIZED)
  }

  pub fn not_found() -> Self {
    Self::from(NOT_FOUND)
  }

  pub fn from_validation_error(err: actix_web_validator::Error) -> Self {
    let mut new = Self::default();
    match err {
      actix_web_validator::Error::Validate(validation_errors) => {
        handle_validation_errors(&mut new, &mut String::new(), &validation_errors);
      }
      actix_web_validator::Error::Deserialize(err) => {
        log::error!("{}", err);
        match err {
          DeserializeErrors::DeserializeQuery(err) => {
            if let Some(captures) = RE_BETWEEN_TICKS.captures(&err.to_string()) {
              if let Some(value) = captures.get(1) {
                new.error(value.as_str(), "invalid");
              }
            }
          }
          DeserializeErrors::DeserializeJson(err) => {
            if let Some(captures) = RE_BETWEEN_TICKS.captures(&err.to_string()) {
              if let Some(value) = captures.get(1) {
                new.error(value.as_str(), "invalid");
              }
            }
          }
          DeserializeErrors::DeserializePath(err) => {
            if let Some(captures) = RE_BETWEEN_TICKS.captures(&err.to_string()) {
              if let Some(value) = captures.get(1) {
                new.error(value.as_str(), "invalid");
              }
            }
          }
        }
      }
      actix_web_validator::Error::JsonPayloadError(err) => {
        if let Some(captures) = RE_BETWEEN_TICKS.captures(&err.to_string()) {
          if let Some(value) = captures.get(1) {
            new.error(value.as_str(), "invalid");
          }
        }
      }
      actix_web_validator::Error::UrlEncodedError(err) => {
        if let Some(captures) = RE_BETWEEN_TICKS.captures(&err.to_string()) {
          if let Some(value) = captures.get(1) {
            new.error(value.as_str(), "invalid");
          }
        }
      }
      actix_web_validator::Error::QsError(err) => {
        if let Some(captures) = RE_BETWEEN_TICKS.captures(&err.to_string()) {
          if let Some(value) = captures.get(1) {
            new.error(value.as_str(), "invalid");
          }
        }
      }
    }
    new
  }

  pub fn error(&mut self, name: impl Into<String>, msg: impl Into<Message>) -> &mut Self {
    self
      .0
      .entry(name.into())
      .or_insert_with(Default::default)
      .error(msg);
    self
  }

  pub fn application_error(&mut self, msg: impl Into<Message>) -> &mut Self {
    self.error(APPLICATION_KEY, msg)
  }
}

fn handle_validation_errors(
  errors: &mut Errors,
  current_name: &str,
  validation_errors: &ValidationErrors,
) {
  for (name, error) in validation_errors.errors() {
    let mut new_name = current_name.to_owned();
    if new_name.is_empty() {
      new_name.push_str(name);
    } else {
      new_name.push_str(&format!(".{}", name));
    }
    handle_validation_errors_kind(errors, &new_name, error);
  }
}

fn handle_validation_errors_kind(
  errors: &mut Errors,
  current_name: &str,
  error_kind: &ValidationErrorsKind,
) {
  match error_kind {
    ValidationErrorsKind::Struct(validation_errors) => {
      handle_validation_errors(errors, current_name, validation_errors);
    }
    ValidationErrorsKind::List(validation_errors) => {
      for (index, e) in validation_errors {
        let mut name = current_name.to_owned();
        name.push_str(&format!("[{}]", index));
        handle_validation_errors(errors, &mut name, e);
      }
    }
    ValidationErrorsKind::Field(validation_errors) => {
      for e in validation_errors {
        errors.error(current_name, e);
      }
    }
  }
}
