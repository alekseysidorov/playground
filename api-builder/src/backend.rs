use actix_web::{self, AsyncResponder, FromRequest, HttpMessage, HttpRequest, HttpResponse, Query};
use failure;
use futures::{Future, IntoFuture};
use serde::de::DeserializeOwned;
use serde::Serialize;

use context::{ApiContext, ApiContextMut};
use EndpointKind;

pub type WebRequestHandler =
    Fn(HttpRequest<ApiContextMut>) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>>;

pub struct EndpointHandler {
    pub name: &'static str,
    pub method: actix_web::http::Method,
    pub handler: Box<WebRequestHandler>,
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

    fn endpoint<Q, R>(
        self,
        name: &'static str,
        f: for<'r> fn(&'r ApiContext, Q) -> Result<R, failure::Error>,
    ) -> Self
    where
        Q: DeserializeOwned + 'static,
        R: Serialize + 'static;

    fn endpoint_mut<Q, R>(
        self,
        name: &'static str,
        f: for<'r> fn(&'r ApiContextMut, Q) -> Result<R, failure::Error>,
    ) -> Self
    where
        Q: DeserializeOwned + 'static,
        R: Serialize + 'static;

    fn raw_handler(
        self,
        name: &'static str,
        method: Self::Method,
        handler: Self::RawHandler,
    ) -> Self;
}

impl ServiceApiBackend for ServiceApiWebBackend {
    type RawHandler = Box<WebRequestHandler>;
    type Method = actix_web::http::Method;

    fn endpoint<Q, R>(
        mut self,
        name: &'static str,
        f: for<'r> fn(&'r ApiContext, Q) -> Result<R, failure::Error>,
    ) -> Self
    where
        Q: DeserializeOwned + 'static,
        R: Serialize + 'static,
    {
        self.endpoints
            .push(EndpointHandler::new(name, EndpointKind::Immutable(f)));
        self
    }

    fn endpoint_mut<Q, R>(
        mut self,
        name: &'static str,
        f: for<'r> fn(&'r ApiContextMut, Q) -> Result<R, failure::Error>,
    ) -> Self
    where
        Q: DeserializeOwned + 'static,
        R: Serialize + 'static,
    {
        self.endpoints
            .push(EndpointHandler::new(name, EndpointKind::Mutable(f)));
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

impl EndpointHandler {
    pub fn new<Q, R>(name: &'static str, kind: EndpointKind<Q, R>) -> EndpointHandler 
        where
        Q: DeserializeOwned + 'static,
        R: Serialize + 'static,
    {
        let (method, handler) = match kind {
            EndpointKind::Immutable(f) => {
                let index = move |request: HttpRequest<ApiContextMut>| -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {
                    let to_response = |request: HttpRequest<ApiContextMut>| -> Result<HttpResponse, actix_web::Error> {
                        let context = request.state();
                        let query: Query<Q> = Query::from_request(&request, &())?;
                        let value = f(context, query.into_inner())?;
                        Ok(HttpResponse::Ok().json(value))
                    };

                    Box::new(to_response(request).into_future())
                };
                let index = Box::new(index) as Box<WebRequestHandler>;
                (actix_web::http::Method::GET, index)
            }
            EndpointKind::Mutable(handler) => {
                let index = move |request: HttpRequest<ApiContextMut>| -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {
                    let context = request.state().clone();
                    request.json().from_err().and_then(move |query: Q| {
                        let value = (handler)(&context, query)?;
                        Ok(HttpResponse::Ok().json(value))
                    }).responder()
                };
                let index = Box::new(index) as Box<WebRequestHandler>;
                (actix_web::http::Method::POST, index)
            }
        };

        EndpointHandler {
            name,
            method,
            handler,
        }
    }
}
