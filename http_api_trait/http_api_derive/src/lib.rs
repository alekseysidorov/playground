extern crate proc_macro;

use proc_macro::TokenStream;

mod from_url_query;
mod http_api;

#[proc_macro_derive(FromUrlQuery, attributes(from_url_query))]
pub fn from_url_query(input: TokenStream) -> TokenStream {
    from_url_query::impl_from_url_query(input)
}

#[proc_macro_attribute]
pub fn http_api(attr: TokenStream, item: TokenStream) -> TokenStream {
    http_api::impl_http_api(attr, item)
}

#[proc_macro_attribute]
#[doc(hidden)]
pub fn http_api_endpoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // We don't modify the input stream, since `endpoint` attribute only
    // provides additional metadata for `http_api` attribute.
    //
    // This however should be a `proc_macro_attribute`, so rust compiler won't complain about
    // unknown attribute.
    item
}
