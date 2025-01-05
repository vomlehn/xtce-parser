use crate::basic::*;

struct StringDataType {
    character_width:        CharacterWidth,
    initial_value:          String,
    restriction_pattern:    String,
}

struct CharacterWidth {
}

struct EnumeratedDataType {
    initial_value:  String,
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
    initial_value:      String,
    one_string_value:   String,
    zero_string_value:  String,
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

