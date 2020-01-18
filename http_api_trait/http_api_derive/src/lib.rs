extern crate proc_macro;

use proc_macro::TokenStream;

mod from_url_query;
mod http_api;

#[proc_macro_derive(FromUrlQuery)]
pub fn from_url_query(input: TokenStream) -> TokenStream {
    from_url_query::impl_from_url_query(input)
}