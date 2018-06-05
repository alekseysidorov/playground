extern crate actix_web;
extern crate exonum;
extern crate failure;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate static_assert_macro;

use context::{ApiContext, ApiContextMut};

pub mod backend;
pub mod context;
pub mod error;

pub struct TypedEndpoint<S, Q, I, R, F>
{
    f: F,
    _context_type: ::std::marker::PhantomData<S>,
    _query_type: ::std::marker::PhantomData<Q>,
    _item_type: ::std::marker::PhantomData<I>,
    _result_type: ::std::marker::PhantomData<R>,
}

impl<Q, I, F> From<F> for TypedEndpoint<ApiContext, Q, I, Result<I, error::Error>, F>
where
    F: for<'r> Fn(&'r ApiContext, Q) -> Result<I, error::Error>,
{
    fn from(f: F) -> Self {
        TypedEndpoint {
            f,
            _context_type: ::std::marker::PhantomData,
            _query_type: ::std::marker::PhantomData,
            _item_type: ::std::marker::PhantomData,
            _result_type: ::std::marker::PhantomData,
        }
    }
}

impl<Q, I, F> From<F> for TypedEndpoint<ApiContextMut, Q, I, Result<I, error::Error>, F>
where
    F: for<'r> Fn(&'r ApiContextMut, Q) -> Result<I, error::Error>,
{
    fn from(f: F) -> Self {
        TypedEndpoint {
            f,
            _context_type: ::std::marker::PhantomData,
            _query_type: ::std::marker::PhantomData,
            _item_type: ::std::marker::PhantomData,
            _result_type: ::std::marker::PhantomData,
        }
    }
}
