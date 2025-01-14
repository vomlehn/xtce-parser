use std::cell::RefCell;
use std::collections::{HashSet, HashMap};
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};
use std::rc::Rc;
use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::{EventReader, XmlEvent};

//use crate::Container;
use crate::xtce_parser_error::{XtceParserError};
//use crate::Parameter;
use crate::space_system::SpaceSystemType;

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
    pub fn new(file: File) -> Result<(XmlEvent, Xtce), XtceParserError> {
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
