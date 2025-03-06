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
pub struct MfaRequestServiceAccount {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl MfaRequestServiceAccount {
    pub fn new(code: String, r#type: Type) -> MfaRequestServiceAccount {
        MfaRequestServiceAccount {
            code,
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "service-account")]
    ServiceAccount,
}

impl Default for Type {
    fn default() -> Type {
        Self::ServiceAccount
    }
}

