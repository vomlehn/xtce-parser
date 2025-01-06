use crate::basic::*;

struct StringDataType {
    character_width:        CharacterWidth,
    initial_value:          r#String,
    restriction_pattern:    r#String,
}

struct CharacterWidth {
}

struct EnumeratedDataType {
    initial_value:  r#String,
}

struct IntegerDataType {
    initial_value:  FixedInteger,
    size_in_bits:   PositiveInteger,
    signed:         Boolean,
}

struct FixedInteger {
}

struct PositiveInteger {
}

struct Boolean {
}

struct BinaryDataType {
    initial_value:  EnumeratedDataType,
}

struct FloatDataType {
    size_in_bits:   PositiveInteger,
    initial_value:  Double,
}

struct Double {
}

struct BooleanDataType {
    initial_value:      r#String,
    one_string_value:   r#String,
    zero_string_value:  r#String,
}

struct RelativeTimeDataType {
    initial_value:      Duration,
}

struct AbsoluteTimeDataType {
}

struct ArrayDataType {
}

struct AggregateDataType {
}

