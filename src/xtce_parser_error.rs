use thiserror::Error;
use xml::reader::XmlEvent;

use crate::parser::{LineNumber};

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
    #[error("Line {0}: Only one root element is allowed\n")]
    OnlyOneRootElement(LineNumber),

    #[error("Line {0}: Element name {1} doesn't match closing name {2}")]
    ElementNameMismatch(LineNumber, String, String),

    #[error("Line {0}: unexpected XML CDATA in input")]
    UnexpectedCData(LineNumber),

    #[error("Line {0}: unexpected character data in input")]
    UnexpectedCharacters(LineNumber),

    #[error("No XTCE elements in input")]
    NoXTCE(),

    #[error("line {0}: Invalid Document format")]
    BadDocumentStart(LineNumber),

    #[error("line {0}: Unknown or misplaced element: <{1}>")]
    UnknownElement(LineNumber, String),

    #[error("line {0}: Expected <SpaceSystem> element, not <{1}>")]
    BadXtceStart(LineNumber, String),

    #[error("line {0}: Expected </SpaceSystem>, not: <{1}>")]
    BadXtceEnd(LineNumber, String),

    #[error("line {0}: Unexpected element termination")]
    UnexpectedTermination(LineNumber),

    #[error("line {0}: StartDocument after StartDocument")]
    StartAfterStart(LineNumber), 

    #[error("line {0}: Misplaced element end: {1}")]
    MisplacedElementEnd(LineNumber, String), 

    #[error("line {0}: Multiple SpaceSystem XTCE elements")]
    MultipleSpaceSystems(LineNumber), 

    #[error("Line {0}: General error: {1}")]
    GeneralError(LineNumber, Box<dyn std::error::Error>),

    #[error("Unexpected XML error: {0:?}")]
    UnexpectedXml(XmlEvent),

    // FIXME: get line number from the XmlEvent
    #[error("Line {0}: XML error: {1}")]
    XmlError(LineNumber, Box<dyn std::error::Error>),

    // FIXME: this is temporary and should eventually be deleted
    #[error("Line {0}: Unknown XTCE parsing error")]
    Unknown(LineNumber),
}
