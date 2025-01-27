use axum::{
  extract::{rejection::JsonRejection, FromRequest, Request},
  Json,
};
use validator::Validate;

use crate::core::error::{InternalError, REQUEST_BODY};

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
  T: Validate,
  Json<T>: FromRequest<S, Rejection = JsonRejection>,
  S: Send + Sync,
{
  type Rejection = InternalError;

  async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
    let Json(value) = match Json::<T>::from_request(request, state).await {
      Ok(value) => value,
      Err(rejection) => {
        return Err(InternalError::bad_request().with_error(REQUEST_BODY, rejection.to_string()));
      }
    };

    match value.validate() {
      Ok(_) => (),
      Err(errors) => return Err(InternalError::from(errors)),
    };

    Ok(Self(value))
  }
}
