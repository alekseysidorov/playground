use exonum::blockchain::Blockchain;
use exonum::storage::Snapshot;

use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct ApiContext {
    pub blockchain: Blockchain
}

#[derive(Debug, Clone)]
pub struct ApiContextMut {
    pub inner: ApiContext
}

impl ApiContextMut {
    pub fn new(blockchain: Blockchain) -> ApiContextMut {
        ApiContextMut {
            inner: ApiContext {blockchain }
        }
    }
}

impl Deref for ApiContextMut {
    type Target = ApiContext;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}