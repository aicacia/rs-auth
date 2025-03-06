/*
 * auth
 *
 * Aicacia Auth API provides authentication services for applications.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tenant {
    #[serde(rename = "algorithm")]
    pub algorithm: models::Algorithm,
    #[serde(rename = "audience", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub audience: Option<Option<String>>,
    #[serde(rename = "client_id")]
    pub client_id: uuid::Uuid,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "expires_in_seconds")]
    pub expires_in_seconds: i64,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "issuer")]
    pub issuer: String,
    #[serde(rename = "oauth2_providers")]
    pub oauth2_providers: Vec<models::TenantOAuth2Provider>,
    #[serde(rename = "private_key", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<Option<String>>,
    #[serde(rename = "public_key", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<Option<String>>,
    #[serde(rename = "refresh_expires_in_seconds")]
    pub refresh_expires_in_seconds: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl Tenant {
    pub fn new(algorithm: models::Algorithm, client_id: uuid::Uuid, created_at: String, expires_in_seconds: i64, id: i64, issuer: String, oauth2_providers: Vec<models::TenantOAuth2Provider>, refresh_expires_in_seconds: i64, updated_at: String) -> Tenant {
        Tenant {
            algorithm,
            audience: None,
            client_id,
            created_at,
            expires_in_seconds,
            id,
            issuer,
            oauth2_providers,
            private_key: None,
            public_key: None,
            refresh_expires_in_seconds,
            updated_at,
        }
    }
}

