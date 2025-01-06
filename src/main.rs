/*
 * Options:
 * --prefix Adds SpaceSystem-derived names to everything
 */

extern crate xml;

use std::error::Error;
use std::fs::File;
use std::io::Read;

mod basic;
mod document;
mod error;
mod parameter_type_set;
mod sequence_container;
mod space_system;
mod stream_set;
mod telemetry_meta_data;
mod xml_lineno;


use document::{Document};
use error::{XtceParserError};
use xml_lineno::{LineReader};
use xml::reader::{EventReader, XmlEvent};

#[derive(Clone, Debug)]
struct Parameter {
    _name: r#String,
    _data_type: r#String,
}

#[derive(Clone, Debug)]
struct Container {
    _name: r#String,
    parameters: Vec<Parameter>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let x = XtceParserError::Unknown;
    println!("x: {}", x);

    let parameters = parse_file("test/test1.xtce")?;
/*
    
    for param in parameters {
        println!("{:?}", param);
    }
*/
    
    Ok(())
}

fn parse_file(file_path: &str) -> Result<Vec<Container>, Box<dyn std::error::Error>> {
    // Similar setup as before...
    let mut containers = Vec::new();
    let mut current_container = Container {
        _name: r#String::new(),
        parameters: Vec::new(),
    };

    let file = File::open(file_path)?;
    let reader= LineReader::new(file);
    let line_ref = reader.line_ref();
    let mut parser = EventReader::new(reader);
    
    let mut parameters: Vec<Parameter> = Vec::new();
    let mut current_name = r#String::new();
    let mut current_type = r#String::new();
    
    // Parsing logic...
println!("calling parse_document");
    let document = parse_document(&mut parser);
println!("called parse_document");
    
/*
    for event in parser {
        let line = *line_ref.borrow();

print!("Line {}: ", line);
/*
if event.is_ok() {
    println!("Line {}: event: {:?}", event.clone().unwrap());
} else {
    println!("Line {}: Is not okay");
}
*/
        match event.clone()? {
            XmlEvent::StartDocument {version, encoding, standalone} => {
println!("StartDocument ({:?}", event);
println!("    version {:?} encoding {:?} standalone {:?}", version, encoding, standalone);
            }
            XmlEvent::EndDocument => {
println!("EndDocument");
            }
            XmlEvent::ProcessingInstruction {..} => {
println!("ProcessingInstruction");
            }
            XmlEvent::StartElement {name, attributes, namespace } => {
    println!("StartElement name: {} {:?}", name.local_name, event);
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
            }
            XmlEvent::EndElement { name } => {
println!("EndElement {:?}", name.local_name);
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
println!("Whitespace");
            }
//            _ => {}
        }
    }
*/
    
    Ok(containers)
} 

fn parse_document<R: Read>(event_reader: &mut EventReader<R>) ->
    Result<Document, XtceParserError> {
    let ev = event_reader.next();
    let document = match ev {
        Err(e) => return Err(XtceParserError::GeneralError(Box::new(e))),
        Ok(d) => {
            match d {
                XmlEvent::StartDocument {version, encoding, standalone} => {
                    Document::new(version, encoding, standalone)
                },
                _ => return Err(XtceParserError::Unknown),
            }
        }
        Err(e) => return Err(XtceParserError::GeneralError(Box::new(e))),
        Ok(d) => {
            match d {
                XmlEvent::StartDocument {version, encoding, standalone} => {
                    Document::new(version, encoding, standalone)
                },
                _ => return Err(XtceParserError::Unknown),
            }
        }
    };
    println!("document: {:?}", document);

    parse_xtce(event_reader);

    let ev = event_reader.next();
    match ev {
        Err(e) => return Err(XtceParserError::GeneralError(Box::new(e))),
        Ok(d) => {
            match d {
                XmlEvent::EndDocument => {
                },
                _ => return Err(XtceParserError::Unknown),
            }
        }
    }

    Ok(document)
}

fn parse_xtce<R: Read>(event_reader: &mut EventReader<R>) ->
    Result<Document, XtceParserError> {
    return Err(XtceParserError::Unknown);
}
