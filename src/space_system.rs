/*
 * XTCE types
 */

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

struct TelemetryMetaDataType {
    parameter_type_set:     Option<ParameterTypeSetType>,
    parameter_set:          Option<ParameterSetType>,
    container_set:          Option<ContainerSetType>,
    message_set:            Option<MessageSetType>,
    stream_set:             Option<StreamSetType>,
    algorithm_set:          Option<AlgorithmSetType>,
}

struct ParameterTypeSetType {
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

struct StringParameterType {
}

struct EnumeratedParameterType {
}

struct IntegerParameterType {
}

struct BinaryParameterType {
}

struct FloatParameterType {
}

struct BooleanParameterType {
}

struct RelativeTimeParameterType {
}

struct AbsoluteTimeParameterType {
}

struct ArrayParameterType {
}

struct AggregateParameterType {
}

struct ParameterSetType {
    parameter:      ParameterType,
    parameter_ref:  ParameterRefType,
}

struct ParameterType {
}

struct ParameterRefType {
}

struct ContainerSetType {
    sequence_countainer:    SequenceContainerType,
}

struct SequenceContainerType {
}

struct MessageSetType {
    message:    Vec<MessageType>,
    name:       String,
}

struct MessageType {
}

struct StreamSetType {
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

struct AlgorithmSetType {
    custom_algorithm:   InputOutputTriggerAlgorithmType,
    math_algorithm:     MathAlgorithmType,
}

struct InputOutputTriggerAlgorithmType {
}

struct MathAlgorithmType {
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
