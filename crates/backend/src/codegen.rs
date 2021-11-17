//! Codegen has the logic of code generation for our wasm module to run in the Holium protocol.

use crate::ast;
use crate::Diagnostic;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn;

/// A trait for converting AST structs into Tokens and adding them to a TokenStream,
/// or providing a diagnostic if conversion fails.
pub trait TryToTokens {
    /// Attempt to convert a `Self` into tokens and add it to the `TokenStream`
    fn try_to_tokens(&self, into: &mut TokenStream) -> Result<(), Diagnostic>;

    /// Attempt to convert a `Self` into a new `TokenStream`
    fn try_to_token_stream(&self) -> Result<TokenStream, Diagnostic> {
        let mut tokens = TokenStream::new();
        self.try_to_tokens(&mut tokens)?;
        Ok(tokens)
    }
}

impl TryToTokens for ast::Program {
    // Generate wrappers for all the items that we've found
    fn try_to_tokens(&self, into: &mut TokenStream) -> Result<(), Diagnostic> {
        // Handling exported functions
        let mut errors = Vec::new();
        for export in self.exports.iter() {
            if let Err(e) = export.try_to_tokens(into) {
                errors.push(e);
            }
        }

        // Handling tagged structures
        for s in self.structs.iter() {
            s.to_tokens(into);
        }

        Diagnostic::from_vec(errors)?;

        Ok(())
    }
}

impl ToTokens for ast::Struct {
    fn to_tokens(&self, into: &mut TokenStream) {
        let name = &self.rust_name;

        // Add derive for serialize & deserialize
        *into = (quote! {
            #[derive(holium_rs_sdk::internal::serde::Serialize, holium_rs_sdk::internal::serde::Deserialize)]
            #[serde( crate = "holium_rs_sdk::internal::serde")]
            #into
        })
            .to_token_stream();

        // For each field of our structure add a new children node
        let mut generate_node_children: Vec<TokenStream> = vec![];

        for field in self.fields.iter() {
            let field_name = field.name.to_string();
            let field_type = &field.ty;

            generate_node_children.push(quote! {
                holium_rs_sdk::internal::key_tree::Node {
                    value: Some(#field_name),
                    children: <#field_type>::generate_node().children
                }
            });
        }

        // Generating conversion from data_tree::Node to structure and implement key_tree::GenerateNode
        // trait
        (quote! {
            impl holium_rs_sdk::internal::key_tree::GenerateNode for #name {
                fn generate_node() -> holium_rs_sdk::internal::key_tree::Node {
                    holium_rs_sdk::internal::key_tree::Node {
                        value: None,
                        children: vec![
                            #(#generate_node_children),*
                        ],
                    }
                }
            }

            impl From<holium_rs_sdk::internal::data_tree::Node> for #name {
                fn from(data_tree: holium_rs_sdk::internal::data_tree::Node) -> Self {
                    let key_node = <#name>::generate_node();
                    let cbor = data_tree.assign_keys(&key_node);
                    let cbor_bytes: Vec<u8> = internal::serde_cbor::to_vec(&cbor).unwrap();
                    holium_rs_sdk::internal::serde_cbor::from_slice(&cbor_bytes).unwrap()
                }
            }
        })
        .to_tokens(into);
    }
}

impl TryToTokens for ast::Export {
    fn try_to_tokens(self: &ast::Export, into: &mut TokenStream) -> Result<(), Diagnostic> {
        let mut input_payload_fields: Vec<TokenStream> = vec![];
        let mut input_payload_node_children: Vec<TokenStream> = vec![];
        let mut converted_args: Vec<TokenStream> = vec![];

        let name = &self.rust_name;
        let receiver = quote! { #name };

        let exported_name = &self.export_name();
        let holium_func_name = &self.rust_symbol();

        // First, generating inputs elements : input payload struct & function arguments
        for (i, arg) in self.function.arguments.iter().enumerate() {
            let field = format!("arg{}", i);
            let field_ident = Ident::new(&field, Span::call_site());
            let input_ident = Ident::new(&format!("input"), Span::call_site());
            let ty = &arg.ty;

            match &*arg.ty {
                // If argument type is mutable reference
                syn::Type::Reference(syn::TypeReference {
                    mutability: Some(_),
                    elem,
                    ..
                }) => {
                    input_payload_fields.push(quote! {
                        #field_ident: #elem
                    });
                    input_payload_node_children.push(quote! {
                        holium_rs_sdk::internal::key_tree::Node {
                            value: Some(#field),
                            children: <#elem>::generate_node().children
                        }
                    });
                    converted_args.push(quote! {
                        &mut #input_ident.#field_ident
                    });
                }
                // If argument type is non-mutable reference
                syn::Type::Reference(syn::TypeReference { elem, .. }) => {
                    input_payload_fields.push(quote! {
                        #field_ident: #elem
                    });
                    input_payload_node_children.push(quote! {
                        holium_rs_sdk::internal::key_tree::Node {
                            value: Some(#field),
                            children: <#elem>::generate_node().children
                        }
                    });
                    // If argument type is non-mutable reference but a &str no need to add &
                    if (quote! {#elem}).to_string() == "str" {
                        converted_args.push(quote! {
                            #input_ident.#field_ident
                        });
                    } else {
                        converted_args.push(quote! {
                            &#input_ident.#field_ident
                        });
                    }
                }
                // For all other types
                _ => {
                    input_payload_fields.push(quote! {
                        #field_ident: #ty
                    });
                    input_payload_node_children.push(quote! {
                        holium_rs_sdk::internal::key_tree::Node {
                            value: Some(#field),
                            children: <#ty>::generate_node().children
                        }
                    });
                    converted_args.push(quote! {
                        #input_ident.#field_ident
                    });
                }
            }
        }

        (quote! {
            #[allow(non_snake_case)]
            #[cfg_attr(
                all(target_arch = "wasm32"),
                export_name = #exported_name,
            )]
            #[allow(clippy::all)]
            pub extern "C" fn #holium_func_name(ptr: *mut u8, len: usize) -> holium_rs_sdk::internal::memory::Slice {
                #[derive(holium_rs_sdk::internal::serde::Serialize, holium_rs_sdk::internal::serde::Deserialize)]
                #[serde( crate = "holium_rs_sdk::internal::serde")]
                struct InputPayload {
                    #(#input_payload_fields),*
                }

                impl holium_rs_sdk::internal::key_tree::GenerateNode for InputPayload {
                    fn generate_node() -> holium_rs_sdk::internal::key_tree::Node {
                        holium_rs_sdk::internal::key_tree::Node {
                            value: None,
                            children: vec![
                                #(#input_payload_node_children),*
                            ]
                        }
                    }
                }

                impl From<holium_rs_sdk::internal::data_tree::Node> for InputPayload {
                    fn from(data_tree: holium_rs_sdk::internal::data_tree::Node) -> Self {
                        let key_node = <InputPayload>::generate_node();
                        let cbor = data_tree.assign_keys(&key_node);
                        let cbor_bytes: Vec<u8> = internal::serde_cbor::to_vec(&cbor).unwrap();
                        holium_rs_sdk::internal::serde_cbor::from_slice(&cbor_bytes).unwrap()
                    }
                }

                let payload_u8: &[u8] = unsafe { std::slice::from_raw_parts(ptr, len) };
                let data_node: holium_rs_sdk::internal::data_tree::Node = holium_rs_sdk::internal::serde_cbor::from_slice(payload_u8).unwrap();

                let input: InputPayload = data_node.into();

                let output = #receiver(#(#converted_args),*);

                let output_cbor = holium_rs_sdk::internal::serde_cbor::value::to_value(vec![output]).unwrap();

                let output_node = holium_rs_sdk::internal::data_tree::Node::new(output_cbor).unwrap();
                let output_node_u8 = holium_rs_sdk::internal::serde_cbor::to_vec(&output_node).unwrap();

                holium_rs_sdk::internal::memory::Slice {
                    ptr: output_node_u8.as_ptr() as u32,
                    len: output_node_u8.len() as u32
                }
            }
        })
            .to_tokens(into);

        Ok(())
    }
}
