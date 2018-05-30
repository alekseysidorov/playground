extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate api_builder;
extern crate failure;
extern crate serde_urlencoded;

use actix_web::*;
use api_builder::*;

use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize)]
pub struct MyRequest {
    pub name: String,
    pub count: u64,
}

#[derive(Debug, Serialize)]
pub struct MyResponse(String);

pub struct MyConcreteEndpoint;
pub struct MyConcreteEndpoint2;

impl ReadEndpoint for MyConcreteEndpoint {
    const ID: EndpointIdentifier = EndpointIdentifier { name: "foo" };

    type Request = MyRequest;
    type Response = MyResponse;

    fn handle(
        &self,
        context: &ApiContext,
        request: &Self::Request,
    ) -> Result<Self::Response, failure::Error> {
        let string = format!(
            "Context value: {} for name: {} with count: {}",
            context.name, request.name, request.count
        );
        Ok(MyResponse(string))
    }
}

impl ReadEndpoint for MyConcreteEndpoint2 {
    const ID: EndpointIdentifier = EndpointIdentifier { name: "boo" };

    type Request = ();
    type Response = String;

    fn handle(
        &self,
        context: &ApiContext,
        request: &Self::Request,
    ) -> Result<Self::Response, failure::Error> {
        Ok("Hello actix".to_owned())
    }
}

fn main() {
    server::new(|| {
        let state = ApiContext {
            name: "context".to_owned(),
        };
        App::with_state(state).scope("/api", |scope| {
            ApiBuilder::new(scope)
                .for_service("rustfest", |api_builder: ServiceApiBuilder| {
                    api_builder
                        .read_endpoint(MyConcreteEndpoint)
                        .read_endpoint(MyConcreteEndpoint2)
                        .read_closure(
                            "baz",
                            |state: &ApiContext,
                             request: &MyRequest|
                             -> Result<MyResponse, failure::Error> {
                                let string = format!(
                                    "Context value: {} for name: {} with count: {}",
                                    state.name, request.name, request.count
                                );
                                Ok(MyResponse(string))
                            },
                        )
                })
                .into_scope()
        })
    }).bind("localhost:8080")
        .unwrap()
        .run()
}
