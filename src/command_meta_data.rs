use crate::basic::*;

#[derive(Debug)]
pub MetaCommandSet {
    block_meta_command: BlockMetaCommand,
    meta_command:       MetaCommandType,
    # Not given a type in the OMG documentation
    argument_name_key:  TBD, 
    meta_command_ref:   NameReferenceType,
}

#[derive(Debug)]
struct AlgorithmSetType {
    /* FIXME: figure this out */
    mixed = false,
    max_occurs = unbounded,
    */
    custom_algorithm:   InputOutputTriggerAlgorithmType,
    math_algorithm:     MathAlgorithmType,
}

#[derive(Debug)]
struct ArgumentTypeSetType {
    integer_argument_type:          IntegerArgumentType,
    float_argument_type:            FloatArgumentType,
    string_argument_type:           StringArgumentType,
    enumerated_argument_type:       EnumberatedDataType,
    binary_argument_type:           BinaryDataType,
    boolean_argument_type:          BooleanArgumentType,
    relative_time_argument_type:    RelativeTimeDataType,
    absolute_time_argument_type:    AbsoluteTimeDataType,
    array_argument_type:            ArrayDataTypeType,
    aggregate_argument_type:        AggregateDataType,
}

#[derive(Debug)]
struct struct ParameterSetType {
    /* FIXME: figure this out */
    max_occurs = unbounded,
    */
    parameter:      Parameter,
    parameter_ref:  ParameterRefType,
}

#[derive(Debug)]
struct StreamSetType {
    fixed_frame_stream:     FixedFrameStreamType,
    variable_frame_stream:  VariableFrameStreamType,
    custom_stream:          CustomStreamType,
}

#[derive(Debug)]
struct CommandContainerSetType {
    command_container:  Vec<SequenceContainerType>,
}

#[derive(Debug)]
struct ParameterTypeSetType {
    integer_parameter_type:         IntegerParameterType,
    float_parameter_type:           FloatParameterType,
    string_parameter_type:          StringParameterType,
    enumerated_parameter_type:      EnumberatedParameterType,
    binary_parameter_type:          BinaryParameterType,
    boolean_parameter_type:         BooleanParameterType,
    relative_time_parameter_type:   RelativeTimeParameterType,
    // FIXME: should these be Data or Parameter?
    absolute_time_parameter_type:   AbsoluteTimeDataType,
    array_parameter_type:           ArrayDataTypeType,
    aggregate_parameter_type:       AggregateDataType,
}
