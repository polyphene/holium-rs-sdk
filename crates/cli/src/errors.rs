//! Module listing errors through the crate
use thiserror::Error;

#[derive(Error, Debug)]
/// Type for CLI common errors
pub(crate) enum CommonError {
    /// An error occurred when no Wasm file was compiled.
    #[error("{0}")]
    WasmCompilationError(String),
}
