extern crate proc_macro;

mod attrs;
mod structs;

use attrs::Attribute;
use structs::{Field, Struct};

use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryFrom;

#[proc_macro_derive(AsRef, attributes(as_ref))]
pub fn derive(tokens: StdTokenStream) -> StdTokenStream {
    let origin = syn::parse_macro_input!(tokens as syn::DeriveInput);
    let input = match Struct::try_from(origin) {
        Ok(input) => input,
        Err(e) => return e.to_compile_error().into(),
    };

    produce_all_impl_asref(&input).into()
}

fn produce_all_impl_asref(input: &Struct) -> TokenStream {
    input
        .fields
        .iter()
        .flat_map(|field| {
            field
                .attrs
                .iter()
                .filter_map(Attribute::to_target)
                .map(move |target| produce_impl_asref(input, field, &target.target))
        })
        .collect()
}

fn produce_impl_asref(input: &Struct, field: &Field, target: &syn::Ident) -> TokenStream {
    let name = &input.ident;
    let field_name = &field.ident;

    quote! {
        impl std::convert::AsRef<#target> for #name {
            fn as_ref(&self) -> &#target {
                &self.#field_name
            }
        }
    }
}
