pub enum OutputType {
    C,
    XtceDesc,
}

pub struct XtceOptions {
    pub telemetry_prefix:   String,
    pub command_prefix:     String,
    pub output_type:        OutputType,
}

impl XtceOptions {
    pub fn new() -> XtceOptions {
        XtceOptions {
            telemetry_prefix:   "".to_string(),
            command_prefix:     "".to_string(),
            output_type:        OutputType::C,
        }
    }
}
