extern crate xml;

use std::fs::File;
use std::io::{self, Read};
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
    let file = io::BufReader::new(file);
    let parser = EventReader::new(file);
    
    let mut parameters = Vec::new();
    let mut current_name = String::new();
    let mut current_type = String::new();
    
    // Parsing logic...
    
    for event in parser {
println!("event: {:?}", event);
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
    let parameters = parse_xtce("test/test1.xtce")?;
    
    for param in parameters {
        println!("{:?}", param);
    }
    
    Ok(())
}
