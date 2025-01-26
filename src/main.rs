use getopts::{Options};
use std::env;
use std::process::exit;
use xml_tree::{Element, XmlDocument};

extern crate xml;

mod xtce_definition;
mod xtce_options;

use xtce_definition::XTCE_DEFINITION;
use xtce_options::{XtceOptions, OutputType};

fn usage(program: &str, opts: Options) -> ! {
    let brief = format!("Usage: {} [options] input-file", program);
    print!("{}", opts.usage(&brief));
    exit(1); 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    let mut options = XtceOptions::new();

    opts.optflag("h", "help", "Print this help message");
    opts.optflag("d", "xsd-desc", "Print XTCE descriptors");
    opts.optopt("t", "telemetry-prefix", "Prefix for telemetry definitions",
        "TELEMETRY_PREFIX");
    opts.optopt("c", "command-prefix", "Prefix for telemetry definitions",
        "COMMAND_PREFIX");

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
    if matches.opt_present("d") {
        options.output_type = OutputType::XtceDesc;
    }
    if matches.opt_present("t") {
        match matches.opt_str("t") {
            None => usage(&program, opts),
            Some(tp) => options.telemetry_prefix = tp,
        };
    };
    if matches.opt_present("c") {
        match matches.opt_str("t") {
            None => usage(&program, opts),
            Some(cp) => options.command_prefix = cp,
        }
    };

    let remaining_args: Vec<&str> = matches.free.iter().map(|s| s.as_str()).collect();
    if remaining_args.len() != 1 {
        usage(&program, opts);
    }

    let input_file = remaining_args[0];

    let document = XmlDocument::new(input_file, &XTCE_DEFINITION);

    match &document {
        Err(e) => {
            println!("Failed: {:?}", e);
            exit(1);
        },
        Ok(document) => {
            println!("=====================================================");
            generate_xml_definition(document);
        }
    };
    
    Ok(())
}

pub fn generate_xml_definition(document: &XmlDocument) {
    generate_element(&document.root);
}

pub fn generate_element(element: &Element) {
    for subelement in &element.subelements {
        match subelement.name.local_name.as_str() {
            "SpaceSystem" => space_system(&subelement),
            _ => {},
        }
    }
}

pub fn space_system(element: &Element) {
    for subelement in &element.subelements {
        match subelement.name.local_name.as_str() {
            "TelemetryMetaData" => telemetry_meta_data(&subelement),
            "CommandMetaData" => telemetry_meta_data(&subelement),
            _ => {},
        }
    }
}

pub fn telemetry_meta_data(element: &Element) {
    for subelement in &element.subelements {
        match subelement.name.local_name.as_str() {
            "ParameterSet" => parameter_set(&subelement),
            _ => {},
        }
    }
}

pub fn parameter_set(element: &Element) {
    for subelement in &element.subelements {
        match subelement.name.local_name.as_str() {
            "Parameter" => parameter(&subelement),
            _ => {},
        }
    }
}

pub fn parameter(element: &Element) {
    let mut parameter_name = None;
    let mut parameter_data_type = None;

    for attribute in &element.element_info.attributes {
        let attr_name = &attribute.name.local_name;
        let attr_value = &attribute.value;
        match attr_name.as_str() {
            "name" => parameter_name = Some(attr_value),
            "dataType" => parameter_data_type = Some(attr_value),
            _ => {},
        }
    }

    println!("typedef {} {}_t;", parameter_data_type.unwrap(), parameter_name.unwrap());
}

pub fn command_meta_data(element: &Element) {
}
