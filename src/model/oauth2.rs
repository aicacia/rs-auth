use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use super::application::{Application, ApplicationRow};

#[derive(Deserialize, Validate, IntoParams)]
pub struct AuthorizeQuery {
  pub client_id: i32,
  pub redirect_uri: String,
  pub response_type: Option<String>,
  pub scope: Option<Vec<String>>,
  pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuth2CodeClaims {
  pub exp: usize,
  pub iat: usize,
  pub iss: String,
  pub nonce: String,
  pub client_id: i32,
  pub redirect_uri: String,
  pub state: Option<String>,
  pub scope: Vec<String>,
}

impl OAuth2CodeClaims {
  pub fn new(
    client_id: i32,
    redirect_uri: String,
    state: Option<String>,
    scope: Vec<String>,
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
      client_id,
      redirect_uri,
      state,
      scope,
    }
  }
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct OAuth2Application {
  application: Application,
  scope: Vec<String>,
  redirect_uri: String,
  state: Option<String>,
}

impl From<(ApplicationRow, Vec<String>, String, Option<String>)> for OAuth2Application {
  fn from(
    (application, scope, redirect_uri, state): (
      ApplicationRow,
      Vec<String>,
      String,
      Option<String>,
    ),
  ) -> Self {
    Self {
      application: Application::from(application),
      scope,
      redirect_uri,
      state,
    }
  }
}
