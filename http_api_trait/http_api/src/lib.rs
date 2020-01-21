pub use serde_urlencoded::de::Error as ParseQueryError;

pub mod warp_backend;

#[doc(hidden)]
pub mod export {
    pub use serde;
    pub use serde_derive;
    pub use serde_urlencoded;
}

pub trait FromUrlQuery: Sized {
    fn from_query_str(query: &str) -> Result<Self, ParseQueryError>;
}