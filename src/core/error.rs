use std::{collections::HashMap, fmt, io};

use axum::{
  http::{header, StatusCode},
  response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

pub const APPLICATION: &str = "application";
pub const REQUEST_BODY: &str = "request-body";

pub const REQUIRED_ERROR: &str = "required";
pub const NOT_FOUND_ERROR: &str = "not-found";
pub const INVALID_ERROR: &str = "invalid";
pub const PARSE_ERROR: &str = "parse";
pub const INTERNAL_ERROR: &str = "internal";
pub const DATEBASE_ERROR: &str = "database";
pub const NOT_ALLOWED_ERROR: &str = "not-allowed";
pub const ALREADY_USED_ERROR: &str = "already-used";
pub const ALREADY_EXISTS_ERROR: &str = "already-exists";

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorMessage {
  code: String,
  parameters: HashMap<String, Value>,
}

impl<'a> From<&'a ValidationError> for ErrorMessage {
  fn from(error: &'a ValidationError) -> Self {
    Self::from((
      error.code.to_string(),
      error
        .params
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect(),
    ))
  }
}

impl<'a> From<&'a str> for ErrorMessage {
  fn from(code: &'a str) -> Self {
    Self::from((code.to_owned(), HashMap::default()))
  }
}

impl From<String> for ErrorMessage {
  fn from(code: String) -> Self {
    Self::from((code, HashMap::default()))
  }
}

impl<'a> From<(&'a str, HashMap<String, Value>)> for ErrorMessage {
  fn from((code, parameters): (&'a str, HashMap<String, Value>)) -> Self {
    Self::from((code.to_owned(), parameters))
  }
}

impl From<(String, HashMap<String, Value>)> for ErrorMessage {
  fn from((code, parameters): (String, HashMap<String, Value>)) -> Self {
    Self { code, parameters }
  }
}

impl From<sqlx::Error> for ErrorMessage {
  fn from(err: sqlx::Error) -> Self {
    Self {
      code: err.to_string(),
      parameters: HashMap::default(),
    }
  }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorMessages(Vec<ErrorMessage>);

impl ErrorMessages {
  pub fn error(&mut self, msg: impl Into<ErrorMessage>) -> &mut Self {
    self.0.push(msg.into());
    self
  }

  pub fn with_error(mut self, msg: impl Into<ErrorMessage>) -> Self {
    self.0.push(msg.into());
    self
  }
}

pub type Errors = HashMap<String, ErrorMessages>;

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct InternalError {
  status_code: u16,
  messages: HashMap<String, ErrorMessages>,
}

impl fmt::Display for InternalError {
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

impl From<StatusCode> for InternalError {
  fn from(status_code: StatusCode) -> Self {
    Self {
      status_code: status_code.as_u16(),
      messages: HashMap::default(),
    }
  }
}

impl From<ValidationErrors> for InternalError {
  fn from(validation_errors: ValidationErrors) -> Self {
    let mut new = Self::bad_request();
    handle_validation_errors(&mut new, &mut String::new(), &validation_errors);
    new
  }
}

impl From<io::Error> for InternalError {
  fn from(error: io::Error) -> Self {
    Self::internal_error().with_application_error(error.to_string())
  }
}

impl From<sqlx::Error> for InternalError {
  fn from(error: sqlx::Error) -> Self {
    Self::internal_error().with_application_error(error.to_string())
  }
}

impl From<config::ConfigError> for InternalError {
  fn from(error: config::ConfigError) -> Self {
    Self::internal_error().with_application_error(error.to_string())
  }
}

impl From<oauth2::url::ParseError> for InternalError {
  fn from(error: oauth2::url::ParseError) -> Self {
    Self::internal_error().with_application_error(error.to_string())
  }
}

impl From<webrtc::Error> for InternalError {
  fn from(error: webrtc::Error) -> Self {
    Self::internal_error().with_application_error(error.to_string())
  }
}

impl From<reqwest::Error> for InternalError {
  fn from(error: reqwest::Error) -> Self {
    Self::internal_error().with_application_error(error.to_string())
  }
}

impl From<async_tungstenite::tungstenite::Error> for InternalError {
  fn from(error: async_tungstenite::tungstenite::Error) -> Self {
    Self::internal_error().with_application_error(error.to_string())
  }
}

impl From<serde_json::Error> for InternalError {
  fn from(error: serde_json::Error) -> Self {
    Self::internal_error().with_application_error(error.to_string())
  }
}

impl IntoResponse for InternalError {
  fn into_response(self) -> Response {
    match StatusCode::from_u16(self.status_code) {
      Ok(status_code) => (status_code, axum::Json(self.messages)).into_response(),
      Err(err) => {
        log::error!("Invalid status code: {}", err);
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          [(header::CONTENT_TYPE, "application/json")],
          axum::Json(self.messages),
        )
          .into_response()
      }
    }
  }
}

impl InternalError {
  pub fn bad_request() -> Self {
    Self::from(StatusCode::BAD_REQUEST)
  }

  pub fn internal_error() -> Self {
    Self::from(StatusCode::INTERNAL_SERVER_ERROR)
  }

  pub fn unauthorized() -> Self {
    Self::from(StatusCode::UNAUTHORIZED)
  }

  pub fn not_found() -> Self {
    Self::from(StatusCode::NOT_FOUND)
  }

  pub fn forbidden() -> Self {
    Self::from(StatusCode::FORBIDDEN)
  }

  pub fn status(&mut self, status: StatusCode) -> &mut Self {
    self.status_code = status.as_u16();
    self
  }

  pub fn with_status(mut self, status: StatusCode) -> Self {
    self.status(status);
    self
  }

  pub fn error(&mut self, name: impl Into<String>, msg: impl Into<ErrorMessage>) -> &mut Self {
    self
      .messages
      .entry(name.into())
      .or_insert_with(Default::default)
      .error(msg);
    self
  }

  pub fn with_error(mut self, name: impl Into<String>, msg: impl Into<ErrorMessage>) -> Self {
    self.error(name, msg);
    self
  }

  pub fn application_error(&mut self, msg: impl Into<ErrorMessage>) -> &mut Self {
    self.error(APPLICATION, msg)
  }

  pub fn with_application_error(self, msg: impl Into<ErrorMessage>) -> Self {
    self.with_error(APPLICATION, msg)
  }
}

fn handle_validation_errors(
  errors: &mut InternalError,
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
  errors: &mut InternalError,
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
