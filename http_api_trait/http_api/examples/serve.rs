use http_api_derive::{http_api, http_api_endpoint, FromUrlQuery};
use http_api::warp_backend::Error;
use serde_derive::{Deserialize, Serialize};

use std::{
    net::SocketAddr,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

#[derive(Debug, FromUrlQuery, Deserialize, Serialize)]
struct Query {
    first: String,
    second: u64,
}

#[http_api(warp = "serve_ping_interface")]
trait PingInterface {
    #[http_api_endpoint(method = "get")]
    fn get(&self) -> Result<Query, Error>;
    #[http_api_endpoint(method = "get")]
    fn check(&self, query: Query) -> Result<bool, Error>;
    #[http_api_endpoint(method = "post")]
    fn set_value(&self, param: Query) -> Result<(), Error>;
    #[http_api_endpoint(method = "post")]
    fn increment(&self) -> Result<(), Error>;
}

#[derive(Debug, Default)]
struct ServiceInner {
    first: String,
    second: u64,
}

#[derive(Clone, Default)]
struct ServiceImpl(Arc<RwLock<ServiceInner>>);

impl ServiceImpl {
    fn new() -> Self {
        Self::default()
    }

    fn read(&self) -> RwLockReadGuard<ServiceInner> {
        self.0.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<ServiceInner> {
        self.0.write().unwrap()
    }
}

impl PingInterface for ServiceImpl {
    fn get(&self) -> Result<Query, Error> {
        let inner = self.read();
        Ok(Query {
            first: inner.first.clone(),
            second: inner.second,
        })
    }

    fn check(&self, query: Query) -> Result<bool, Error> {
        let inner = self.read();
        Ok(inner.first == query.first && inner.second == query.second)
    }

    fn set_value(&self, param: Query) -> Result<(), Error> {
        let mut inner = self.write();
        inner.first = param.first;
        inner.second = param.second;
        Ok(())
    }

    fn increment(&self) -> Result<(), Error> {
        self.write().second += 1;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    serve_ping_interface(ServiceImpl::new(), addr).await
}
