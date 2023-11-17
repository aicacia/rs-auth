use std::{
  future::{self, Ready},
  sync::Arc,
};

use crate::service::{
  application::{get_application_by_id, get_application_jwt_secret},
  user::get_user_by_id,
};
use actix_web::{
  body::EitherBody,
  dev::{Service, ServiceRequest, ServiceResponse, Transform},
  web::Data,
  HttpMessage, HttpResponse,
};
use futures::future::LocalBoxFuture;
use sqlx::{Pool, Postgres};

use crate::{core::jwt::Claims, model::error::Errors};

pub struct Authorization;

impl<S, B> Transform<S, ServiceRequest> for Authorization
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = actix_web::Error;
  type InitError = ();
  type Transform = AuthorizationMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    future::ready(Ok(AuthorizationMiddleware {
      service: Arc::new(service),
    }))
  }
}

pub struct AuthorizationMiddleware<S> {
  service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
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
      match req.headers().get("authorization") {
        None => {
          let res = req
            .into_response(HttpResponse::Unauthorized().json(Errors::from("missing_authorization")))
            .map_into_right_body();
          return Ok(res);
        }
        Some(jwt_header) => {
          let jwt = match jwt_header.to_str() {
            Ok(jwt) => &jwt["Bearer ".len()..jwt.len()],
            Err(err) => {
              log::error!("Error: {}", err);
              let res = req
                .into_response(
                  HttpResponse::Unauthorized().json(Errors::from("invalid_authorization")),
                )
                .map_into_right_body();
              return Ok(res);
            }
          };

          let unvalidated_claims = match Claims::parse_no_validation(jwt) {
            Ok(c) => c,
            Err(err) => {
              log::error!("Failed to parse JWT: {}", err);
              let res = req
                .into_response(
                  HttpResponse::Unauthorized().json(Errors::from("invalid_authorization")),
                )
                .map_into_right_body();
              return Ok(res);
            }
          };

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

          let secret = get_application_jwt_secret(pool.as_ref(), unvalidated_claims.app).await;
          let token_data = match Claims::parse(jwt, &secret) {
            Ok(c) => c,
            Err(err) => {
              log::error!("Error: {}", err);
              let res = req
                .into_response(
                  HttpResponse::Unauthorized().json(Errors::from("invalid_authorization")),
                )
                .map_into_right_body();
              return Ok(res);
            }
          };

          let application = match get_application_by_id(pool, token_data.claims.app).await {
            Ok(Some(a)) => a,
            Ok(None) => {
              let res = req
                .into_response(HttpResponse::BadRequest().finish())
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
          };
          let user = match get_user_by_id(pool, token_data.claims.sub).await {
            Ok(u) => u,
            Err(e) => {
              log::error!("Error: {}", e);
              let res = req
                .into_response(HttpResponse::InternalServerError().json(Errors::internal_error()))
                .map_into_right_body();
              return Ok(res);
            }
          };
          let mut extensions = req.extensions_mut();
          extensions.insert(application);
          extensions.insert(user);
        }
      }

      let res = service.call(req).await?.map_into_left_body();
      Ok(res)
    })
  }
}
