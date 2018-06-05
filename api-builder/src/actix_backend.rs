use actix_web::{self, AsyncResponder, FromRequest, HttpMessage, HttpRequest, HttpResponse, Query};
use futures::{Future, IntoFuture};
use serde::de::DeserializeOwned;
use serde::Serialize;

use error;
use service::{ServiceApiBackend, ServiceApiContext, ServiceApiContextMut};
use NamedFn;

pub type RawHandler = Fn(HttpRequest<ServiceApiContextMut>)
    -> Box<Future<Item = HttpResponse, Error = actix_web::Error>>;

pub struct RequestHandler {
    pub name: &'static str,
    pub method: actix_web::http::Method,
    pub inner: Box<RawHandler>,
}

impl<Q, I, F> From<NamedFn<ServiceApiContext, Q, I, Result<I, error::Error>, F>> for RequestHandler
where
    F: for<'r> Fn(&'r ServiceApiContext, Q) -> Result<I, error::Error> + 'static,
    Q: DeserializeOwned + 'static,
    I: Serialize + 'static,
{
    fn from(f: NamedFn<ServiceApiContext, Q, I, Result<I, error::Error>, F>) -> Self {
        let handler = f.inner.f;
        let index = move |request: HttpRequest<ServiceApiContextMut>|
         -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {
            let to_response = |request: HttpRequest<ServiceApiContextMut>|
             -> Result<HttpResponse, actix_web::Error> {
                let context = request.state();
                let query: Query<Q> = Query::from_request(&request, &())?;
                let value = handler(context, query.into_inner())?;
                Ok(HttpResponse::Ok().json(value))
            };

            Box::new(to_response(request).into_future())
        };

        RequestHandler {
            name: f.name,
            method: actix_web::http::Method::GET,
            inner: Box::new(index) as Box<RawHandler>,
        }
    }
}

impl<Q, I, F> From<NamedFn<ServiceApiContextMut, Q, I, Result<I, error::Error>, F>>
    for RequestHandler
where
    F: for<'r> Fn(&'r ServiceApiContextMut, Q) -> Result<I, error::Error> + 'static + Clone,
    Q: DeserializeOwned + 'static,
    I: Serialize + 'static,
{
    fn from(f: NamedFn<ServiceApiContextMut, Q, I, Result<I, error::Error>, F>) -> Self {
        let handler = f.inner.f;
        let index = move |request: HttpRequest<ServiceApiContextMut>|
         -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {
            let handler = handler.clone();
            let context = request.state().clone();
            request.json().from_err().and_then(move |query: Q| {
                let value = handler(&context, query)?;
                Ok(HttpResponse::Ok().json(value))
            }).responder()
        };

        RequestHandler {
            name: f.name,
            method: actix_web::http::Method::POST,
            inner: Box::new(index) as Box<RawHandler>,
        }
    }
}

#[derive(Default)]
pub struct BackendBuilder {
    handlers: Vec<RequestHandler>,
}

impl ServiceApiBackend for BackendBuilder {
    type Handler = RequestHandler;

    fn raw_handler(&mut self, handler: Self::Handler) -> &mut Self {
        self.handlers.push(handler);
        self
    }
}

impl BackendBuilder {
    pub fn new() -> BackendBuilder {
        BackendBuilder::default()
    }

    pub fn finish(self) -> Vec<RequestHandler> {
        self.handlers
    }
}
