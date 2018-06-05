use actix_web::{self, AsyncResponder, FromRequest, HttpMessage, HttpRequest, HttpResponse, Query};
use futures::{Future, IntoFuture};
use serde::de::DeserializeOwned;
use serde::Serialize;

use context::{ApiContext, ApiContextMut};
use error;
use {NamedFn, TypedFn};

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

    fn endpoint<S, Q, I, R, F, E>(self, name: &'static str, e: E) -> Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        F: for<'r> Fn(&'r S, Q) -> R + 'static + Clone,
        E: Into<TypedFn<S, Q, I, R, F>>,
        EndpointHandler: From<NamedFn<S, Q, I, R, F>>;

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

    fn endpoint<S, Q, I, R, F, E>(mut self, name: &'static str, f: E) -> Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        F: for<'r> Fn(&'r S, Q) -> R + 'static + Clone,
        E: Into<TypedFn<S, Q, I, R, F>>,
        EndpointHandler: From<NamedFn<S, Q, I, R, F>>,
    {
        let named_fn = NamedFn { name, inner: f.into() };
        self.endpoints.push(EndpointHandler::from(named_fn));
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

impl<Q, I, F> From<NamedFn<ApiContext, Q, I, Result<I, error::Error>, F>> for EndpointHandler
where
    F: for<'r> Fn(&'r ApiContext, Q) -> Result<I, error::Error> + 'static,
    Q: DeserializeOwned + 'static,
    I: Serialize + 'static,
{
    fn from(f: NamedFn<ApiContext, Q, I, Result<I, error::Error>, F>) -> Self {
        let handler = f.inner.f;
        let index = move |request: HttpRequest<ApiContextMut>| -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {
            let to_response = |request: HttpRequest<ApiContextMut>| -> Result<HttpResponse, actix_web::Error> {
                let context = request.state();
                let query: Query<Q> = Query::from_request(&request, &())?;
                let value = handler(context, query.into_inner())?;
                Ok(HttpResponse::Ok().json(value))
            };

            Box::new(to_response(request).into_future())
        };

        EndpointHandler {
            name: f.name,
            method: actix_web::http::Method::GET,
            handler: Box::new(index) as Box<WebRequestHandler>
        }
    }
}

impl<Q, I, F> From<NamedFn<ApiContextMut, Q, I, Result<I, error::Error>, F>> for EndpointHandler
where
    F: for<'r> Fn(&'r ApiContextMut, Q) -> Result<I, error::Error> + 'static + Clone,
    Q: DeserializeOwned + 'static,
    I: Serialize + 'static,
{
    fn from(f: NamedFn<ApiContextMut, Q, I, Result<I, error::Error>, F>) -> Self {
        let handler = f.inner.f;
        let index = move |request: HttpRequest<ApiContextMut>| -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {
            let handler = handler.clone();
            let context = request.state().clone();
            request.json().from_err().and_then(move |query: Q| {
                let value = handler(&context, query)?;
                Ok(HttpResponse::Ok().json(value))
            }).responder()
        };

        EndpointHandler {
            name: f.name,
            method: actix_web::http::Method::POST,
            handler: Box::new(index) as Box<WebRequestHandler>
        }
    }
}