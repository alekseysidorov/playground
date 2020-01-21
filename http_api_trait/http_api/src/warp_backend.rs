use serde::{de, ser};
use warp::{filters::BoxedFilter, reject::Reject, Filter};

use super::FromUrlQuery;

#[derive(Debug)]
pub struct IncorrectQuery;

impl Reject for IncorrectQuery {}

pub type JsonReply = BoxedFilter<(warp::reply::Json,)>;

pub fn simple_get<F, R, E>(name: &'static str, handler: F) -> JsonReply
where
    F: Fn() -> Result<R, E> + Clone + Send + Sync + 'static,
    R: ser::Serialize,
    E: Reject,
{
    warp::get()
        .and(warp::path(name))
        .and_then(move || {
            let handler = handler.clone();
            async move {
                match handler() {
                    Ok(value) => Ok(warp::reply::json(&value)),
                    Err(e) => Err(warp::reject::custom(e)),
                }
            }
        })
        .boxed()
}

pub fn query_get<F, Q, R, E>(name: &'static str, handler: F) -> JsonReply
where
    F: Fn(Q) -> Result<R, E> + Clone + Send + Sync + 'static,
    Q: FromUrlQuery,
    R: ser::Serialize,
    E: Reject,
{
    warp::get()
        .and(warp::path(name))
        .and(warp::filters::query::raw())
        .and_then(move |raw_query: String| {
            let handler = handler.clone();
            async move {
                let query = Q::from_query_str(&raw_query)
                    .map_err(|_| warp::reject::custom(IncorrectQuery))?;

                match handler(query) {
                    Ok(value) => Ok(warp::reply::json(&value)),
                    Err(e) => Err(warp::reject::custom(e)),
                }
            }
        })
        .boxed()
}

pub fn simple_post<F, R, E>(name: &'static str, handler: F) -> JsonReply
where
    F: Fn() -> Result<R, E> + Clone + Send + Sync + 'static,
    R: ser::Serialize,
    E: Reject,
{
    warp::post()
        .and(warp::path(name))
        .and_then(move || {
            let handler = handler.clone();
            async move {
                match handler() {
                    Ok(value) => Ok(warp::reply::json(&value)),
                    Err(e) => Err(warp::reject::custom(e)),
                }
            }
        })
        .boxed()
}

pub fn params_post<F, Q, R, E>(name: &'static str, handler: F) -> JsonReply
where
    F: Fn(Q) -> Result<R, E> + Clone + Send + Sync + 'static,
    Q: de::DeserializeOwned + Send + 'static,
    R: ser::Serialize,
    E: Reject,
{
    warp::get()
        .and(warp::path(name))
        .and(warp::body::json())
        .and_then(move |query| {
            let handler = handler.clone();
            async move {
                match handler(query) {
                    Ok(value) => Ok(warp::reply::json(&value)),
                    Err(e) => Err(warp::reject::custom(e)),
                }
            }
        })
        .boxed()
}
