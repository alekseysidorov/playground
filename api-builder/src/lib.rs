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

pub type Result<I> = ::std::result::Result<I, error::Error>;
pub type AsyncResult<I> = Box<Future<Item = I, Error = error::Error>>;

pub struct TypedFn<S, Q, I, R, F> {
    pub(crate) f: F,
    _context_type: ::std::marker::PhantomData<S>,
    _query_type: ::std::marker::PhantomData<Q>,
    _item_type: ::std::marker::PhantomData<I>,
    _result_type: ::std::marker::PhantomData<R>,
}

pub struct NamedFn<S, Q, I, R, F> {
    pub(crate) name: &'static str,
    pub(crate) inner: TypedFn<S, Q, I, R, F>,
}

impl<Q, I, F> From<F> for TypedFn<ServiceApiContext, Q, I, Result<I>, F>
where
    F: for<'r> Fn(&'r ServiceApiContext, Q) -> Result<I>,
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

impl<Q, I, F> From<F> for TypedFn<ServiceApiContextMut, Q, I, Result<I>, F>
where
    F: for<'r> Fn(&'r ServiceApiContextMut, Q) -> Result<I>,
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
    for TypedFn<ServiceApiContext, Q, I, AsyncResult<I>, F>
where
    F: for<'r> Fn(&'r ServiceApiContext, Q) -> AsyncResult<I>,
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
    for TypedFn<ServiceApiContextMut, Q, I, AsyncResult<I>, F>
where
    F: for<'r> Fn(&'r ServiceApiContextMut, Q) -> AsyncResult<I>,
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
