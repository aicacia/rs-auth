use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    current_user_api: Box<dyn crate::apis::CurrentUserApi>,
    jwt_api: Box<dyn crate::apis::JwtApi>,
    mfa_api: Box<dyn crate::apis::MfaApi>,
    oauth2_api: Box<dyn crate::apis::Oauth2Api>,
    oauth2_provider_api: Box<dyn crate::apis::Oauth2ProviderApi>,
    openapi_api: Box<dyn crate::apis::OpenapiApi>,
    p2p_api: Box<dyn crate::apis::P2pApi>,
    register_api: Box<dyn crate::apis::RegisterApi>,
    service_account_api: Box<dyn crate::apis::ServiceAccountApi>,
    tenant_api: Box<dyn crate::apis::TenantApi>,
    token_api: Box<dyn crate::apis::TokenApi>,
    user_api: Box<dyn crate::apis::UserApi>,
    users_api: Box<dyn crate::apis::UsersApi>,
    util_api: Box<dyn crate::apis::UtilApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            current_user_api: Box::new(crate::apis::CurrentUserApiClient::new(rc.clone())),
            jwt_api: Box::new(crate::apis::JwtApiClient::new(rc.clone())),
            mfa_api: Box::new(crate::apis::MfaApiClient::new(rc.clone())),
            oauth2_api: Box::new(crate::apis::Oauth2ApiClient::new(rc.clone())),
            oauth2_provider_api: Box::new(crate::apis::Oauth2ProviderApiClient::new(rc.clone())),
            openapi_api: Box::new(crate::apis::OpenapiApiClient::new(rc.clone())),
            p2p_api: Box::new(crate::apis::P2pApiClient::new(rc.clone())),
            register_api: Box::new(crate::apis::RegisterApiClient::new(rc.clone())),
            service_account_api: Box::new(crate::apis::ServiceAccountApiClient::new(rc.clone())),
            tenant_api: Box::new(crate::apis::TenantApiClient::new(rc.clone())),
            token_api: Box::new(crate::apis::TokenApiClient::new(rc.clone())),
            user_api: Box::new(crate::apis::UserApiClient::new(rc.clone())),
            users_api: Box::new(crate::apis::UsersApiClient::new(rc.clone())),
            util_api: Box::new(crate::apis::UtilApiClient::new(rc.clone())),
        }
    }

    pub fn current_user_api(&self) -> &dyn crate::apis::CurrentUserApi{
        self.current_user_api.as_ref()
    }

    pub fn jwt_api(&self) -> &dyn crate::apis::JwtApi{
        self.jwt_api.as_ref()
    }

    pub fn mfa_api(&self) -> &dyn crate::apis::MfaApi{
        self.mfa_api.as_ref()
    }

    pub fn oauth2_api(&self) -> &dyn crate::apis::Oauth2Api{
        self.oauth2_api.as_ref()
    }

    pub fn oauth2_provider_api(&self) -> &dyn crate::apis::Oauth2ProviderApi{
        self.oauth2_provider_api.as_ref()
    }

    pub fn openapi_api(&self) -> &dyn crate::apis::OpenapiApi{
        self.openapi_api.as_ref()
    }

    pub fn p2p_api(&self) -> &dyn crate::apis::P2pApi{
        self.p2p_api.as_ref()
    }

    pub fn register_api(&self) -> &dyn crate::apis::RegisterApi{
        self.register_api.as_ref()
    }

    pub fn service_account_api(&self) -> &dyn crate::apis::ServiceAccountApi{
        self.service_account_api.as_ref()
    }

    pub fn tenant_api(&self) -> &dyn crate::apis::TenantApi{
        self.tenant_api.as_ref()
    }

    pub fn token_api(&self) -> &dyn crate::apis::TokenApi{
        self.token_api.as_ref()
    }

    pub fn user_api(&self) -> &dyn crate::apis::UserApi{
        self.user_api.as_ref()
    }

    pub fn users_api(&self) -> &dyn crate::apis::UsersApi{
        self.users_api.as_ref()
    }

    pub fn util_api(&self) -> &dyn crate::apis::UtilApi{
        self.util_api.as_ref()
    }

}
