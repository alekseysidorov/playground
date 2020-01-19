use darling::{self, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
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
            _ => Err(invalid_method(&sig)),
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
}

#[derive(Debug)]
struct ParsedApiDefinition {
    item_trait: syn::ItemTrait,
    endpoints: Vec<ParsedEndpoint>,
}

impl ParsedApiDefinition {
    fn parse(item_trait: syn::ItemTrait) -> Result<Self, darling::Error> {
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

        Ok(Self {
            item_trait,
            endpoints,
        })
    }
}

pub fn impl_http_api(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_trait: syn::ItemTrait = parse_macro_input!(item);
    let _attrs: syn::AttributeArgs = parse_macro_input!(attr);

    let api_definition = match ParsedApiDefinition::parse(item_trait.clone()) {
        Ok(parsed) => parsed,
        Err(e) => return e.write_errors().into(),
    };

    dbg!(&api_definition);

    let tokens = quote! { #item_trait };
    tokens.into()
}
