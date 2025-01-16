use crate::Xtce;
use crate::xml_document::XtceDocument;

pub fn generate_xtce(document: &XtceDocument) {
    document.dump();
}

