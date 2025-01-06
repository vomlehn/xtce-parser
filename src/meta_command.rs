use crate::basic::*;

struct MetaCommandType {
    /* FIXME: figure this out
    mixed = false,
    complex_content_mixed = false
    */
    r#abstract:         Boolean,
    short_description:  r#String,
    name:               NameType,
}

struct ParameterToSetList {
    parmameter_to_set:  Vec<ParameterToSet>,
}

struct ContextSignificanceList {
    context_significance:   Vec<ContextSignificance>,
}

struct Interlock {
    scope_to_space_system:              NameReferenceType,
    verification_to_wait_for:           VerifierEnumerationType,
    verification_progress_percentage:   Decimal,
    suspendable:                        Boolean,
}

struct BaseMetaCommand {
    argument_assignment_list:   ArgumentAssignmentList,
    meta_command_ref:           NameReferenceType,
}

struct VerifierSet {
    transferred_to_range_verifier:  Vec<TransferredToRangeVerifier>,
    sent_from_range_verifier:       Vec<SentFromRangeVerifier>,
    received_verifier:              Option<ReceivedVerifier>,
    accepted_verifier:              Option<AcceptedVErifier>,
    queued_verifier:                Option<QueuedVerifier>,
    execution_verifier:             Option<ExecutionVerifier>,
    complete_verifier:              Vec<CompleteVerifier>,
    failed_verifier:                Vec<CompleteVerifier>,
}

struct ArgumentList [
    /* FIXME: figure this out
    max_occurs = unbounded,
    */
    argument:   Vec<Argument>,
}

struct CommandContainerType {
    /* FIXME: figure this out
    mixed = false,
    complex_content_mixed = false
    */
    entry_list:     CommandContainerEntryListType,
    base_container: Option<BaseContainer>,
}

struct SignificanceType {
    /* FIXME: figure this out
    mixed = false,
    complex_content_mixed = false
    */
    consequence_level:      ConsequenceLevel,
    space_system_at_risk:   NameReferenceType,
    reason_for_warning:     r#String,
}

struct transmissionConstraintList {
    transmission_constraint:    Vec<TranmissionConstraint>,
}

struct ParametersToSuspendAlarmsOnSet {
    parameter_to_suspend_alarms_on: Vec<ParameterToSuspendAlarmsOn>,
}
