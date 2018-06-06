extern crate actix_web;
extern crate exonum;
extern crate failure;
extern crate futures;
extern crate serde;
extern crate serde_json;

use futures::Future;

use service::{ServiceApiContext, ServiceApiContextMut};

pub mod actix_backend;
pub mod error;
pub mod service;

pub struct TypedFn<S, Q, I, R, F> {
    f: F,
    _context_type: ::std::marker::PhantomData<S>,
    _query_type: ::std::marker::PhantomData<Q>,
    _item_type: ::std::marker::PhantomData<I>,
    _result_type: ::std::marker::PhantomData<R>,
}

pub struct NamedFn<S, Q, I, R, F> {
    pub name: &'static str,
    pub inner: TypedFn<S, Q, I, R, F>,
}

impl<Q, I, F> From<F> for TypedFn<ServiceApiContext, Q, I, Result<I, error::Error>, F>
where
    F: for<'r> Fn(&'r ServiceApiContext, Q) -> Result<I, error::Error>,
{
    fn from(f: F) -> Self {
        TypedFn {
            f,
            _context_type: ::std::marker::PhantomData,
            _query_type: ::std::marker::PhantomData,
            _item_type: ::std::marker::PhantomData,
            _result_type: ::std::marker::PhantomData,
        }
    }
}

impl<Q, I, F> From<F> for TypedFn<ServiceApiContextMut, Q, I, Result<I, error::Error>, F>
where
    F: for<'r> Fn(&'r ServiceApiContextMut, Q) -> Result<I, error::Error>,
{
    fn from(f: F) -> Self {
        TypedFn {
            f,
            _context_type: ::std::marker::PhantomData,
            _query_type: ::std::marker::PhantomData,
            _item_type: ::std::marker::PhantomData,
            _result_type: ::std::marker::PhantomData,
        }
    }
}

impl<Q, I, F> From<F>
    for TypedFn<ServiceApiContext, Q, I, Box<Future<Item = I, Error = error::Error>>, F>
where
    F: for<'r> Fn(&'r ServiceApiContext, Q) -> Box<Future<Item = I, Error = error::Error>>,
{
    fn from(f: F) -> Self {
        TypedFn {
            f,
            _context_type: ::std::marker::PhantomData,
            _query_type: ::std::marker::PhantomData,
            _item_type: ::std::marker::PhantomData,
            _result_type: ::std::marker::PhantomData,
        }
    }
}

impl<Q, I, F> From<F>
    for TypedFn<ServiceApiContextMut, Q, I, Box<Future<Item = I, Error = error::Error>>, F>
where
    F: for<'r> Fn(&'r ServiceApiContextMut, Q) -> Box<Future<Item = I, Error = error::Error>>,
{
    fn from(f: F) -> Self {
        TypedFn {
            f,
            _context_type: ::std::marker::PhantomData,
            _query_type: ::std::marker::PhantomData,
            _item_type: ::std::marker::PhantomData,
            _result_type: ::std::marker::PhantomData,
        }
    }
}
