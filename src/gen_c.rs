/*
 * Produce C/C++ code
 *
 * FIXME: This is pretty crude and assumes its input is correct
 */

use crate::xml_document::{Element, XtceDocument};

pub fn generate_c(document: &XtceDocument) {
    const XML_DOCUMENT_H: &str = "_XML_DOCUMENT_H_";

    println!("/*");
    println!(" * This is automatically generated code that may be regenerated.");
    println!(" * Manual changes may be lost at the next build");
    println!(" */");
    println!();
    println!("#ifdef {}", XML_DOCUMENT_H);
    println!("#define {}", XML_DOCUMENT_H);
    println!();
    println!("#ifdef __cplusplus");
    println!("#include <cstdint>");
    println!("#else");
    println!("#include <stdint.h>");
    println!("#endif");
    println!();

    for element_vec in document.root.subelements.values() {
        for element in element_vec {
            generate_c_element(&element);
        }
    }

    println!();
    println!("#endif /* {} */", XML_DOCUMENT_H);
}

fn generate_c_element(element: &Element) {
    match element.name.local_name.as_str() {
        "ArgumentTypeSet" => generate_argument_type_set(element),
        "ParameterTypeSet" => generate_parameter_type_set(element),
        _ => {
            for element_vec in element.subelements.values () {
                for elem in element_vec {
                    generate_c_element(&elem);
                }
            }
        }
    }
}

fn generate_argument_type_set(element: &Element) {
    for element_vec in element.subelements.values() {
        for element in element_vec {
            match element.name.local_name.as_str() {
                "BinaryArgumentType" => {
                    println!("typedef int32 {};", tc_name(element));
                },
                "EnumeratedArgumentType" => {
                    println!("// skipping EnumeratedArgumentType for {}", tc_name(element));
                },
                _ => {
                    println!("// Unknown argument type {}", tc_name(element));
                }
            }
        }
    }
}

fn tc_name(element: &Element) -> String {
    "tc_".to_owned() + name_attribute(element)
}

fn generate_parameter_type_set(element: &Element) {
    for element_vec in element.subelements.values() {
        for element in element_vec {
            match element.name.local_name.as_str() {
                "BinaryParameterType" => {
                    println!("typedef int32 {};", tm_name(element));
                },
                "FloatParameterType" => {
                    println!("typedef float {};", tm_name(element));
                },
                "EnumeratedParameterType" => {
                    println!("// skipping EnumeratedParameterType for {}", tm_name(element));
                },
                _ => {
                    println!("// Unknown parameter type {}", tm_name(element));
                }
            }
        }
    }
}

fn tm_name(element: &Element) -> String {
    "tm_".to_owned() + name_attribute(element)
}

fn name_attribute(element: &Element) -> &str {
    match element.get_attribute("name") {
        None => "\"attribute name not found\"",
        Some(n) => n,
    }
}
