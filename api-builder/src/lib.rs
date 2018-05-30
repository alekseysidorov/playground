extern crate actix_web;
extern crate serde;
extern crate serde_json;
extern crate failure;

use actix_web::{Scope, Query, FromRequest};
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::io;

#[derive(Clone)]
pub struct ApiContext {
    pub name: String,
}

#[derive(Clone)]
pub struct ApiContextMut {
    pub count: u64,
}

pub enum ApiScope {
    Public,
    Internal,
}

pub struct EndpointIdentifier {
    pub name: &'static str,
}

pub trait ReadEndpoint: 'static {
    const ID: EndpointIdentifier;

    type Request: DeserializeOwned;
    type Response: Serialize;

    fn handle(
        &self,
        context: &ApiContext,
        request: &Self::Request,
    ) -> Result<Self::Response, failure::Error>;
}

pub trait WriteEndpoint {
    const ID: EndpointIdentifier;

    type Request: DeserializeOwned;
    type Response: Serialize;

    fn handle(
        &self,
        context: &ApiContextMut,
        request: &Self::Request,
    ) -> Result<Self::Response, failure::Error>;
}

pub struct ApiBuilder {
    scope: Scope<ApiContext>,
}

impl ApiBuilder {
    pub fn new(scope: Scope<ApiContext>) -> ApiBuilder {
        ApiBuilder {
            scope: Scope::new(),
        }
    }

    pub fn for_service<F>(mut self, name: &str, builder: F) -> Self
    where
        F: FnOnce(ServiceApiBuilder) -> ServiceApiBuilder,
    {
        let f = |scope: Scope<ApiContext>| -> Scope<ApiContext> {
            builder(ServiceApiBuilder { scope }).scope
        };
        self.scope = self.scope.nested(name, f);
        self
    }

    pub fn into_scope(self) -> Scope<ApiContext> {
        self.scope
    }
}

pub struct ServiceApiBuilder {
    scope: Scope<ApiContext>,
}

impl ServiceApiBuilder {
    pub fn read_endpoint<E: ReadEndpoint>(mut self, endpoint: E) -> ServiceApiBuilder {
        let index = move |request: actix_web::HttpRequest<ApiContext>| -> actix_web::Result<String> {
            let state = request.state();
            let query = Query::from_request(&request, &())?;
            let response = endpoint.handle(state, &query)?;
            serde_json::to_string(&response).map_err(From::from)
        };

        self.scope = self.scope.route(E::ID.name, actix_web::http::Method::GET, index);
        self
    }

    pub fn read_closure<P, R, F>(mut self, name: &str, closure: F) -> ServiceApiBuilder 
        where P: DeserializeOwned,
            R: Serialize,
            F: Fn(&ApiContext, &P) -> Result<R, failure::Error> + 'static
    {
        let index = move |request: actix_web::HttpRequest<ApiContext>| -> actix_web::Result<String> {
            let state = request.state();
            let query = Query::from_request(&request, &())?;
            let response = closure(state, &query)?;
            serde_json::to_string(&response).map_err(From::from)
        };

        self.scope = self.scope.route(name, actix_web::http::Method::GET, index);
        self
    }
}
