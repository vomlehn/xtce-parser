use crate::basic::*;

struct StreamSetType {
/* FIXME: figure this out
    max_occurs = unbounded,
*/
}

struct VariableFrameSreamType {
    sync_strategy:  SyncStrategy,
}

struct FrameStreamType {
    stream_ref: Option<StreamRefType>,
}

struct CustomStreamType {
    encoding_algorithm: InputAlgorithmType,
    decoding_algorithm: InputOutputAlgorithmType,
    encoded_stream_ref: NameReferenceType,
    decoded_stream_ref: NameReferenceType,
}

struct PCMStreamType {
    pcm_type:           PCMType,
    bit_rate_in_bps:    Double,
    inverted:           Boolean,
}

struct FixedFrameStreamType {
    sync_strategy:          SyncStrategy,
    sync_aperature_in_bits: NonNegativeInteger,
    frame_length_in_bits:   Long,
}


