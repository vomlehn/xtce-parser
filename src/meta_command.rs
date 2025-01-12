use crate::basic::*;
use crate::sequence_container::*;

#[derive(Debug)]
pub struct MetaCommandType {
    /* FIXME: figure this out
    mixed = false,
    complex_content_mixed = false
    */
    r#abstract:         Boolean,
    short_description:  r#String,
    name:               NameType,
}

#[derive(Debug)]
pub struct ParameterToSetList {
    parmameter_to_set:  Vec<ParameterToSet>,
}

#[derive(Debug)]
pub struct ParameterToSet {
}

#[derive(Debug)]
pub struct ContextSignificanceList {
    context_significance:   Vec<ContextSignificance>,
}

#[derive(Debug)]
pub struct ContextSignificance {
}

#[derive(Debug)]
pub struct Interlock {
    scope_to_space_system:              NameReferenceType,
    verification_to_wait_for:           VerifierEnumerationType,
    verification_progress_percentage:   Decimal,
    suspendable:                        Boolean,
}

#[derive(Debug)]
pub struct VerifierEnumerationType {
}

#[derive(Debug)]
pub struct BaseMetaCommand {
    argument_assignment_list:   ArgumentAssignmentList,
    meta_command_ref:           NameReferenceType,
}

#[derive(Debug)]
pub struct ArgumentAssignmentList {
}

#[derive(Debug)]
pub struct VerifierSet {
    transferred_to_range_verifier:  Vec<TransferredToRangeVerifier>,
    sent_from_range_verifier:       Vec<SentFromRangeVerifier>,
    received_verifier:              Option<ReceivedVerifier>,
    accepted_verifier:              Option<AcceptedVerifier>,
    queued_verifier:                Option<QueuedVerifier>,
    execution_verifier:             Option<ExecutionVerifier>,
    complete_verifier:              Vec<CompleteVerifier>,
    failed_verifier:                Vec<FailedVerifier>,
}

#[derive(Debug)]
pub struct TransferredToRangeVerifier {
}

#[derive(Debug)]
pub struct SentFromRangeVerifier {
}

#[derive(Debug)]
pub struct ReceivedVerifier {
}

#[derive(Debug)]
pub struct AcceptedVerifier {
}

#[derive(Debug)]
pub struct QueuedVerifier {
}

#[derive(Debug)]
pub struct ExecutionVerifier {
}

#[derive(Debug)]
pub struct CompleteVerifier {
}

#[derive(Debug)]
pub struct FailedVerifier {
}

#[derive(Debug)]
pub struct ArgumentList {
    /* FIXME: figure this out
    max_occurs = unbounded,
    */
    argument:   Vec<Argument>,
}

#[derive(Debug)]
pub struct Argument {
}

#[derive(Debug)]
pub struct CommandContainerType {
    /* FIXME: figure this out
    mixed = false,
    complex_content_mixed = false
    */
    entry_list:     CommandContainerEntryListType,
    base_container: Option<BaseContainer>,
}

#[derive(Debug)]
pub struct CommandContainerEntryListType {
}

#[derive(Debug)]
pub struct SignificanceType {
    /* FIXME: figure this out
    mixed = false,
    complex_content_mixed = false
    */
    consequence_level:      ConsequenceLevel,
    space_system_at_risk:   NameReferenceType,
    reason_for_warning:     r#String,
}

#[derive(Debug)]
pub struct ConsequenceLevel {
}

#[derive(Debug)]
pub struct TransmissionConstraintList {
    transmission_constraint:    Vec<TransmissionConstraint>,
}

#[derive(Debug)]
pub struct TransmissionConstraint {
}

#[derive(Debug)]
pub struct ParametersToSuspendAlarmsOnSet {
    parameter_to_suspend_alarms_on: Vec<ParameterToSuspendAlarmsOn>,
}

#[derive(Debug)]
pub struct ParameterToSuspendAlarmsOn {
}
