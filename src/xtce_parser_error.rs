use thiserror::Error;
use xml::reader::XmlEvent;

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
    #[error("No XTCE elements in input")]
    NoXTCE(),
    #[error("line {0}: Invalid Document format")]
    BadDocumentStart(usize),
    #[error("line {0}: Unknown or misplaced element: <{1}>")]
    UnknownElement(usize, String),
    #[error("line {0}: Unexpected element termination {1}:")]
    UnexpectedTermination(usize, &'static str),
    #[error("line {0}: Multiple SpaceSystem XTCE elements")]
    MultipleSpaceSystems(usize), 
    #[error("Line {0}: General error: {1}")]
    GeneralError(usize, Box<dyn std::error::Error>),
    #[error("Unexpected XML: {0:?}")]
    UnexpectedXml(XmlEvent),
    #[error("Line {0}: XML error: {1}")]
    XmlError(usize, Box<dyn std::error::Error>),
    // FIXME: this is temporary and should eventually be deleted
    #[error("Line {0}: Unknown XTCE parsing error")]
    Unknown(usize),
}
