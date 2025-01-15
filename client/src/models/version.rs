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
pub struct Version {
    #[serde(rename = "build")]
    pub build: String,
    #[serde(rename = "version")]
    pub version: String,
}

impl Version {
    pub fn new(build: String, version: String) -> Version {
        Version {
            build,
            version,
        }
    }
}

