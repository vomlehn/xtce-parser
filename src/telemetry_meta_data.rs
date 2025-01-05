use crate::basic::*;
use crate::space_system::*;

pub struct TelemetryMetaDataType {
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

pub struct ParameterSetType {
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
    r#abstract:     Boolean,      // FIXME: abstract is a reserved word, change
    idle_pattern:   FixedIntegerValueType,
}

struct MessageSetType {
    message:    Vec<MessageType>,
    name:       String,
}

struct MessageType {
}

pub struct AlgorithmSetType {
    custom_algorithm:   InputOutputTriggerAlgorithmType,
    math_algorithm:     MathAlgorithmType,
}

struct InputOutputTriggerAlgorithmType {
}

struct MathAlgorithmType {
}
