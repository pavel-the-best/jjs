use crate::security::TokenFromRequestError;
use std::sync::Arc;

pub(crate) type DbPool = Box<dyn db::repo::Repo>;

//FIXME: Do not clone Context on every request
pub(crate) struct Context {
    pub(crate) pool: DbPool,
    pub(crate) cfg: Arc<cfg::Config>,
    //pub(crate) access_control: crate::security::AccessCheckService<'static>,
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for Context {
    type Error = TokenFromRequestError;

    fn from_request(
        request: &'a rocket::request::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let factory: rocket::State<ContextFactory> = request
            .guard::<rocket::State<ContextFactory>>()
            .expect("State<ContextFactory> missing");

        rocket::Outcome::Success(Context {
            pool: factory.pool.clone(),
            cfg: factory.cfg.clone(),
            //access_control: AccessCheckService::from_request(request)?.upgrade_static(),
        })
    }
}

pub(crate) struct ContextFactory {
    pub(crate) pool: DbPool,
    pub(crate) cfg: Arc<cfg::Config>,
}

impl ContextFactory {
    pub(crate) fn create_context_unrestricted(&self) -> Context {
        Context {
            pool: self.pool.clone(),
            cfg: self.cfg.clone(),
        }
    }
}

impl juniper::Context for Context {}
