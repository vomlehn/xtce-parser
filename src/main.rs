/*
 * Options:
 * --prefix Adds SpaceSystemType-derived names to everything
 */

extern crate xml;

mod gen_c;
mod gen_xtce;
mod parser;
mod rust_data_types;
mod space_system;
mod space_system_converted;
mod xtce;
mod xml_document;
mod xtce_parser_error;

use gen_c::generate_c;
use gen_xtce::generate_xtce;
use xml_document::XtceDocument;
use xtce_parser_error::{XtceParserError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = parse_file("test/test5.xtce".to_string());

    match &document {
        Err(e) => println!("Failed: {:?}", e),
        Ok(d) => generate_c(d),
    }
/*
    
    for param in parameters {
        println!("{:?}", param);
    }
*/
    
    Ok(())
}

fn parse_file(file_path: String) -> Result<XtceDocument, XtceParserError> {
    // Similar setup as before...
/*

    let file = match File::open(file_path) {
        Err(e) => return Err(XtceParserError::GeneralError(0, Box::new(e))),
        Ok(file) => file,
    };

    let mut buf_reader = BufReader::new(file);
*/

    XtceDocument::new(file_path)
}
