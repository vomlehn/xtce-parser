use crate::space_system_converted::*;

/* I don't know what these are for yet */
struct ParameterNameKey {}
struct ParameterTypeNameKey {}
struct MetaCommandNameKey {}
struct AlgorithmNameKey {}
struct StreamNameKey {}
struct ServiceNameKey {}
struct ContainerNameKey {}
struct MessageNameKey {}
struct ArgumentTypeNameKey {}
struct BlockMetaCommandNameKey {}

#[derive(Debug)]
pub struct SpaceSystemType {
    pub name_description_type:  NameDescriptionType,
    pub header:                 Option<HeaderType>,
    pub telemetry_meta_data:    Option<TelemetryMetaDataType>,
    pub command_meta_data:      Option<Box<CommandMetaDataType>>,
    pub service_set:            Option<ServiceSetType>,
//    pub space_system_ref:       Vec<SpaceSystemV1_1>,
// FIXME: this refers to xml:base, whatever that is. This may be a non-standard
// It could, perhaps, since it is the root XTCE type, pick up other XML attributes.
}

impl SpaceSystemType {
    pub fn new() -> SpaceSystemType {
        SpaceSystemType {
            header:                 None,
            telemetry_meta_data:    None,
            command_meta_data:      None,
            service_set:            None,
//            space_system_ref:       Vec::<SpaceSystemV1_1>::new(),
        }
    }
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
pub struct CommandMetaDataType {
    parameter_type_set:     Option<Box<SpaceSystemType>>,
/* FIXME: resolve this
    parameter_set:          Option<ParameterSetType>,
*/
    argument_type_set:      Option<ArgumentTypeSetType>,
    /* FIXME: resolve this
    meta_command_set:       MetaCommandSetType,
    */
    command_container_set:  Option<CommandContainerSetType>,
    stream_set:             Option<StreamSetType>,
    algorithm_set:          Option<AlgorithmSetType>,
}

impl CommandMetaDataType {
    pub fn new() -> CommandMetaDataType {
        CommandMetaDataType {
            parameter_type_set:     None,
        /* FIXME: resolve this
            parameter_set:          None,
        */
            argument_type_set:      None,
        /* FIXME: resolve this
            meta_command_set:       MetaCommandSetType,
        */
            command_container_set:  None,
            stream_set:             None,
            algorithm_set:          None,
        }
    }
}

#[derive(Debug)]
pub struct ServiceSetType {
    service:    Vec<ServiceType>,
}

#[derive(Debug)]
pub struct ParameterSetType {
    /* FIXME: figure this out
    max_occurs = unbounded,
    */
    parameter:      ParameterType,
    parameter_ref:  ParameterRefType,
}

#[derive(Debug)]
pub struct ArgumentTypeSetType {
    integer_argument_type:          IntegerArgumentType,
    float_argument_type:            FloatArgumentType,
    string_argument_type:           StringArgumentType,
/* FIXME: resolve these
    enumerated_argument_type:       EnumeratedDataType,
    binary_argument_type:           BinaryDataType,
    boolean_argument_type:          BooleanArgumentType,
    relative_time_argument_type:    RelativeTimeDataType,
    absolute_time_argument_type:    AbsoluteTimeDataType,
    array_argument_type:            ArrayDataTypeType,
    aggregate_argument_type:        AggregateDataType,
*/
}

#[derive(Debug)]
pub struct MetaCommandSetType {
}

#[derive(Debug)]
pub struct CommandContainerSetType {
}

#[derive(Debug)]
pub struct ContainerSetType {
    sequence_countainer:    SequenceContainerType,
}

#[derive(Debug)]
pub struct MessageSetType {
    message:    Vec<MessageType>,
    name:       r#String,
}

#[derive(Debug)]
pub struct TelemetryMetaDataType {
    parameter_type_set:     Option<ParameterTypeSetType>,
    parameter_set:          Option<ParameterSetType>,
    container_set:          Option<ContainerSetType>,
    message_set:            Option<MessageSetType>,
    stream_set:             Option<StreamSetType>,
    algorithm_set:          Option<AlgorithmSetType>,
}

impl TelemetryMetaDataType {
    pub fn new() -> TelemetryMetaDataType {
        TelemetryMetaDataType {
            parameter_type_set:     None,
            parameter_set:          None,
            container_set:          None,
            message_set:            None,
            stream_set:             None,
            algorithm_set:          None,
        }
    }
}
