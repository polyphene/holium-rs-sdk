//! The `macro-support` is responsible for the logic coordination behind the `holium_bindgen` macro

extern crate proc_macro2;
extern crate quote;
extern crate syn;
#[macro_use]
extern crate holium_backend as backend;

use crate::parser::MacroParse;
use backend::{Diagnostic, TryToTokens};
use proc_macro2::TokenStream;

mod parser;

/// Takes the parsed input from a `#[holium_bindgen]` macro and returns the generated bindings
pub fn expand(input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let item = syn::parse2::<syn::Item>(input)?;

    let mut tokens = proc_macro2::TokenStream::new();
    let mut program = backend::ast::Program::default();

    // First step is to parse the `TokenStream` to copy source tokens & generate custom AST structures
    // for the codegen step
    item.macro_parse(&mut program, &mut tokens)?;

    // Second step is to generate code custom tokens based on custom AST structures & append it to
    // the `TokenStream`
    program.try_to_tokens(&mut tokens)?;

    Ok(tokens)
}
