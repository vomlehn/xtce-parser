use crate::basic::*;
use crate::meta_command::*;
use crate::space_system::*;
use crate::telemetry_meta_data::*;

#[derive(Debug)]
pub struct CommandMetaDataType {
    parameter_type_set:     Option<Box<SpaceSystemType>>,
/* FIXME: resolve this
    parameter_set:          Option<ParameterSetType>,
*/
    argument_type_set:      Option<ArgumentTypeSetType>,
    meta_command_set:       MetaCommandSetType,
    command_container_set:  Option<CommandContainerSetType>,
    stream_set:             Option<StreamSetType>,
    algorithm_set:          Option<AlgorithmSetType>,
}

#[derive(Debug)]
pub struct MetaCommandSet {
    /* FIXME: resolve this
    block_meta_command: BlockMetaCommand,
    */
    meta_command:       MetaCommandType,
    /* FIXME: Not given a type in the OMG documentation
    argument_name_key:  TBD, 
    */
    meta_command_ref:   NameReferenceType,
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
pub struct StringArgumentType {
}

#[derive(Debug)]
pub struct EnumeratedArgumentType {
}

#[derive(Debug)]
pub struct IntegerArgumentType {
}

#[derive(Debug)]
pub struct BinaryArgumentType {
}

#[derive(Debug)]
pub struct FloatArgumentType {
}

#[derive(Debug)]
pub struct BooleanArgumentType {
}

#[derive(Debug)]
pub struct RelativeTimeArgumentType {
}

#[derive(Debug)]
pub struct AbsoluteTimeArgumentType {
}

#[derive(Debug)]
pub struct ArrayArgumentType {
}

#[derive(Debug)]
pub struct AggregateArgumentType {
}

/* FIXME: resolve this
#[derive(Debug)]
pub struct ParameterSetType {
    /* FIXME: figure this out
    max_occurs = unbounded,
    */
    parameter:      Parameter,
    parameter_ref:  ParameterRefType,
}
*/

#[derive(Debug)]
pub struct StreamSetType {
    fixed_frame_stream:     FixedFrameStreamType,
    variable_frame_stream:  VariableFrameStreamType,
    custom_stream:          CustomStreamType,
}

#[derive(Debug)]
pub struct CommandContainerSetType {
    command_container:  Vec<SequenceContainerType>,
}

/* FIXME: resolve this
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
*/
