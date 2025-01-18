use getopts::{Options};
use std::env;
use std::process::exit;

/*
 * Options:
 * --prefix Adds SpaceSystemType-derived names to everything
 */

extern crate xml;

mod gen_c;
mod gen_xtce;
mod parser;
mod rust_data_types;
mod space_system_desc;
mod space_system_converted_desc;
mod xtce;
mod xml_document;
mod xtce_parser_error;

use gen_c::generate_c;
use gen_xtce::generate_xtce;
use xml_document::XtceDocument;
use xtce_parser_error::{XtceParserError};

fn usage(program: &str, opts: Options) -> ! {
    let brief = format!("Usage: {} [options] input-file", program);
    print!("{}", opts.usage(&brief));
    exit(1); 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "Print this help message");
    opts.optflag("x", "xtce", "Generate XTCE output");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => { 
            println!("{}", f); 
            usage(&program, opts);
        }
    };

    if matches.opt_present("h") {
        usage(&program, opts);
    }

    let remaining_args: Vec<&str> = matches.free.iter().map(|s| s.as_str()).collect();
    if remaining_args.len() != 1 {
        usage(&program, opts);
    }

    let input_file = remaining_args[0];

    let document = parse_file(input_file.to_string());

    match &document {
        Err(e) => {
            println!("Failed: {:?}", e);
            exit(1);
        },
        Ok(d) => {
            println!("=====================================================");

            if matches.opt_present("x") {
                    generate_xtce(d)
            } else {
                    generate_c(d)
            }
        }
    };
    
    Ok(())
}

fn parse_file(file_path: String) -> Result<XtceDocument, XtceParserError> {
    XtceDocument::new(file_path)
}
