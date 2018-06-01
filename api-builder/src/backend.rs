use actix_web::pred::*;
use actix_web::{self, AsyncResponder, FromRequest, HttpMessage, HttpRequest, HttpResponse, Query};
use futures::{Future, IntoFuture};
use serde_json;

use context::{ApiContext, ApiContextMut};
use {Endpoint, EndpointMut};

pub type RequestHandler =
    Fn(HttpRequest<ApiContextMut>) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>>;

pub struct EndpointHandler {
    pub name: &'static str,
    pub method: actix_web::http::Method,
    pub handler: Box<RequestHandler>,
}

impl EndpointHandler {
    pub fn from_endpoint<E: Endpoint>(_: E) -> EndpointHandler {
        let index = move |request: HttpRequest<ApiContextMut>| -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {
            let to_response = |request: HttpRequest<ApiContextMut>| -> Result<HttpResponse, actix_web::Error> {
                let context = request.state();
                let query = Query::<E::Request>::from_request(&request, &())?;
                let value = E::handle(context, query.into_inner())?;
                Ok(HttpResponse::Ok().json(value))
            };

            Box::new(to_response(request).into_future())
        };

        EndpointHandler {
            name: E::NAME,
            handler: Box::new(index),
            method: actix_web::http::Method::GET,
        }
    }

    pub fn from_endpoint_mut<E: EndpointMut>(_: E) -> EndpointHandler {
        let index = move |request: HttpRequest<ApiContextMut>| -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {
            let context = request.state().clone();
            request.json().from_err().and_then(move |query: E::Request| {
                let value = E::handle(&context, query)?;
                Ok(HttpResponse::Ok().json(value))
            }).responder()
        };

        EndpointHandler {
            name: E::NAME,
            handler: Box::new(index),
            method: actix_web::http::Method::POST,
        }
    }
}

pub struct ServiceApiWebBackend {
    endpoints: Vec<EndpointHandler>,
}

impl ServiceApiWebBackend {
    pub fn new() -> ServiceApiWebBackend {
        ServiceApiWebBackend {
            endpoints: Vec::new(),
        }
    }

    pub fn endpoints(self) -> Vec<EndpointHandler> {
        self.endpoints
    }
}

pub trait ServiceApiBackend {
    type RawHandler;
    type Method;

    fn endpoint<E: Endpoint>(self, endpoint: E) -> Self;
    fn endpoint_mut<E: EndpointMut>(self, endpoint: E) -> Self;
    fn raw_handler(
        self,
        name: &'static str,
        method: Self::Method,
        handler: Self::RawHandler,
    ) -> Self;
}

impl ServiceApiBackend for ServiceApiWebBackend {
    type RawHandler = Box<RequestHandler>;
    type Method = actix_web::http::Method;

    fn endpoint<E: Endpoint>(mut self, endpoint: E) -> Self {
        self.endpoints
            .push(EndpointHandler::from_endpoint(endpoint));
        self
    }

    fn endpoint_mut<E: EndpointMut>(mut self, endpoint: E) -> Self {
        self.endpoints
            .push(EndpointHandler::from_endpoint_mut(endpoint));
        self
    }

    fn raw_handler(
        mut self,
        name: &'static str,
        method: Self::Method,
        handler: Self::RawHandler,
    ) -> Self {
        self.endpoints.push(EndpointHandler {
            name,
            method,
            handler,
        });
        self
    }
}
