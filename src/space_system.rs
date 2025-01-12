/*
 * XTCE types
 */

use xml::common::{XmlVersion};

use crate::telemetry_meta_data::*;

#[derive(Debug)]
pub struct SpaceSystemType {
    header:                 Option<HeaderType>,
    telemetry_meta_data:    Option<TelemetryMetaDataType>,
    command_meta_data:      Option<Box<CommandMetaDataType>>,
    service_set:            Option<ServiceSetType>,
    space_system_ref:       Vec<SpaceSystemV1_1>,
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
struct NameDescriptionType {
    name:   NameType
}

type NameType = r#String;

#[derive(Debug)]
struct HeaderType {
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
struct AuthorSetType {
}

#[derive(Debug)]
struct NoteSetType {
}

#[derive(Debug)]
struct HistorySetType {
}

#[derive(Debug)]
struct ValidationStatusType {
}

#[derive(Debug)]
pub struct StreamSetType {
    fixed_frame_stream:     FixedFrameStreamType,
    variable_frame_stream:  VariableFrameStreamType,
    custom_stream:          CustomStreamType,
}

#[derive(Debug)]
struct FixedFrameStreamType {
}

#[derive(Debug)]
struct VariableFrameStreamType {
}

#[derive(Debug)]
struct CustomStreamType {
}

#[derive(Debug)]
struct CommandMetaDataType {
    parameter_type_set:     Option<Box<SpaceSystemType>>,
    parameter_set:          Option<ParameterSetType>,
    argument_type_set:      Option<ArgumentTypeSetType>,
    meta_command_set:       MetaCommandSetType,
    command_container_set:  Option<CommandContainerSetType>,
    stream_set:             Option<StreamSetType>,
    algorithm_set:          Option<AlgorithmSetType>,
}

#[derive(Debug)]
struct ArgumentTypeSetType {
}

#[derive(Debug)]
struct MetaCommandSetType {
}

#[derive(Debug)]
struct CommandContainerSetType {
}

#[derive(Debug)]
struct ServiceSetType {
    service:    Vec<ServiceType>,
}

#[derive(Debug)]
struct ServiceType {
}

#[derive(Debug)]
struct SpaceSystemV1_1 {
}
