/*
 * Options:
 * --prefix Adds SpaceSystemType-derived names to everything
 */

extern crate xml;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::cell::RefCell;

mod basic;
mod gen_c;
mod parameter_type_set;
mod sequence_container;
mod space_system;
mod stream_set;
mod telemetry_meta_data;
mod xml_lineno;
mod xtce;
mod xtce_parser_error;


use gen_c::generate_c;
use space_system::{SpaceSystemType};
use xtce_parser_error::{XtceParserError};
use xml_lineno::{LineReader};
use xml::reader::{EventReader, XmlEvent};
use xtce::Xtce;

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

    let x = XtceParserError::Unknown(12345);
    println!("x: {}", x);

    let xtce = parse_file("test/test1.xtce");

    println!("xtce: {:?}", xtce);
/*
    
    for param in parameters {
        println!("{:?}", param);
    }
*/
    
    Ok(())
}

fn parse_file(file_path: &str) -> Result<Xtce, XtceParserError> {
    // Similar setup as before...

    let file = match File::open(file_path) {
        Err(e) => return Err(XtceParserError::GeneralError(0, Box::new(e))),
        Ok(file) => file,
    };

    match Xtce::new(file) {
        Err(e) => Err(e),
        Ok(xtce) => {
            generate_c(&xtce);
            Ok(xtce)
        }
    }
}
