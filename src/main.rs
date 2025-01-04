mod xml_lineno;

extern crate xml;

use std::fs::File;
use xml_lineno::{LineReader};
use xml::reader::{EventReader, XmlEvent};

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

println!("{}: event: {:?}", line, event);
        match event? {
            XmlEvent::StartElement { name, attributes, .. } if name.local_name == "container" => {
                // Handle container start...
                for attr in attributes {
                    match attr.name.local_name.as_str() {
                        "name" => current_name = attr.value,
                        "dataType" => current_type = attr.value,
                        _ => {}
                    }
                }
            }
            XmlEvent::EndElement { name } if name.local_name == "container" => {
                containers.push(current_container.clone());
                current_container.parameters.clear();
            }
            XmlEvent::StartElement { name, attributes, .. } if name.local_name == "parameter" => {
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
            XmlEvent::StartElement { name, attributes, .. } if name.local_name == "parameter" => {
            }
            _ => {}
        }
    }
    
    Ok(containers)
} 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parameters = parse_xtce("test/test5.xtce")?;
    
    for param in parameters {
        println!("{:?}", param);
    }
    
    Ok(())
}
