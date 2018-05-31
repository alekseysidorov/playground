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

pub struct MyConcreteEndpoint;
pub struct MyConcreteEndpoint2;
pub struct MyConcreteEndpoint3;

impl Endpoint for MyConcreteEndpoint {
    const NAME: &'static str = "foo";

    type Request = MyRequest;
    type Response = MyResponse;

    fn handle(
        &self,
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
    type Response = String;

    fn handle(
        &self,
        context: &ApiContextMut,
        request: Self::Request,
    ) -> Result<Self::Response, failure::Error> {
        let hash = exonum::crypto::hash(request.seed.as_bytes());
        let mut fork = context.blockchain.fork();
        {
            let mut index = exonum::storage::ListIndex::new("foo", &mut fork);
            index.push(hash);
        }
        context.blockchain.clone().merge(fork.into_patch())?;
        Ok(hash.to_string())
    }
}

fn api_builder(context: ApiContextMut) -> App<ApiContextMut> {
    App::with_state(context).prefix("builder").scope("api", |scope| {
        ApiBuilder::new(scope)
            .for_service("rustfest", |scope| {
                scope
                    .endpoint(MyConcreteEndpoint)
                    .endpoint_mut(MyConcreteEndpoint2)
            })
            .into_scope()
    })
}

fn api_aggregator(context: ApiContextMut) -> App<ApiContextMut> {
    let endpoints = ServiceApiAggregator::new()
        .endpoint(MyConcreteEndpoint)
        .endpoint_mut(MyConcreteEndpoint2)
        .endpoints();

    App::with_state(context).prefix("aggregator").scope("api", |scope| {
        scope.nested("rustfest", |mut scope| {
            for endpoint in endpoints {
                scope = scope.route(
                    endpoint.name,
                    endpoint.method.clone(),
                    move |request| {
                        (endpoint.handler)(request)
                    },
                );
            }
            scope
        })
    })
}

fn main() {
    exonum::helpers::init_logger().unwrap();

    let keypair = exonum::crypto::gen_keypair();

    let api_sender = exonum::node::ApiSender::new(futures::sync::mpsc::channel(1).0);
    let db = exonum::storage::MemoryDB::new();
    let blockchain = Blockchain::new(db, vec![], keypair.0, keypair.1, api_sender);
    let context = ApiContextMut::new(blockchain);

    server::new(move || {
        vec![
            api_builder(context.clone()),
            api_aggregator(context.clone()),
        ]
    }).bind("localhost:8080")
        .unwrap()
        .run()
}
