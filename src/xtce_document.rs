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

use crate::Container;
use crate::Parameter;
use crate::parser::{LineNumber, Parser, XmlElement};
use crate::space_system::*;
use crate::xtce::Xtce;
use crate::xtce_parser_error::{XtceParserError};

pub struct ElementDesc {
    pub name:                   &'static str, 
    pub allowable_subelements:  &'static[ElementDesc]
}

impl ElementDesc {
    pub fn position(&self, target: &String) -> Option<usize> {
        let mut pos: usize = 0;

        for element in self.allowable_subelements {
            if element.name == target {
                return Some(pos);
            }
            pos += 1;
        }

        return None
    }

    pub fn dump(&self, indent: usize) {
        print!("{}", "   ".to_string().repeat(indent));
        println!("{}", self.name);
        for n in self.allowable_subelements {
            n.dump(indent + 1);
        }
    }
}

impl fmt::Debug for ElementDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.allowable_subelements)
    }
}

/*
 * Define the structure used to construct the tree for the parsed document.
 */
#[derive(Clone, Debug)]
pub struct Element {
    lineno:             LineNumber,
    name:               OwnedName,
    attributes:         Vec<OwnedAttribute>,
    namespace:          Namespace,
    pub subelements:    Vec<Element>,
    before_comments:    Vec<String>,
    after_comments:     Vec<String>,
}

impl Element {
    fn new(lineno: LineNumber, name: OwnedName, attributes: Vec<OwnedAttribute>,
        namespace: Namespace) -> Element {
        Element {
            lineno:             lineno,
            name:               name,
            attributes:         attributes,
            namespace:          namespace,
            subelements:        Vec::<Element>::new(),
            before_comments:    Vec::<String>::new(),
            after_comments:     Vec::<String>::new(),
        }
    }
}

/*
 * Define the XTCE document description tree structure
 */
pub const ROOT_DESC: ElementDesc = ElementDesc {
    name:                   "DOCUMENT_ROOT",
    allowable_subelements:  &[XTCE_DESC],
};

pub const XTCE_DESC: ElementDesc = ElementDesc {
    name:                   "XTCE",
    allowable_subelements:  &[SPACE_SYSTEM_DESC],
};

const SPACE_SYSTEM_DESC: ElementDesc = ElementDesc {
    name:                   "SpaceSystem",
    allowable_subelements:  &[TELEMETRY_META_DATA_DESC, COMMAND_META_DATA_DESC],
};

const TELEMETRY_META_DATA_DESC: ElementDesc = ElementDesc {
    name:                   "TelemetryMetaData",
    allowable_subelements:  &[PARAMETER_SET_DESC, CONTAINER_SET_DESC,
                            MESSAGE_SET_DESC, STREAM_SET_DESC, ALGORITHM_SET_DESC],
};

const PARAMETER_SET_DESC: ElementDesc = ElementDesc {
    name:                   "ParameterSet",
    allowable_subelements:  &[PARAMETER_DESC],
};

const PARAMETER_DESC: ElementDesc = ElementDesc {
    name:                   "Parameter",
    allowable_subelements:  &[DESCRIPTION_DESC],
};

// FIXME: is <Description> valid?
const DESCRIPTION_DESC: ElementDesc = ElementDesc {
    name:                   "Description",
    allowable_subelements:  &[],
};

const CONTAINER_SET_DESC: ElementDesc = ElementDesc {
    name:                   "ContainerSet",
    allowable_subelements:  &[],
};

const MESSAGE_SET_DESC: ElementDesc = ElementDesc {
    name:                   "MessageSet",
    allowable_subelements:  &[],
};

const STREAM_SET_DESC: ElementDesc = ElementDesc {
    name:                   "StreamSet",
    allowable_subelements:  &[],
};

const ALGORITHM_SET_DESC: ElementDesc = ElementDesc {
    name:                   "AlgorithmSet",
    allowable_subelements:  &[],
};

const COMMAND_META_DATA_DESC: ElementDesc = ElementDesc {
    name:                   "CommandMetaData",
    allowable_subelements:  &[],
};

#[derive(Debug)]
pub struct XtceDocument {
    version:        XmlVersion,
    encoding:       String,
    standalone:     Option<bool>,
    root:           Element,
}

impl XtceDocument {
    pub fn new(path: String) -> Result<XtceDocument, XtceParserError> {
        let file = match File::open(path) {
            Err(e) => return Err(XtceParserError::XmlError(0, Box::new(e))),
            Ok(f) => f,
        };
        let mut buf_reader = BufReader::new(file);
        Self::new_from_reader(buf_reader)
    }

    pub fn new_from_reader<R: Read>(buf_reader: BufReader<R>) -> Result<XtceDocument, XtceParserError> {
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
        let space_system = SpaceSystemType::new();

        let version = XmlVersion::Version11;
        let encoding = "unknown".to_string();
        let standalone: Option<bool> = None;
                        
        let (lineno, version, encoding, standalone) =
            Self::parse_start_document(&mut parser)?;
println!("Processing document");
        let xtce_document = Self::parse_end_document(&mut parser, &ROOT_DESC,
            (lineno, version, encoding, standalone));

        xtce_document
    }

    /*
     * Parse the StartDocument event.
     */
    fn parse_start_document<R: Read>(parser: &mut Parser<R>) ->
        Result<(LineNumber, XmlVersion, String, Option<bool>), XtceParserError> {
        let mut comments_before = Vec::<String>::new();
        let mut comments_after = Vec::<String>::new();

        let (lineno, version, encoding, standalone) = loop {
            let xml_element = parser.next();

            match xml_element {
                Err(e) => return Err(XtceParserError::NoXTCE()),
                Ok(evt) => {
                    let lineno = evt.lineno;

                    match evt.event {
                        XmlEvent::StartDocument{version, encoding, standalone} => {
println!("Got StartDocument");
                            break (lineno, version, encoding, standalone)
                        },
                        XmlEvent::EndDocument => {
                            return Err(XtceParserError::NoXTCE());
                        },
                        XmlEvent::Comment(cmnt) => {
println!("Got comment");
                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(ws) => {
println!("Skipping whitespace");
                            continue;
                        },
                        _ => return Err(XtceParserError::UnexpectedXml(evt.event))
                    }
                }
            };
        };

        Ok((lineno, version, encoding, standalone))
    }

    /*
     * Parse until we find an EndDocument
     */
    fn parse_end_document<R: Read>(parser: &mut Parser<R>, desc: &ElementDesc,
        info: (LineNumber, XmlVersion, String, Option<bool>)) ->
        Result<XtceDocument, XtceParserError> {
println!("parse_end_document: {:?}", desc);

        let mut subelements = Vec::<Element>::new();
        let mut start_name = "".to_string();

        let mut comments_before = Vec::<String>::new();
        let mut comments_after = Vec::<String>::new();

        loop {
            let xml_element = parser.next();

            match xml_element {
                Err(e) => {
                    return Err(XtceParserError::XmlError(0, Box::new(e))); // FIXME: line number
                },
                Ok(evt) => {
                    let lineno = evt.lineno;

                    match evt.event {
                        XmlEvent::StartDocument{..} => {
                            return Err(XtceParserError::StartAfterStart(lineno));
                        },
                        XmlEvent::EndDocument => {
                            return Err(XtceParserError::Unknown(0));
                        },
                        XmlEvent::StartElement{name, attributes, namespace} => {
                            start_name = name.local_name.clone();
        println!("parse_end_document: StartElement {:?}: attributes {:?} namespace {:?}", start_name, attributes, namespace);
        println!("desc: {:?}", desc);
println!("desc.position({}: {:?}", start_name, desc.allowable_subelements.iter().position(|x| {println!("--> name: {:?}", x.name); x.name == start_name}));
                            match desc.allowable_subelements.iter().position(|x| x.name == start_name) {
                                None => return Err(XtceParserError::UnknownElement(lineno, start_name)),
                                Some(pos) => {
                                    let new_desc = &desc.allowable_subelements[pos];
                                    let subelement = Self::parse_subelement(parser,
                                        lineno, name, attributes, namespace,
                                        &new_desc)?;
                                    subelements.push(subelement);
                                    break;
                                }
                            };
                        }
                        XmlEvent::EndElement{name} => {
                            return Err(XtceParserError::MisplacedElementEnd(lineno,
                                name.local_name));
                        },
                        XmlEvent::Comment(cmnt) => {
println!("Skipping comment");
//                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(ws) => {
println!("Skipping whitespace");
                            continue;
                        },
                        XmlEvent::Characters(characters) => {
println!("Skipping characters");
                            continue;
                        },
                        XmlEvent::CData(cdata) => {
println!("Skipping cdata");
                            continue;
                        },
/*
                        XmlEvent::ProcessingInstruction(processing_instruction) => {
println!("Skipping processing_instruction");
                            continue;
                        },
*/
                        _ => return Err(XtceParserError::UnexpectedXml(evt.event))
                    }
                }
            }
        }

        if subelements.len() != 1 {
            return Err(XtceParserError::OnlyOneRootElement(info.0));
        }

        Ok(XtceDocument {
            version:    info.1,
            encoding:   info.2,
            standalone: info.3,
            root:       subelements[0].clone(),
        })
    }

    fn parse_subelement<R: Read>(parser: &mut Parser<R>, lineno: LineNumber,
        name: OwnedName, attributes: Vec<OwnedAttribute>, namespace: Namespace,
        desc: &ElementDesc) ->
        Result<Element, XtceParserError> {
println!("\nparse_subelement: {}/{}", name.local_name, desc.name);
        let mut subelements = Vec::<Element>::new();
        let mut start_name = "".to_string();

        loop {
            let xml_element = parser.next();

            match xml_element {
                Err(e) => {
                    return Err(XtceParserError::XmlError(0, Box::new(e))); // FIXME: line number
                },
                Ok(evt) => {
                    let lineno = evt.lineno;

                    match evt.event {
                        XmlEvent::StartDocument{..} => {
                            return Err(XtceParserError::StartAfterStart(lineno));
                        },
                        XmlEvent::EndDocument => {
                            return Err(XtceParserError::Unknown(0));
                        },
                        XmlEvent::StartElement{name, attributes, namespace} => {
                            let start_name = name.local_name.clone();
println!("desc: {:?}", desc);
println!("desc.position({}: {:?}", start_name, desc.allowable_subelements.iter().position(|x| {println!("--> name: {:?}", x.name); x.name == start_name}));
                            match desc.allowable_subelements.iter().position(|x| x.name == start_name) {
                                None => return Err(XtceParserError::UnknownElement(lineno, start_name)),
                                Some(pos) => {
                                    let new_desc = &desc.allowable_subelements[pos];
                                    let subelement = Self::parse_subelement(parser,
                                        lineno, name, attributes, namespace,
                                        &new_desc)?;
                                    subelements.push(subelement);
                                }
                            }
                            
                        }
                        XmlEvent::EndElement{name} => {
println!("EndElement: name.local_name {}, desc.name {}", name.local_name, desc.name);
                            if name.local_name != desc.name {
                                return Err(XtceParserError::MisplacedElementEnd(lineno,
                                    name.local_name));
                            }

                            return Ok(Element::new(lineno, name, attributes, namespace));
                        },
                        XmlEvent::Comment(cmnt) => {
println!("Skipping comment");
//                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(ws) => {
println!("Skipping whitespace");
                            continue;
                        },
                        XmlEvent::Characters(characters) => {
println!("Skipping characters");
                            continue;
                        },
                        XmlEvent::CData(cdata) => {
println!("Skipping cdata");
                            continue;
                        },
/*
                        XmlEvent::ProcessingInstruction(processing_instruction) => {
println!("Skipping processing_instruction");
                            continue;
                        },
*/
                        _ => {
                            println!("Unknown event: {:?}", evt.event);
                            return Err(XtceParserError::UnexpectedXml(evt.event));
                        }
//                        _ => return Err(XtceParserError::UnexpectedXml(evt.event));
                    }
                }
            }
        }
    }
}

impl Display for XtceDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<?xml {} {} {:?}\n{:?}\n",
            self.version, self.encoding, self.standalone, self.root)
    }
}
