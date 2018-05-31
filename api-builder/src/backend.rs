use actix_web::pred::*;
use actix_web::{self, HttpRequest, Query, Scope, FromRequest};
use serde_json;

use context::{ApiContext, ApiContextMut};
use {Endpoint, EndpointMut, ServiceApi};

impl ServiceApi for Scope<ApiContextMut> {
    fn endpoint<E: Endpoint>(self, endpoint: E) -> Self {
        let index = move |request: HttpRequest<ApiContextMut>| -> actix_web::Result<String> {
            let context = request.state();
            let query = Query::from_request(&request, &())?;
            let response = endpoint.handle(context, &query)?;
            serde_json::to_string(&response).map_err(From::from)
        };

        self.route(E::NAME, actix_web::http::Method::GET, index)
    }

    fn endpoint_mut<E: EndpointMut>(self, endpoint: E) -> Self {
        let index = move |request: HttpRequest<ApiContextMut>| -> actix_web::Result<String> {
            let context = request.state();
            let query = Query::from_request(&request, &())?;
            let response = endpoint.handle(context, &query)?;
            serde_json::to_string(&response).map_err(From::from)
        };

        self.route(E::NAME, actix_web::http::Method::GET, index)
    }
}

pub struct ServiceApiBuilder {
    actix_backend: Scope<ApiContextMut>
}

impl ServiceApi for ServiceApiBuilder {
    fn endpoint<E: Endpoint>(mut self, endpoint: E) -> Self {
        self.actix_backend = self.actix_backend.endpoint(endpoint);
        self
    }

    fn endpoint_mut<E: EndpointMut>(mut self, endpoint: E) -> Self {
        self.actix_backend = self.actix_backend.endpoint_mut(endpoint);
        self
    }    
}
