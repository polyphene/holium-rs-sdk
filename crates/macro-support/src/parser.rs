use backend::ast;
use backend::Diagnostic;
use proc_macro2::{TokenStream};
use quote::ToTokens;
use syn;

/// Conversion trait with context.
///
/// Used to convert syn tokens into an AST, that we can then use to generate glue code.
trait ConvertToAst {
    /// What we are converting to.
    type Target;
    /// Convert into our target.
    ///
    /// Since this is used in a procedural macro, use panic to fail.
    fn convert(self) -> Result<Self::Target, Diagnostic>;
}

impl<'a> ConvertToAst for &'a mut syn::ItemStruct {
    type Target = ast::Struct;

    fn convert(self) -> Result<Self::Target, Diagnostic> {
        if self.generics.params.len() > 0 {
            bail_span!(
                self.generics,
                "structs with #[holium_bindgen] cannot have lifetime or \
                 type parameters currently"
            );
        }
        let mut fields = Vec::new();
        for (i, field) in self.fields.iter_mut().enumerate() {
            match field.vis {
                syn::Visibility::Public(..) => {}
                _ => continue,
            }
            let (name, member) = match &field.ident {
                Some(ident) => (ident.to_string(), syn::Member::Named(ident.clone())),
                None => (i.to_string(), syn::Member::Unnamed(i.into())),
            };

            fields.push(ast::StructField {
                rust_name: member,
                name,
                struct_name: self.ident.clone(),
                ty: field.ty.clone(),
            });
        }
        Ok(ast::Struct {
            rust_name: self.ident.clone(),
            name: self.ident.to_string(),
            fields,
        })
    }
}


impl ConvertToAst for syn::ItemFn {
    type Target = ast::Function;

    fn convert(self) -> Result<Self::Target, Diagnostic> {
        match self.vis {
            syn::Visibility::Public(_) => {}
            _ => bail_span!(self, "can only #[holium_bindgen] public functions"),
        }
        if self.sig.constness.is_some() {
            bail_span!(
                self.sig.constness,
                "can only #[holium_bindgen] non-const functions"
            );
        }
        if self.sig.unsafety.is_some() {
            bail_span!(
                self.sig.unsafety,
                "can only #[holium_bindgen] safe functions"
            );
        }

        let f = function_from_decl(&self.sig.ident, self.sig.clone())?;
        Ok(f)
    }
}

/// Construct a function (and gets the self type if appropriate) for our AST from a syn function.
fn function_from_decl(
    decl_name: &syn::Ident,
    sig: syn::Signature,
) -> Result<ast::Function, Diagnostic> {
    if sig.variadic.is_some() {
        bail_span!(sig.variadic, "can't #[holium_bindgen] variadic functions");
    }
    if sig.generics.params.len() > 0 {
        bail_span!(
            sig.generics,
            "can't #[holium_bindgen] functions with lifetime or type parameters",
        );
    }
    if sig.asyncness.is_some() {
        bail_span!(
            sig.generics,
            "can't #[holium_bindgen] async functions functions",
        );
    }
    // TODO maybe one day lifetime could be handled
    assert_no_lifetimes(&sig)?;

    let syn::Signature { inputs, output, .. } = sig;

    let arguments = inputs
        .into_iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(c) => Some(c),
            syn::FnArg::Receiver(_) => {
                panic!("arguments cannot be `self`")
            }
        })
        .collect::<Vec<_>>();

    let ret = match output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ty) => Some(*ty),
    };

    Ok(ast::Function {
        arguments,
        name: decl_name.to_string(),
        ret,
    })
}

pub(crate) trait MacroParse<Ctx> {
    /// Parse the contents of an object into our AST, with a context if necessary.
    ///
    /// The context is used to have access to the attributes on `#[holium_bindgen]`, and to allow
    /// writing to the output `TokenStream`.
    fn macro_parse(self, program: &mut ast::Program, context: Ctx) -> Result<(), Diagnostic>;
}

impl<'a> MacroParse<&'a mut TokenStream> for syn::Item {
    fn macro_parse(
        self,
        program: &mut ast::Program,
        tokens: &'a mut TokenStream,
    ) -> Result<(), Diagnostic> {
        match self {
            syn::Item::Fn(mut f) => {
                let no_mangle = f
                    .attrs
                    .iter()
                    .enumerate()
                    .filter_map(|(i, m)| m.parse_meta().ok().map(|m| (i, m)))
                    .find(|(_, m)| m.path().is_ident("no_mangle"));
                match no_mangle {
                    Some((i, _)) => {
                        f.attrs.remove(i);
                    }
                    _ => {}
                }
                f.to_tokens(tokens);

                let method_kind = ast::MethodKind::Operation(ast::Operation {
                    is_static: true,
                    kind: ast::OperationKind::Regular,
                });
                let rust_name = f.sig.ident.clone();

                program.exports.push(ast::Export {
                    function: f.convert()?,
                    method_kind,
                    rust_class: None,
                    rust_name,
                });
            },
            syn::Item::Struct(mut s) => {
                program.structs.push((&mut s).convert()?);
                s.to_tokens(tokens);
            }
            _ => {
                bail_span!(
                    self,
                    "#[holium_bindgen] can only be applied to a public function or struct",
                );
            }
        }

        Ok(())
    }
}

/// Check there are no lifetimes on the function.
fn assert_no_lifetimes(sig: &syn::Signature) -> Result<(), Diagnostic> {
    struct Walk {
        diagnostics: Vec<Diagnostic>,
    }

    impl<'ast> syn::visit::Visit<'ast> for Walk {
        fn visit_lifetime(&mut self, i: &'ast syn::Lifetime) {
            self.diagnostics.push(err_span!(
                &*i,
                "it is currently not sound to use lifetimes in function \
                 signatures"
            ));
        }
    }
    let mut walk = Walk {
        diagnostics: Vec::new(),
    };
    syn::visit::Visit::visit_signature(&mut walk, sig);
    Diagnostic::from_vec(walk.diagnostics)
}
