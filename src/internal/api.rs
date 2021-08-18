/*********************************************************
 * The interface module is the library that can be used in a Wasm runtime to interact with an host.
 *********************************************************/

use serde::*;
use crate::internal::data_tree::Node;

/// ExecutionError represents all error that might have happened on the host
#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Invalid index")]
    InvalidIndexError,
    #[error("No content to set")]
    NoContentError,
    #[error("Out of memory")]
    OutOfMemoryError,
    #[error("Serialization error")]
    ExteralSerializationError,
    #[error("Serialization error")]
    SerializationError(#[from] serde_json::Error),
    #[error("Unknown error")]
    UnknownError,
}

impl From<crate::internal::host_interface::Error> for ExecutionError {
    fn from(e: crate::internal::host_interface::Error) -> Self {
        match e {
            crate::internal::host_interface::Error::HoliumError(errno) => match errno {
                1 => ExecutionError::InvalidIndexError,
                2 => ExecutionError::NoContentError,
                3 => ExecutionError::OutOfMemoryError,
                4 => ExecutionError::ExteralSerializationError,
                _ => ExecutionError::UnknownError,
            },
        }
    }
}

/// Function meant to be a generic serializer for any data that is supposed to be set on a storage
/// on the host.
pub fn set_payload<T>(output_index: &str, payload: &T) -> Result<(), ExecutionError>
    where
        T: ?Sized + Serialize,
{
    if output_index.len() == 0 {
        return Err(ExecutionError::InvalidIndexError);
    }

    let output_index_slice = serde_json::to_vec(output_index)?;
    let payload_slice = serde_json::to_vec(&payload)?;
    crate::internal::host_interface::set_payload(
        output_index_slice.as_ptr(),
        output_index_slice.len(),
        payload_slice.as_ptr(),
        payload_slice.len(),
    )?;

    Ok(())
}

/// Function meant to be a generic deserializer for any data retrieved from a storage on the host.
pub fn get_payload<T: serde::de::DeserializeOwned>(input_index: &str) -> Result<Node, ExecutionError> {
    if input_index.len() == 0 {
        return Err(ExecutionError::InvalidIndexError);
    }

    let input_index_slice = serde_json::to_vec(input_index)?;

    let capacity = 64 * 1024;
    let mut buf = vec![0u8; capacity];

    return match crate::internal::host_interface::get_payload(
        input_index_slice.as_ptr(),
        input_index_slice.len(),
        buf.as_mut_ptr(),
    ) {
        Ok(written) => {
            buf.truncate(written);
            let payload_data = serde_cbor::from_slice(&buf)?;
            Ok(payload_data)
        }
        Err(e) => Err(e.into()),
    };
}