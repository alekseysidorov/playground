extern crate actix_web;
extern crate serde;
extern crate serde_json;
extern crate failure;
extern crate exonum;

use actix_web::{Scope, Query, FromRequest, Responder};
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

    fn handle(&self, context: &context::ApiContext, request: &Self::Request) -> Result<Self::Response, error::Error>;
}

pub trait EndpointMut: 'static {
    const NAME: &'static str;

    type Request: DeserializeOwned;
    type Response: Serialize;

    fn handle(&self, context: &context::ApiContextMut, request: &Self::Request) -> Result<Self::Response, error::Error>;
}

pub trait ServiceApi {
    fn endpoint<E: Endpoint>(self, endpoint: E) -> Self;
    fn endpoint_mut<E: EndpointMut>(self, endpoint: E) -> Self;
}
