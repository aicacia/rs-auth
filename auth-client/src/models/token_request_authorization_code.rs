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
pub struct TokenRequestAuthorizationCode {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "grant_type")]
    pub grant_type: GrantType,
    #[serde(rename = "scope", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Option<String>>,
}

impl TokenRequestAuthorizationCode {
    pub fn new(code: String, grant_type: GrantType) -> TokenRequestAuthorizationCode {
        TokenRequestAuthorizationCode {
            code,
            grant_type,
            scope: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GrantType {
    #[serde(rename = "authorization-code")]
    AuthorizationCode,
}

impl Default for GrantType {
    fn default() -> GrantType {
        Self::AuthorizationCode
    }
}

