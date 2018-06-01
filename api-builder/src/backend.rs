use actix_web::pred::*;
use actix_web::{self, FromRequest, HttpRequest, Query, Scope};
use serde_json;

use context::{ApiContext, ApiContextMut};
use {Endpoint, EndpointMut, ServiceApi};

pub struct EndpointHandler {
    pub name: &'static str,
    pub method: actix_web::http::Method,
    pub handler: Box<Fn(HttpRequest<ApiContextMut>) -> actix_web::Result<String>>,
}

pub struct ServiceApiAggregator {
    endpoints: Vec<EndpointHandler>,
}

impl ServiceApiAggregator {
    pub fn new() -> ServiceApiAggregator {
        ServiceApiAggregator {
            endpoints: Vec::new(),
        }
    }

    pub fn endpoints(self) -> Vec<EndpointHandler> {
        self.endpoints
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
        self.endpoints.push(EndpointHandler {
            name: E::NAME,
            handler: Box::new(index),
            method: actix_web::http::Method::GET,
        });
        self
    }

    fn endpoint_mut<E: EndpointMut>(mut self, endpoint: E) -> Self {
        let index = move |request: HttpRequest<ApiContextMut>| -> actix_web::Result<String> {
            let context = request.state();
            let query = Query::<E::Request>::from_request(&request, &())?;
            let response = endpoint.handle(context, query.into_inner())?;
            serde_json::to_string(&response).map_err(From::from)
        };
        self.endpoints.push(EndpointHandler {
            name: E::NAME,
            handler: Box::new(index),
            method: actix_web::http::Method::POST,
        });
        self
    }
}
