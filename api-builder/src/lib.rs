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

pub trait ServiceApi {
    fn endpoint<E: Endpoint>(self, endpoint: E) -> Self;
    fn endpoint_mut<E: EndpointMut>(self, endpoint: E) -> Self;
}
