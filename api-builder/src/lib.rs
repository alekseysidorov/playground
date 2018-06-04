extern crate actix_web;
extern crate exonum;
extern crate failure;
extern crate futures;
extern crate serde;
extern crate serde_json;

use context::{ApiContext, ApiContextMut};

pub mod backend;
pub mod context;
pub mod error;

pub type Endpoint<Q, R> = for<'r> fn(&'r ApiContext, Q) -> Result<R, error::Error>;
pub type EndpointMut<Q, R> = for<'r> fn(&'r ApiContextMut, Q) -> Result<R, error::Error>;

pub enum EndpointKind<Q, R>
{
    Immutable(Endpoint<Q, R>),
    Mutable(EndpointMut<Q, R>)
}