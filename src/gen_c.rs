/*
 * Produce C/C++ code
 *
 * FIXME: This is pretty crude and assumes its input is correct
 */

use xml_tree::{Element, XmlTree};

pub fn generate_c(document: &XmlTree) {
    const XML_DOCUMENT_H: &str = "_XML_DOCUMENT_H_";
    let mut last_was_big = false;

    println!("/*");
    println!(" * This is automatically generated code that may be regenerated.");
    println!(" * Manual changes may be lost at the next build");
    println!(" */");
    println!();
    println!("#ifndef {}", XML_DOCUMENT_H);
    println!("#define {}", XML_DOCUMENT_H);
    println!();
    println!("#ifdef __cplusplus");
    println!("#include <cstdint>");
    println!("#else");
    println!("#include <stdint.h>");
    println!("#endif");
    println!();

    for element in &document.root.subelements{
        generate_c_element(&element, &mut last_was_big);
    }

    println!();
    println!("#endif /* {} */", XML_DOCUMENT_H);
}

fn generate_c_element(element: &Element, last_was_big: &mut bool) {
    match element.name.local_name.as_str() {
        "ArgumentTypeSet" => generate_argument_type_set(element, last_was_big),
        "ParameterTypeSet" => generate_parameter_type_set(element, last_was_big),
        _ => {
            for element in &element.subelements {
                generate_c_element(&element, last_was_big);
            }
        }
    }
}

fn generate_argument_type_set(element: &Element, last_was_big: &mut bool) {
    println!();
    println!("/* Telemetry parameter definitions */");
    println!();

    for element in &element.subelements {
        match element.name.local_name.as_str() {
            "BinaryArgumentType" => {
                if *last_was_big {
                    println!();
                }
                println!("typedef int32_t {};", tc_name(element));
                *last_was_big = false;
            },
            "EnumeratedArgumentType" => {
                println!();
                generate_enumerated_types(element);
                *last_was_big = true;
            },
            _ => {
                println!("// Unknown argument type {}", tc_name(element));
            }
        }
    }
}

fn tc_name(element: &Element) -> String {
    "tc_".to_owned() + &name_attribute(element)
}

fn generate_parameter_type_set(element: &Element, last_was_big: &mut bool) {
    println!();
    println!("/* Command argument definitions */");
    println!();

    for element in &element.subelements {
        match element.name.local_name.as_str() {
            "BinaryParameterType" => {
                if *last_was_big {
                    println!();
                }
                println!("typedef int32_t {};", tm_name(element));
                *last_was_big = false;
            },
            "FloatParameterType" => {
                if *last_was_big {
                    println!();
                }
                println!("typedef float {};", tm_name(element));
                *last_was_big = false;
            },
            "EnumeratedParameterType" => {
                println!();
                generate_enumerated_types(element);
                *last_was_big = true;
            },
            _ => {
                println!("// Unknown parameter type {}", tm_name(element));
            }
        }
    }
}

fn tm_name(element: &Element) -> String {
    "tm_".to_owned() + &name_attribute(element)
}

fn name_attribute(element: &Element) -> String {
    temporary_get_attribute(element, "name")
}

// FIXME: temporary function until error handling is better defined
fn temporary_get_attribute(element: &Element, name: &str) -> String {
    match element.get_attribute(name) {
        None => "\"attribute \"".to_string() + name + "\" not found",
        Some(n) => n.clone(),
    }
}

fn generate_enumerated_types(element: &Element) {
    let enum_name = tc_name(element);

    println!("typedef enum {} {{", enum_name);

    for element in &element.subelements {
        match element.name.local_name.as_str() {
            "EnumerationList" => {
                generate_enumerated_elements(element);
            },
            "UnitSet" | "IntegerDataEncoding" => {
                // FIXME: Implement these
            }
            _ => {
                // FIXME: handle these errors
            }
        }
    }

    println!("}} {}_t;", enum_name);
}

fn generate_enumerated_elements(element: &Element) {
    for element in &element.subelements {
        match element.name.local_name.as_str() {
            "Enumeration" => {
                generate_enumerated_element(element);
            },
            "UnitSet" | "IntegerDataEncoding" => {
                // FIXME: Implement these
            },
            _ => {
                // FIXME: handle these errors
            }
        }
    }
}

fn generate_enumerated_element(element: &Element) {
    let label = temporary_get_attribute(element, "label");
    let value = temporary_get_attribute(element, "value");
    println!("\t{} = {},", label, value);
}
