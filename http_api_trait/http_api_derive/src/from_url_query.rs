use darling::{ast, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_named))]
struct FromUrlQuery {
    ident: syn::Ident,
    data: ast::Data<(), QueryField>,
}

#[derive(Clone, Debug, FromField)]
struct QueryField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

impl FromUrlQuery {
    fn serde_wrapper_ident(&self) -> syn::Ident {
        let ident_str = format!("{}Serde", self.ident);
        syn::Ident::new(&ident_str, proc_macro2::Span::call_site())
    }

    fn impl_serde_wrapper(&self) -> impl ToTokens {
        let fields = self.data.clone().take_struct().unwrap();

        let wrapped_fields = fields.iter().map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;
            quote! { #ident: #ty }
        });
        let from_fields = fields.iter().map(|field| {
            let ident = &field.ident;
            quote! { #ident: v.#ident }
        });
        let wrapped_ident = self.serde_wrapper_ident();
        let ident = &self.ident;

        quote! {
            use http_api::export::serde_derive::Deserialize;

            #[derive(Deserialize)]
            #[serde(crate = "http_api::export::serde")]
            struct #wrapped_ident {
                #( #wrapped_fields, )*
            }

            impl From<#wrapped_ident> for #ident {
                fn from(v: #wrapped_ident) -> Self {
                    Self {
                        #( #from_fields, )*
                    }
                }
            }
        }
    }
}

impl ToTokens for FromUrlQuery {
    fn to_tokens(&self, out: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let serde_wrapper_ty = self.serde_wrapper_ident();
        let serde_wrapper = self.impl_serde_wrapper();

        let tokens = quote! {
            impl http_api::FromUrlQuery for #ident {
                fn from_str(query: &str) -> Result<Self, http_api::ParseQueryError> {
                    #serde_wrapper

                    let wrapper: #serde_wrapper_ty = 
                        http_api::export::serde_urlencoded::from_str(query)?;
                    
                    Ok(wrapper.into())
                }
            }

            #serde_wrapper
        };
        out.extend(tokens)
    }
}

pub fn impl_from_url_query(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let from_url_query = match FromUrlQuery::from_derive_input(&input) {
        Ok(parsed) => parsed,
        Err(e) => return e.write_errors().into(),
    };
    let tokens = quote! { #from_url_query };
    println!("{}", tokens);
    tokens.into()
}
