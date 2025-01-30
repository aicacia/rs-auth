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

pub struct TenantApiClient<C: Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> TenantApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> TenantApiClient<C> {
        TenantApiClient {
            configuration,
        }
    }
}

pub trait TenantApi: Send + Sync {
    fn all_tenants(&self, offset: Option<i32>, limit: Option<i32>, show_private_key: Option<bool>) -> Pin<Box<dyn Future<Output = Result<models::Pagination, Error>> + Send>>;
    fn create_tenant(&self, create_tenant: models::CreateTenant) -> Pin<Box<dyn Future<Output = Result<models::Tenant, Error>> + Send>>;
    fn delete_tenant(&self, tenant_id: i64) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn get_tenant_by_client_id(&self, tenant_client_id: &str, show_private_key: Option<bool>) -> Pin<Box<dyn Future<Output = Result<models::Tenant, Error>> + Send>>;
    fn get_tenant_by_id(&self, tenant_id: i64, show_private_key: Option<bool>) -> Pin<Box<dyn Future<Output = Result<models::Tenant, Error>> + Send>>;
    fn update_tenant(&self, tenant_id: i64, update_tenant: models::UpdateTenant) -> Pin<Box<dyn Future<Output = Result<models::Tenant, Error>> + Send>>;
}

impl<C: Connect>TenantApi for TenantApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn all_tenants(&self, offset: Option<i32>, limit: Option<i32>, show_private_key: Option<bool>) -> Pin<Box<dyn Future<Output = Result<models::Pagination, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/tenants".to_string())
        ;
        if let Some(ref s) = offset {
            let query_value = s.to_string();
            req = req.with_query_param("offset".to_string(), query_value);
        }
        if let Some(ref s) = limit {
            let query_value = s.to_string();
            req = req.with_query_param("limit".to_string(), query_value);
        }
        if let Some(ref s) = show_private_key {
            let query_value = s.to_string();
            req = req.with_query_param("show_private_key".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn create_tenant(&self, create_tenant: models::CreateTenant) -> Pin<Box<dyn Future<Output = Result<models::Tenant, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/tenants".to_string())
        ;
        req = req.with_body_param(create_tenant);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_tenant(&self, tenant_id: i64) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/tenants/{tenant_id}".to_string())
        ;
        req = req.with_path_param("tenant_id".to_string(), tenant_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_tenant_by_client_id(&self, tenant_client_id: &str, show_private_key: Option<bool>) -> Pin<Box<dyn Future<Output = Result<models::Tenant, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/tenants/by-client-id/{tenant_client_id}".to_string())
        ;
        if let Some(ref s) = show_private_key {
            let query_value = s.to_string();
            req = req.with_query_param("show_private_key".to_string(), query_value);
        }
        req = req.with_path_param("tenant_client_id".to_string(), tenant_client_id.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_tenant_by_id(&self, tenant_id: i64, show_private_key: Option<bool>) -> Pin<Box<dyn Future<Output = Result<models::Tenant, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/tenants/{tenant_id}".to_string())
        ;
        if let Some(ref s) = show_private_key {
            let query_value = s.to_string();
            req = req.with_query_param("show_private_key".to_string(), query_value);
        }
        req = req.with_path_param("tenant_id".to_string(), tenant_id.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_tenant(&self, tenant_id: i64, update_tenant: models::UpdateTenant) -> Pin<Box<dyn Future<Output = Result<models::Tenant, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/tenants/{tenant_id}".to_string())
        ;
        req = req.with_path_param("tenant_id".to_string(), tenant_id.to_string());
        req = req.with_body_param(update_tenant);

        req.execute(self.configuration.borrow())
    }

}
