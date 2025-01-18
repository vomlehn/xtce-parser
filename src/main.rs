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
mod rust_data_types;
mod space_system_desc;
mod space_system_converted_desc;
mod xtce;
mod xtce_parser_error;

use gen_c::generate_c;
use gen_xtce::generate_xtce;
use space_system_desc::ROOT_DESC;
use xml_tree::XmlTree;

fn usage(program: &str, opts: Options) -> ! {
    let brief = format!("Usage: {} [options] input-file", program);
    print!("{}", opts.usage(&brief));
    exit(1); 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug)]
    enum OutputType {
        C,
        Xtce,
        XtceDesc,
    }
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    let mut output_type = OutputType::C;

    opts.optflag("h", "help", "Print this help message");
    opts.optflag("x", "xtce", "Generate XTCE output");
    opts.optflag("d", "xtce-desc", "Generate XML descriptor output");

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
    if matches.opt_present("x") {
        output_type = OutputType::Xtce;
    }
    if matches.opt_present("d") {
        output_type = OutputType::XtceDesc;
    }
println!("output_type {:?}", output_type);

    let remaining_args: Vec<&str> = matches.free.iter().map(|s| s.as_str()).collect();
    if remaining_args.len() != 1 {
        usage(&program, opts);
    }

    if let OutputType::XtceDesc = output_type {
        println!("{}", ROOT_DESC);
        exit(0);
    }

    let input_file = remaining_args[0];

    let document = XmlTree::new(input_file.to_string(), &ROOT_DESC);

    match &document {
        Err(e) => {
            println!("Failed: {:?}", e);
            exit(1);
        },
        Ok(document) => {
            println!("=====================================================");

            match output_type {
                OutputType::C => generate_c(document),
                OutputType::Xtce => generate_xtce(document),
                _ => {
                    eprintln!("Unexpected output type: {:?}", output_type);
                    exit(1);
                }
            }
        }
    };
    
    Ok(())
}
