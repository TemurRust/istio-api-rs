use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HttpDirectResponse : HTTPDirectResponse can be used to send a fixed response to clients. For example, the following rule returns a fixed 503 status with a body to requests for /v1/getProductRatings API.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpDirectResponse {
    /// Specifies the HTTP response status to be returned.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<super::HttpBody>>,
}

impl HttpDirectResponse {
    /// HTTPDirectResponse can be used to send a fixed response to clients. For example, the following rule returns a fixed 503 status with a body to requests for /v1/getProductRatings API.
    pub fn new() -> HttpDirectResponse {
        HttpDirectResponse {
            status: None,
            body: None,
        }
    }
}