/*
 * XTCE types
 */

use xml::common::{XmlVersion};

use crate::basic::*;
use crate::command_meta_data::*;
use crate::telemetry_meta_data::*;

#[derive(Debug)]
pub struct SpaceSystemType {
    pub header:                 Option<HeaderType>,
    pub telemetry_meta_data:    Option<TelemetryMetaDataType>,
    pub command_meta_data:      Option<Box<CommandMetaDataType>>,
    pub service_set:            Option<ServiceSetType>,
    pub space_system_ref:       Vec<SpaceSystemV1_1>,
}

impl SpaceSystemType {
    pub fn new() -> SpaceSystemType {
        SpaceSystemType {
            header:                 None,
            telemetry_meta_data:    None,
            command_meta_data:      None,
            service_set:            None,
            space_system_ref:       Vec::<SpaceSystemV1_1>::new(),
        }
    }
}

#[derive(Debug)]
pub struct NameDescriptionType {
    name:   NameType,
}

#[derive(Debug)]
pub struct HeaderType {
    author_set:                     Option<AuthorSetType>,
    note_set:                       Option<NoteSetType>,
    history_set:                    Option<HistorySetType>,
    validation_status:              ValidationStatusType,
    version:                        r#String,
    date:                           r#String,
    classification:                 r#String,
    classification_instructions:    r#String,
}

#[derive(Debug)]
pub struct AuthorSetType {
}

#[derive(Debug)]
pub struct NoteSetType {
}

#[derive(Debug)]
pub struct HistorySetType {
}

#[derive(Debug)]
pub struct ValidationStatusType {
}

#[derive(Debug)]
pub struct StreamSetType {
    fixed_frame_stream:     FixedFrameStreamType,
    variable_frame_stream:  VariableFrameStreamType,
    custom_stream:          CustomStreamType,
}

#[derive(Debug)]
pub struct FixedFrameStreamType {
}

#[derive(Debug)]
pub struct VariableFrameStreamType {
}

#[derive(Debug)]
pub struct CustomStreamType {
}

/* FIXME: remove this
#[derive(Debug)]
pub struct CommandMetaDataType {
    parameter_type_set:     Option<Box<SpaceSystemType>>,
    parameter_set:          Option<ParameterSetType>,
    argument_type_set:      Option<ArgumentTypeSetType>,
    meta_command_set:       MetaCommandSetType,
    command_container_set:  Option<CommandContainerSetType>,
    stream_set:             Option<StreamSetType>,
    algorithm_set:          Option<AlgorithmSetType>,
}
*/

#[derive(Debug)]
pub struct ArgumentTypeSetType {
}

#[derive(Debug)]
pub struct MetaCommandSetType {
}

#[derive(Debug)]
pub struct CommandContainerSetType {
}

#[derive(Debug)]
pub struct ServiceSetType {
    service:    Vec<ServiceType>,
}

#[derive(Debug)]
pub struct ServiceType {
}

#[derive(Debug)]
pub struct SpaceSystemV1_1 {
}
