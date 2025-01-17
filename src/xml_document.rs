use std::fmt::Display;

use std::collections::{HashMap};
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

use crate::parser::{LineNumber, Parser};
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
    lineno:                 LineNumber,
    name:                   OwnedName,
    attributes:             Vec<OwnedAttribute>,
    namespace:              Namespace,
    subelements:            HashMap<String, Vec<Element>>,
    before_comments:        Vec<String>,
    after_comments:         Vec<String>,
}

impl Element {
    fn new(lineno: LineNumber, name: OwnedName, attributes: Vec<OwnedAttribute>,
        namespace: Namespace) -> Element {
        Element {
            lineno:             lineno,
            name:               name,
            attributes:         attributes,
            namespace:          namespace,
            subelements:        HashMap::<String, Vec<Element>>::new(),
            before_comments:    Vec::<String>::new(),
            after_comments:     Vec::<String>::new(),
        }
    }

    fn dump(&self) {
        let indent = 0;

        self.dump_indented(indent);
    }

    fn dump_indented(&self, indent: usize) {
        let indent_string = "   ".to_string().repeat(indent);
        print!("{}<{}", indent_string, self.name.local_name);

        for attribute in self.attributes.clone() {
            print!(" {}={}", attribute.name.local_name, attribute.value);
        }

        if self.subelements.len() == 0 {
            println!(" /> (line {})", self.lineno);
        } else {
            println!("> (line {})", self.lineno);
            for element_vec in self.subelements.values() {
                for element in element_vec {
                    element.dump_indented(indent + 1);
                }
            }
            println!("{}</{}>", indent_string, self.name.local_name);
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Element:\n")?;
        write!(f, "   Line {}: <{}>", self.lineno, self.name.local_name)?;
        write!(f, "   {} subelements\n", self.subelements.len())?;
        for element_vec in self.subelements.values() {
            for element in element_vec {
                write!(f, "   {}", element)?;
            }
        }
        write!(f, "")
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
        let buf_reader = BufReader::new(file);
        Self::new_from_reader(buf_reader)
    }

    pub fn new_from_reader<R: Read>(buf_reader: BufReader<R>) -> Result<XtceDocument, XtceParserError> {
        let mut parser = Parser::<R>::new(buf_reader);
        let (lineno, version, encoding, standalone) =
            Self::parse_start_document(&mut parser)?;
        let xml_document = Self::parse_end_document(&mut parser, &ROOT_DESC,
            (lineno, version, encoding, standalone));

        xml_document
    }

    /*
     * Parse the StartDocument event.
     */
    fn parse_start_document<R: Read>(parser: &mut Parser<R>) ->
        Result<(LineNumber, XmlVersion, String, Option<bool>), XtceParserError> {
        let mut comments_before = Vec::<String>::new();

        let (lineno, version, encoding, standalone) = loop {
            let xml_element = parser.next();

            match xml_element {
                Err(_) => return Err(XtceParserError::NoXTCE()),
                Ok(evt) => {
                    let lineno = evt.lineno;

                    match evt.event {
                        XmlEvent::StartDocument{version, encoding, standalone} => {
                            break (lineno, version, encoding, standalone)
                        },
                        XmlEvent::EndDocument => {
                            return Err(XtceParserError::NoXTCE());
                        },
                        XmlEvent::Comment(cmnt) => {
                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(_ws) => {
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

        let mut start_name = "".to_string();
        let mut subelements = HashMap::<String, Vec::<Element>>::new();

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
                            match desc.allowable_subelements.iter().position(|x| x.name == start_name) {
                                None => return Err(XtceParserError::UnknownElement(lineno, start_name)),
                                Some(pos) => {
                                    let new_desc = &desc.allowable_subelements[pos];
                                    let subelement = Self::parse_subelement(parser,
                                        attributes, namespace, &new_desc)?;
                                    Self::push_subelement(&mut subelements, start_name.clone(),
                                        subelement);
                                    break;
                                }
                            };
                        }
                        XmlEvent::EndElement{name} => {
                            return Err(XtceParserError::MisplacedElementEnd(lineno,
                                name.local_name));
                        },
                        XmlEvent::Comment(_cmnt) => {
//                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(_ws) => {
                            continue;
                        },
                        XmlEvent::Characters(_characters) => {
                            continue;
                        },
                        XmlEvent::CData(_cdata) => {
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

        // Get the root element
        if subelements.len() != 1 {
            return Err(XtceParserError::OnlyOneRootElement(info.0));
        }

        let root = match subelements.get(&start_name) {
            None => return Err(XtceParserError::Unknown(0)),
            Some(root_vec) => {
                if root_vec.len() != 1 {
                    return Err(XtceParserError::OnlyOneRootElement(info.0));
                }

                match root_vec.iter().next() {
                    // FIXME: Internal error
                    None => return Err(XtceParserError::Unknown(0)),
                    Some(r) => r,
                }
            }
        };

        Ok(XtceDocument {
            version:    info.1,
            encoding:   info.2,
            standalone: info.3,
            root:       root.clone(),
        })
    }

    fn parse_subelement<R: Read>(parser: &mut Parser<R>,
        attributes: Vec<OwnedAttribute>, namespace: Namespace,
        desc: &ElementDesc) ->
        Result<Element, XtceParserError> {
        let mut subelements = HashMap::<String, Vec::<Element>>::new();

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
                            match desc.allowable_subelements.iter().position(|x| x.name == start_name) {
                                None => return Err(XtceParserError::UnknownElement(lineno, start_name)),
                                Some(pos) => {
                                    let new_desc = &desc.allowable_subelements[pos];
                                    let subelement = Self::parse_subelement(parser,
                                        attributes, namespace,
                                        &new_desc)?;
                                    Self::push_subelement(&mut subelements, start_name.clone(),
                                        subelement);
                                }
                            }
                            
                        }
                        XmlEvent::EndElement{name} => {
                            if name.local_name != desc.name {
                                return Err(XtceParserError::MisplacedElementEnd(lineno,
                                    name.local_name));
                            }

                            let mut element = Element::new(lineno, name, attributes, namespace);
                            element.subelements = subelements;
                            return Ok(element)
                        },
                        XmlEvent::Comment(_cmnt) => {
//                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(_ws) => {
                            continue;
                        },
                        XmlEvent::Characters(_characters) => {
                            continue;
                        },
                        XmlEvent::CData(_cdata) => {
                            continue;
                        },
/*
                        XmlEvent::ProcessingInstruction(processing_instruction) => {
println!("Skipping processing_instruction");
                            continue;
                        },
*/
                        _ => {
                            return Err(XtceParserError::UnexpectedXml(evt.event));
                        }
//                        _ => return Err(XtceParserError::UnexpectedXml(evt.event));
                    }
                }
            }
        }
    }

    fn push_subelement(subelements: &mut HashMap<String, Vec<Element>>,
        name: String, subelement: Element) {
        match subelements.get_mut(&name) {
            None => {
                let mut v = Vec::<Element>::new();
                v.push(subelement);
                subelements.insert(name, v);
            },
            Some(v) => {
                v.push(subelement)
            }
        };
    }

    pub fn dump(&self) {
        println!("<?xml {} {} {:?}>",
            self.version, self.encoding, self.standalone);
        self.root.dump();
    }
}

impl Display for XtceDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<?xml {} {} {:?}>\n",
            self.version, self.encoding, self.standalone)?;
        write!(f, "{}", self.root)       
    }
}
