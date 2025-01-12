use std::fmt::Display;

use std::collections::{HashSet, HashMap};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};
use std::rc::Rc;
use std::cell::RefCell;
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::{EventReader, XmlEvent};

use crate::command_meta_data::*;
use crate::Container;
use crate::Parameter;
use crate::parser::{Element, Parser};
use crate::space_system::*;

use crate::space_system::{SpaceSystemType, HeaderType, ServiceSetType, SpaceSystemV1_1};
use crate::telemetry_meta_data::*;
use crate::xtce::Xtce;
use crate::xtce_parser_error::{XtceParserError};

#[derive(Debug)]
pub struct XtceDocument {
    version:        XmlVersion,
    encoding:       String,
    standalone:     Option<bool>,
    space_system:   SpaceSystemType,
}

impl XtceDocument {

fn x() -> Result<(), Box<dyn std::error::Error>> {
    let _file = File::open("path/to/file")?; 
    Ok(()) 
}

    pub fn new(path: String) -> Result<XtceDocument, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        Self::new_from_reader(buf_reader)
    }

    pub fn new_from_reader<R: Read>(buf_reader: BufReader<R>) -> Result<XtceDocument, Box<dyn Error>> {
//        let mut containers = Vec::new();
        let mut current_container = Container {
            _name: r#String::new(),
            parameters: Vec::new(),
        };
        let mut parameters: Vec<Parameter> = Vec::new();
        let mut current_name = r#String::new();
        let mut current_type = r#String::new();
        let mut event: Result<XmlEvent, xml::reader::Error>;
        let mut space_system = SpaceSystemType::new();

        let mut parser = Parser::<R>::new(buf_reader);
        let mut comment = None;

        /* The only thing legal now is a StartDocument */
        let (version, encoding, standalone) = loop {
            let element = parser.next();
println!("Got element {:?}", element);
            let info = match element {
                Err(e) => return Err(Box::new(XtceParserError::NoXTCE())),
                Ok(element) => {
                    match element.event {
                        XmlEvent::StartDocument{version, encoding, standalone} => {
println!("Got StartDocument");
                            break (version, encoding, standalone);
                        },
                        XmlEvent::Comment(cmnt) => {
println!("Got comment");
                            if comment.is_some() {
                                comment = Some("\n".to_owned() + &cmnt);
                            }
                        },
                        XmlEvent::Whitespace(ws) => {
println!("Skipping whitespace")
                        },
                        _ => return Err(Box::new(XtceParserError::UnexpectedXml(element.event)))
                    }
                }
            };

            info
        };
println!("Processing document");

        match Self::xtce_document_new(&mut parser) {
            Err(e) => return Err(Box::new(e)),
            Ok(space_system) => {
                return Ok(XtceDocument {
                    version:        version,
                    encoding:       encoding,
                    standalone:     standalone,
                    space_system:   space_system,
                })
            }
        }
    }

    fn xtce_document_new<R: Read>(parser: &mut Parser<R>) -> Result<SpaceSystemType, XtceParserError> {
//        let mut containers = Vec::new();
        let mut current_container = Container {
            _name: r#String::new(),
            parameters: Vec::new(),
        };
        let mut parameters: Vec<Parameter> = Vec::new();
        let mut current_name = r#String::new();
        let mut current_type = r#String::new();
        let mut event: Result<XmlEvent, xml::reader::Error>;

/*
"SpaceSystem"
"TelemetryMetaData"
"ParameterSet"
"Parameter"
"Description"
"Parameter"
*/
        /*
         * Once we've seen StartDocument, we expect <SpaceSystem>.
         */
        let element = parser.next();
println!("Expecting SpaceSystem: {:?}", element);
        let info = match element {
            Err(e) => return Err(XtceParserError::NoXTCE()),
            Ok(element) => {
                match element.event {
                    XmlEvent::StartElement{name, attributes, namespace} => {
                        match name.local_name.as_str() {
                            "XTCE" => (name, attributes, namespace),
                            _ => return Err(XtceParserError::BadXtceStart(element.lineno, name.local_name)),
                        }
                    },
                    _ => return Err(XtceParserError::UnexpectedXml(element.event))
                }
            }
        };

        let mut space_system = SpaceSystemType::new();

        loop {
            let element = parser.next();
//println!("Next element: {:?}", element);

            match element {
                Err(e) => return Err(XtceParserError::Unknown(0)),
                Ok(el) => {
                    let lineno = el.lineno;

                    match el.event {
                        XmlEvent::StartDocument {version, encoding, standalone} => {
println!("StartDocument");
                            return Err(XtceParserError::MultipleSpaceSystems(lineno));
                        }
                        XmlEvent::EndDocument => {
println!("EndDocument");
                            return Ok(space_system);
                        }
                        XmlEvent::ProcessingInstruction {..} => {
            println!("ProcessingInstruction");
                        }
                        XmlEvent::StartElement {name, attributes, namespace } => {
//                            Self::start_element(lineno, parser, name, attributes, namespace);
                println!("Line {}: StartElement name: {:?} [start_element]", lineno, name.local_name);

                            match name.local_name.as_str() {
                                "SpaceSystem" => {

                                    match Self::parse_space_system_body(parser) {
                                        Err(e) => return Err(e),
                                        Ok((header, telemetry_meta_data, command_meta_data, service_set,
                                            space_system_ref)) => {
                                            space_system.header = header;
                                            space_system.telemetry_meta_data = telemetry_meta_data;
                                            space_system.command_meta_data = command_meta_data;
                                            space_system.service_set = service_set;
                                            space_system.space_system_ref = space_system_ref;
                                        },
                                    }
                                },
                                _ => return Err(XtceParserError::UnknownElement(lineno, name.local_name))
                            }
                        }
                        XmlEvent::EndElement { name } => {
            println!("Line {}: EndElement {:?}", lineno, name.local_name);
                            match name.local_name.as_str() {
                                "SpaceSystem" => {
                                },
                                _ => {
                                    return Err(XtceParserError::BadXtceEnd(lineno, name.local_name));
                                }
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
                    }
                }
            }
        }

        return Err(XtceParserError::UnexpectedTermination(0, "FIXME TBD"));
    }

    fn parse_space_system_body<R: Read>(parser: &mut Parser<R>) ->
        Result<(Option<HeaderType>, Option<TelemetryMetaDataType>, Option<Box<CommandMetaDataType>>,
            Option<ServiceSetType>, Vec<SpaceSystemV1_1>), XtceParserError> {
        Ok((None, None, None, None, Vec::<SpaceSystemV1_1>::new()))
    }
}

impl Display for XtceDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "version: {} encoding {} standalone{:?} space_system {:?}",
            self.version, self.encoding, self.standalone, self.space_system)
    }
}
