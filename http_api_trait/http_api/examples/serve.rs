use warp::{Filter};

use std::{net::SocketAddr, future::Future};

trait PingInterface {
    fn ping(&self) -> Result<String, String>;

    fn pong(&self, param: String) -> Result<(), String>;
}

fn serve_ping_interface<T>(service: T, addr: impl Into<SocketAddr>) -> impl Future<Output = ()>
where
    T: PingInterface + Clone + Send + Sync + 'static,
{
    let out = service.clone();
    let ping = warp::get().and(warp::path("ping")).map(move || {
        let value = out.ping().unwrap();
        warp::reply::json(&value)
    });

    let out = service.clone();
    let pong = warp::post()
        .and(warp::path("pong"))
        .and(warp::body::json())
        .map(move |value| {
            let value = out.pong(value);
            warp::reply::json(&value)
        });

    warp::serve(ping.or(pong)).run(addr.into())
}

#[derive(Clone, Copy)]
struct ServiceImpl;

impl PingInterface for ServiceImpl {
    fn ping(&self) -> Result<String, String> {
        Ok("foo".to_owned())
    }

    fn pong(&self, param: String) -> Result<(), String> {
        eprintln!("{}", param);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    serve_ping_interface(ServiceImpl, addr).await
}
