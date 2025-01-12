/*
 * Options:
 * --prefix Adds SpaceSystemType-derived names to everything
 */

extern crate xml;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::rc::Rc;
use std::cell::RefCell;

mod basic;
mod gen_c;
mod parameter_type_set;
mod parser;
mod sequence_container;
mod space_system;
mod stream_set;
mod telemetry_meta_data;
mod xml_lineno;
mod xtce;
mod xtce_document;
mod xtce_parser_error;

use gen_c::generate_c;
use space_system::{SpaceSystemType};
use xml::reader::{XmlEvent};
use xtce::Xtce;
use xtce_document::XtceDocument;
use xtce_parser_error::{XtceParserError};

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
    let document = parse_file("test/test1.xtce");

    println!("document: {:?}", document);
/*
    
    for param in parameters {
        println!("{:?}", param);
    }
*/
    
    Ok(())
}

fn parse_file(file_path: &str) -> Result<XtceDocument, XtceParserError> {
    // Similar setup as before...

    let file = match File::open(file_path) {
        Err(e) => return Err(XtceParserError::GeneralError(0, Box::new(e))),
        Ok(file) => file,
    };

    let mut buf_reader = BufReader::new(file);

    match XtceDocument::new(buf_reader) {
        Err(e) => Err(e),
        Ok(document) => {
            generate_c(&document);
            Ok(document)
        }
    }
}
