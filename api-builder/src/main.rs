extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate api_builder;
extern crate exonum;
extern crate failure;
extern crate futures;
extern crate serde_urlencoded;

use actix_web::*;
use api_builder::backend::*;
use api_builder::context::{ApiContext, ApiContextMut};

use exonum::blockchain::Blockchain;

#[derive(Debug, Deserialize)]
pub struct MyRequest {
    pub name: String,
    pub count: u64,
}

#[derive(Debug, Deserialize)]
pub struct Seed {
    pub seed: String,
}

#[derive(Debug, Serialize)]
pub struct MyResponse {
    name: String,
    value: u64,
}

pub trait MyServiceApi {
    type Error;

    fn foo(&self, request: MyRequest) -> Result<MyResponse, failure::Error>;

    fn baz(&self, request: (String, String)) -> Result<String, failure::Error>;
}

pub trait MyServiceApiMut {
    type Error;

    fn bar(&self, Seed) -> Result<(u64, exonum::crypto::Hash), failure::Error>;
}

impl MyServiceApi for ApiContext {
    type Error = failure::Error;

    fn foo(&self, request: MyRequest) -> Result<MyResponse, Self::Error> {
        Ok(MyResponse {
            name: request.name,
            value: request.count * 2,
        })
    }

    fn baz(&self, request: (String, String)) -> Result<String, Self::Error> {
        Ok(format!("first is {}, second id {}", request.0, request.1))
    }
}

impl MyServiceApiMut for ApiContextMut {
    type Error = failure::Error;

    fn bar(&self, request: Seed) -> Result<(u64, exonum::crypto::Hash), Self::Error>
    {
        let hash = exonum::crypto::hash(request.seed.as_bytes());
        let mut fork = self.blockchain.fork();
        let len = {
            let mut index = exonum::storage::ListIndex::new("foo", &mut fork);
            index.push(hash);
            index.len()
        };
        self.blockchain.clone().merge(fork.into_patch())?;
        Ok((len, hash))
    }
}

fn api_aggregator(context: ApiContextMut) -> App<ApiContextMut> {
    let endpoints = ServiceApiWebBackend::new()
        .method("foo", MyServiceApi::foo)
        .method("baz", MyServiceApi::baz)
        .method_mut("bar", MyServiceApiMut::bar)
        .endpoints();

    App::with_state(context)
        .scope("api", |scope| {
            scope.nested("rustfest", |mut scope| {
                for endpoint in endpoints {
                    scope = scope.route(endpoint.name, endpoint.method.clone(), move |request| {
                        (endpoint.handler)(request)
                    });
                }
                scope
            })
        })
}

fn main() {
    exonum::helpers::init_logger().unwrap();

    let keypair = exonum::crypto::gen_keypair();
    let api_sender = exonum::node::ApiSender::new(futures::sync::mpsc::channel(1).0);
    let db = exonum::storage::RocksDB::open("/tmp/actix", &exonum::storage::DbOptions::default())
        .unwrap();
    let blockchain = Blockchain::new(db, vec![], keypair.0, keypair.1, api_sender);

    server::new(move || api_aggregator(ApiContextMut::new(blockchain.clone())))
        .bind("localhost:8080")
        .unwrap()
        .run()
}
