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
pub struct CreateApplication {
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateApplication {
    pub fn new(name: String) -> CreateApplication {
        CreateApplication {
            name,
        }
    }
}

