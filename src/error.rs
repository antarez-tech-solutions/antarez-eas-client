//! Error types — implemented in Step 2
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EasError {
    #[error("not yet implemented")]
    NotImplemented,
}
