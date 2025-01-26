/*
 * Definition of the XML used in XSD files. Probably not complete
 */

use xml_tree::{ElementDefinition, XmlDefinition};

pub static XTCE_DEFINITION: XmlDefinition = XmlDefinition {
    root_name: "document_root",
    element_definitions: &[
        ElementDefinition {
            name:                   "document_root",
            allowable_subelements: &["XTCE"],
        },
        ElementDefinition {
            name:                   "XTCE",
            allowable_subelements: &["SpaceSystem"],
        },
        ElementDefinition {
            name:                   "SpaceSystem",
            allowable_subelements: &["TelemetryMetaData"],
        },
        ElementDefinition {
            name:                   "TelemetryMetaData",
            allowable_subelements: &["ParameterSet"],
        },
        ElementDefinition {
            name:                   "ParameterSet",
            allowable_subelements: &["Parameter"],
        },
        ElementDefinition {
            name:                   "Parameter",
            allowable_subelements: &["Description"],
        },
        ElementDefinition {
            name:                   "Description",
            allowable_subelements: &[],
        },
    ],
};
