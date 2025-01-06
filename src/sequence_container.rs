use crate::basic::*;

struct ContainerSetType {
}

struct EntryListType {
/* FIXME: figure this out.
    mixed = false,
    min_occurs = 0,
    max_occurs = unbounded,
*/
}

struct ContainerType {
/* FIXME: figure this out.
    mixed = false,
    complex_content_mixed = false,
*/
}

struct BaseContainer {
    container_ref:  NameReferenceType,
}

struct ParameterRefEntryType {
    parameter_ref:  NameReferenceType,
}

struct ArrayParameterRefEntryType {
    parameter_ref:                      NameReferenceType,
    last_entry_for_this_array_instance: Boolean,
}

struct ArgumentRefEntry {
    argument_ref:   NameReferenceType,
}

struct ContainerRefEntryType {
    container_ref:  NameReferenceType,
}

struct StreamSegmentEntryType {
    stream_ref:     NameReferenceType,
    order:          PositiveInteger,
    size_in_bits:   PositiveInteger,
}

struct SequenceEntryType {
}

struct ParameterSegmentRefEntryType {
    parameter_ref:  NameReferenceType,
    order:          PositiveInteger,
    size_in_bits:   PositiveInteger,
}

struct ContainerSegmentRefEntryType {
    parameter_ref:  NameReferenceType,
    order:          PositiveInteger,
    size_in_bits:   PositiveInteger,
}

struct IndirectParameterRefEntryType {
    alias_name_space:   r#String,
}
