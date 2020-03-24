/*
 * JJS main API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use hyper;

pub struct Configuration<C: hyper::client::connect::Connect + 'static> {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: hyper::client::Client<C, hyper::Body>,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl<C: hyper::client::connect::Connect + 'static> Configuration<C> {
    pub fn new(client: hyper::client::Client<C, hyper::Body>) -> Configuration<C> {
        Configuration {
            base_path: "http://localhost".to_owned(),
            user_agent: Some("OpenAPI-Generator/1.0.0/rust".to_owned()),
            client,
            basic_auth: None,
            oauth_access_token: None,
            api_key: None,
        }
    }
}
