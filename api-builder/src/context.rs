use exonum::blockchain::Blockchain;
use exonum::storage::Snapshot;

use std::ops::Deref;

pub struct ApiContext {
    blockchain: Blockchain
}

pub struct ApiContextMut {
    inner: ApiContext
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