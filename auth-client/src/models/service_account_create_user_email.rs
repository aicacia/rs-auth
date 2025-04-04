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
pub struct ServiceAccountCreateUserEmail {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "primary", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary: Option<Option<bool>>,
    #[serde(rename = "verified", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub verified: Option<Option<bool>>,
}

impl ServiceAccountCreateUserEmail {
    pub fn new(email: String) -> ServiceAccountCreateUserEmail {
        ServiceAccountCreateUserEmail {
            email,
            primary: None,
            verified: None,
        }
    }
}

