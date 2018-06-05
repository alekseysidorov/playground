use actix_web::{self, AsyncResponder, FromRequest, HttpMessage, HttpRequest, HttpResponse, Query};
use futures::{Future, IntoFuture};
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::ops::Deref;

use exonum::blockchain::Blockchain;

use error;
use {NamedFn, TypedFn};

#[derive(Debug, Clone)]
pub struct ServiceApiContext {
    pub blockchain: Blockchain,
}

#[derive(Debug, Clone)]
pub struct ServiceApiContextMut {
    pub inner: ServiceApiContext,
}

impl ServiceApiContextMut {
    pub fn new(blockchain: Blockchain) -> ServiceApiContextMut {
        ServiceApiContextMut {
            inner: ServiceApiContext { blockchain },
        }
    }
}

impl Deref for ServiceApiContextMut {
    type Target = ServiceApiContext;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait Service {}

pub trait ServiceApiBackend2: Sized {
    type Handler;

    fn endpoint<S, Q, I, R, F, E>(self, name: &'static str, e: E) -> Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        F: for<'r> Fn(&'r S, Q) -> R + 'static + Clone,
        E: Into<TypedFn<S, Q, I, R, F>>,
        Self::Handler: From<NamedFn<S, Q, I, R, F>>,
    {
        let named_fn = NamedFn {
            name,
            inner: e.into(),
        };
        self.raw_handler(Self::Handler::from(named_fn))
    }

    fn raw_handler(self, handler: Self::Handler) -> Self;
}

pub struct ServiceBuilder {}

pub struct ServiceInitializer;
