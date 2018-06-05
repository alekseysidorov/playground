use serde::de::DeserializeOwned;
use serde::Serialize;

use std::ops::Deref;

use exonum::blockchain::Blockchain;

use actix_backend;
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

pub trait ServiceApiBackend: Sized {
    type Handler;

    fn endpoint<S, Q, I, R, F, E>(&mut self, name: &'static str, e: E) -> &mut Self
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

    fn raw_handler(&mut self, handler: Self::Handler) -> &mut Self;
}

#[derive(Default)]
pub struct ServiceApiBuilder {
    pub web_backend: actix_backend::BackendBuilder,
}

impl ServiceApiBuilder {
    pub fn endpoint<S, Q, I, R, F, E>(&mut self, name: &'static str, e: E) -> &mut Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        F: for<'r> Fn(&'r S, Q) -> R + 'static + Clone,
        E: Into<TypedFn<S, Q, I, R, F>>,
        actix_backend::RequestHandler: From<NamedFn<S, Q, I, R, F>>,
    {
        self.web_backend.endpoint(name, e);
        self
    }
}

#[derive(Default)]
pub struct ServiceApiInitializer {
    pub public_api_builder: ServiceApiBuilder,
}

impl ServiceApiInitializer {
    pub fn public_api(&mut self) -> &mut ServiceApiBuilder {
        &mut self.public_api_builder
    }
}

pub trait Service {
    fn initialize_api(&self, initializer: &mut ServiceApiInitializer);
}
