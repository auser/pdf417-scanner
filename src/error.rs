use thiserror::Error;

pub type DecodeResult<T = (), E = DecodeError> = Result<T, E>;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Failed to decode barcode")]
    FailedToDecode(#[from] rxing::Exceptions),

    #[error("Runtime error: {0}")]
    RuntimeError(#[from] std::io::Error),
}
