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
