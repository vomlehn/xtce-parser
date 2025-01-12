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
pub struct StringParameterType {
}

#[derive(Debug)]
pub struct EnumeratedParameterType {
}

#[derive(Debug)]
pub struct IntegerParameterType {
}

#[derive(Debug)]
pub struct BinaryParameterType {
}

#[derive(Debug)]
pub struct FloatParameterType {
}

#[derive(Debug)]
pub struct BooleanParameterType {
}

#[derive(Debug)]
pub struct RelativeTimeParameterType {
}

#[derive(Debug)]
pub struct AbsoluteTimeParameterType {
}

#[derive(Debug)]
pub struct ArrayParameterType {
}

#[derive(Debug)]
pub struct AggregateParameterType {
}

#[derive(Debug)]
pub struct ParameterSetType {
    parameter:      ParameterType,
    parameter_ref:  ParameterRefType,
}

#[derive(Debug)]
pub struct ParameterType {
}

#[derive(Debug)]
pub struct ParameterRefType {
}

#[derive(Debug)]
pub struct ContainerSetType {
    sequence_countainer:    SequenceContainerType,
}

#[derive(Debug)]
pub struct SequenceContainerType {
    r#abstract:     Boolean,      // FIXME: abstract is a reserved word, change
    idle_pattern:   FixedIntegerValueType,
}

#[derive(Debug)]
pub struct MessageSetType {
    message:    Vec<MessageType>,
    name:       r#String,
}

#[derive(Debug)]
pub struct MessageType {
}

// FIXME: should the algorithm stuff get its own file. Is it shared
// between command and telemetry?
#[derive(Debug)]
pub struct AlgorithmSetType {
    custom_algorithm:   InputOutputTriggerAlgorithmType,
    math_algorithm:     MathAlgorithmType,
}

#[derive(Debug)]
pub struct InputOutputTriggerAlgorithmType {
}

#[derive(Debug)]
pub struct MathAlgorithmType {
}
