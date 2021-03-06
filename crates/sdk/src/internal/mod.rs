//! These API functions are intended for internal usage in generated code.
//! They shouldn't be used by a transformation developer for any implementation purposes.

pub use serde;
pub use serde_cbor;

pub mod data_tree;
pub mod key_tree;
pub mod version_embedder;

#[allow(dead_code)]
pub mod memory;
