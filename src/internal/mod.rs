/// These API functions are intended for internal usage in generated code.
/// Normally, you shouldn't use them.

pub use serde::{Deserialize, Serialize};

pub mod data_tree;
pub mod key_tree;

#[allow(dead_code)]
mod host_interface;
pub mod api;
