use crate::model::error::Errors;
use crate::{
  core::openapi::SecurityAddon, model::application as application_model, model::auth as auth_model,
  model::error as error_model, model::oauth2 as oauth2_model, model::user as user_model,
  model::util as util_model,
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

use crate::controller::{application, auth, oauth2, openapi, user, util};

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
    user::create_email,
    user::delete_email,
    user::set_primary_email,
    user::send_confirmation_email,
    user::reset_password,
    user::change_username,
    user::applications,
    user::users,
    application::index,
    application::show,
    application::create,
    application::update,
    application::reset_secret,
    application::remove,
    application::config,
    application::update_config,
    oauth2::authorize,
    oauth2::application,
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
      user_model::CreateUserEmailRequest,
      user_model::PaginationUser,
      application_model::Application,
      application_model::ApplicationWithSecret,
      application_model::ApplicationConfig,
      application_model::ApplicationPermission,
      application_model::PaginationApplication,
      application_model::CreateApplicationRequest,
      application_model::UpdateApplicationRequest,
      application_model::UpdateApplicationConfigRequest,
      oauth2_model::OAuth2Application,
    )
  ),
  tags(
    (name = "util", description = "Utility endpoints"),
    (name = "auth", description = "Authentication endpoints"),
    (name = "oauth2", description = "OAuth2 endpoints"),
    (name = "user", description = "Users endpoints"),
    (name = "application", description = "Applications endpoints"),
  ),
  modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

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
  let json_config =
    JsonConfig::default().error_handler(|err, _req| Errors::from_validation_error(err).into());

  App::new()
    .app_data(json_config)
    .app_data(web::Data::new(pool.clone()))
    .app_data(web::Data::new(ApiDoc::openapi()))
    .wrap(Logger::default())
    .wrap(
      Cors::default()
        .allow_any_header()
        .allow_any_method()
        .allow_any_origin()
        .expose_any_header()
        .supports_credentials(),
    )
    .configure(openapi::configure())
    .configure(util::configure())
    .configure(auth::configure())
    .configure(oauth2::configure())
    .configure(user::configure())
    .configure(application::configure())
}
