/*
 * JJS main API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// SessionToken : Type that represents session You shouldn't do any assumptions about this type representation

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionToken {
    /// Opaque string that represents session data On all subsequent requests, put this string as value of header `X-Jjs-Auth`
    #[serde(rename = "data")]
    pub data: String,
    /// in dev mode, contains session data in unencrypted form
    #[serde(rename = "raw_data")]
    pub raw_data: Option<String>,
}

impl SessionToken {
    /// Type that represents session You shouldn't do any assumptions about this type representation
    pub fn new(data: String, raw_data: Option<String>) -> SessionToken {
        SessionToken { data, raw_data }
    }
}
