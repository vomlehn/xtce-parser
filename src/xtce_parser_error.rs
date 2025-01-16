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
    #[error("Line {0}: Element name {1} doesn't match closing name {2}")]
    ElementNameMismatch(usize, String, String),

    #[error("Line {0}: unexpected XML CDATA in input")]
    UnexpectedCData(usize),

    #[error("Line {0}: unexpected character data in input")]
    UnexpectedCharacters(usize),

    #[error("No XTCE elements in input")]
    NoXTCE(),

    #[error("line {0}: Invalid Document format")]
    BadDocumentStart(usize),

    #[error("line {0}: Unknown or misplaced element: <{1}>")]
    UnknownElement(usize, String),

    #[error("line {0}: Expected <SpaceSystem> element, not <{1}>")]
    BadXtceStart(usize, String),

    #[error("line {0}: Expected </SpaceSystem>, not: <{1}>")]
    BadXtceEnd(usize, String),

    #[error("line {0}: Unexpected element termination")]
    UnexpectedTermination(usize),

    #[error("line {0}: StartDocument after StartDocument")]
    StartAfterStart(usize), 

    #[error("line {0}: Misplaced element end: {1}")]
    MisplacedElementEnd(usize, String), 

    #[error("line {0}: Multiple SpaceSystem XTCE elements")]
    MultipleSpaceSystems(usize), 

    #[error("Line {0}: General error: {1}")]
    GeneralError(usize, Box<dyn std::error::Error>),

    #[error("Unexpected XML error: {0:?}")]
    UnexpectedXml(XmlEvent),

    // FIXME: get line number from the XmlEvent
    #[error("Line {0}: XML error: {1}")]
    XmlError(usize, Box<dyn std::error::Error>),

    // FIXME: this is temporary and should eventually be deleted
    #[error("Line {0}: Unknown XTCE parsing error")]
    Unknown(usize),
}
