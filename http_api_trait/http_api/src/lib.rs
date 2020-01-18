pub use serde_urlencoded::de::Error as ParseQueryError;

#[doc(hidden)]
pub mod export {
    pub use serde;
    pub use serde_urlencoded;
    pub use serde_derive;
}

pub trait FromUrlQuery: Sized {
    fn from_str(query: &str) -> Result<Self, ParseQueryError>;
}