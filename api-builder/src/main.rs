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

#[derive(Debug, Serialize)]
pub struct MyResponse(String);

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
        request: &Self::Request,
    ) -> Result<Self::Response, failure::Error> {
        unimplemented!();
    }
}

impl EndpointMut for MyConcreteEndpoint2 {
    const NAME: &'static str = "bar";

    type Request = ();
    type Response = String;

    fn handle(
        &self,
        context: &ApiContextMut,
        request: &Self::Request,
    ) -> Result<Self::Response, failure::Error> {
        unimplemented!();
    }
}

fn api_builder(context: ApiContextMut) -> App<ApiContextMut> {
    App::with_state(context).scope("api_builder", |scope| {
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
    let endpoints = ServiceApiAggregator::new(&context)
        .endpoint(MyConcreteEndpoint)
        .endpoint_mut(MyConcreteEndpoint2)
        .endpoints();

    App::with_state(context).scope("api_aggregator", |scope| {
        scope.nested("rustfest", |mut scope| {
            for endpoint in endpoints.0 {
                scope = scope.route(
                    endpoint.0,
                    actix_web::http::Method::GET,
                    move |request: HttpRequest<ApiContextMut>| -> actix_web::Result<String> {
                        endpoint.1(request)
                    },
                );
            }
            for endpoint_mut in endpoints.1 {
                scope = scope.route(
                    endpoint_mut.0,
                    actix_web::http::Method::POST,
                    move |request: HttpRequest<ApiContextMut>| -> actix_web::Result<String> {
                        endpoint_mut.1(request)
                    },
                );
            }
            scope
        })
    })
}

fn main() {
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
