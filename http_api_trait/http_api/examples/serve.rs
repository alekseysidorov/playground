use http_api::warp_backend;
use http_api_derive::FromUrlQuery;
use warp::{reject::Reject, Filter};

use std::{future::Future, net::SocketAddr};

#[derive(Debug, FromUrlQuery)]
struct Query {
    first: String,
    second: u64,
}

#[derive(Debug)]
struct Error;

impl Reject for Error {}

trait PingInterface {
    fn ping(&self) -> Result<String, Error>;

    fn get(&self, query: Query) -> Result<(), Error>;

    fn pong(&self, param: String) -> Result<(), Error>;
}

fn serve_ping_interface<T>(
    service: T,
    addr: impl Into<std::net::SocketAddr>,
) -> impl std::future::Future<Output = ()>
where
    T: PingInterface + Clone + Send + Sync + 'static,
{
    let ping = warp_backend::simple_get("ping", {
        let out = service.clone();
        move || out.ping()
    });

    let get = warp_backend::query_get("get", {
        let out = service.clone();
        move |query| out.get(query)
    });

    let pong = warp_backend::params_post("pong", {
        let out = service.clone();
        move |query| out.pong(query)
    });

    warp::serve(ping.or(get).or(pong)).run(addr.into())
}

#[derive(Clone, Copy)]
struct ServiceImpl;

impl PingInterface for ServiceImpl {
    fn ping(&self) -> Result<String, Error> {
        Ok("foo".to_owned())
    }

    fn pong(&self, param: String) -> Result<(), Error> {
        eprintln!("{}", param);
        Ok(())
    }

    fn get(&self, query: Query) -> Result<(), Error> {
        eprintln!("{:?}", query);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    serve_ping_interface(ServiceImpl, addr).await
}
