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
pub struct TokenRequestServiceAccount {
    #[serde(rename = "client_id")]
    pub client_id: uuid::Uuid,
    #[serde(rename = "client_secret")]
    pub client_secret: uuid::Uuid,
    #[serde(rename = "grant_type")]
    pub grant_type: GrantType,
}

impl TokenRequestServiceAccount {
    pub fn new(client_id: uuid::Uuid, client_secret: uuid::Uuid, grant_type: GrantType) -> TokenRequestServiceAccount {
        TokenRequestServiceAccount {
            client_id,
            client_secret,
            grant_type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GrantType {
    #[serde(rename = "service-account")]
    ServiceAccount,
}

impl Default for GrantType {
    fn default() -> GrantType {
        Self::ServiceAccount
    }
}

