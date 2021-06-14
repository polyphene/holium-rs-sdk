use crate::ast;
use crate::Diagnostic;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn;

/// A trait for converting AST structs into Tokens and adding them to a TokenStream,
/// or providing a diagnostic if conversion fails.
pub trait TryToTokens {
    /// Attempt to convert a `Self` into tokens and add it to the `TokenStream`
    fn try_to_tokens(&self, tokens: &mut TokenStream) -> Result<(), Diagnostic>;

    /// Attempt to convert a `Self` into a new `TokenStream`
    fn try_to_token_stream(&self) -> Result<TokenStream, Diagnostic> {
        let mut tokens = TokenStream::new();
        self.try_to_tokens(&mut tokens)?;
        Ok(tokens)
    }
}

impl TryToTokens for ast::Program {
    // Generate wrappers for all the items that we've found
    fn try_to_tokens(&self, tokens: &mut TokenStream) -> Result<(), Diagnostic> {
        let mut errors = Vec::new();
        for export in self.exports.iter() {
            if let Err(e) = export.try_to_tokens(tokens) {
                errors.push(e);
            }
        }

        Diagnostic::from_vec(errors)?;

        Ok(())
    }
}
impl TryToTokens for ast::Export {
    fn try_to_tokens(self: &ast::Export, into: &mut TokenStream) -> Result<(), Diagnostic> {
        let generated_name = self.rust_symbol();
        let export_name = self.export_name();
        let mut arg_conversions: Vec<TokenStream> = vec![];
        let mut converted_arguments: Vec<TokenStream> = vec![];
        let mut ret_conversions: Vec<TokenStream> = vec![];

        let name = &self.rust_name;
        let receiver = quote! { #name };


        for (i, arg) in self.function.arguments.iter().enumerate() {
            let i_string = format!("{}", i);
            let ident = Ident::new(&format!("arg{}", i), Span::call_site());
            let ty = &arg.ty;

            match &*arg.ty {
                syn::Type::Reference(syn::TypeReference {
                    mutability: Some(_),
                    elem,
                    ..
                }) => {
                    arg_conversions.push(quote! {
                        //TODO unwrap here is pretty brutal, need to find way to have better error handling
                        let mut #ident: #elem  = holium_rust_sdk::interface::api::get_payload(#i_string).unwrap();
                        let #ident: #ty = &mut #ident;
                    });
                }
                syn::Type::Reference(syn::TypeReference { elem, .. }) => {
                    if  (quote! {#elem}).to_string() == "str" {
                        arg_conversions.push(quote! {
                            //TODO unwrap here is pretty brutal, need to find way to have better error handling
                            let #ident: String = holium_rust_sdk::interface::api::get_payload(#i_string).unwrap();
                            let #ident: #ty = #ident.as_str();
                        });
                    } else {
                        arg_conversions.push(quote! {
                            //TODO unwrap here is pretty brutal, need to find way to have better error handling
                            let #ident: #elem = holium_rust_sdk::interface::api::get_payload(#i_string).unwrap();
                            let #ident: #ty = &#ident;
                        });
                    }
                }
                _ => {
                    arg_conversions.push(quote! {
                        //TODO unwrap here is pretty brutal, need to find way to have better error handling
                        let #ident: #ty = holium_rust_sdk::interface::api::get_payload(#i_string).unwrap();
                    });
                }
            }
            converted_arguments.push(quote! { #ident });
        }


        let syn_unit = syn::Type::Tuple(syn::TypeTuple {
            elems: Default::default(),
            paren_token: Default::default(),
        });
        let syn_ret = self.function.ret.as_ref().unwrap_or(&syn_unit);
        // TODO handle all types, not only tuples
        let ret: TokenStream = match syn_ret {
            syn::Type::Reference(_) => {
                bail_span!(syn_ret, "cannot return a borrowed ref with #[holium_bindgen]",)
            },
            syn::Type::Path(_) => {
                let ident = Ident::new(&format!("ret{}", 0), Span::call_site());
                let i_string = format!("{}", 0);
                ret_conversions.push(quote! {
                        holium_rust_sdk::interface::api::set_payload(#i_string, &#ident).unwrap();
                    });
                quote!{ #ident }
            },
            syn::Type::Tuple(t) => {
                let mut converted_returns: Vec<TokenStream> = vec![];

                for (i, elem) in t.elems.iter().enumerate() {
                    let ident = Ident::new(&format!("ret{}", i), Span::call_site());
                    let i_string = format!("{}", i);

                    match elem {
                        syn::Type::Reference(_) => ret_conversions.push(quote! {
                            holium_rust_sdk::interface::api::set_payload(#i_string, #ident).unwrap();
                        }),
                        _ => ret_conversions.push(quote! {
                            holium_rust_sdk::interface::api::set_payload(#i_string, &#ident).unwrap();
                        })
                    };

                    converted_returns.push(quote! { #ident });
                }
                quote! { (#(#converted_returns),*) }
            },
            _ => {
                bail_span!(
                syn_ret,
                "for now only tuples or single values are valid return types with #[holium_bindgen]",
            )
            }
        };

        (quote! {
            #[allow(non_snake_case)]
            #[cfg_attr(
                all(target_arch = "wasm32"),
                export_name = #export_name,
            )]
            #[allow(clippy::all)]
            pub extern "C" fn #generated_name() {
                #(#arg_conversions)*
                let #ret  = #receiver(#(#converted_arguments),*);
                #(#ret_conversions)*
            }
        })
        .to_tokens(into);

        Ok(())
    }
}