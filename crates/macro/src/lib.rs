extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn holium_bindgen(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
