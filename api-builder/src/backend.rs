use actix_web::pred::*;
use actix_web::{self, FromRequest, HttpRequest, Query, Scope};
use serde_json;

use context::{ApiContext, ApiContextMut};
use {Endpoint, EndpointMut, ServiceApi};

impl ServiceApi for Scope<ApiContextMut> {
    fn endpoint<E: Endpoint>(self, endpoint: E) -> Self {
        let index = move |request: HttpRequest<ApiContextMut>| -> actix_web::Result<String> {
            let context = request.state();
            let query = Query::<E::Request>::from_request(&request, &())?;
            let response = endpoint.handle(context, query.into_inner())?;
            serde_json::to_string(&response).map_err(From::from)
        };

        self.route(E::NAME, actix_web::http::Method::GET, index)
    }

    fn endpoint_mut<E: EndpointMut>(self, endpoint: E) -> Self {
        let index = move |request: HttpRequest<ApiContextMut>| -> actix_web::Result<String> {
            let context = request.state();
            let query = Query::<E::Request>::from_request(&request, &())?;
            let response = endpoint.handle(context, query.into_inner())?;
            serde_json::to_string(&response).map_err(From::from)
        };

        self.route(E::NAME, actix_web::http::Method::GET, index)
    }
}

pub struct ApiBuilder {
    scope: Scope<ApiContextMut>,
}

impl ApiBuilder {
    pub fn new(scope: Scope<ApiContextMut>) -> ApiBuilder {
        ApiBuilder {
            scope: Scope::new(),
        }
    }

    pub fn for_service<F>(mut self, name: &str, f: F) -> Self
    where
        F: FnOnce(Scope<ApiContextMut>) -> Scope<ApiContextMut>,
    {
        self.scope = self.scope.nested(name, f);
        self
    }

    pub fn into_scope(self) -> Scope<ApiContextMut> {
        self.scope
    }
}

pub struct ServiceApiAggregator {
    endpoints: Vec<(
        &'static str,
        Box<Fn(HttpRequest<ApiContextMut>) -> actix_web::Result<String>>,
    )>,
    endpoints_mut: Vec<(
        &'static str,
        Box<Fn(HttpRequest<ApiContextMut>) -> actix_web::Result<String>>,
    )>,
}

impl ServiceApiAggregator {
    pub fn new() -> ServiceApiAggregator {
        ServiceApiAggregator {
            endpoints: Vec::new(),
            endpoints_mut: Vec::new(),
        }
    }

    pub fn endpoints(
        self,
    ) -> (
        Vec<(
            &'static str,
            Box<Fn(HttpRequest<ApiContextMut>) -> actix_web::Result<String>>,
        )>,
        Vec<(
            &'static str,
            Box<Fn(HttpRequest<ApiContextMut>) -> actix_web::Result<String>>,
        )>,
    ) {
        (self.endpoints, self.endpoints_mut)
    }
}

impl ServiceApi for ServiceApiAggregator {
    fn endpoint<E: Endpoint>(mut self, endpoint: E) -> Self {
        let index = move |request: HttpRequest<ApiContextMut>| -> actix_web::Result<String> {
            let context = request.state();
            let query = Query::<E::Request>::from_request(&request, &())?;
            let response = endpoint.handle(context, query.into_inner())?;
            serde_json::to_string(&response).map_err(From::from)
        };
        self.endpoints.push((E::NAME, Box::new(index)));
        self
    }

    fn endpoint_mut<E: EndpointMut>(mut self, endpoint: E) -> Self {
        let index = move |request: HttpRequest<ApiContextMut>| -> actix_web::Result<String> {
            let context = request.state();
            let query = Query::<E::Request>::from_request(&request, &())?;
            let response = endpoint.handle(context, query.into_inner())?;
            serde_json::to_string(&response).map_err(From::from)
        };
        self.endpoints.push((E::NAME, Box::new(index)));
        self
    }
}
