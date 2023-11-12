use crate::model::error::Errors;
use crate::{
  core::openapi::SecurityAddon, model::application as application_model, model::auth as auth_model,
  model::error as error_model, model::user as user_model, model::util as util_model,
};
use actix_cors::Cors;
use actix_web::{
  body::MessageBody,
  dev::{ServiceFactory, ServiceRequest, ServiceResponse},
  error,
  middleware::Logger,
  web, App,
};
use actix_web_validator::JsonConfig;

use sqlx::{Pool, Postgres};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::controller::{application, auth, user, util};

#[derive(OpenApi)]
#[openapi(
  paths(
    util::health,
    util::version,
    auth::sign_in_with_password,
    auth::sign_up_with_password,
    auth::request_reset_password,
    auth::reset_password_with_token,
    auth::confirm_email,
    auth::sign_up_methods,
    user::current,
    user::set_primary_email,
    user::reset_password,
    user::refresh_token,
    user::change_username,
    user::applications,
    application::index,
  ),
  components(
    schemas(
      util_model::Version,
      util_model::Health,
      error_model::Errors,
      error_model::Message,
      error_model::Messages,
      auth_model::SignInWithPasswordRequest,
      auth_model::SignUpWithPasswordRequest,
      auth_model::ResetPasswordRequest,
      auth_model::RequestResetPasswordRequest,
      auth_model::SignUpMethods,
      user_model::Email,
      user_model::User,
      user_model::ResetUserPasswordRequest,
      user_model::ChangeUsernameRequest,
      application_model::Application,
    )
  ),
  tags(
    (name = "util", description = "Utility endpoints"),
    (name = "auth", description = "Authentication endpoints"),
    (name = "user", description = "Users endpoints"),
    (name = "application", description = "Applications endpoints"),
  ),
  modifiers(&SecurityAddon)
)]
struct ApiDoc;

pub fn create_app(
  pool: &Pool<Postgres>,
) -> App<
  impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<impl MessageBody>,
    Config = (),
    InitError = (),
    Error = error::Error,
  >,
> {
  let openapi = ApiDoc::openapi();

  let json_config =
    JsonConfig::default().error_handler(|err, _req| Errors::from_validation_error(err).into());

  App::new()
    .app_data(json_config)
    .app_data(web::Data::new(pool.clone()))
    .wrap(Logger::default())
    .wrap(
      Cors::default()
        .allow_any_header()
        .allow_any_method()
        .allow_any_origin()
        .expose_any_header()
        .supports_credentials(),
    )
    .configure(util::configure())
    .configure(auth::configure())
    .configure(user::configure())
    .configure(application::configure())
    .service(
      SwaggerUi::new("/api-docs/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
    )
}
