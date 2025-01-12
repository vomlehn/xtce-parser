use crate::basic::*;

#[derive(Debug)]
pub struct ContainerSetType {
}

#[derive(Debug)]
pub struct EntryListType {
/* FIXME: figure this out.
    mixed = false,
    min_occurs = 0,
    max_occurs = unbounded,
*/
}

#[derive(Debug)]
pub struct ContainerType {
/* FIXME: figure this out.
    mixed = false,
    complex_content_mixed = false,
*/
}

#[derive(Debug)]
pub struct BaseContainer {
    container_ref:  NameReferenceType,
}

#[derive(Debug)]
pub struct ParameterRefEntryType {
    parameter_ref:  NameReferenceType,
}

#[derive(Debug)]
pub struct ArrayParameterRefEntryType {
    parameter_ref:                      NameReferenceType,
    last_entry_for_this_array_instance: Boolean,
}

#[derive(Debug)]
pub struct ArgumentRefEntry {
    argument_ref:   NameReferenceType,
}

#[derive(Debug)]
pub struct ContainerRefEntryType {
    container_ref:  NameReferenceType,
}

#[derive(Debug)]
pub struct StreamSegmentEntryType {
    stream_ref:     NameReferenceType,
    order:          PositiveInteger,
    size_in_bits:   PositiveInteger,
}

#[derive(Debug)]
pub struct SequenceEntryType {
}

#[derive(Debug)]
pub struct ParameterSegmentRefEntryType {
    parameter_ref:  NameReferenceType,
    order:          PositiveInteger,
    size_in_bits:   PositiveInteger,
}

#[derive(Debug)]
pub struct ContainerSegmentRefEntryType {
    parameter_ref:  NameReferenceType,
    order:          PositiveInteger,
    size_in_bits:   PositiveInteger,
}

#[derive(Debug)]
pub struct IndirectParameterRefEntryType {
    alias_name_space:   r#String,
}
