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
use api_builder::*;

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

#[derive(Clone)]
pub struct MyConcreteEndpoint;
#[derive(Clone)]
pub struct MyConcreteEndpoint2;
#[derive(Clone)]
pub struct MyConcreteEndpoint3;

impl Endpoint for MyConcreteEndpoint {
    const NAME: &'static str = "foo";

    type Request = MyRequest;
    type Response = MyResponse;

    fn handle(
        context: &ApiContext,
        request: Self::Request,
    ) -> Result<Self::Response, failure::Error> {
        Ok(MyResponse {
            name: request.name,
            value: request.count * 2,
        })
    }
}

impl EndpointMut for MyConcreteEndpoint2 {
    const NAME: &'static str = "bar";

    type Request = Seed;
    type Response = (u64, exonum::crypto::Hash);

    fn handle(
        context: &ApiContextMut,
        request: Self::Request,
    ) -> Result<Self::Response, failure::Error> {
        let hash = exonum::crypto::hash(request.seed.as_bytes());
        let mut fork = context.blockchain.fork();
        let len = {
            let mut index = exonum::storage::ListIndex::new("foo", &mut fork);
            index.push(hash);
            index.len()
        };
        context.blockchain.clone().merge(fork.into_patch())?;
        Ok((len, hash))
    }
}

fn api_aggregator(context: ApiContextMut) -> App<ApiContextMut> {
    let endpoints = ServiceApiWebBackend::new()
        .endpoint(MyConcreteEndpoint)
        .endpoint_mut(MyConcreteEndpoint2)
        .endpoints();

    App::with_state(context)
        .prefix("aggregator")
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
