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
use futures::Future;

use api_builder::service::{Service, ServiceApiContext, ServiceApiContextMut, ServiceApiInitializer};

use exonum::blockchain::Blockchain;

use std::sync::{Arc, Mutex};

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

    fn foo(&self, request: MyRequest) -> Result<MyResponse, Self::Error>;

    fn baz(&self, request: (String, String)) -> Result<String, Self::Error>;

    fn hello(&self, request: ()) -> Result<String, Self::Error>;

    fn hello_async(&self, request: ()) -> Box<Future<Item = String, Error = Self::Error>>;
}

pub trait MyServiceApiMut {
    type Error;

    fn bar(&self, Seed) -> Result<(u64, exonum::crypto::Hash), Self::Error>;

    fn bar_async(
        &self,
        Seed,
    ) -> Box<Future<Item = (u64, exonum::crypto::Hash), Error = Self::Error>>;
}

impl MyServiceApi for ServiceApiContext {
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

    fn hello(&self, _: ()) -> Result<String, failure::Error> {
        Ok("Hello Actix".to_owned())
    }

    fn hello_async(&self, _: ()) -> Box<Future<Item = String, Error = Self::Error>> {
        let future = futures::lazy(|| Ok("Hello async response".to_owned()));
        Box::new(future)
    }
}

impl MyServiceApiMut for ServiceApiContextMut {
    type Error = failure::Error;

    fn bar(&self, request: Seed) -> Result<(u64, exonum::crypto::Hash), Self::Error> {
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

    fn bar_async(
        &self,
        request: Seed,
    ) -> Box<Future<Item = (u64, exonum::crypto::Hash), Error = Self::Error>> {
        let self_ = self.clone();
        let future = futures::lazy(move || self_.bar(request));
        Box::new(future)
    }
}

pub struct MyService;

#[derive(Default, Clone)]
pub struct SharedState {
    count: Arc<Mutex<u64>>,
}

impl SharedState {
    fn new() -> SharedState {
        println!("Created shared state");
        SharedState::default()
    }

    fn increment(&self) {
        (*self.count.lock().unwrap()) += 1;
    }

    fn count(&self) -> u64 {
        *self.count.lock().unwrap()
    }
}

impl Service for MyService {
    fn initialize_api(&self, initializer: &mut ServiceApiInitializer) {
        println!("Initialize api");
        let public_api = initializer.public_api();

        let shared_state = SharedState::new();
        let stateful_endpoint =
            move |_: &ServiceApiContextMut, _: String| -> Result<(u64), failure::Error> {
                let count = shared_state.count();
                shared_state.increment();
                println!("Increment shared state: {}", shared_state.count());
                Ok(count)
            };

        public_api
            .endpoint("foo", <ServiceApiContext as MyServiceApi>::foo)
            .endpoint("hello", <ServiceApiContext as MyServiceApi>::hello)
            .endpoint(
                "hello_async",
                <ServiceApiContext as MyServiceApi>::hello_async,
            )
            .endpoint("baz", <ServiceApiContext as MyServiceApi>::baz)
            .endpoint("bar", <ServiceApiContextMut as MyServiceApiMut>::bar)
            .endpoint(
                "bar_async",
                <ServiceApiContextMut as MyServiceApiMut>::bar_async,
            )
            .endpoint("counter", stateful_endpoint);
    }
}

fn main() {
    exonum::helpers::init_logger().unwrap();

    let keypair = exonum::crypto::gen_keypair();
    let api_sender = exonum::node::ApiSender::new(futures::sync::mpsc::channel(1).0);
    let db = exonum::storage::RocksDB::open("/tmp/actix", &exonum::storage::DbOptions::default())
        .unwrap();
    let blockchain = Blockchain::new(db, vec![], keypair.0, keypair.1, api_sender);

    let mut initalizer = ServiceApiInitializer::default();

    let service = MyService;
    service.initialize_api(&mut initalizer);

    let endpoints = Arc::from(Mutex::from(
        initalizer.public_api_builder.web_backend.finish(),
    ));
    let context = ServiceApiContextMut::new(blockchain.clone());
    server::new(move || {
        let context = context.clone();
        let endpoints = endpoints.clone();
        App::with_state(context).scope("api", move |scope| {
            scope.nested("rustfest", move |mut scope| {
                let endpoints = endpoints.lock().unwrap().clone();
                for endpoint in endpoints {
                    scope = scope.route(endpoint.name, endpoint.method.clone(), move |request| {
                        (endpoint.inner)(request)
                    });
                }
                scope
            })
        })
    }).bind("localhost:8080")
        .unwrap()
        .run()
}
