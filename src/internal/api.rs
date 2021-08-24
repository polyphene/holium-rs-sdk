/*********************************************************
 * The interface module is the library that can be used in a Wasm runtime to interact with an host.
 *********************************************************/

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
    ExternalSerializationError,
    #[error("Cbor Serialization error")]
    CborSerializationError(#[from] serde_cbor::Error),
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
                4 => ExecutionError::ExternalSerializationError,
                _ => ExecutionError::UnknownError,
            },
        }
    }
}

/// Function meant to be a generic serializer for any data that is supposed to be set on a storage
/// on the host.
pub fn set_payload(payload: &crate::internal::data_tree::Node) -> Result<(), ExecutionError> {
    let payload_slice = serde_cbor::to_vec(&payload)?;
    crate::internal::host_interface::set_payload(payload_slice.as_ptr(), payload_slice.len())?;

    Ok(())
}

/// Function meant to be a generic deserializer for any data retrieved from a storage on the host.
pub fn get_payload() -> Result<crate::internal::data_tree::Node, ExecutionError> {
    let capacity = 64 * 1024;
    let mut buf = vec![0u8; capacity];

    return match crate::internal::host_interface::get_payload(buf.as_mut_ptr()) {
        Ok(written) => {
            buf.truncate(written);
            Ok(serde_cbor::from_slice(&buf)?)
        }
        Err(e) => Err(e.into()),
    };
}
