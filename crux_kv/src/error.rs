use facet::Facet;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error type for `KeyValue` operations
#[derive(Facet, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[repr(C)]
#[serde(rename_all = "camelCase")]
pub enum KeyValueError {
    #[error("IO error: {message}")]
    Io { message: String },
    #[error("timeout")]
    Timeout,
    #[error("cursor not found")]
    CursorNotFound,
    #[error("other error: {message}")]
    Other { message: String },
}
