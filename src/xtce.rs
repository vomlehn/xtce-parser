
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
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

impl fmt::Display for Element {
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
println!("Processing document");
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
                        XmlEvent::Whitespace(_) => {
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
println!("parse_end_document: {:?}", desc.name);

        let mut subelements = HashMap::<String, Vec::<Element>>::new();
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
                            start_name = name.local_name.clone();
        println!("parse_end_document: StartElement {:?}: attributes {:?} namespace {:?}", start_name, attributes, namespace);
                            match desc.allowable_subelements.iter().position(|x| x.name == start_name) {
                                None => return Err(XtceParserError::UnknownElement(lineno, start_name)),
                                Some(pos) => {
                                    let new_desc = &desc.allowable_subelements[pos];
                                    let subelement = Self::parse_subelement(parser,
                                        name, attributes, namespace,
                                        &new_desc)?;
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
println!("Skipping comment");
//                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(_ws) => {
println!("Skipping whitespace");
                            continue;
                        },
                        XmlEvent::Characters(_characters) => {
println!("Skipping characters");
                            continue;
                        },
                        XmlEvent::CData(_cdata) => {
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

        // Get the root element
println!("{} subelement types", subelements.len());
        if subelements.len() != 1 {
            return Err(XtceParserError::OnlyOneRootElement(info.0));
        }

        let root = match subelements.get(&start_name) {
            None => return Err(XtceParserError::Unknown(0)),
            Some(root_vec) => {
println!("{} subelements in type {}", root_vec.len(), start_name);
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

println!("root subelements finally {:?}", subelements.len());
        Ok(XtceDocument {
            version:    info.1,
            encoding:   info.2,
            standalone: info.3,
            root:       root.clone(),
        })
    }

    fn parse_subelement<R: Read>(parser: &mut Parser<R>,
        name: OwnedName, attributes: Vec<OwnedAttribute>, namespace: Namespace,
        desc: &ElementDesc) ->
        Result<Element, XtceParserError> {
println!("\nparse_subelement: enter {}/{}", name.local_name, desc.name);
        let mut subelements = HashMap::<String, Vec::<Element>>::new();
        let start_name = "".to_string();

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
println!("parse_subelement: event {:?}", desc);
                            match desc.allowable_subelements.iter().position(|x| x.name == start_name) {
                                None => return Err(XtceParserError::UnknownElement(lineno, start_name)),
                                Some(pos) => {
                                    let new_desc = &desc.allowable_subelements[pos];
                                    let subelement = Self::parse_subelement(parser,
                                        name, attributes, namespace,
                                        &new_desc)?;
                                    Self::push_subelement(&mut subelements, start_name.clone(),
                                        subelement);
println!("{} subelements now {:?}", start_name, subelements.len());
                                }
                            }
                            
                        }
                        XmlEvent::EndElement{name} => {
println!("EndElement: name.local_name {}, desc.name {}", name.local_name, desc.name);
println!("{} subelements finaally {:?}", start_name, subelements.len());
                            if name.local_name != desc.name {
                                return Err(XtceParserError::MisplacedElementEnd(lineno,
                                    name.local_name));
                            }

                            let mut element = Element::new(lineno, name, attributes, namespace);
                            element.subelements = subelements;
                            return Ok(element)
                        },
                        XmlEvent::Comment(_cmnt) => {
println!("Skipping comment");
//                            comments_before.push(cmnt);
                            continue;
                        },
                        XmlEvent::Whitespace(_ws) => {
println!("Skipping whitespace");
                            continue;
                        },
                        XmlEvent::Characters(_characters) => {
println!("Skipping characters");
                            continue;
                        },
                        XmlEvent::CData(_cdata) => {
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

    fn push_subelement(subelements: &mut HashMap<String, Vec<Element>>,
        name: String, subelement: Element) {
        match subelements.get_mut(&name) {
            None => {
println!("Insert new HashMap element for {}", name);
                let mut v = Vec::<Element>::new();
                v.push(subelement);
                subelements.insert(name, v);
            },
            Some(v) => {
println!("Inserting to existing HashMap for {}", name);
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

impl fmt::Display for XtceDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<?xml {} {} {:?}>\n",
            self.version, self.encoding, self.standalone)?;
        write!(f, "{}", self.root)       
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Item {
    name: String,
    dependencies: Vec<Item>,
}

impl Item {
    fn new(name: &str) -> Self {
        Item {
            name: name.to_string(),
            dependencies: Vec::new(),
        }
    }

    fn add_dependency(&mut self, item: Item) {
        self.dependencies.push(item);
    }
}

impl Hash for Item {
    fn hash<H>(&self, _: &mut H)
    where H: Hasher
    {
        todo!();
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct TSortItem {
    value:  String,
    deps:   [String; 2],
}

impl TSortItem {
    fn new(v: &str, deps: [&str; 2]) -> TSortItem{
        TSortItem {
            value:  v.to_string(),
            deps:   [deps[0].to_string(), deps[1].to_string()],
        }
    }
}

#[derive(Debug)]
pub struct Xtce {
}

impl Xtce {
    pub fn new(_file: File) -> Result<(XmlEvent, Xtce), XtceParserError> {
        Err(XtceParserError::Unknown(0))
    }

/*
    fn parse_document_elements<R: Read>(&mut self, lineno: Rc<RefCell<usize>>, parser: &mut EventReader<R>) ->
       Result<XmlEvent, XtceParserError> {
        
        let mut containers = Vec::new();
        let mut current_container = Container {
            _name: r#String::new(),
            parameters: Vec::new(),
        };
        let mut parameters: Vec<Parameter> = Vec::new();
        let mut current_name = r#String::new();
        let mut current_type = r#String::new();
        let mut event: Result<XmlEvent, xml::reader::Error>;

        loop {
            let event = parser.next();
            let ev = event.clone();

            if let Err(e) = ev {
                return Err(XtceParserError::XmlError(*lineno.borrow(), Box::new(e)));
            }

            match ev.unwrap() {
                XmlEvent::StartDocument {version, encoding, standalone} => {
                    return Err(XtceParserError::MultipleSpaceSystems(*lineno.borrow()));
                }
                XmlEvent::EndDocument => {
    println!("EndDocument");
                    return Ok(event.unwrap());
                }
                XmlEvent::ProcessingInstruction {..} => {
    println!("ProcessingInstruction");
                }
                XmlEvent::StartElement {name, attributes, namespace } => {
                    Self::start_element(lineno.clone(), parser, name, attributes, namespace);
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
    println!("Line {}: EndElement {:?}", *lineno.borrow(), name.local_name);
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

        return Err(XtceParserError::UnexpectedTermination(*lineno.borrow(), "FIXME TBD"));
    }

    fn start_element<R: Read>(lineno: Rc<RefCell<usize>>, parser: &mut EventReader<R>, name: OwnedName, attributes: Vec<OwnedAttribute>, namespace: Namespace) ->
       Result<XmlEvent, XtceParserError> {
        let mut containers = Vec::new();
        let mut current_container = Container {
            _name: r#String::new(),
            parameters: Vec::new(),
        };
        println!("Line {}: StartElement name: {:?} [start_element]", *lineno.borrow(), name.local_name);
        let mut parameters: Vec<Parameter> = Vec::new();
        let mut current_name = r#String::new();
        let mut current_type = r#String::new();
        let mut event: Result<XmlEvent, xml::reader::Error>;

        match current_name.as_str() {
            "SpaceSystem" => {},
            "TelemetryMetaData" => {},
            "ParameterSet" => {},
            "Parameter" => {},
            "Description" => {}, // FIXME: verify this is right
            "Parameter" => {},
            _ => { return Err(XtceParserError::UnknownElement(*lineno.borrow(), current_name)); }
        };

        loop {
            let event = parser.next();
            let ev = event.clone();

            if let Err(e) = ev {
                return Err(XtceParserError::XmlError(*lineno.borrow(), Box::new(e)));
            }

            match ev.unwrap() {
                XmlEvent::StartDocument {version, encoding, standalone} => {
                    return Err(XtceParserError::MultipleSpaceSystems(*lineno.borrow()));
                }
                XmlEvent::EndDocument => {
                    return Err(XtceParserError::MultipleSpaceSystems(*lineno.borrow()));
                }
                XmlEvent::ProcessingInstruction {..} => {
    println!("ProcessingInstruction");
                }
                XmlEvent::StartElement {name, attributes, namespace } => {
                    Self::start_element(lineno.clone().clone(), parser, name, attributes, namespace);
                }
                XmlEvent::EndElement { name } => {
    println!("Line {}: EndElement {:?}", *lineno.borrow(), name.local_name);
                    if name.local_name == "container" {
                        containers.push(current_container.clone());
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

        return Err(XtceParserError::UnexpectedTermination(*lineno.borrow(), "FIXME TBD"));
    }

    // Thanks to ChapGPT for this
    fn resolve_dependencies<'a>(
        item: &'a Item,
        resolved: &mut Vec<&'a Item>,
        unresolved: &mut HashSet<&'a Item>,
    ) -> Result<(), String> {
        if unresolved.contains(item) {
            return Err(format!("Circular dependency detected: {}", item.name));
        }

        if resolved.contains(&item) {
            return Ok(()); // Already resolved
        }

        unresolved.insert(item);

        // Resolve all dependencies recursively
        for dep in &item.dependencies {
            Self::resolve_dependencies(dep, resolved, unresolved)?;
        }

        // Mark the item as resolved
        resolved.push(item);
        unresolved.remove(item);

        Ok(())
    }

    fn generate_output(items: &Vec<Item>) -> Result<Vec<&Item>, String> {
        let mut resolved: Vec<&Item> = Vec::new();
        let mut unresolved: HashSet<&Item> = HashSet::new();

        for item in items {
            Self::resolve_dependencies(item, &mut resolved, &mut unresolved)?;
        }

        Ok(resolved)
    }
*/

/* FIXME: remove this
    fn main() {
        let mut item1 = Item::new("item1");
        let mut item2 = Item::new("item2");
        let mut item3 = Item::new("item3");

        // Defining dependencies
        item1.add_dependency(item2);
        item2.add_dependency(item3);

        let items = vec![item1, item2, item3];

        match generate_output(items) {
            Ok(output) => {
                for item in output {
                    println!("{}", item.name);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
*/

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
