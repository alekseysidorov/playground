extern crate actix_web;
extern crate exonum;
extern crate failure;
extern crate futures;
extern crate serde;
extern crate serde_json;

use actix_web::{FromRequest, Query, Responder, Scope};
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::io;

pub mod backend;
pub mod context;
pub mod error;

pub trait Endpoint: 'static {
    const NAME: &'static str;

    type Request: DeserializeOwned;
    type Response: Serialize;

    fn handle(
        context: &context::ApiContext,
        request: Self::Request,
    ) -> Result<Self::Response, error::Error>;
}

pub trait EndpointMut: 'static {
    const NAME: &'static str;

    type Request: DeserializeOwned;
    type Response: Serialize;

    fn handle(
        context: &context::ApiContextMut,
        request: Self::Request,
    ) -> Result<Self::Response, error::Error>;
}

pub struct EndpointSpec<Q, R, F>
where
    Q: DeserializeOwned,
    R: Serialize,
    F: Fn(&context::ApiContext, Q) -> Result<R, error::Error>,
{
    pub name: &'static str,
    pub handler: F,
    _query: ::std::marker::PhantomData<Q>,
    _response: ::std::marker::PhantomData<R>,
}

pub struct EndpointMutSpec<Q, R, F>
where
    Q: DeserializeOwned,
    R: Serialize,
    F: Fn(&context::ApiContextMut, Q) -> Result<R, error::Error>,
{
    pub name: &'static str,
    pub handler: F,
    _query: ::std::marker::PhantomData<Q>,
    _response: ::std::marker::PhantomData<R>,
}

impl<E: Endpoint> From<E>
    for EndpointSpec<
        E::Request,
        E::Response,
        for<'r> fn(&'r context::ApiContext, E::Request) -> Result<E::Response, failure::Error>,
    >
{
    fn from(_: E) -> Self {
        EndpointSpec {
            name: E::NAME,
            handler: E::handle,
            _query: ::std::marker::PhantomData,
            _response: ::std::marker::PhantomData,
        }
    }
}

impl<E: EndpointMut> From<E>
    for EndpointMutSpec<
        E::Request,
        E::Response,
        for<'r> fn(&'r context::ApiContextMut, E::Request) -> Result<E::Response, failure::Error>,
    >
{
    fn from(_: E) -> Self {
        EndpointMutSpec {
            name: E::NAME,
            handler: E::handle,
            _query: ::std::marker::PhantomData,
            _response: ::std::marker::PhantomData,
        }
    }
}
