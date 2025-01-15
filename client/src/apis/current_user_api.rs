/*
 * auth
 *
 * Aicacia Auth API provides authentication services for applications.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::sync::Arc;
use std::borrow::Borrow;
use std::pin::Pin;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use futures::Future;

use crate::models;
use super::{Error, configuration};
use super::request as __internal_request;

pub struct CurrentUserApiClient<C: Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> CurrentUserApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> CurrentUserApiClient<C> {
        CurrentUserApiClient {
            configuration,
        }
    }
}

pub trait CurrentUserApi: Send + Sync {
    fn create_current_user_add_oauth2_provider_url(&self, provider: &str) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send>>;
    fn create_current_user_email(&self, create_user_email: models::CreateUserEmail) -> Pin<Box<dyn Future<Output = Result<models::UserEmail, Error>> + Send>>;
    fn create_current_user_phone_number(&self, create_user_phone_number: models::CreateUserPhoneNumber) -> Pin<Box<dyn Future<Output = Result<models::UserPhoneNumber, Error>> + Send>>;
    fn create_current_user_totp(&self, create_totp_request: models::CreateTotpRequest) -> Pin<Box<dyn Future<Output = Result<models::UserTotp, Error>> + Send>>;
    fn deactivate_current_user(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn delete_current_user_email(&self, email_id: i64) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn delete_current_user_phone_number(&self, phone_number_id: i64) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn delete_current_user_totp(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn get_current_user(&self, ) -> Pin<Box<dyn Future<Output = Result<models::User, Error>> + Send>>;
    fn reset_current_user_password(&self, reset_password_request: models::ResetPasswordRequest) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn set_current_user_email_as_primary(&self, email_id: i64) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn set_current_user_phone_number_as_primary(&self, phone_number_id: i64) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn update_current_user(&self, update_username: models::UpdateUsername) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn update_current_user_config(&self, update_user_config_request: models::UpdateUserConfigRequest) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn update_current_user_info(&self, update_user_info_request: models::UpdateUserInfoRequest) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
}

impl<C: Connect>CurrentUserApi for CurrentUserApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn create_current_user_add_oauth2_provider_url(&self, provider: &str) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/current-user/oauth2/{provider}".to_string())
        ;
        req = req.with_path_param("provider".to_string(), provider.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn create_current_user_email(&self, create_user_email: models::CreateUserEmail) -> Pin<Box<dyn Future<Output = Result<models::UserEmail, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/current-user/emails".to_string())
        ;
        req = req.with_body_param(create_user_email);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn create_current_user_phone_number(&self, create_user_phone_number: models::CreateUserPhoneNumber) -> Pin<Box<dyn Future<Output = Result<models::UserPhoneNumber, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/current-user/phone-numbers".to_string())
        ;
        req = req.with_body_param(create_user_phone_number);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn create_current_user_totp(&self, create_totp_request: models::CreateTotpRequest) -> Pin<Box<dyn Future<Output = Result<models::UserTotp, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/current-user/totp".to_string())
        ;
        req = req.with_body_param(create_totp_request);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn deactivate_current_user(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/current-user".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_current_user_email(&self, email_id: i64) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/current-user/emails/{email_id}".to_string())
        ;
        req = req.with_path_param("email_id".to_string(), email_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_current_user_phone_number(&self, phone_number_id: i64) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/current-user/phone-numbers/{phone_number_id}".to_string())
        ;
        req = req.with_path_param("phone_number_id".to_string(), phone_number_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_current_user_totp(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/current-user/totp".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_current_user(&self, ) -> Pin<Box<dyn Future<Output = Result<models::User, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/current-user".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn reset_current_user_password(&self, reset_password_request: models::ResetPasswordRequest) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/current-user/reset-password".to_string())
        ;
        req = req.with_body_param(reset_password_request);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn set_current_user_email_as_primary(&self, email_id: i64) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/current-user/emails/{email_id}/set-as-primary".to_string())
        ;
        req = req.with_path_param("email_id".to_string(), email_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn set_current_user_phone_number_as_primary(&self, phone_number_id: i64) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/current-user/phone-numbers/{phone_number_id}/set-as-primary".to_string())
        ;
        req = req.with_path_param("phone_number_id".to_string(), phone_number_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_current_user(&self, update_username: models::UpdateUsername) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/current-user".to_string())
        ;
        req = req.with_body_param(update_username);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_current_user_config(&self, update_user_config_request: models::UpdateUserConfigRequest) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/current-user/config".to_string())
        ;
        req = req.with_body_param(update_user_config_request);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_current_user_info(&self, update_user_info_request: models::UpdateUserInfoRequest) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/current-user/info".to_string())
        ;
        req = req.with_body_param(update_user_info_request);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}
