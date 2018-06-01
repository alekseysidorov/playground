use actix_web::pred::*;
use actix_web::{self, AsyncResponder, FromRequest, HttpMessage, HttpRequest, HttpResponse, Query};
use futures::{Future, IntoFuture};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

use context::{ApiContext, ApiContextMut};
use error;
use {Endpoint, EndpointMut, EndpointMutSpec, EndpointSpec};

pub type RequestHandler =
    Fn(HttpRequest<ApiContextMut>) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>>;

pub struct EndpointHandler {
    pub name: &'static str,
    pub method: actix_web::http::Method,
    pub handler: Box<RequestHandler>,
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

    fn endpoint<Q, R, F, E>(self, endpoint: E) -> Self
    where
        Q: DeserializeOwned + 'static,
        R: Serialize + 'static,
        F: 'static,
        for<'r> F: Fn(&'r ApiContext, Q) -> Result<R, error::Error>,
        E: Into<EndpointSpec<Q, R, F>>;
    fn endpoint_mut<Q, R, F, E>(self, endpoint: E) -> Self
    where
        Q: DeserializeOwned + 'static,
        R: Serialize + 'static,
        F: 'static + Clone,
        for<'r> F: Fn(&'r ApiContextMut, Q) -> Result<R, error::Error>,
        E: Into<EndpointMutSpec<Q, R, F>>;    
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

    fn endpoint<Q, R, F, E>(mut self, endpoint: E) -> Self
    where
        Q: DeserializeOwned + 'static,
        R: Serialize + 'static,
        F: 'static,
        for<'r> F: Fn(&'r ApiContext, Q) -> Result<R, error::Error>,
        E: Into<EndpointSpec<Q, R, F>>,
    {
        let spec = endpoint.into();
        self.endpoints.push(spec.into());
        self
    }

    fn endpoint_mut<Q, R, F, E>(mut self, endpoint: E) -> Self 
        where
        Q: DeserializeOwned + 'static,
        R: Serialize + 'static,
        F: 'static + Clone,
        for<'r> F: Fn(&'r ApiContextMut, Q) -> Result<R, error::Error>,
        E: Into<EndpointMutSpec<Q, R, F>>
    {
        let spec = endpoint.into();
        self.endpoints.push(spec.into());
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

impl<Q, R, F> From<EndpointSpec<Q, R, F>> for EndpointHandler
where
    Q: DeserializeOwned + 'static,
    R: Serialize + 'static,
    F: 'static,
    for<'r> F: Fn(&'r ApiContext, Q) -> Result<R, error::Error>,
{
    fn from(spec: EndpointSpec<Q, R, F>) -> Self {
        let name = spec.name;
        let index = move |request: HttpRequest<ApiContextMut>| -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {
            let to_response = |request: HttpRequest<ApiContextMut>| -> Result<HttpResponse, actix_web::Error> {
                let context = request.state();
                let query: Query<Q> = Query::from_request(&request, &())?;
                let value = (spec.handler)(context, query.into_inner())?;
                Ok(HttpResponse::Ok().json(value))
            };

            Box::new(to_response(request).into_future())
        };

        EndpointHandler {
            name: name,
            handler: Box::new(index),
            method: actix_web::http::Method::GET,
        }
    }
}

impl<Q, R, F> From<EndpointMutSpec<Q, R, F>> for EndpointHandler
where
    Q: DeserializeOwned + 'static,
    R: Serialize + 'static,
    F: 'static + Clone,
    for<'r> F: Fn(&'r ApiContextMut, Q) -> Result<R, error::Error>,
{
    fn from(spec: EndpointMutSpec<Q, R, F>) -> Self {
        let name = spec.name;
        let index = move |request: HttpRequest<ApiContextMut>| -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {
            let context = request.state().clone();
            let handler = spec.handler.clone();
            request.json().from_err().and_then(move |query: Q| {
                let value = (handler)(&context, query)?;
                Ok(HttpResponse::Ok().json(value))
            }).responder()
        };

        EndpointHandler {
            name: name,
            handler: Box::new(index),
            method: actix_web::http::Method::GET,
        }
    }
}
