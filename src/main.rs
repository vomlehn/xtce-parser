/*
 * Options:
 * --prefix Adds SpaceSystem-derived names to everything
 */

mod xml_lineno;
mod basic;
mod parameter_type_set;
mod sequence_container;
mod space_system;
mod stream_set;
mod telemetry_meta_data;

extern crate xml;
use std::fs::File;
use xml_lineno::{LineReader};
use xml::reader::{EventReader, XmlEvent};

use crate::basic::*;
use crate::telemetry_meta_data::*;
use crate::stream_set::*;

#[derive(Clone, Debug)]
struct Parameter {
    name: String,
    data_type: String,
}

#[derive(Clone, Debug)]
struct Container {
    name: String,
    parameters: Vec<Parameter>,
}

fn parse_xtce(file_path: &str) -> Result<Vec<Container>, Box<dyn std::error::Error>> {
    // Similar setup as before...
    let mut containers = Vec::new();
    let mut current_container = Container {
        name: String::new(),
        parameters: Vec::new(),
    };

    let file = File::open(file_path)?;
    let reader= LineReader::new(file);
    let line_ref = reader.line_ref();
    let parser = EventReader::new(reader);
    
    let mut parameters: Vec<Parameter> = Vec::new();
    let mut current_name = String::new();
    let mut current_type = String::new();
    
    // Parsing logic...
    
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
            XmlEvent::ProcessingInstruction {name, data} => {
println!("ProcessingInstruction");
            }
            XmlEvent::StartElement { name, attributes, namespace } => {
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
                        name: current_name.clone(),
                        data_type: current_type.clone(),
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
    
    Ok(containers)
} 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parameters = parse_xtce("test/test1.xtce")?;
    
    for param in parameters {
        println!("{:?}", param);
    }
    
    Ok(())
}
