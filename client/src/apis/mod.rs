use std::fmt;
use std::fmt::Debug;

use hyper;
use hyper::http;
use hyper_util::client::legacy::connect::Connect;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Header(http::header::InvalidHeaderValue),
    Http(http::Error),
    Hyper(hyper::Error),
    HyperClient(hyper_util::client::legacy::Error),
    Serde(serde_json::Error),
    UriError(http::uri::InvalidUri),
}

pub struct ApiError {
    pub code: hyper::StatusCode,
    pub body: hyper::body::Incoming,
}

impl Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiError")
         .field("code", &self.code)
         .field("body", &"hyper::body::Incoming")
         .finish()
    }
}

impl From<(hyper::StatusCode, hyper::body::Incoming)> for Error {
    fn from(e: (hyper::StatusCode, hyper::body::Incoming)) -> Self {
        Error::Api(ApiError {
            code: e.0,
            body: e.1,
        })
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Error::Http(e)
    }
}

impl From<hyper_util::client::legacy::Error> for Error {
    fn from(e: hyper_util::client::legacy::Error) -> Self {
        Error::HyperClient(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

mod request;

mod current_user_api;
pub use self::current_user_api::{ CurrentUserApi, CurrentUserApiClient };
mod jwt_api;
pub use self::jwt_api::{ JwtApi, JwtApiClient };
mod mfa_api;
pub use self::mfa_api::{ MfaApi, MfaApiClient };
mod oauth2_api;
pub use self::oauth2_api::{ Oauth2Api, Oauth2ApiClient };
mod oauth2_provider_api;
pub use self::oauth2_provider_api::{ Oauth2ProviderApi, Oauth2ProviderApiClient };
mod openapi_api;
pub use self::openapi_api::{ OpenapiApi, OpenapiApiClient };
mod p2p_api;
pub use self::p2p_api::{ P2pApi, P2pApiClient };
mod register_api;
pub use self::register_api::{ RegisterApi, RegisterApiClient };
mod service_account_api;
pub use self::service_account_api::{ ServiceAccountApi, ServiceAccountApiClient };
mod tenant_api;
pub use self::tenant_api::{ TenantApi, TenantApiClient };
mod token_api;
pub use self::token_api::{ TokenApi, TokenApiClient };
mod user_api;
pub use self::user_api::{ UserApi, UserApiClient };
mod users_api;
pub use self::users_api::{ UsersApi, UsersApiClient };
mod util_api;
pub use self::util_api::{ UtilApi, UtilApiClient };

pub mod configuration;
pub mod client;
