use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::rc::Rc;
use xml::common::XmlVersion;
use xml::reader::{EventReader, XmlEvent};

use crate::Container;
use crate::xtce_parser_error::{XtceParserError};
use crate::LineReader;
use crate::Parameter;
use crate::SpaceSystemType;

#[derive(Debug)]
pub struct Xtce {
    version:    XmlVersion,
    encoding:   String,
    standalone: Option<bool>,
}

impl Xtce {
    pub fn new(file: File) -> Result<Xtce, XtceParserError> {
        let mut buf_reader = BufReader::new(file);
        let line_reader = LineReader::new(buf_reader);
        let line_ref = line_reader.line_ref();
        let mut parser = EventReader::new(line_reader);
        
        let start_ev = parser.next();
        let mut xtce = match start_ev {
            Err(e) => return Err(XtceParserError::GeneralError(*line_ref.borrow(), Box::new(e))),
            Ok(x) => {
                match x.clone() {
                    XmlEvent::StartDocument {version, encoding, standalone} => {
println!("Start document is {:?}", x);
                        Xtce {
                            version:    version,
                            encoding:   encoding,
                            standalone: standalone,
                        }
                    },
                    _ => return Err(XtceParserError::Unknown(*line_ref.borrow())),
                }
            }
        };
        println!("xtce: {:?}", xtce);

        let end_ev = parse_xtce_elements(&mut xtce, line_ref.clone(), &mut parser);

        match end_ev {
            Err(e) => return Err(XtceParserError::GeneralError(*line_ref.borrow(), Box::new(e))),
            Ok(d) => {
                match d {
                    XmlEvent::EndDocument => {
                    },
                    _ => return Err(XtceParserError::Unknown(*line_ref.borrow())),
                }
            }
        }

        Ok(xtce)
    }
}

fn parse_xtce_elements<R: Read>(xtce: &mut Xtce, line_ref: Rc<RefCell<usize>>, parser: &mut EventReader<R>) ->
   Result<XmlEvent, XtceParserError> {
    
    let mut containers = Vec::new();
    let mut current_container = Container {
        _name: r#String::new(),
        parameters: Vec::new(),
    };
    let mut parameters: Vec<Parameter> = Vec::new();
    let mut current_name = r#String::new();
    let mut current_type = r#String::new();

    loop {
        let event = parser.next();
        let ev = event.clone();

        if let Err(e) = ev {
            return Err(XtceParserError::GeneralError(*line_ref.borrow(), Box::new(e)));
        }

        match ev.unwrap() {
            XmlEvent::StartDocument {version, encoding, standalone} => {
                return Err(XtceParserError::MultipleSpaceSystems(*line_ref.borrow()));
            }
            XmlEvent::EndDocument => {
println!("EndDocument");
                return Err(XtceParserError::Unknown(*line_ref.borrow()));
            }
            XmlEvent::ProcessingInstruction {..} => {
println!("ProcessingInstruction");
            }
            XmlEvent::StartElement {name, attributes, namespace } => {
    println!("Line {}: StartElement name: {}", *line_ref.borrow(), name.local_name);
/*
    println!("    attributes:");
    let a = attributes.clone();
for attr in a {
    println!("        {:?}", attr);
}
println!("    {:?}", attributes);
println!("    namespace: {:?}", namespace);
                if name.local_name == "container" {
                    // Handle container start...
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "name" => current_name = attr.value,
                            "dataType" => current_type = attr.value,
                            _ => {}
                        }
                    }
                } else if name.local_name == "parameter" {
                    // Handle parameter...
                    parameters.push(Parameter {
                        _name: current_name.clone(),
                        _data_type: current_type.clone(),
                    });
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "name" => current_name = attr.value,
                            "dataType" => current_type = attr.value,
                            _ => {}
                        }
                    }
                    current_name.clear();
                    current_type.clear();
                }
*/
            }
            XmlEvent::EndElement { name } => {
println!("Line {}: EndElement {:?}", *line_ref.borrow(), name.local_name);
                if name.local_name == "container" {
                    containers.push(current_container.clone());
                    current_container.parameters.clear();
                } else if name.local_name == "parameter" {
                }
            }
            XmlEvent::CData(_string) => {
println!("CData");
            }
            XmlEvent::Comment(_string) => {
println!("Comment");
            }
            XmlEvent::Characters(_string) => {
println!("Characters");
            }
            XmlEvent::Whitespace(_string) => {
//println!("Whitespace");
            }
//            _ => {}
        };
    }

    return Err(XtceParserError::Unknown(*line_ref.borrow()));
}

impl fmt::Display for Xtce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(version {}, encoding {} standalone {:?})", self.version, self.encoding, self.standalone)
    }
}

/*
impl fmt::Debug for Xtce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize the debug output
//        f.debug_struct("Xtce")
//            .field("x", &self.x)
//            .field("y", &self.y)
//            .finish()
        write!(f, "Debug not done for xtce")
    }
}
*/
