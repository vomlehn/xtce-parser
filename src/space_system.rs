/*
 * XTCE types
 */

use crate::basic::*;
use crate::space_system::*;
use crate::telemetry_meta_data::*;

struct SpaceSystemType {
    header:                 Option<HeaderType>,
    telemetry_meta_data:    Option<TelemetryMetaDataType>,
    command_meta_data:      Option<Box<CommandMetaDataType>>,
    service_set:            Option<ServiceSetType>,
    space_system_ref:       Vec<SpaceSystemV1_1>,
}

struct NameDescriptionType {
    name:   NameType
}

type NameType = String;

struct HeaderType {
    author_set:                     Option<AuthorSetType>,
    note_set:                       Option<NoteSetType>,
    history_set:                    Option<HistorySetType>,
    validation_status:              ValidationStatusType,
    version:                        String,
    date:                           String,
    classification:                 String,
    classification_instructions:    String,
}

struct AuthorSetType {
}

struct NoteSetType {
}

struct HistorySetType {
}

struct ValidationStatusType {
}

pub struct StreamSetType {
    fixed_frame_stream:     FixedFrameStreamType,
    variable_frame_stream:  VariableFrameStreamType,
    custom_stream:          CustomStreamType,
}

struct FixedFrameStreamType {
}

struct VariableFrameStreamType {
}

struct CustomStreamType {
}

struct CommandMetaDataType {
    parameter_type_set:     Option<Box<SpaceSystemType>>,
    parameter_set:          Option<ParameterSetType>,
    argument_type_set:      Option<ArgumentTypeSetType>,
    meta_command_set:       MetaCommandSetType,
    command_container_set:  Option<CommandContainerSetType>,
    stream_set:             Option<StreamSetType>,
    algorithm_set:          Option<AlgorithmSetType>,
}

struct ArgumentTypeSetType {
}

struct MetaCommandSetType {
}

struct CommandContainerSetType {
}

struct ServiceSetType {
    service:    Vec<ServiceType>,
}

struct ServiceType {
}

struct SpaceSystemV1_1 {
}
