//! The `macro` crate is the entry point for the `holium_bindgen` procedural macro

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn holium_bindgen(_attr: TokenStream, input: TokenStream) -> TokenStream {
    match holium_macro_support::expand(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(diagnostic) => (quote! { #diagnostic }).into(),
    }
}
