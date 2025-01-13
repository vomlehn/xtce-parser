
/* I don't know what these are for yet */
struct parameterNameKey {}
struct parameterTypeNameKey {}
struct metaCommandNameKey {}
struct algorithmNameKey {}
struct streamNameKey {}
struct serviceNameKey {}
struct containerNameKey {}
struct messageNameKey {}
struct argumentTypeNameKey {}
struct blockMetaCommandNameKey {}

#[derive(Debug)]
pub struct SpaceSystemType {
    pub name_description_type:  NameDescriptionType,
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
pub struct CommandMetaDataType {
    parameter_type_set:     Option<Box<SpaceSystemType>>,
    parameter_set:          Option<ParameterSetType>,
    argument_type_set:      Option<ArgumentTypeSetType>,
    meta_command_set:       MetaCommandSetType,
    command_container_set:  Option<CommandContainerSetType>,
    stream_set:             Option<StreamSetType>,
    algorithm_set:          Option<AlgorithmSetType>,
}

#[derive(Debug)]
pub struct ParameterTypeSetType {
    integer_parameter_type:         IntegerParameterType,
    float_parameter_type:           FloatParameterType,
    string_parameter_type:          StringParameterType,
    enumerated_parameter_type:      EnumeratedParameterType,
    binary_parameter_type:          BinaryParameterType,
    boolean_parameter_type:         BooleanParameterType,
    relative_time_parameter_type:   RelativeTimeParameterType,
    // FIXME: should these be Data or Parameter?
    absolute_time_parameter_type:   AbsoluteTimeDataType,
    array_parameter_type:           ArrayDataTypeType,
    aggregate_parameter_type:       AggregateDataType,
}

#[derive(Debug)]
pub struct ParameterSetType {
    /* FIXME: figure this out
    max_occurs = unbounded,
    */
    parameter:      Parameter,
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
pub struct StreamSetType {
    fixed_frame_stream:     FixedFrameStreamType,
    variable_frame_stream:  VariableFrameStreamType,
    custom_stream:          CustomStreamType,
}

#[derive(Debug)]
pub struct AlgorithmSetType {
    /* FIXME: figure this out
    mixed = false,
    max_occurs = unbounded,
    */
    custom_algorithm:   InputOutputTriggerAlgorithmType,
    math_algorithm:     MathAlgorithmType,
}

#[derive(Debug)]
pub struct TeemetryMetaDataType {
}

#[derive(Debug)]
pub struct ParameterTypeSetType {
    string_parameter_type:          StringParameterType,
    enumerated_parameter_type:      EnumeratedParameterType,
    integer_parameter_type:         IntegerParameterType,
    binary_parameter_type:          BinaryParameterType,
    float_parameter_type:           FloatParameterType,
    boolean_parameter_type:         BooleanParameterType,
    relative_time_parameter_type:   RelativeTimeParameterType,
    absolute_time_parameter_type:   AbsoluteTimeParameterType,
    array_parameter_type:           ArrayParameterType,
    aggregate_parameter_type:       AggregateParameterType,
}

#[derive(Debug)]
pub struct ParameterSetType {
    parameter:      ParameterType,
    parameter_ref:  ParameterRefType,
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
pub struct StreamSetType {
/* FIXME: figure this out
    max_occurs = unbounded,
*/
    fixed_frame_stream:     FixedFrameStreamType,
    variable_frame_stream:  VariableFrameStreamType,
    custom_stream:          CustomStreamType,
}

// FIXME: should the algorithm stuff get its own file. Is it shared
// between command and telemetry?
#[derive(Debug)]
pub struct AlgorithmSetType {
    custom_algorithm:   InputOutputTriggerAlgorithmType,
    math_algorithm:     MathAlgorithmType,
}
