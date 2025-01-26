use thiserror::Error;

#[derive(Debug, Error)]
pub enum XtcError {
    #[error("Unknown error: {0}")]
    Unknown(String),
}
