use crate::basic::*;
use crate::space_system::*;

#[derive(Debug)]
pub struct TelemetryMetaDataType {
    parameter_type_set:     Option<ParameterTypeSetType>,
    parameter_set:          Option<ParameterSetType>,
    container_set:          Option<ContainerSetType>,
    message_set:            Option<MessageSetType>,
    stream_set:             Option<StreamSetType>,
    algorithm_set:          Option<AlgorithmSetType>,
}

#[derive(Debug)]
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

#[derive(Debug)]
struct StringParameterType {
}

#[derive(Debug)]
struct EnumeratedParameterType {
}

#[derive(Debug)]
struct IntegerParameterType {
}

#[derive(Debug)]
struct BinaryParameterType {
}

#[derive(Debug)]
struct FloatParameterType {
}

#[derive(Debug)]
struct BooleanParameterType {
}

#[derive(Debug)]
struct RelativeTimeParameterType {
}

#[derive(Debug)]
struct AbsoluteTimeParameterType {
}

#[derive(Debug)]
struct ArrayParameterType {
}

#[derive(Debug)]
struct AggregateParameterType {
}

#[derive(Debug)]
pub struct ParameterSetType {
    parameter:      ParameterType,
    parameter_ref:  ParameterRefType,
}

#[derive(Debug)]
struct ParameterType {
}

#[derive(Debug)]
struct ParameterRefType {
}

#[derive(Debug)]
struct ContainerSetType {
    sequence_countainer:    SequenceContainerType,
}

#[derive(Debug)]
struct SequenceContainerType {
    r#abstract:     Boolean,      // FIXME: abstract is a reserved word, change
    idle_pattern:   FixedIntegerValueType,
}

#[derive(Debug)]
struct MessageSetType {
    message:    Vec<MessageType>,
    name:       r#String,
}

#[derive(Debug)]
struct MessageType {
}

#[derive(Debug)]
pub struct AlgorithmSetType {
    custom_algorithm:   InputOutputTriggerAlgorithmType,
    math_algorithm:     MathAlgorithmType,
}

#[derive(Debug)]
struct InputOutputTriggerAlgorithmType {
}

#[derive(Debug)]
struct MathAlgorithmType {
}
