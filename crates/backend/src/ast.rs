use proc_macro2::{Ident, Span};
use syn;

/// An abstract syntax tree representing a rust program. Contains
/// extra information for joining up this rust code with javascript.
#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Default, Clone)]
pub struct Program {
    /// rust -> js interfaces
    pub exports: Vec<Export>,
}

impl Program {
    /// Returns true if the Program is empty
    pub fn is_empty(&self) -> bool {
        self.exports.is_empty()
    }
}

/// A rust to js interface. Allows interaction with rust objects/functions
/// from javascript.
#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Clone)]
pub struct Export {
    /// The rust function
    pub function: Function,
    /// The kind (static, named, regular)
    pub method_kind: MethodKind,
    /// The struct name, in Rust, this is attached to
    pub rust_class: Option<Ident>,
    /// The name of the rust function/method on the rust side.
    pub rust_name: Ident,
}

/// The type of a method
#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub enum MethodKind {
    /// Any other kind of method
    Operation(Operation),
}

/// The operation performed by a class method
#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Operation {
    /// Whether this method is static
    pub is_static: bool,
    /// The internal kind of this Operation
    pub kind: OperationKind,
}

/// The kind of operation performed by a method
#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub enum OperationKind {
    /// A standard method, nothing special
    Regular,
}

/// Information about a function being imported or exported
#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Clone)]
pub struct Function {
    /// The name of the function
    pub name: String,
    /// The arguments to the function
    pub arguments: Vec<syn::PatType>,
    /// The return type of the function, if provided
    pub ret: Option<syn::Type>,
}

impl Export {
    /// Generate unique function name for our exported Rust function
    pub(crate) fn rust_symbol(&self) -> Ident {
        let mut generated_name = String::from("__holium_bindgen_generated");
        generated_name.push_str("_");
        generated_name.push_str(&self.function.name.to_string());
        Ident::new(&generated_name, Span::call_site())
    }

    /// This is the name of the shim function that gets exported and takes the raw
    /// ABI form of its arguments and converts them back into their normal,
    /// "high level" form before calling the actual function.
    pub(crate) fn export_name(&self) -> String {
       self.function.name.to_string()
    }
}