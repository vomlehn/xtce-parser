use crate::Xtce;
use crate::xml_document::XtceDocument;

pub fn generate_c(document: &XtceDocument) {
    document.dump();
}

