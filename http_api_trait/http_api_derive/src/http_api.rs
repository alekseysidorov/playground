use darling::{self, FromMeta};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

fn find_meta_attrs(name: &str, args: &[syn::Attribute]) -> Option<syn::NestedMeta> {
    args.as_ref()
        .iter()
        .filter_map(|a| a.parse_meta().ok())
        .find(|m| m.path().is_ident(name))
        .map(syn::NestedMeta::from)
}

fn invalid_method(span: &impl syn::spanned::Spanned) -> darling::Error {
    darling::Error::custom(
        "API method should have one of `fn foo(&self) -> Result<Bar, Error>` or \
         `fn foo(&self, arg: Foo) -> Result<Bar, Error>` form",
    )
    .with_span(span)
}

#[derive(Debug)]
enum SupportedHttpMethod {
    Get,
    Post,
}

impl FromMeta for SupportedHttpMethod {
    fn from_string(value: &str) -> Result<Self, darling::Error> {
        match value {
            "get" => Ok(SupportedHttpMethod::Get),
            "post" => Ok(SupportedHttpMethod::Post),
            other => Err(darling::Error::unknown_value(other)),
        }
    }
}

#[derive(Debug, FromMeta)]
struct ApiAttrs {
    warp: syn::Ident,
}

#[derive(Debug, FromMeta)]
struct EndpointAttrs {
    method: SupportedHttpMethod,
    #[darling(default)]
    rename: Option<String>,
}

#[derive(Debug)]
struct ParsedEndpoint {
    ident: syn::Ident,
    arg: Option<Box<syn::Type>>,
    ret: Box<syn::Type>,
    attrs: EndpointAttrs,
}

impl ParsedEndpoint {
    fn parse(sig: &syn::Signature, attrs: &[syn::Attribute]) -> Result<Self, darling::Error> {
        let mut args = sig.inputs.iter();

        // Check receiver.
        if let Some(arg) = args.next() {
            match arg {
                syn::FnArg::Receiver(syn::Receiver {
                    reference: Some(_),
                    mutability: None,
                    ..
                }) => {}
                _ => {
                    return Err(invalid_method(&arg));
                }
            }
        } else {
            return Err(invalid_method(&sig));
        }

        // Extract arg type.
        let arg = args
            .next()
            .map(|arg| match arg {
                syn::FnArg::Typed(arg) => Ok(arg.ty.clone()),
                _ => Err(invalid_method(&arg)),
            })
            .transpose()?;

        // Extract return type.
        let ret = match &sig.output {
            syn::ReturnType::Type(_, ty) => Ok(ty.clone()),
            _ => unreachable!("Only first argument can be receiver."),
        }?;

        // Extract attributes.
        let attrs = find_meta_attrs("http_api_endpoint", attrs)
            .map(|meta| EndpointAttrs::from_nested_meta(&&meta))
            .unwrap_or_else(|| Err(darling::Error::custom("todo")))?;

        Ok(Self {
            ident: sig.ident.clone(),
            arg,
            ret,
            attrs,
        })
    }

    fn endpoint_path(&self) -> String {
        self.attrs
            .rename
            .clone()
            .unwrap_or_else(|| self.ident.to_string())
    }

    fn impl_endpoint_handler(&self) -> impl ToTokens {
        let path = self.endpoint_path();
        let ident = &self.ident;

        match (&self.attrs.method, &self.arg) {
            (SupportedHttpMethod::Get, None) => {
                quote! {
                    let #ident = http_api::warp_backend::simple_get(#path, {
                        let out = service.clone();
                        move || out.#ident()
                    });
                }
            }

            (SupportedHttpMethod::Get, Some(_arg)) => {
                quote! {
                    let #ident = http_api::warp_backend::query_get(#path, {
                        let out = service.clone();
                        move |query| out.#ident(query)
                    });
                }
            }

            (SupportedHttpMethod::Post, None) => {
                quote! {
                    let #ident = http_api::warp_backend::simple_post(#path, {
                        let out = service.clone();
                        move || out.#ident()
                    });
                }
            }

            (SupportedHttpMethod::Post, Some(_arg)) => {
                quote! {
                    let #ident = http_api::warp_backend::params_post(#path, {
                        let out = service.clone();
                        move |params| out.#ident(params)
                    });
                }
            }
        }
    }
}

#[derive(Debug)]
struct ParsedApiDefinition {
    item_trait: syn::ItemTrait,
    endpoints: Vec<ParsedEndpoint>,
    attrs: ApiAttrs,
}

impl ParsedApiDefinition {
    fn parse(
        item_trait: syn::ItemTrait,
        attrs: &[syn::NestedMeta],
    ) -> Result<Self, darling::Error> {
        let endpoints = item_trait
            .items
            .iter()
            .filter_map(|item| {
                if let syn::TraitItem::Method(method) = item {
                    Some(method)
                } else {
                    None
                }
            })
            .map(|method| ParsedEndpoint::parse(&method.sig, method.attrs.as_ref()))
            .collect::<Result<Vec<_>, darling::Error>>()?;

        // Extract attributes.
        let attrs = ApiAttrs::from_list(attrs)?;

        Ok(Self {
            item_trait,
            endpoints,
            attrs,
        })
    }
}

impl ToTokens for ParsedApiDefinition {
    fn to_tokens(&self, out: &mut proc_macro2::TokenStream) {
        let fn_name = &self.attrs.warp;
        let interface = &self.item_trait.ident;

        let (filters, idents): (Vec<_>, Vec<_>) = self
            .endpoints
            .iter()
            .map(|endpoint| {
                let ident = &endpoint.ident;
                let handler = endpoint.impl_endpoint_handler();

                (handler, ident)
            })
            .unzip();

        let mut tail = idents.into_iter();
        let head = tail.next().unwrap();
        let serve_impl = quote! {
            #head #( .or(#tail) )*
        };

        let tokens = quote! {
            fn #fn_name<T>(
                service: T,
                addr: impl Into<std::net::SocketAddr>,
            ) -> impl std::future::Future<Output = ()>
            where
                T: #interface + Clone + Send + Sync + 'static,
            {
                use warp::Filter;

                #( #filters )*

                warp::serve(#serve_impl).run(addr.into())
            }

        };
        out.extend(tokens)
    }
}

pub fn impl_http_api(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_trait: syn::ItemTrait = parse_macro_input!(item);
    let attrs: syn::AttributeArgs = parse_macro_input!(attr);

    let api_definition = match ParsedApiDefinition::parse(item_trait.clone(), &attrs) {
        Ok(parsed) => parsed,
        Err(e) => return e.write_errors().into(),
    };

    let tokens = quote! {
        #item_trait
        #api_definition
    };

    tokens.into()
}
