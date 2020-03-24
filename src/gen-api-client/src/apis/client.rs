use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient {
    default_api: Box<dyn crate::apis::DefaultApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static>(
        configuration: Configuration<C>,
    ) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            default_api: Box::new(crate::apis::DefaultApiClient::new(rc.clone())),
        }
    }

    pub fn default_api(&self) -> &dyn crate::apis::DefaultApi {
        self.default_api.as_ref()
    }
}
