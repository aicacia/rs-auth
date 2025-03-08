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
pub struct UpdateUser {
    #[serde(rename = "active", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active: Option<Option<bool>>,
    #[serde(rename = "username", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub username: Option<Option<String>>,
}

impl UpdateUser {
    pub fn new() -> UpdateUser {
        UpdateUser {
            active: None,
            username: None,
        }
    }
}

