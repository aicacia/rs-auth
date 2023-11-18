use std::{
  future::{self, Ready},
  sync::Arc,
};

use crate::{
  core::config::get_config,
  model::{error::Errors, user::UserRow},
  service::user::user_has_permissions,
};
use actix_web::{
  body::EitherBody,
  dev::{Service, ServiceRequest, ServiceResponse, Transform},
  web::Data,
  HttpMessage, HttpResponse,
};
use futures::future::LocalBoxFuture;
use sqlx::{Pool, Postgres};

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
      let user_id_option = req.extensions().get::<UserRow>().map(|u| u.id);

      if let Some(user_id) = user_id_option {
        let pool = match req.app_data::<Data<Pool<Postgres>>>() {
          Some(pool) => pool,
          None => {
            log::error!("Error: missing db pool");
            let res = req
              .into_response(HttpResponse::InternalServerError().json(Errors::internal_error()))
              .map_into_right_body();
            return Ok(res);
          }
        };
        match user_has_permissions(pool, user_id, get_config().admin_application_id, "admin").await
        {
          Ok(true) => {}
          Ok(false) => {
            let res = req
              .into_response(HttpResponse::Unauthorized().json(Errors::unauthorized()))
              .map_into_right_body();
            return Ok(res);
          }
          Err(e) => {
            log::error!("Error: {}", e);
            let res = req
              .into_response(HttpResponse::InternalServerError().json(Errors::internal_error()))
              .map_into_right_body();
            return Ok(res);
          }
        }
        let res = service.call(req).await?.map_into_left_body();
        return Ok(res);
      }
      let res = req
        .into_response(HttpResponse::Unauthorized().json(Errors::unauthorized()))
        .map_into_right_body();
      return Ok(res);
    })
  }
}
