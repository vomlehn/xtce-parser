use crate::Xtce;
use crate::xtce_document::XtceDocument;

pub fn generate_c(document: &XtceDocument) {
    document.dump();
}

