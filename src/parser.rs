use std::fmt;

use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;
use std::io::{BufReader, Read};

use xml::reader::{EventReader, XmlEvent};

use crate::xtce_parser_error::{XtceParserError};

pub struct ElementDesc {
    allowable_subelements:  &'static[ElementDesc]
}

const root_desc: ElementDesc = ElementDesc {
    allowable_subelements: &[space_system_desc],
};

const space_system_desc: ElementDesc = ElementDesc {
    allowable_subelements: &[telemetry_meta_data_desc, command_meta_data_desc],
};

const telemetry_meta_data_desc: ElementDesc = ElementDesc {
    allowable_subelements: &[],
};

const command_meta_data_desc: ElementDesc = ElementDesc {
    allowable_subelements: &[],
};

type LineNumber = usize;

#[derive(Debug)]
pub struct Element {
    pub lineno:         LineNumber,
    pub event:          XmlEvent,
}

impl Element {
    fn new(lineno: LineNumber, event: XmlEvent) -> Element {
        Element { lineno: lineno, event: event }
    }
}

pub struct Parser<R: Read> {
    lineno_ref:     Rc<RefCell<LineNumber>>,
    event_reader:   EventReader<LinenoReader<BufReader<R>>>,
}

impl<R: Read> fmt::Debug for Parser<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parser: lineno: {}", *self.lineno_ref.borrow())
    }
}


impl<R: Read> Parser<R> {
    pub fn new<T: Read>(buf_reader: BufReader<T>) -> Parser<T> {
        let line_reader = LinenoReader::new(buf_reader);
        let lineno_ref = line_reader.lineno_ref();
        let mut event_reader = EventReader::new(line_reader);
        Parser {
            lineno_ref:      lineno_ref,
            event_reader:   event_reader,
        }
    }

    pub fn next(&mut self) -> Result<Element, XtceParserError> {
        let lineno = *self.lineno_ref.borrow();

        match self.event_reader.next() {
            Err(e) => Err(XtceParserError::XmlError(lineno, Box::new(e))),
            Ok(event) => Ok(Element::new(lineno, event)),
        }
    }
}

// This is how we get line numbers for the XML file
// FIXME: handle nested files
pub struct LinenoReader<R: Read> {
    inner: BufReader<R>,
    line: Rc<RefCell<usize>>,
}

impl<R: Read> LinenoReader<R> {
    pub fn new(inner: R) -> Self {
        LinenoReader {
            inner: BufReader::new(inner),
            line: Rc::new(RefCell::new(1)),
        }
    }

    pub fn lineno_ref(&self) -> Rc<RefCell<usize>> {
        Rc::clone(&self.line)
    }
}

impl<R: Read> Read for LinenoReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        let mut line = self.line.borrow_mut();
        *line += buf[..bytes_read].iter().filter(|&&c| c == b'\n').count();
        Ok(bytes_read)
    }
}
