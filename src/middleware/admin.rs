use std::{
  future::{self, Ready},
  sync::Arc,
};

use crate::{
  core::config::get_config,
  model::{application::ApplicationRow, error::Errors},
};
use actix_web::{
  body::EitherBody,
  dev::{Service, ServiceRequest, ServiceResponse, Transform},
  HttpMessage, HttpResponse,
};
use futures::future::LocalBoxFuture;

pub struct AdminAuthorization;

impl<S, B> Transform<S, ServiceRequest> for AdminAuthorization
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = actix_web::Error;
  type InitError = ();
  type Transform = AdminAuthorizationMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    future::ready(Ok(AdminAuthorizationMiddleware {
      service: Arc::new(service),
    }))
  }
}

pub struct AdminAuthorizationMiddleware<S> {
  service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for AdminAuthorizationMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = actix_web::Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  fn poll_ready(
    &self,
    ctx: &mut core::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    self.service.poll_ready(ctx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let service = self.service.clone();

    Box::pin(async move {
      let application_id_option = req.extensions().get::<ApplicationRow>().map(|a| a.id);
      if let Some(application_id) = application_id_option {
        if application_id == get_config().admin_application_id {
          let res = service.call(req).await?.map_into_left_body();
          return Ok(res);
        }
      }
      let res = req
        .into_response(HttpResponse::Unauthorized().json(Errors::from("invalid_credentials")))
        .map_into_right_body();
      return Ok(res);
    })
  }
}
