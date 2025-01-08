use thiserror::Error;

#[derive(Error, Debug)]
pub enum XtceParserError {
/*
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
*/
    #[error("line {0}: Multiple SpaceSystem XTCE elements")]
    MultipleSpaceSystems(usize), 
    #[error("Line {0}: General error: {1}")]
    GeneralError(usize, Box<dyn std::error::Error>),
    #[error("Line {0}: Unknown XTCE parsing error")]
    Unknown(usize),
}
