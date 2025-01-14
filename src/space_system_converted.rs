    // FIXME: not quite sure this is right
/*

 FIXME: When the conversion to Rust code is complete, this should be divided
 into the following files:
    packaging.rs
    telemetry.rs
    command.rs
    algorithm.rs
    stream_definitions.rs
    data_types.rs
    common_types.rs
*/

use crate::rust_data_types::*;

/* ****** Packaging Schema ******************************* */

/* packaging.rs
#[derive(Debug)]
pub struct ArgumentArgumentRefEntryType {
<complexContent>
<extension base="/*xtce:*/ArgumentSequenceEntryType">
	argumentRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentArrayArgumentRefEntryType {
<complexContent>
<extension base="/*xtce:*/ArgumentSequenceEntryType">
<sequence minOccurs="0">
	DimensionList:	/*xtce:*/ArgumentDimensionListType, // [element]
</sequence>
	argumentRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
	lastEntryForThisArrayInstance:	XtceBoolean, // default="false"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentArrayParameterRefEntryType {
<complexContent>
<extension base="/*xtce:*/ArgumentSequenceEntryType">
<sequence minOccurs="0">
	DimensionList:	/*xtce:*/DimensionListType, // [element]
</sequence>
	parameterRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
	lastEntryForThisArrayInstance:	XtceBoolean, // default="false"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentContainerRefEntryType {
<complexContent>
<extension base="/*xtce:*/ArgumentSequenceEntryType">
	containerRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentContainerSegmentRefEntryType {
<complexContent>
<extension base="/*xtce:*/ArgumentSequenceEntryType">
	containerRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
	order:	/*xtce:*/PositiveLongType, // [attribute]
	sizeInBits:	/*xtce:*/PositiveLongType, // use="required"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentFixedValueEntryType {
<complexContent>
<extension base="/*xtce:*/ArgumentSequenceEntryType">
	name:	XtceString, // use="optional" [attribute]
</attribute>
	binaryValue:	hexBinary, // use="required" [attribute]
</attribute>
	sizeInBits:	/*xtce:*/PositiveLongType, // use="required" [attribute]
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentIndirectParameterRefEntryType {
<complexContent>
<extension base="/*xtce:*/ArgumentSequenceEntryType">
<sequence>
	ParameterInstance:	/*xtce:*/ParameterInstanceRefType, // [element]
</sequence>
	aliasNameSpace:	XtceString, // [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentParameterRefEntryType {
<complexContent>
<extension base="/*xtce:*/ArgumentSequenceEntryType">
	parameterRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentParameterSegmentRefEntryType {
<complexContent>
<extension base="/*xtce:*/ArgumentSequenceEntryType">
	parameterRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
	order:	/*xtce:*/PositiveLongType, // [attribute]
	sizeInBits:	/*xtce:*/PositiveLongType, // use="required"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentStreamSegmentEntryType {
<complexContent>
<extension base="/*xtce:*/ArgumentSequenceEntryType">
	streamRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
	order:	/*xtce:*/PositiveLongType, // [attribute]
	sizeInBits:	/*xtce:*/PositiveLongType, // use="required"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArrayParameterRefEntryType {
<complexContent>
<extension base="/*xtce:*/SequenceEntryType">
<sequence minOccurs="0">
	DimensionList:	/*xtce:*/DimensionListType, // [element]
</sequence>
	parameterRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct BaseAlarmType { // abstract="true"
<sequence>
	AncillaryDataSet:	/*xtce:*/AncillaryDataSetType, // minOccurs="0"/ [element]
</sequence>
	name:	XtceString, // use="optional" [attribute]
</attribute>
	shortDescription:	/*xtce:*/ShortDescriptionType, // [attribute]
</attribute>
}
*/

#[derive(Debug)]
pub struct BaseContainerType {
//<sequence>
//	RestrictionCriteria:	/*xtce:*/RestrictionCriteriaType, // minOccurs="0" [element]
//</sequence>
	containerRef:	/*xtce:*/NameReferenceType, // use="required" [attribute]
//</attribute>
}

#[derive(Debug)]
pub struct ContainerType { // abstract="true" mixed="false"
//<complexContent>
//<extension base="/*xtce:*/NameDescriptionType">
//<sequence>
//	DefaultRateInStream:	/*xtce:*/RateInStreamType, // minOccurs="0"/ [element]
//	RateInStreamSet:	/*xtce:*/RateInStreamSetType, // minOccurs="0"/ [element]
//	BinaryEncoding:	/*xtce:*/BinaryDataEncodingType, // minOccurs="0" [element]
//</sequence>
//</extension>
//</complexContent>
}

#[derive(Debug)]
pub struct ContainerRefSetType {
//<sequence>
	ContainerRef:	/*xtce:*/ContainerRefType, // maxOccurs="unbounded"/ [element]
//</sequence>
}

#[derive(Debug)]
pub struct ContainerRefType {
	containerRef:	/*xtce:*/NameReferenceType, // use="required" [attribute]
}

#[derive(Debug)]
pub struct ContainerRefEntryType {
//<complexContent>
//<extension base="/*xtce:*/SequenceEntryType">
	containerRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct ContainerSegmentRefEntryType {
<complexContent>
<extension base="/*xtce:*/SequenceEntryType">
	containerRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
	order:	/*xtce:*/PositiveLongType, // [attribute]
	sizeInBits:	/*xtce:*/PositiveLongType, // use="required"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ContainerSetType {
<choice maxOccurs="unbounded">
	SequenceContainer:	/*xtce:*/SequenceContainerType, // [element]
</choice>
}
*/

#[derive(Debug)]
pub struct EntryListType { // mixed="false"
//<choice minOccurs="0" maxOccurs="unbounded">
//	ParameterRefEntry:	/*xtce:*/ParameterRefEntryType, // [element]
//	ParameterSegmentRefEntry:	/*xtce:*/ParameterSegmentRefEntryType, // [element]
//	ContainerRefEntry:	/*xtce:*/ContainerRefEntryType, // [element]
//	ContainerSegmentRefEntry:	/*xtce:*/ContainerSegmentRefEntryType, // [element]
//	StreamSegmentEntry:	/*xtce:*/StreamSegmentEntryType, // [element]
//	IndirectParameterRefEntry:	/*xtce:*/IndirectParameterRefEntryType, // [element]
//	ArrayParameterRefEntry:	/*xtce:*/ArrayParameterRefEntryType, // [element]
//</choice>
}

/*
#[derive(Debug)]
pub struct IndirectParameterRefEntryType {
<complexContent>
<extension base="/*xtce:*/SequenceEntryType">
<sequence>
	ParameterInstance:	/*xtce:*/ParameterInstanceRefType, // [element]
</sequence>
	aliasNameSpace:	XtceString, // [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct LocationInContainerInBitsType {
<complexContent>
<extension base="/*xtce:*/IntegerValueType">
	referenceLocation:	/*xtce:*/ReferenceLocationType, // default="previousEntry" [attribute]
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentLocationInContainerInBitsType {
<complexContent>
<extension base="/*xtce:*/ArgumentIntegerValueType">
	referenceLocation:	/*xtce:*/ReferenceLocationType, // default="previousEntry"/ [attribute]
</extension>
</complexContent>
}
*/

#[derive(Debug)]
pub struct MessageRefType {
	messageRef:	/*xtce:*/NameReferenceType, // use="required" [attribute]
}

/*
#[derive(Debug)]
pub struct ParameterRefEntryType {
<complexContent>
<extension base="/*xtce:*/SequenceEntryType">
	parameterRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ParameterSegmentRefEntryType {
<complexContent>
<extension base="/*xtce:*/SequenceEntryType">
	parameterRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
	order:	/*xtce:*/PositiveLongType, // [attribute]
	sizeInBits:	/*xtce:*/PositiveLongType, // use="required"/ [attribute]
</extension>
</complexContent>
}
*/

#[derive(Debug)]
pub struct SequenceContainerType {
//<complexContent>
//<extension base="/*xtce:*/ContainerType">
//<sequence>
	EntryList:	/*xtce:*/EntryListType, // [element]
	BaseContainer:	/*xtce:*/BaseContainerType, // minOccurs="0" [element]
//</sequence>
	r#abstract:	bool, // default="false" [attribute]
	idlePattern:	/*xtce:*/FixedIntegerValueType, // default="0x0" [attribute]
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct SequenceEntryType { // abstract="true"
<sequence>
	LocationInContainerInBits:	/*xtce:*/LocationInContainerInBitsType, // minOccurs="0" [element]
	RepeatEntry:	/*xtce:*/RepeatType, // minOccurs="0" [element]
	IncludeCondition:	/*xtce:*/MatchCriteriaType, // minOccurs="0" [element]
	TimeAssociation:	/*xtce:*/TimeAssociationType, // minOccurs="0" [element]
	AncillaryDataSet:	/*xtce:*/AncillaryDataSetType, // minOccurs="0" [element]
</sequence>
	shortDescription:	/*xtce:*/ShortDescriptionType, // use="optional" [attribute]
</attribute>
}

#[derive(Debug)]
pub struct ArgumentSequenceEntryType { // abstract="true"
<sequence>
	LocationInContainerInBits:	/*xtce:*/ArgumentLocationInContainerInBitsType, // minOccurs="0" [element]
	RepeatEntry:	/*xtce:*/ArgumentRepeatType, // minOccurs="0" [element]
	IncludeCondition:	/*xtce:*/ArgumentMatchCriteriaType, // minOccurs="0" [element]
	AncillaryDataSet:	/*xtce:*/AncillaryDataSetType, // minOccurs="0" [element]
</sequence>
	shortDescription:	/*xtce:*/ShortDescriptionType, // use="optional" [attribute]
</attribute>
}
*/

#[derive(Debug)]
pub struct ServiceType {
//<complexContent>
//<extension base="/*xtce:*/NameDescriptionType">
//<choice>
//	MessageRefSet:	/*xtce:*/MessageRefSetType, // [element]
//	ContainerRefSet:	/*xtce:*/ContainerRefSetType, // [element]
//</choice>
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct StreamSegmentEntryType {
<complexContent>
<extension base="/*xtce:*/SequenceEntryType">
	streamRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
	order:	/*xtce:*/PositiveLongType, // [attribute]
	sizeInBits:	/*xtce:*/PositiveLongType, // use="required"/ [attribute]
</extension>
</complexContent>
}
*/

#[derive(Debug)]
pub struct MessageType {
//<complexContent>
//<extension base="/*xtce:*/NameDescriptionType">
//<sequence>
//	MatchCriteria:	/*xtce:*/MatchCriteriaType, // [element]
	ContainerRef:	/*xtce:*/ContainerRefType, // [element]
//</sequence>
//</extension>
//</complexContent>
}

#[derive(Debug)]
pub struct MessageSetType {
//<complexContent>
//<extension base="/*xtce:*/OptionalNameDescriptionType">
//<sequence>
	Message:	/*xtce:*/MessageType, // maxOccurs="unbounded"/ [element]
//</sequence>
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct RateInStreamSetType {
<sequence>
	RateInStream:	/*xtce:*/RateInStreamWithStreamNameType, // maxOccurs="unbounded"/ [element]
</sequence>
}

#[derive(Debug)]
pub struct RateInStreamType {
	basis:	/*xtce:*/BasisType, // default="perSecond" [attribute]
</attribute>
	minimumValue:	XtceDouble, // [attribute]
</attribute>
	maximumValue:	XtceDouble, // [attribute]
</attribute>
}

#[derive(Debug)]
pub struct RateInStreamWithStreamNameType {
<complexContent>
<extension base="/*xtce:*/RateInStreamType">
	streamRef:	/*xtce:*/NameReferenceType, // use="required" [attribute]
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ReferenceLocationType {
<restriction base="string">
<enumeration value="containerStart"/>
<enumeration value="containerEnd"/>
<enumeration value="previousEntry"/>
<enumeration value="nextEntry"/>
</restriction>

#[derive(Debug)]
pub struct ReferencePointType {
<restriction base="string">
<enumeration value="start"/>
<enumeration value="end"/>
</restriction>
*/

/*
#[derive(Debug)]
pub struct RestrictionCriteriaType {
//<complexContent>
//<extension base="/*xtce:*/MatchCriteriaType">
//<choice>
//	NextContainer:	/*xtce:*/ContainerRefType, // minOccurs="0" [element]
//</choice>
//</extension>
//</complexContent>
}

*/ /* packaging.rs */

/* ****** End of Packaging Schema ********************** */
/* ************************************************************* */
/* ****** Telemetry Schema ******************************* */

/* telemetry.rs
*/
#[derive(Debug)]
pub struct AbsoluteTimeParameterType {
//<complexContent>
//<extension base="/*xtce:*/AbsoluteTimeDataType"/>
//</complexContent>
}

#[derive(Debug)]
pub struct AggregateParameterType {
//<complexContent>
//<extension base="/*xtce:*/AggregateDataType"/>
//</complexContent>
}

#[derive(Debug)]
pub struct ArrayParameterType {
//<complexContent>
//<extension base="/*xtce:*/ArrayDataTypeType">
//<sequence>
	DimensionList:	/*xtce:*/DimensionListType, // [element]
//</sequence>
//</extension>
//</complexContent>
}

#[derive(Debug)]
pub struct BinaryParameterType {
//<complexContent>
//<extension base="/*xtce:*/BinaryDataType">
//<sequence>
//	DefaultAlarm:	/*xtce:*/BinaryAlarmType, // minOccurs="0" [element]
//	BinaryContextAlarmList:	/*xtce:*/BinaryContextAlarmListType, // minOccurs="0" [element]
//</sequence>
//</extension>
//</complexContent>
}

#[derive(Debug)]
pub struct BooleanParameterType {
//<complexContent>
//<extension base="/*xtce:*/BooleanDataType">
//<sequence>
//	DefaultAlarm:	/*xtce:*/BooleanAlarmType, // minOccurs="0" [element]
//	ContextAlarmList:	/*xtce:*/BooleanContextAlarmListType, // minOccurs="0" [element]
//</sequence>
//</extension>
//</complexContent>
}

#[derive(Debug)]
pub struct EnumeratedParameterType {
//<complexContent>
//<extension base="/*xtce:*/EnumeratedDataType">
//<sequence>
//	DefaultAlarm:	/*xtce:*/EnumerationAlarmType, // minOccurs="0" [element]
//	ContextAlarmList:	/*xtce:*/EnumerationContextAlarmListType, // minOccurs="0" [element]
//</sequence>
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct EnumerationContextAlarmListType {
<sequence>
	ContextAlarm:	/*xtce:*/EnumerationContextAlarmType, // maxOccurs="unbounded" [element]
</sequence>
}
*/

#[derive(Debug)]
pub struct FloatParameterType {
//<complexContent>
//<extension base="/*xtce:*/FloatDataType">
//<sequence>
//	DefaultAlarm:	/*xtce:*/NumericAlarmType, // minOccurs="0" [element]
//	ContextAlarmList:	/*xtce:*/NumericContextAlarmListType, // minOccurs="0" [element]
//</sequence>
//</extension>
//</complexContent>
}

#[derive(Debug)]
pub struct IntegerParameterType {
//<complexContent>
//<extension base="/*xtce:*/IntegerDataType">
//<sequence>
//	DefaultAlarm:	/*xtce:*/NumericAlarmType, // minOccurs="0" [element]
//	ContextAlarmList:	/*xtce:*/NumericContextAlarmListType, // minOccurs="0" [element]
//</sequence>
//</extension>
//</complexContent>
}

#[derive(Debug)]
pub struct MessageRefSetType {
//<sequence>
	MessageRef:	/*xtce:*/MessageRefType, // maxOccurs="unbounded"/ [element]
//</sequence>
}

/*
#[derive(Debug)]
pub struct NumericContextAlarmListType {
<sequence>
	ContextAlarm:	/*xtce:*/NumericContextAlarmType, // maxOccurs="unbounded" [element]
</sequence>
}
*/

#[derive(Debug)]
pub struct ParameterInstanceRefType {
//<complexContent>
//<extension base="/*xtce:*/ParameterRefType">
//	instance:	XtceLong, // default="0"/ [attribute]
//	useCalibratedValue:	XtceBoolean, // default="true"/ [attribute]
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct ParameterPropertiesType { // mixed="false"
<sequence>
	SystemName:	XtceString, // minOccurs="0" [element]
	ValidityCondition:	/*xtce:*/MatchCriteriaType, // minOccurs="0" [element]
	PhysicalAddressSet:	/*xtce:*/PhysicalAddressSetType, // minOccurs="0" [element]
	TimeAssociation:	/*xtce:*/TimeAssociationType, // minOccurs="0" [element]
</sequence>
	dataSource:	/*xtce:*/TelemetryDataSourceType, // use="optional" [attribute]
</attribute>
	readOnly:	XtceBoolean, // use="optional" default="false" [attribute]
</attribute>
	persistence:	XtceBoolean, // default="true" [attribute]
</attribute>
}
*/

#[derive(Debug)]
pub struct ParameterType {
//<complexContent>
//<extension base="/*xtce:*/NameDescriptionType">
//<sequence>
	//ParameterProperties:	/*xtce:*/ParameterPropertiesType, // minOccurs="0" [element]
//</sequence>
	parameterTypeRef:	/*xtce:*/NameReferenceType, // use="required" [attribute]
//</attribute>
	//initialValue:	XtceString, // use="optional" [attribute]
//<appinfo>The value type must match the Parameter type</appinfo>
//</attribute>
//</extension>
//</complexContent>
}

#[derive(Debug)]
pub struct ParameterTypeSetType {
// <choice maxOccurs="unbounded">
	StringParameterType:	/*xtce:*/StringParameterType, // [element]
	EnumeratedParameterType:	/*xtce:*/EnumeratedParameterType, // [element]
	IntegerParameterType:	/*xtce:*/IntegerParameterType, // [element]
	BinaryParameterType:	/*xtce:*/BinaryParameterType, // [element]
	FloatParameterType:	/*xtce:*/FloatParameterType, // [element]
	BooleanParameterType:	/*xtce:*/BooleanParameterType, // [element]
	RelativeTimeParameterType:	/*xtce:*/RelativeTimeParameterType, // [element]
	AbsoluteTimeParameterType:	/*xtce:*/AbsoluteTimeParameterType, // [element]
	ArrayParameterType:	/*xtce:*/ArrayParameterType, // [element]
	AggregateParameterType:	/*xtce:*/AggregateParameterType, // [element]
// </choice>
}

#[derive(Debug)]
pub struct ParameterRefType {
	parameterRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
}

/*
#[derive(Debug)]
pub struct PhysicalAddressSetType {
<sequence>
	PhysicalAddress:	/*xtce:*/PhysicalAddressType, // minOccurs="0" maxOccurs="unbounded" [element]
</sequence>
}

#[derive(Debug)]
pub struct PhysicalAddressType { // mixed="false"
<sequence>
	SubAddress:	/*xtce:*/PhysicalAddressType, // minOccurs="0" [element]
</sequence>
	sourceName:	XtceString, // [attribute]
</attribute>
	sourceAddress:	XtceString, // [attribute]
</attribute>
}

#[derive(Debug)]
pub struct ServiceSetType {
<sequence>
	Service:	/*xtce:*/ServiceType, // maxOccurs="unbounded"/ [element]
</sequence>
}
*/

#[derive(Debug)]
pub struct StringParameterType {
//<complexContent>
//<extension base="/*xtce:*/StringDataType">
//<sequence>
//	DefaultAlarm:	/*xtce:*/StringAlarmType, // minOccurs="0" [element]
//	ContextAlarmList:	/*xtce:*/StringContextAlarmListType, // minOccurs="0" [element]
//</sequence>
//</extension>
//</complexContent>
}

#[derive(Debug)]
pub struct RelativeTimeParameterType {
//<complexContent>
//<extension base="/*xtce:*/RelativeTimeDataType">
//<sequence>
//	DefaultAlarm:	/*xtce:*/TimeAlarmType, // minOccurs="0" [element]
//	ContextAlarmList:	/*xtce:*/TimeContextAlarmListType, // minOccurs="0" [element]
//</sequence>
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct TelemetryDataSourceType {
<restriction base="string">
<enumeration value="telemetered"/>
<enumeration value="derived"/>
<enumeration value="constant"/>
<enumeration value="local"/>
<enumeration value="ground"/>
</restriction>

#[derive(Debug)]
pub struct TimeAssociationType {
<complexContent>
<extension base="/*xtce:*/ParameterInstanceRefType">
	interpolateTime:	XtceBoolean, // default="true" [attribute]
</attribute>
	offset:	XtceDouble, // [attribute]
</attribute>
	unit:	/*xtce:*/TimeAssociationUnitType, // default="si_second" [attribute]
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct TimeWindowIsRelativeToType {
<restriction base="string">
<enumeration value="commandRelease"/>
<enumeration value="timeLastVerifierPassed"/>
</restriction>

*/ /* telemetry.rs */

/* ****** End of Telemetry Schema *********************** */
/* ************************************************************* */
/* ****** Command Schema ******************************* */

/* command.rs

#[derive(Debug)]
pub struct AbsoluteTimeArgumentType {
<complexContent>
<extension base="/*xtce:*/ArgumentAbsoluteTimeDataType"/>
</complexContent>
}

#[derive(Debug)]
pub struct ArrayArgumentType {
<complexContent>
<extension base="/*xtce:*/ArrayDataTypeType">
<sequence>
	DimensionList:	/*xtce:*/ArgumentDimensionListType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct AggregateArgumentType {
<complexContent>
<extension base="/*xtce:*/AggregateDataType"/>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentAssignmentListType {
<sequence>
	ArgumentAssignment:	/*xtce:*/ArgumentAssignmentType, // maxOccurs="unbounded" [element]
</sequence>
}

#[derive(Debug)]
pub struct ArgumentAssignmentType {
	argumentName:	/*xtce:*/NameReferenceType, // use="required" [attribute]
</attribute>
	argumentValue:	XtceString, // use="required" [attribute]
</attribute>
}

#[derive(Debug)]
pub struct ArgumentComparisonType {
<choice>
	ParameterInstanceRef:	/*xtce:*/ParameterInstanceRefType, // [element]
	ArgumentInstanceRef:	/*xtce:*/ArgumentInstanceRefType, // [element]
</choice>
	comparisonOperator:	/*xtce:*/ComparisonOperatorsType, // default="==" [attribute]
</attribute>
	value:	XtceString, // use="required" [attribute]
</attribute>
}

#[derive(Debug)]
pub struct ArgumentComparisonCheckType {
<complexContent>
<extension base="/*xtce:*/BaseConditionsType">
<sequence>
<choice>
	ParameterInstanceRef:	/*xtce:*/ParameterInstanceRefType, // [element]
	ArgumentInstanceRef:	/*xtce:*/ArgumentInstanceRefType, // [element]
</choice>
	ComparisonOperator:	/*xtce:*/ComparisonOperatorsType, // [element]
<choice>
<choice>
	ParameterInstanceRef:	/*xtce:*/ParameterInstanceRefType, // [element]
	ArgumentInstanceRef:	/*xtce:*/ArgumentInstanceRefType, // [element]
</choice>
	Value:	XtceString, // [element]
</choice>
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentComparisonListType {
<sequence>
	Comparison:	/*xtce:*/ArgumentComparisonType, // maxOccurs="unbounded" [element]
</sequence>
}

#[derive(Debug)]
pub struct ArgumentDiscreteLookupType {
<complexContent>
<extension base="/*xtce:*/ArgumentMatchCriteriaType">
	value:	XtceLong, // use="required" [attribute]
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentDiscreteLookupListType {
<sequence>
	DiscreteLookup:	/*xtce:*/ArgumentDiscreteLookupType, // maxOccurs="unbounded" [element]
</sequence>
}

#[derive(Debug)]
pub struct ArgumentDynamicValueType {
<sequence>
<choice>
	ArgumentInstanceRef:	/*xtce:*/ArgumentInstanceRefType, // [element]
	ParameterInstanceRef:	/*xtce:*/ParameterInstanceRefType, // [element]
</choice>
	LinearAdjustment:	/*xtce:*/LinearAdjustmentType, // minOccurs="0" [element]
</sequence>
}

#[derive(Debug)]
pub struct ArgumentInputAlgorithmType {
<complexContent>
<extension base="/*xtce:*/SimpleAlgorithmType">
<sequence>
	InputSet:	/*xtce:*/ArgumentInputSetType, // minOccurs="0" [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentInputSetType {
<choice maxOccurs="unbounded">
	InputParameterInstanceRef:	/*xtce:*/InputParameterInstanceRefType, // [element]
	InputArgumentInstanceRef:	/*xtce:*/ArgumentInstanceRefType, // [element]
</choice>
}

#[derive(Debug)]
pub struct ArgumentInstanceRefType {
	argumentRef:	/*xtce:*/NameType, // use="required" [attribute]
</attribute>
	useCalibratedValue:	XtceBoolean, // default="true" [attribute]
</attribute>
}

#[derive(Debug)]
pub struct ArgumentListType {
<sequence>
	Argument:	/*xtce:*/ArgumentType, // maxOccurs="unbounded" [element]
<appinfo>Need to ensure that the named types actually exist</appinfo>
</sequence>
}

#[derive(Debug)]
pub struct ArgumentBooleanExpressionType {
<choice>
	Condition:	/*xtce:*/ArgumentComparisonCheckType, // [element]
	ANDedConditions:	/*xtce:*/ArgumentANDedConditionsType, // [element]
	ORedConditions:	/*xtce:*/ArgumentORedConditionsType, // [element]
</choice>
}

#[derive(Debug)]
pub struct ArgumentANDedConditionsType {
<complexContent>
<extension base="/*xtce:*/BaseConditionsType">
<choice minOccurs="2" maxOccurs="unbounded">
	Condition:	/*xtce:*/ArgumentComparisonCheckType, // [element]
	ORedConditions:	/*xtce:*/ArgumentORedConditionsType, // [element]
</choice>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentORedConditionsType {
<complexContent>
<extension base="/*xtce:*/BaseConditionsType">
<choice minOccurs="2" maxOccurs="unbounded">
	Condition:	/*xtce:*/ArgumentComparisonCheckType, // [element]
	ANDedConditions:	/*xtce:*/ArgumentANDedConditionsType, // [element]
</choice>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentMatchCriteriaType {
<choice>
	Comparison:	/*xtce:*/ArgumentComparisonType, // [element]
	ComparisonList:	/*xtce:*/ArgumentComparisonListType, // [element]
	BooleanExpression:	/*xtce:*/ArgumentBooleanExpressionType, // [element]
	CustomAlgorithm:	/*xtce:*/ArgumentInputAlgorithmType, // [element]
</choice>
}

#[derive(Debug)]
pub struct ArgumentType {
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
	argumentTypeRef:	/*xtce:*/NameReferenceType, // use="required" [attribute]
</attribute>
	initialValue:	XtceString, // [attribute]
<appinfo>The value type must match the Argument type</appinfo>
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ArgumentTypeSetType {
<choice maxOccurs="unbounded">
	StringArgumentType:	/*xtce:*/StringArgumentType, // [element]
	EnumeratedArgumentType:	/*xtce:*/EnumeratedArgumentType, // [element]
	IntegerArgumentType:	/*xtce:*/IntegerArgumentType, // [element]
	BinaryArgumentType:	/*xtce:*/BinaryArgumentType, // [element]
	FloatArgumentType:	/*xtce:*/FloatArgumentType, // [element]
	BooleanArgumentType:	/*xtce:*/BooleanArgumentType, // [element]
	RelativeTimeAgumentType:	/*xtce:*/RelativeTimeArgumentType, // [element]
	AbsoluteTimeArgumentType:	/*xtce:*/AbsoluteTimeArgumentType, // [element]
	ArrayArgumentType:	/*xtce:*/ArrayArgumentType, // [element]
	AggregateArgumentType:	/*xtce:*/AggregateArgumentType, // [element]
</choice>
}

#[derive(Debug)]
pub struct BaseMetaCommandType {
<sequence>
	ArgumentAssignmentList:	/*xtce:*/ArgumentAssignmentListType, // minOccurs="0" [element]
</sequence>
	metaCommandRef:	/*xtce:*/NameReferenceType, // use="required" [attribute]
</attribute>
}

#[derive(Debug)]
pub struct BinaryArgumentType {
<complexContent>
<extension base="/*xtce:*/ArgumentBinaryDataType"/>
</complexContent>
}

#[derive(Debug)]
pub struct BlockMetaCommandType {
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
<sequence>
	MetaCommandStepList:	/*xtce:*/MetaCommandStepListType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct BooleanArgumentType {
<complexContent>
<extension base="/*xtce:*/ArgumentBooleanDataType"/>
</complexContent>
}

#[derive(Debug)]
pub struct CommandContainerEntryListType {
<choice minOccurs="0" maxOccurs="unbounded">
	ParameterRefEntry:	/*xtce:*/ArgumentParameterRefEntryType, // [element]
	ParameterSegmentRefEntry:	/*xtce:*/ArgumentParameterSegmentRefEntryType, // [element]
	ContainerRefEntry:	/*xtce:*/ArgumentContainerRefEntryType, // [element]
	ContainerSegmentRefEntry:	/*xtce:*/ArgumentContainerSegmentRefEntryType, // [element]
	StreamSegmentEntry:	/*xtce:*/ArgumentStreamSegmentEntryType, // [element]
	IndirectParameterRefEntry:	/*xtce:*/ArgumentIndirectParameterRefEntryType, // [element]
	ArrayParameterRefEntry:	/*xtce:*/ArgumentArrayParameterRefEntryType, // [element]
	ArgumentRefEntry:	/*xtce:*/ArgumentArgumentRefEntryType, // [element]
	ArrayArgumentRefEntry:	/*xtce:*/ArgumentArrayArgumentRefEntryType, // [element]
	FixedValueEntry:	/*xtce:*/ArgumentFixedValueEntryType, // [element]
</choice>
}

#[derive(Debug)]
pub struct CommandContainerSetType {
<sequence>
	CommandContainer:	/*xtce:*/SequenceContainerType, // maxOccurs="unbounded"/ [element]
</sequence>
}

#[derive(Debug)]
pub struct CommandContainerType { // mixed="false"
<complexContent>
<extension base="/*xtce:*/ContainerType">
<sequence>
	EntryList:	/*xtce:*/CommandContainerEntryListType, // [element]
	BaseContainer:	/*xtce:*/BaseContainerType, // minOccurs="0" [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct CommandVerifierType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/OptionalNameDescriptionType">
<sequence>
<choice>
	ComparisonList:	/*xtce:*/ComparisonListType, // [element]
	ContainerRef:	/*xtce:*/ContainerRefType, // [element]
	ParameterValueChange:	/*xtce:*/ParameterValueChangeType, // [element]
	CustomAlgorithm:	/*xtce:*/InputAlgorithmType, // [element]
	BooleanExpression:	/*xtce:*/BooleanExpressionType, // [element]
	Comparison:	/*xtce:*/ComparisonType, // [element]
</choice>
<choice>
	CheckWindow:	/*xtce:*/CheckWindowType, // [element]
	CheckWindowAlgorithms:	/*xtce:*/CheckWindowAlgorithmsType, // [element]
</choice>
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct EnumeratedArgumentType {
<complexContent>
<extension base="/*xtce:*/ArgumentEnumeratedDataType"/>
</complexContent>
}
*/

#[derive(Debug)]
pub struct FloatArgumentType {
//<complexContent>
//<extension base="/*xtce:*/ArgumentFloatDataType">
//<sequence>
//	ValidRangeSet:	/*xtce:*/ValidFloatRangeSetType, // minOccurs="0" [element]
//</sequence>
//</extension>
//</complexContent>
}

#[derive(Debug)]
pub struct IntegerArgumentType {
//<complexContent>
//<extension base="/*xtce:*/ArgumentIntegerDataType">
//<sequence>
//	ValidRangeSet:	/*xtce:*/ValidIntegerRangeSetType, // minOccurs="0" [element]
//</sequence>
//</extension>
//</complexContent>
}

#[derive(Debug)]
pub struct MetaCommandSetType {
//<choice maxOccurs="unbounded">
//	MetaCommand:	/*xtce:*/MetaCommandType, // [element]
//<key name="ArgumentNameKey">
//<selector xpath="/*xtce:*/ArgumentList/*"/>
//<field xpath="@name"/>
//</key>
//	MetaCommandRef:	/*xtce:*/NameReferenceType, // [element]
//	BlockMetaCommand:	/*xtce:*/BlockMetaCommandType, // [element]
//</choice>
}

/*

#[derive(Debug)]
pub struct MetaCommandStepListType {
<sequence>
	MetaCommandStep:	/*xtce:*/MetaCommandStepType, // maxOccurs="unbounded" [element]
</sequence>
}

#[derive(Debug)]
pub struct MetaCommandStepType {
<sequence>
	ArgumentAssigmentList:	/*xtce:*/ArgumentAssignmentListType, // minOccurs="0"/ [element]
</sequence>
	metaCommandRef:	/*xtce:*/NameReferenceType, // use="required"/ [attribute]
}

#[derive(Debug)]
pub struct MetaCommandType { // mixed="false"
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
<sequence>
	BaseMetaCommand:	/*xtce:*/BaseMetaCommandType, // minOccurs="0" [element]
	SystemName:	XtceString, // minOccurs="0" [element]
	ArgumentList:	/*xtce:*/ArgumentListType, // minOccurs="0" [element]
	CommandContainer:	/*xtce:*/CommandContainerType, // minOccurs="0" [element]
	TransmissionConstraintList:	/*xtce:*/TransmissionConstraintListType, // minOccurs="0" [element]
	DefaultSignificance:	/*xtce:*/SignificanceType, // minOccurs="0" [element]
	ContextSignificanceList:	/*xtce:*/ContextSignificanceListType, // minOccurs="0" [element]
	Interlock:	/*xtce:*/InterlockType, // minOccurs="0" [element]
	VerifierSet:	/*xtce:*/VerifierSetType, // minOccurs="0" [element]
	ParameterToSetList:	/*xtce:*/ParameterToSetListType, // minOccurs="0" [element]
	ParametersToSuspendAlarmsOnSet:	/*xtce:*/ParametersToSuspendAlarmsOnSetType, // minOccurs="0" [element]
</sequence>
	abstract:	XtceBoolean, // default="false" [attribute]
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct RelativeTimeArgumentType {
<complexContent>
<extension base="/*xtce:*/ArgumentRelativeTimeDataType"/>
</complexContent>
}

#[derive(Debug)]
pub struct SignificanceType { // mixed="false"
	spaceSystemAtRisk:	/*xtce:*/NameReferenceType, // [attribute]
</attribute>
	reasonForWarning:	XtceString, // [attribute]
	consequenceLevel:	/*xtce:*/ConsequenceLevelType, // default="normal"/ [attribute]
}
*/

#[derive(Debug)]
pub struct StringArgumentType {
//<complexContent>
//<extension base="/*xtce:*/ArgumentStringDataType"/>
//</complexContent>
}

/* ************************************************************* */
/* ******** Types used with Cmd Execution Control **** */

/*
#[derive(Debug)]
pub struct AcceptedVerifierType {
<complexContent>
<extension base="/*xtce:*/CommandVerifierType"/>
</complexContent>
}

#[derive(Debug)]
pub struct CheckWindowType {
	timeToStartChecking:	/*xtce:*/RelativeTimeType, // [attribute]
	timeToStopChecking:	/*xtce:*/RelativeTimeType, // use="required"/ [attribute]
	timeWindowIsRelativeTo:	/*xtce:*/TimeWindowIsRelativeToType, // default="timeLastVerifierPassed"/ [attribute]
}

#[derive(Debug)]
pub struct CheckWindowAlgorithmsType {
<sequence>
	StartCheck:	/*xtce:*/InputAlgorithmType, // [element]
	StopTime:	/*xtce:*/InputAlgorithmType, // [element]
</sequence>
}

#[derive(Debug)]
pub struct CompleteVerifierType {
<complexContent>
<extension base="/*xtce:*/CommandVerifierType">
<sequence minOccurs="0">
	ReturnParmRef:	/*xtce:*/ParameterRefType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ContextSignificanceType {
<sequence>
	ContextMatch:	/*xtce:*/ContextMatchType, // [element]
	Significance:	/*xtce:*/SignificanceType, // [element]
</sequence>
}

#[derive(Debug)]
pub struct ContextSignificanceListType {
<sequence>
	ContextSignificance:	/*xtce:*/ContextSignificanceType, // maxOccurs="unbounded" [element]
</sequence>
}

#[derive(Debug)]
pub struct ExecutionVerifierType {
<complexContent>
<extension base="/*xtce:*/CommandVerifierType">
<sequence minOccurs="0">
	PercentComplete:	/*xtce:*/PercentCompleteType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct FailedVerifierType {
<complexContent>
<extension base="/*xtce:*/CommandVerifierType">
<sequence minOccurs="0">
	ReturnParmRef:	/*xtce:*/ParameterRefType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct InterlockType {
	scopeToSpaceSystem:	/*xtce:*/NameReferenceType, // [attribute]
</attribute>
	verificationToWaitFor:	/*xtce:*/VerifierEnumerationType, // default="complete" [attribute]
</attribute>
	verificationProgressPercentage:	XtceDouble, // [attribute]
</attribute>
	suspendable:	XtceBoolean, // default="false" [attribute]
</attribute>
}

#[derive(Debug)]
pub struct ParameterToSetType {
<appinfo>Value type must match Parameter type.</appinfo>
<complexContent>
<extension base="/*xtce:*/ParameterRefType">
<choice>
	Derivation:	/*xtce:*/MathOperationType, // [element]
	NewValue:	XtceString, // [element]
</choice>
	setOnVerification:	/*xtce:*/VerifierEnumerationType, // default="complete" [attribute]
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ParameterToSetListType {
<sequence>
	ParameterToSet:	/*xtce:*/ParameterToSetType, // maxOccurs="unbounded"/ [element]
</sequence>
}

#[derive(Debug)]
pub struct ParameterToSuspendAlarmsOnType {
<complexContent>
<extension base="/*xtce:*/ParameterRefType">
	suspenseTime:	/*xtce:*/RelativeTimeType, // use="required"/ [attribute]
	verifierToTriggerOn:	/*xtce:*/VerifierEnumerationType, // default="release"/ [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ParametersToSuspendAlarmsOnSetType {
<sequence>
	ParameterToSuspendAlarmsOn:	/*xtce:*/ParameterToSuspendAlarmsOnType, // maxOccurs="unbounded"/ [element]
</sequence>
}

#[derive(Debug)]
pub struct ParameterValueChangeType {
<sequence>
	ParameterRef:	/*xtce:*/ParameterRefType, // [element]
	Change:	/*xtce:*/ChangeValueType, // [element]
</sequence>
}

#[derive(Debug)]
pub struct QueuedVerifierType {
<complexContent>
<extension base="/*xtce:*/CommandVerifierType"/>
</complexContent>
}

#[derive(Debug)]
pub struct ReceivedVerifierType {
<complexContent>
<extension base="/*xtce:*/CommandVerifierType"/>
</complexContent>
}

#[derive(Debug)]
pub struct SentFromRangeVerifierType {
<complexContent>
<extension base="/*xtce:*/CommandVerifierType"/>
</complexContent>
}

#[derive(Debug)]
pub struct TimeAssociationUnitType {
<restriction base="string">
<enumeration value="si_nanosecond"/>
<enumeration value="si_microsecond"/>
<enumeration value="si_millsecond"/>
<enumeration value="si_second"/>
<enumeration value="minute"/>
<enumeration value="day"/>
<enumeration value="julianYear"/>
</restriction>

#[derive(Debug)]
pub struct TransferredToRangeVerifierType {
<complexContent>
<extension base="/*xtce:*/CommandVerifierType"/>
</complexContent>
}

#[derive(Debug)]
pub struct TransmissionConstraintType {
<complexContent>
<extension base="/*xtce:*/MatchCriteriaType">
	timeOut:	/*xtce:*/RelativeTimeType, // [attribute]
/*  removed for CASTOR: default="PT0S"  */
</attribute>
	suspendable:	XtceBoolean, // default="false" [attribute]
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct TransmissionConstraintListType {
<sequence>
	TransmissionConstraint:	/*xtce:*/TransmissionConstraintType, // maxOccurs="unbounded"/ [element]
</sequence>
}

#[derive(Debug)]
pub struct VerifierEnumerationType {
<restriction base="string">
<enumeration value="release"/>
<enumeration value="transferredToRange"/>
<enumeration value="sentFromRange"/>
<enumeration value="received"/>
<enumeration value="accepted"/>
<enumeration value="queued"/>
<enumeration value="executing"/>
<enumeration value="complete"/>
<enumeration value="failed"/>
</restriction>

#[derive(Debug)]
pub struct VerifierSetType {
<sequence>
	TransferredToRangeVerifier:	/*xtce:*/TransferredToRangeVerifierType, // minOccurs="0"/ [element]
	SentFromRangeVerifier:	/*xtce:*/SentFromRangeVerifierType, // minOccurs="0"/ [element]
	ReceivedVerifier:	/*xtce:*/ReceivedVerifierType, // minOccurs="0"/ [element]
	AcceptedVerifier:	/*xtce:*/AcceptedVerifierType, // minOccurs="0"/ [element]
	QueuedVerifier:	/*xtce:*/QueuedVerifierType, // minOccurs="0"/ [element]
	ExecutionVerifier:	/*xtce:*/ExecutionVerifierType, // minOccurs="0" maxOccurs="unbounded"/ [element]
	CompleteVerifier:	/*xtce:*/CompleteVerifierType, // minOccurs="0" maxOccurs="unbounded"/ [element]
	FailedVerifier:	/*xtce:*/FailedVerifierType, // minOccurs="0"/ [element]
</sequence>
}

*/ /* command.rs */

/* ******** End of Command Schema ********************* */
/* ************************************************************* */
/* ****** Algorithm Schema ******************************** */

/* algorithm.rs

#[derive(Debug)]
pub struct AlgorithmTextType {
<simpleContent>
<extension base="string">
	language:	XtceString, // default="pseudo"/ [attribute]
</extension>
</simpleContent>
}

#[derive(Debug)]
pub struct BaseTriggerType { // abstract="true"
}

#[derive(Debug)]
pub struct ChecksumType {
<sequence>
	InputAlgorithm:	/*xtce:*/InputAlgorithmType, // minOccurs="0" [element]
</sequence>
	bitsFromReference:	/*xtce:*/NonNegativeLongType, //
	reference:	/*xtce:*/ReferencePointType, // default="start"
    name:   Any, // use="required">

#[derive(Debug)]

#[derive(Debug)]
<simpleType>
<restriction base="string">
<enumeration value="unix_sum"/>
<enumeration value="sum8"/>
<enumeration value="sum16"/>
<enumeration value="sum24"/>
<enumeration value="sum32"/>
<enumeration value="fletcher4"/>
<enumeration value="fletcher8"/>
<enumeration value="fletcher16"/>
<enumeration value="fletcher32"/>
<enumeration value="adler32"/>
<enumeration value="luhn"/>
<enumeration value="verhoeff"/>
<enumeration value="damm"/>
<enumeration value="custom">
</enumeration>
</restriction>
}
	hashSizeInBits:	/*xtce:*/PositiveLongType, //
}

#[derive(Debug)]
pub struct ConstantType {
	constantName:	XtceString, //
	value:	XtceString, // required
}

#[derive(Debug)]
pub struct ContextCalibratorListType {
<sequence>
	ContextCalibrator:	Vec</*xtce:*/ContextCalibratorType>,
</sequence>
}

#[derive(Debug)]
pub struct ExternalAlgorithmType {
	implementationName:	XtceString, // required
	algorithmLocation:	XtceString, // required
}

#[derive(Debug)]
pub struct ExternalAlgorithmSetType {
<sequence>
	ExternalAlgorithm:	Vec</*xtce:*/ExternalAlgorithmType>,
</sequence>
}

#[derive(Debug)]
pub struct InputAlgorithmType {
<complexContent>
<extension base="/*xtce:*/SimpleAlgorithmType">
<sequence>
	InputSet:	Option</*xtce:*/InputSetType>,
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct InputOutputAlgorithmType {
<complexContent>
<extension base="/*xtce:*/InputAlgorithmType">
<sequence>
	OutputSet:	Option</*xtce:*/OutputSetType>,
</sequence>
	thread:	XtceBoolean, // use="optional" default="false"
</extension>
</complexContent>
}
*/

#[derive(Debug)]
pub struct InputOutputTriggerAlgorithmType {
//<complexContent>
//<extension base="/*xtce:*/InputOutputAlgorithmType">
//<sequence>
//	TriggerSet:	Option</*xtce:*/TriggerSetType>,
//</sequence>
//	triggerContainer:	/*xtce:*/NameReferenceType, // use="optional" [attribute]
	priority:	XtceInt, // use="optional" [attribute]
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct InputParameterInstanceRefType {
<complexContent>
<extension base="/*xtce:*/ParameterInstanceRefType">
	inputName:	XtceString, //
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct InputSetType {
<choice maxOccurs="unbounded">
	InputParameterInstanceRef:	/*xtce:*/InputParameterInstanceRefType, // [element]
	Constant:	Option</*xtce:*/ConstantType>,
</choice>
}
*/

#[derive(Debug)]
pub struct MathAlgorithmType {
//<complexContent>
//<extension base="/*xtce:*/NameDescriptionType">
//<sequence>
//	MathOperation:	/*xtce:*/TriggeredMathOperationType, // [element]
//</sequence>
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct OnContainerUpdateTriggerType {
<complexContent>
<extension base="/*xtce:*/BaseTriggerType">
	containerRef:	/*xtce:*/NameReferenceType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct OnPeriodicRateTriggerType {
<complexContent>
<extension base="/*xtce:*/BaseTriggerType">
	fireRateInSeconds:	XtceDouble, // required
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct OnParameterUpdateTriggerType {
<complexContent>
<extension base="/*xtce:*/BaseTriggerType">
	parameterRef:	/*xtce:*/NameReferenceType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct OutputParameterRefType {
<complexContent>
<extension base="/*xtce:*/ParameterRefType">
	outputName:	XtceString, //
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct OutputSetType {
<sequence>
	OutputParameterRef:	Vec</*xtce:*/OutputParameterRefType>,
</sequence>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct SimpleAlgorithmType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
<sequence>
	AlgorithmText:	Option</*xtce:*/AlgorithmTextType>,
	ExternalAlgorithmSet:	Option</*xtce:*/ExternalAlgorithmSetType>,
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct TriggeredMathOperationType {
<complexContent>
<extension base="/*xtce:*/MathOperationType">
<sequence>
	TriggerSet:	/*xtce:*/TriggerSetType, // [element]
</sequence>
	outputParameterRef:	/*xtce:*/NameReferenceType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct TriggerSetType {
<choice maxOccurs="unbounded">
	OnParameterUpdateTrigger:	/*xtce:*/OnParameterUpdateTriggerType, // [element]
	OnContainerUpdateTrigger:	/*xtce:*/OnContainerUpdateTriggerType, // [element]
	OnPeriodicRateTrigger:	/*xtce:*/OnPeriodicRateTriggerType, // [element]
</choice>
	name:	XtceString, // use="optional" [attribute]
	triggerRate:	/*xtce:*/NonNegativeLongType, // use="optional" default="1" [attribute]
}

/* ************************************************************* */
/* ******** Calibrator Algorithm Types *********************** */

#[derive(Debug)]

#[derive(Debug)]
pub struct BaseCalibratorType { // abstract="true"
<sequence>
	AncillaryDataSet:	Option</*xtce:*/AncillaryDataSetType>,
</sequence>
	name:	XtceString, // [attribute]
	shortDescription:	/*xtce:*/ShortDescriptionType, // [attribute]
}

#[derive(Debug)]
pub struct CalibratorType {
<complexContent>
<extension base="/*xtce:*/BaseCalibratorType">
<choice>
	SplineCalibrator:	/*xtce:*/SplineCalibratorType, // [element]
	PolynomialCalibrator:	/*xtce:*/PolynomialCalibratorType, // [element]
	MathOperationCalibrator:	/*xtce:*/MathOperationCalibratorType, // [element]
</choice>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ChangeValueType {
	value:	XtceDouble, // required
}

#[derive(Debug)]
pub struct MathOperationCalibratorType {
<complexContent>
<extension base="/*xtce:*/BaseCalibratorType">
<choice maxOccurs="unbounded">
	ValueOperand:	XtceString, // [element]
	ThisParameterOperand:	XtceString, // fixed="" [element]
	Operator:	/*xtce:*/MathOperatorsType, // [element]
	ParameterInstanceRefOperand:	/*xtce:*/ParameterInstanceRefType, // [element]
</choice>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct PolynomialCalibratorType {
<complexContent>
<extension base="/*xtce:*/BaseCalibratorType">
<sequence>
	Term:	Vec</*xtce:*/TermType>,
<appinfo>Generally only up to second order powers are reflexive. Implementations may limit the maximum number of terms supported.</appinfo>
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct SplineCalibratorType {
<complexContent>
<extension base="/*xtce:*/BaseCalibratorType">
<sequence>
	SplinePoint:	Vec</*xtce:*/SplinePointType>,
</sequence>
	order:	/*xtce:*/NonNegativeLongType, // default="1" [attribute]
	extrapolate:	XtceBoolean, // default="false" [attribute]
</extension>
</complexContent>
}

*/ /* algorithm.rs */

/* ******** End of Algorithm Schema ********************* */
/* ************************************************************* */
/* ******** Stream Definitions Schema ******************* */

/* stream_definitions.rs

#[derive(Debug)]
pub struct AutoInvertType {
<sequence>
	InvertAlgorithm:	Option</*xtce:*/InputAlgorithmType>,
</sequence>
	badFramesToAutoInvert:	/*xtce:*/PositiveLongType, // default="1024"
}
*/

#[derive(Debug)]
pub struct CustomStreamType {
//<complexContent>
//<extension base="/*xtce:*/PCMStreamType">
//<sequence>
//	EncodingAlgorithm:	/*xtce:*/InputAlgorithmType, // [element]
//	DecodingAlgorithm:	/*xtce:*/InputOutputAlgorithmType, // [element]
//</sequence>
	encodedStreamRef:	/*xtce:*/NameReferenceType, // required
	decodedStreamRef:	/*xtce:*/NameReferenceType, // required
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct FlagBitType {
<restriction base="string">
<enumeration value="zeros"/>
<enumeration value="ones"/>
</restriction>
}

#[derive(Debug)]
pub struct FlagType {
	flagSizeInBits:	/*xtce:*/PositiveLongType, // default="6"
	flagBitType:	/*xtce:*/FlagBitType, // default="ones"
}

#[derive(Debug)]
pub struct FrameStreamType {
<complexContent>
<extension base="/*xtce:*/PCMStreamType">
<sequence>
<choice>
	ContainerRef:	/*xtce:*/ContainerRefType, // [element]
	ServiceRef:	/*xtce:*/ServiceRefType, // [element]
</choice>
	StreamRef:	Option</*xtce:*/StreamRefType>,
</sequence>
</extension>
</complexContent>
}
*/

#[derive(Debug)]
pub struct FixedFrameStreamType {
//<complexContent>
//<extension base="/*xtce:*/FrameStreamType">
//<sequence>
//	SyncStrategy:	/*xtce:*/FixedFrameSyncStrategyType, // [element]
//</sequence>
//	syncApertureInBits:	/*xtce:*/NonNegativeLongType, // default="0" [attribute]
	frameLengthInBits:	XtceLong, // required
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct FixedFrameSyncStrategyType {
<complexContent>
<extension base="/*xtce:*/SyncStrategyType">
<sequence>
	SyncPattern:	/*xtce:*/SyncPatternType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct PCMStreamType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
	bitRateInBPS:	XtceDouble, //
	pcmType:	/*xtce:*/PCMType, // default="NRZL"
	inverted:	XtceBoolean, // default="false"
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct PCMType {
<restriction base="string">
<enumeration value="NRZL"/>
<enumeration value="NRZM"/>
<enumeration value="NRZS"/>
<enumeration value="BiPhaseL"/>
<enumeration value="BiPhaseM"/>
<enumeration value="BiPhaseS"/>
</restriction>
}

#[derive(Debug)]
pub struct StreamRefType {
	streamRef:	/*xtce:*/NameReferenceType, // required
}

#[derive(Debug)]
pub struct StreamSetType {
<choice maxOccurs="unbounded">
	FixedFrameStream:	/*xtce:*/FixedFrameStreamType, // [element]
	VariableFrameStream:	/*xtce:*/VariableFrameStreamType, // [element]
	CustomStream:	/*xtce:*/CustomStreamType, // [element]
</choice>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct SyncStrategyType { // abstract="true"
<sequence>
	AutoInvert:	Option</*xtce:*/AutoInvertType>,
</sequence>
	verifyToLockGoodFrames:	/*xtce:*/NonNegativeLongType, // default="4"
	checkToLockGoodFrames:	/*xtce:*/NonNegativeLongType, // default="1"
	maxBitErrorsInSyncPattern:	/*xtce:*/NonNegativeLongType, // default="0" [attribute]
}
*/

#[derive(Debug)]
pub struct StreamSetType {
/* FIXME: figure this out
    max_occurs = unbounded,
*/
    fixed_frame_stream:     FixedFrameStreamType,
    variable_frame_stream:  VariableFrameStreamType,
    custom_stream:          CustomStreamType,
}

/*
#[derive(Debug)]
pub struct SyncPatternType {
	pattern:	hexBinary, // required
	bitLocationFromStartOfContainer:	XtceLong, // default="0"
	mask:	hexBinary, //
	maskLengthInBits:	/*xtce:*/PositiveLongType, // [attribute]
	patternLengthInBits:	/*xtce:*/PositiveLongType, // required
}
*/

#[derive(Debug)]
pub struct VariableFrameStreamType {
//<complexContent>
//<extension base="/*xtce:*/FrameStreamType">
//<sequence>
//	SyncStrategy:	/*xtce:*/VariableFrameSyncStrategyType, // [element]
//</sequence>
//</extension>
//</complexContent>
}

/*
#[derive(Debug)]
pub struct VariableFrameSyncStrategyType {
<complexContent>
<extension base="/*xtce:*/SyncStrategyType">
<sequence>
	Flag:	/*xtce:*/FlagType, // [element]
</sequence>
</extension>
</complexContent>
}

*/ /* stream_definitions.rs */

/* ******** End of Stream Definition Schema ************ */
/* ************************************************************* */
/* ******** DataTypes *************************************** */

/* data_types.rs

#[derive(Debug)]

#[derive(Debug)]
pub struct ArgumentAbsoluteTimeDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/ArgumentBaseTimeDataType">
	initialValue:	dateTime, // [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct AbsoluteTimeDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/BaseTimeDataType">
	initialValue:	dateTime, // [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct AggregateDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
<sequence>
	MemberList:	/*xtce:*/MemberListType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct ArrayDataTypeType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
	arrayTypeRef:	/*xtce:*/NameReferenceType, // required
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct BaseDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
<sequence>
	UnitSet:	Option</*xtce:*/UnitSetType>,
<choice minOccurs="0">
	BinaryDataEncoding:	/*xtce:*/BinaryDataEncodingType, // [element]
	FloatDataEncoding:	/*xtce:*/FloatDataEncodingType, // [element]
	IntegerDataEncoding:	/*xtce:*/IntegerDataEncodingType, // [element]
	StringDataEncoding:	/*xtce:*/StringDataEncodingType, // [element]
</choice>
</sequence>
	baseType:	/*xtce:*/NameReferenceType, // [attribute]
<appinfo>Must be derived from a like type (e.g,, String from String). No circular derivations.</appinfo>
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct ArgumentBaseDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
<sequence>
	UnitSet:	Option</*xtce:*/UnitSetType>,
<choice minOccurs="0">
	BinaryDataEncoding:	/*xtce:*/ArgumentBinaryDataEncodingType, // [element]
	FloatDataEncoding:	/*xtce:*/FloatDataEncodingType, // [element]
	IntegerDataEncoding:	/*xtce:*/IntegerDataEncodingType, // [element]
	StringDataEncoding:	/*xtce:*/ArgumentStringDataEncodingType, // [element]
</choice>
</sequence>
	baseType:	/*xtce:*/NameReferenceType, // [attribute]
<appinfo>Must be derived from a like type (e.g,, String from String). No circular derivations.</appinfo>
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct ArgumentBaseTimeDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
<sequence>
	Encoding:	Option</*xtce:*/EncodingType>,
	ReferenceTime:	Option</*xtce:*/ReferenceTimeType>,
</sequence>
	baseType:	/*xtce:*/NameReferenceType, // [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct BaseTimeDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
<sequence>
	Encoding:	Option</*xtce:*/EncodingType>,
	ReferenceTime:	Option</*xtce:*/ReferenceTimeType>,
</sequence>
	baseType:	/*xtce:*/NameReferenceType, // [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct ArgumentBinaryDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/ArgumentBaseDataType">
	initialValue:	hexBinary, // [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct BinaryDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/BaseDataType">
	initialValue:	hexBinary, // [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct ArgumentBooleanDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/ArgumentBaseDataType">
	initialValue:	XtceString, // [attribute]
<appinfo>Initial value must match either the oneStringValue or the zeroStringValue</appinfo>
	oneStringValue:	XtceString, // default="True" [attribute]
	zeroStringValue:	XtceString, // default="False" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct BooleanDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/BaseDataType">
	initialValue:	XtceString, // [attribute]
<appinfo>Initial value must match either the oneStringValue or the zeroStringValue</appinfo>
	oneStringValue:	XtceString, // default="True" [attribute]
	zeroStringValue:	XtceString, // default="False" [attribute]
</extension>
</complexContent>
}
*/

#[derive(Debug)]
pub struct DimensionType { // mixed="false"
//<sequence>
	StartingIndex:	/*xtce:*/IntegerValueType, // [element]
	EndingIndex:	/*xtce:*/IntegerValueType, // [element]
//</sequence>
}

#[derive(Debug)]
pub struct ArgumentDimensionType {
//<sequence>
	StartingIndex:	/*xtce:*/ArgumentIntegerValueType, // [element]
	EndingIndex:	/*xtce:*/ArgumentIntegerValueType, // [element]
//</sequence>
}

#[derive(Debug)]
pub struct DimensionListType {
//<sequence>
	Dimension:	Vec</*xtce:*/DimensionType>,
//</sequence>
}

#[derive(Debug)]
pub struct ArgumentDimensionListType {
//<sequence>
	Dimension:	Vec</*xtce:*/ArgumentDimensionType>,
//</sequence>
}

/*
#[derive(Debug)]
pub struct ArgumentEnumeratedDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/ArgumentBaseDataType">
<sequence>
	EnumerationList:	/*xtce:*/EnumerationListType, // [element]
<appinfo>Check that values do not overlap in the mappings.</appinfo>
</sequence>
	initialValue:	XtceString, // [attribute]
<appinfo>Label must be in the enumeration list to be valid.</appinfo>
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct EnumeratedDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/BaseDataType">
<sequence>
	EnumerationList:	/*xtce:*/EnumerationListType, // [element]
<appinfo>Check that values do not overlap in the mappings.</appinfo>
</sequence>
	initialValue:	XtceString, // [attribute]
<appinfo>Label must be in the enumeration list to be valid.</appinfo>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct EnumerationListType {
<sequence>
	Enumeration:	Vec</*xtce:*/ValueEnumerationType>,
</sequence>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct ArgumentFloatDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/ArgumentBaseDataType">
<sequence>
	ToString:	Option</*xtce:*/ToStringType>,
</sequence>
	initialValue:	XtceDouble, // [attribute]
	sizeInBits:	/*xtce:*/FloatSizeInBitsType, // default="32" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct FloatDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/BaseDataType">
<sequence>
	ToString:	Option</*xtce:*/ToStringType>,
<element name="ValidRange" minOccurs="0">

#[derive(Debug)]

#[derive(Debug)]
<complexType>
<complexContent>
<extension base="/*xtce:*/FloatRangeType">
	validRangeAppliesToCalibrated:	XtceBoolean, // default="true" [attribute]
</extension>
</complexContent>
}
</sequence>
	initialValue:	XtceDouble, // [attribute]
	sizeInBits:	/*xtce:*/FloatSizeInBitsType, // default="32" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct ArgumentIntegerDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/ArgumentBaseDataType">
<sequence>
	ToString:	Option</*xtce:*/ToStringType>,
</sequence>
	initialValue:	/*xtce:*/FixedIntegerValueType, // [attribute]
	sizeInBits:	/*xtce:*/PositiveLongType, // default="32" [attribute]
	signed:	XtceBoolean, // default="true" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct IntegerDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/BaseDataType">
<sequence>
	ToString:	Option</*xtce:*/ToStringType>,
<element name="ValidRange" minOccurs="0">

#[derive(Debug)]

#[derive(Debug)]
<complexType>
<complexContent>
<extension base="/*xtce:*/IntegerRangeType">
	validRangeAppliesToCalibrated:	XtceBoolean, // default="true" [attribute]
</extension>
</complexContent>
}
</sequence>
	initialValue:	XtceLong, // [attribute]
	sizeInBits:	/*xtce:*/PositiveLongType, // default="32" [attribute]
	signed:	XtceBoolean, // default="true" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct MemberType {
<appinfo>ensure no circular references</appinfo>
<complexContent>
<extension base="/*xtce:*/NameDescriptionType">
	typeRef:	/*xtce:*/NameReferenceType, // required
	initialValue:	XtceString, // use="optional" [attribute]
<appinfo>The value type must match the Parameter type</appinfo>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct MemberListType {
<sequence>
	Member:	Vec</*xtce:*/MemberType>,
</sequence>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct ArgumentRelativeTimeDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/ArgumentBaseTimeDataType">
	initialValue:	duration, //
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct RelativeTimeDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/BaseTimeDataType">
	initialValue:	duration, //
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct ArgumentStringDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/ArgumentBaseDataType">
<sequence>
	SizeRangeInCharacters:	Option</*xtce:*/IntegerRangeType>,
</sequence>
	initialValue:	XtceString, // [attribute]
	restrictionPattern:	XtceString, // [attribute]
	characterWidth:	/*xtce:*/CharacterWidthType, //
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct StringDataType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/BaseDataType">
<sequence>
	SizeRangeInCharacters:	Option</*xtce:*/IntegerRangeType>,
</sequence>
	initialValue:	XtceString, // [attribute]
	restrictionPattern:	XtceString, // [attribute]
	characterWidth:	/*xtce:*/CharacterWidthType, //
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct UnitSetType {
<sequence>
	Unit:	Vec</*xtce:*/UnitType>,
</sequence>
}

#[derive(Debug)]
pub struct ValidFloatRangeSetType {
<sequence>
	ValidRange:	Vec</*xtce:*/FloatRangeType>,
</sequence>
	validRangeAppliesToCalibrated:	XtceBoolean, // default="true" [attribute]
}

#[derive(Debug)]
pub struct ValidIntegerRangeSetType {
<sequence>
	ValidRange:	Vec</*xtce:*/IntegerRangeType>,
</sequence>
	validRangeAppliesToCalibrated:	XtceBoolean, // default="true" [attribute]
}

/* ************************************************************* */
/* ******** Data Types used with Encoding ************** */

#[derive(Debug)]
pub struct BitOrderType {
<restriction base="string">
<enumeration value="leastSignificantBitFirst"/>
<enumeration value="mostSignificantBitFirst"/>
</restriction>
}

#[derive(Debug)]
pub struct ArgumentBinaryDataEncodingType {
<complexContent>
<extension base="/*xtce:*/DataEncodingType">
<sequence>
	SizeInBits:	/*xtce:*/ArgumentIntegerValueType, // [element]
	FromBinaryTransformAlgorithm:	Option</*xtce:*/ArgumentInputAlgorithmType>,
	ToBinaryTransformAlgorithm:	Option</*xtce:*/ArgumentInputAlgorithmType>,
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct BinaryDataEncodingType {
<complexContent>
<extension base="/*xtce:*/DataEncodingType">
<sequence>
	SizeInBits:	/*xtce:*/IntegerValueType, // [element]
	FromBinaryTransformAlgorithm:	Option</*xtce:*/InputAlgorithmType>,
	ToBinaryTransformAlgorithm:	Option</*xtce:*/InputAlgorithmType>,
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ByteType {
	byteSignificance:	/*xtce:*/NonNegativeLongType, // required
}

#[derive(Debug)]
pub struct CharacterWidthType {
<restriction base="integer">
<enumeration value="8"/>
<enumeration value="16"/>
</restriction>
}

#[derive(Debug)]
pub struct CRCType {
<sequence>
	Polynomial:	hexBinary, // [element]
	InitRemainder:	Option<hexBinary>,
	FinalXOR:	Option<hexBinary>,
</sequence>
	width:	/*xtce:*/PositiveLongType, //
	reflectData:	XtceBoolean, // default="false"
	reflectRemainder:	XtceBoolean, // default="false"
	bitsFromReference:	/*xtce:*/NonNegativeLongType, //
	reference:	/*xtce:*/ReferencePointType, // default="start"
}

#[derive(Debug)]

#[derive(Debug)]
pub struct DataEncodingType { // abstract="true"
<sequence>
	ErrorDetectCorrect:	Option</*xtce:*/ErrorDetectCorrectType>,
</sequence>
	bitOrder:	/*xtce:*/BitOrderType, // default="mostSignificantBitFirst"
	byteOrder:	/*xtce:*/ByteOrderType, // default="mostSignificantByteFirst"
}

#[derive(Debug)]
pub struct EncodingType {
<choice>
	BinaryDataEncoding:	/*xtce:*/BinaryDataEncodingType, // [element]
	FloatDataEncoding:	/*xtce:*/FloatDataEncodingType, // [element]
	IntegerDataEncoding:	/*xtce:*/IntegerDataEncodingType, // [element]
	StringDataEncoding:	/*xtce:*/StringDataEncodingType, // [element]
</choice>
	units:	/*xtce:*/TimeUnitsType, // default="seconds" [attribute]
	scale:	XtceDouble, // default="1" [attribute]
	offset:	XtceDouble, // default="0" [attribute]
}

#[derive(Debug)]
pub struct EpochType {
<union memberTypes="date dateTime /*xtce:*/EpochTimeEnumsType"/>
}

#[derive(Debug)]
pub struct FloatDataEncodingType {
<complexContent>
<extension base="/*xtce:*/DataEncodingType">
<sequence>
	DefaultCalibrator:	Option</*xtce:*/CalibratorType>,
	ContextCalibratorList:	Option</*xtce:*/ContextCalibratorListType>,
</sequence>
	encoding:	/*xtce:*/FloatEncodingType, // default="IEEE754_1985" [attribute]
	sizeInBits:	/*xtce:*/FloatEncodingSizeInBitsType, // default="32" [attribute]
<appinfo>Verify the number of bits for encoding is valid for the encoding method.</appinfo>
	changeThreshold:	XtceDouble, // use="optional" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct FloatEncodingSizeInBitsType {
<restriction base="unsignedShort">
<enumeration value="16">
</enumeration>
<enumeration value="32">
</enumeration>
<enumeration value="40">
</enumeration>
<enumeration value="48">
</enumeration>
<enumeration value="64">
</enumeration>
<enumeration value="80">
</enumeration>
<enumeration value="128">
</enumeration>
</restriction>
}

#[derive(Debug)]
pub struct FloatEncodingType {
<restriction base="string">
<enumeration value="IEEE754_1985"/>
<enumeration value="IEEE754"/>
<enumeration value="MILSTD_1750A"/>
<enumeration value="DEC"/>
<enumeration value="IBM"/>
<enumeration value="TI"/>
</restriction>
}

#[derive(Debug)]
pub struct FloatSizeInBitsType {
<restriction base="/*xtce:*/PositiveLongType">
<enumeration value="32"/>
<enumeration value="64"/>
<enumeration value="128"/>
</restriction>
}

#[derive(Debug)]
pub struct IntegerDataEncodingType {
<complexContent>
<extension base="/*xtce:*/DataEncodingType">
<sequence>
	DefaultCalibrator:	Option</*xtce:*/CalibratorType>,
	ContextCalibratorList:	Option</*xtce:*/ContextCalibratorListType>,
</sequence>
	encoding:	/*xtce:*/IntegerEncodingType, // default="unsigned" [attribute]
	sizeInBits:	/*xtce:*/PositiveLongType, // default="8" [attribute]
	changeThreshold:	/*xtce:*/NonNegativeLongType, // use="optional" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct IntegerEncodingType {
<restriction base="string">
<enumeration value="unsigned"/>
<enumeration value="signMagnitude"/>
<enumeration value="twosComplement"/>
<enumeration value="onesComplement"/>
<enumeration value="BCD"/>
<enumeration value="packedBCD"/>
</restriction>
}

#[derive(Debug)]
pub struct LeadingSizeType {
	sizeInBitsOfSizeTag:	/*xtce:*/PositiveLongType, // default="16"
}

#[derive(Debug)]
pub struct ParityFormType {
<restriction base="string">
<enumeration value="Even"/>
<enumeration value="Odd"/>
</restriction>
}

#[derive(Debug)]
pub struct ParityType {
	type:	/*xtce:*/ParityFormType, // required
	bitsFromReference:	/*xtce:*/NonNegativeLongType, // required
	reference:	/*xtce:*/ReferencePointType, // default="start"
}

#[derive(Debug)]
pub struct SizeInBitsType {
<sequence>
<element name="Fixed">

#[derive(Debug)]

#[derive(Debug)]
<complexType>
<sequence>
	FixedValue:	/*xtce:*/PositiveLongType, // [element]
</sequence>
}
	TerminationChar:	Option<hexBinary>,
	LeadingSize:	Option</*xtce:*/LeadingSizeType>,
</sequence>
}

#[derive(Debug)]
pub struct ArgumentVariableStringType {
<sequence>
<choice>
	DynamicValue:	/*xtce:*/ArgumentDynamicValueType, // [element]
	DiscreteLookupList:	/*xtce:*/ArgumentDiscreteLookupListType, // [element]
</choice>
	LeadingSize:	Option</*xtce:*/LeadingSizeType>,
	TerminationChar:	Option<hexBinary>,
</sequence>
	maxSizeInBits:	/*xtce:*/PositiveLongType, // required
}

#[derive(Debug)]
pub struct VariableStringType {
<sequence>
<choice>
	DynamicValue:	/*xtce:*/DynamicValueType, // [element]
	DiscreteLookupList:	/*xtce:*/DiscreteLookupListType, // [element]
</choice>
	LeadingSize:	Option</*xtce:*/LeadingSizeType>,
	TerminationChar:	Option<hexBinary>,
</sequence>
	maxSizeInBits:	/*xtce:*/PositiveLongType, // required
}

#[derive(Debug)]
pub struct ArgumentStringDataEncodingType {
<complexContent>
<extension base="/*xtce:*/DataEncodingType">
<choice>
	SizeInBits:	/*xtce:*/SizeInBitsType, // [element]
	Variable:	/*xtce:*/ArgumentVariableStringType, // [element]
</choice>
	encoding:	/*xtce:*/StringEncodingType, // default="UTF-8" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct StringDataEncodingType {
<complexContent>
<extension base="/*xtce:*/DataEncodingType">
<choice>
	SizeInBits:	/*xtce:*/SizeInBitsType, // [element]
	Variable:	/*xtce:*/VariableStringType, // [element]
</choice>
	encoding:	/*xtce:*/StringEncodingType, // default="UTF-8" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct StringEncodingType {
<restriction base="string">
<enumeration value="US-ASCII"/>
<enumeration value="ISO-8859-1"/>
<enumeration value="Windows-1252"/>
<enumeration value="UTF-8"/>
<enumeration value="UTF-16">
</enumeration>
<enumeration value="UTF-16LE">
</enumeration>
<enumeration value="UTF-16BE">
</enumeration>
<enumeration value="UTF-32">
</enumeration>
<enumeration value="UTF-32LE">
</enumeration>
<enumeration value="UTF-32BE">
</enumeration>
</restriction>
}

#[derive(Debug)]
pub struct ToStringType {
<sequence>
	NumberFormat:	/*xtce:*/NumberFormatType, // minOccurs="1" maxOccurs="1" [element]
</sequence>
}

#[derive(Debug)]
pub struct EpochTimeEnumsType {
<restriction base="string">
<enumeration value="TAI"/>
<enumeration value="J2000"/>
<enumeration value="UNIX"/>
<enumeration value="GPS"/>
</restriction>
}

*/ /* data_types.rs */

/* ******** DataTypes ************************************** */
/* ************************************************************* */
/* ******** Common Types Schema ********************** */
/*  Basic elements used for in all dictionaries  */

/* common_types.rs

*/
#[derive(Debug)]
pub struct AlgorithmSetType {
    /* FIXME: figure this out
    mixed = false,
    max_occurs = unbounded,
    */
    custom_algorithm:   InputOutputTriggerAlgorithmType,
    math_algorithm:     MathAlgorithmType,
}

/*
#[derive(Debug)]
pub struct AlgorithmSetType { // mixed="false"
<choice maxOccurs="unbounded">
	CustomAlgorithm:	/*xtce:*/InputOutputTriggerAlgorithmType, // [element]
	MathAlgorithm:	/*xtce:*/MathAlgorithmType, // [element]
</choice>
}

#[derive(Debug)]
pub struct AliasSetType {
<appinfo>Applications should enforce uniqueness of individual nameSpace attribute values. Aliases are usually unique within the same nameSpace attribute value, depending on the physical meaning of that nameSpace. There are some cases where Alias values can be duplicated in a single nameSpace value.</appinfo>
<sequence>
	Alias:	Vec</*xtce:*/AliasType>,
</sequence>
}

#[derive(Debug)]
pub struct AliasType {
	nameSpace:	XtceString, // required
	alias:	XtceString, // required
}

#[derive(Debug)]
pub struct AncillaryDataType {
<simpleContent>
<extension base="string">
	name:	XtceString, // required
	mimeType:	XtceString, // default="text/plain" [attribute]
	href:	anyURI, // [attribute]
</extension>
</simpleContent>
}

#[derive(Debug)]
pub struct AncillaryDataSetType {
<sequence>
	AncillaryData:	Vec</*xtce:*/AncillaryDataType>,
</sequence>
}

#[derive(Debug)]
pub struct ANDedConditionsType {
<complexContent>
<extension base="/*xtce:*/BaseConditionsType">
<choice minOccurs="2" maxOccurs="unbounded">
	Condition:	/*xtce:*/ComparisonCheckType, // [element]
	ORedConditions:	/*xtce:*/ORedConditionsType, // [element]
</choice>
</extension>
</complexContent>
}
*/

#[derive(Debug)]
pub struct AuthorSetType {
// <sequence>
	Author:	Vec</*xtce:*/AuthorType>,
// </sequence>
}

#[derive(Debug)]
pub struct AuthorType {
//<restriction base="string"/>
}

/*
#[derive(Debug)]
pub struct BaseConditionsType { // abstract="true"
}

#[derive(Debug)]
pub struct BinaryType {
<restriction base="string">
<pattern value="0[bB][0-1]+"/>
</restriction>
}

#[derive(Debug)]
pub struct BooleanExpressionType {
<choice>
	Condition:	/*xtce:*/ComparisonCheckType, // [element]
	ANDedConditions:	/*xtce:*/ANDedConditionsType, // [element]
	ORedConditions:	/*xtce:*/ORedConditionsType, // [element]
</choice>
}

#[derive(Debug)]
pub struct ByteOrderType {
<union memberTypes="/*xtce:*/ByteOrderCommonType /*xtce:*/ByteOrderArbitraryType"/>
}

#[derive(Debug)]
pub struct ByteOrderCommonType {
<restriction base="string">
<enumeration value="mostSignificantByteFirst"/>
<enumeration value="leastSignificantByteFirst"/>
</restriction>
}

#[derive(Debug)]
pub struct ByteOrderArbitraryType {
<restriction base="string">
<pattern value="(0|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15)(,(0|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15))*"/>
</restriction>
}

#[derive(Debug)]
pub struct ComparisonCheckType {
<complexContent>
<extension base="/*xtce:*/BaseConditionsType">
<sequence>
	ParameterInstanceRef:	/*xtce:*/ParameterInstanceRefType, // [element]
	ComparisonOperator:	/*xtce:*/ComparisonOperatorsType, // [element]
<choice>
	ParameterInstanceRef:	/*xtce:*/ParameterInstanceRefType, // [element]
	Value:	XtceString, // [element]
</choice>
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ComparisonListType {
<sequence>
	Comparison:	Vec</*xtce:*/ComparisonType>,
</sequence>
}

#[derive(Debug)]
pub struct ComparisonOperatorsType {
<restriction base="string">
<enumeration value="=="/>
<enumeration value="!="/>
<enumeration value="<"/>
<enumeration value="<="/>
<enumeration value=">"/>
<enumeration value=">="/>
</restriction>
}

#[derive(Debug)]
pub struct ComparisonType {
<complexContent>
<extension base="/*xtce:*/ParameterInstanceRefType">
	comparisonOperator:	/*xtce:*/ComparisonOperatorsType, // default="==" [attribute]
	value:	XtceString, // required
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ContextCalibratorType {
<sequence>
	ContextMatch:	/*xtce:*/ContextMatchType, // [element]
	Calibrator:	/*xtce:*/CalibratorType, // [element]
</sequence>
}

#[derive(Debug)]
pub struct ContextMatchType {
<complexContent>
<extension base="/*xtce:*/MatchCriteriaType"/>
</complexContent>
}

#[derive(Debug)]
pub struct CustomAlarmType {
<complexContent>
<extension base="/*xtce:*/BaseAlarmType">
<sequence>
	InputAlgorithm:	/*xtce:*/InputAlgorithmType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct PercentCompleteType {
<choice>
<element name="FixedValue">

#[derive(Debug)]
<simpleType>
<restriction base="double">
<minInclusive value="0.0"/>
<maxInclusive value="100.0"/>
</restriction>
}
	DynamicValue:	/*xtce:*/DynamicValueType, // [element]
</choice>
}

#[derive(Debug)]
pub struct DescriptionType { // abstract="true"
<sequence>
	LongDescription:	Option</*xtce:*/LongDescriptionType>,
	AliasSet:	Option</*xtce:*/AliasSetType>,
	AncillaryDataSet:	Option</*xtce:*/AncillaryDataSetType>,
</sequence>
	shortDescription:	/*xtce:*/ShortDescriptionType, // use="optional" [attribute]
}

#[derive(Debug)]
pub struct DiscreteLookupListType {
<sequence>
	DiscreteLookup:	Vec</*xtce:*/DiscreteLookupType>,
</sequence>
}

#[derive(Debug)]
pub struct DynamicValueType {
<sequence>
	ParameterInstanceRef:	/*xtce:*/ParameterInstanceRefType, // [element]
	LinearAdjustment:	Option</*xtce:*/LinearAdjustmentType>,
</sequence>
}

#[derive(Debug)]
pub struct ErrorDetectCorrectType {
<choice>
	Checksum:	/*xtce:*/ChecksumType, // [element]
	CRC:	/*xtce:*/CRCType, // [element]
	Parity:	/*xtce:*/ParityType, // [element]
</choice>
}
*/

#[derive(Debug)]
pub struct FixedIntegerValueType {
//<union memberTypes="integer /*xtce:*/HexadecimalType /*xtce:*/OctalType /*xtce:*/BinaryType"/>
}

#[derive(Debug)]
pub struct HeaderType {
//<sequence>
	AuthorSet:	Option</*xtce:*/AuthorSetType>,
	NoteSet:	Option</*xtce:*/NoteSetType>,
	HistorySet:	Option</*xtce:*/HistorySetType>,
//</sequence>
	version:	XtceString, // [attribute]
	date:	XtceString, // [attribute]
	classification:	XtceString, // default="NotClassified" [attribute]
	classificationInstructions:	XtceString, // [attribute]
	validationStatus:	/*xtce:*/ValidationStatusType, // required
}

/*
#[derive(Debug)]
pub struct HexadecimalType {
<restriction base="string">
<pattern value="0[xX][0-9a-fA-F]+"/>
</restriction>
}
*/

#[derive(Debug)]
pub struct HistorySetType {
//<sequence>
	History:	Vec</*xtce:*/HistoryType>,
//</sequence>
}

#[derive(Debug)]
pub struct HistoryType {
//<restriction base="string"/>
}

#[derive(Debug)]
pub struct IntegerValueType {
//<choice>
	FixedValue:	XtceLong, // [element]
//	DynamicValue:	/*xtce:*/DynamicValueType, // [element]
//	DiscreteLookupList:	/*xtce:*/DiscreteLookupListType, // [element]
//</choice>
}

#[derive(Debug)]
pub struct ArgumentIntegerValueType {
//<choice>
	FixedValue:	XtceLong, // [element]
//	DynamicValue:	/*xtce:*/ArgumentDynamicValueType, // [element]
//	DiscreteLookupList:	/*xtce:*/ArgumentDiscreteLookupListType, // [element]
//</choice>
}

/*
#[derive(Debug)]
pub struct LongDescriptionType {
<restriction base="string"/>
}

#[derive(Debug)]
pub struct MathOperatorsType {
<restriction base="string">
<enumeration value="+">
</enumeration>
<enumeration value="-">
</enumeration>
<enumeration value="*">
</enumeration>
<enumeration value="/">
<appinfo>An undefined condition exists if x2 is 0</appinfo>
</enumeration>
<enumeration value="%">
<appinfo>An undefined condition exists if x2 is 0. Implementations should verify modulo versus remainder behavior.</appinfo>
</enumeration>
<enumeration value="^">
<appinfo>An undefined condition exists if an imaginary number is the result. Imaginary numbers are not supported</appinfo>
</enumeration>
<enumeration value="y^x">
</enumeration>
<enumeration value="ln">
<appinfo>An undefined condition exists if x is less than or equal to 0</appinfo>
</enumeration>
<enumeration value="log">
<appinfo>An undefined condition exists if x is less than or equal to 0</appinfo>
</enumeration>
<enumeration value="e^x">
</enumeration>
<enumeration value="1/x">
<appinfo>An undefined condition exists if x is less than 0</appinfo>
</enumeration>
<enumeration value="x!">
<appinfo>An undefined condition exists if x is less than 0</appinfo>
</enumeration>
<enumeration value="tan">
</enumeration>
<enumeration value="cos">
</enumeration>
<enumeration value="sin">
</enumeration>
<enumeration value="atan">
</enumeration>
<enumeration value="atan2">
<appinfo>An undefined condition exists if x1 and x2 are 0</appinfo>
</enumeration>
<enumeration value="acos">
</enumeration>
<enumeration value="asin">
</enumeration>
<enumeration value="tanh">
</enumeration>
<enumeration value="cosh">
</enumeration>
<enumeration value="sinh">
</enumeration>
<enumeration value="atanh">
<appinfo>An undefined condition exists if x is outside the range [-1.0,+1.0]</appinfo>
</enumeration>
<enumeration value="acosh">
<appinfo>An undefined condition exists if n is less than 1</appinfo>
</enumeration>
<enumeration value="asinh">
</enumeration>
<enumeration value="swap">
</enumeration>
<enumeration value="drop">
</enumeration>
<enumeration value="dup">
</enumeration>
<enumeration value="over">
</enumeration>
<enumeration value="<<">
<appinfo>Limitation from SEI INT13-C. Use bitwise operators only on unsigned operands</appinfo>
</enumeration>
<enumeration value=">>">
<appinfo>Limitation from SEI INT13-C. Use bitwise operators only on unsigned operands</appinfo>
</enumeration>
<enumeration value="&">
<appinfo>Limitation from SEI INT13-C. Use bitwise operators only on unsigned operands</appinfo>
</enumeration>
<enumeration value="|">
<appinfo>Limitation from SEI INT13-C. Use bitwise operators only on unsigned operands</appinfo>
</enumeration>
<enumeration value="&&">
<appinfo>The result of this can only be 0 or 1</appinfo>
</enumeration>
<enumeration value="||">
<appinfo>The result of this can only be 0 or 1</appinfo>
</enumeration>
<enumeration value="!">
<appinfo>The result of this can only be 0 or 1</appinfo>
</enumeration>
<enumeration value="abs">
</enumeration>
<enumeration value="div">
</enumeration>
<enumeration value="int">
</enumeration>
<enumeration value=">">
<appinfo>The result of this can only be 0 or 1</appinfo>
</enumeration>
<enumeration value=">=">
<appinfo>The result of this can only be 0 or 1</appinfo>
</enumeration>
<enumeration value="<">
<appinfo>The result of this can only be 0 or 1</appinfo>
</enumeration>
<enumeration value="<=">
<appinfo>The result of this can only be 0 or 1</appinfo>
</enumeration>
<enumeration value="==">
<appinfo>The result of this can only be 0 or 1</appinfo>
</enumeration>
<enumeration value="!=">
<appinfo>The result of this can only be 0 or 1</appinfo>
</enumeration>
<enumeration value="min">
</enumeration>
<enumeration value="max">
</enumeration>
<enumeration value="xor">
<appinfo>Limitation from SEI INT13-C. Use bitwise operators only on unsigned operands</appinfo>
</enumeration>
<enumeration value="~">
<appinfo>Limitation from SEI INT13-C. Use bitwise operators only on unsigned operands</appinfo>
</enumeration>
</restriction>
}

#[derive(Debug)]
pub struct MatchCriteriaType {
<choice>
	Comparison:	/*xtce:*/ComparisonType, // [element]
	ComparisonList:	/*xtce:*/ComparisonListType, // [element]
	BooleanExpression:	/*xtce:*/BooleanExpressionType, // [element]
	CustomAlgorithm:	/*xtce:*/InputAlgorithmType, // [element]
</choice>
}

#[derive(Debug)]
pub struct MathOperationType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/MathOperationCalibratorType"/>
</complexContent>
}
*/

#[derive(Debug)]
pub struct NameType {
//<restriction base="normalizedString">
//<pattern value="[^./:\[\] ]+"/>
//</restriction>
}

#[derive(Debug)]
pub struct NameDescriptionType {
// <complexContent>
// <extension base="/*xtce:*/DescriptionType">
//	name:	/*xtce:*/NameType, // required
// </extension>
// </complexContent>
    name:   NameType, // required
}

#[derive(Debug)]
pub struct NameReferenceType {
//<restriction base="normalizedString">
//<pattern value="/?(([^./:\[\]]+|\.|\.\.)/)*([^./:\[\]]+)+"/>
//</restriction>
}

#[derive(Debug)]
pub struct NoteSetType {
// <sequence>
	Note:	Vec</*xtce:*/NoteType>,
//</sequence>
}

#[derive(Debug)]
pub struct NoteType {
// <restriction base="string"/>
}
/*

#[derive(Debug)]
pub struct NumberFormatType {
	numberBase:	/*xtce:*/RadixType, // use="optional" default="Decimal" [attribute]
	minimumFractionDigits:	/*xtce:*/NonNegativeLongType, // use="optional" default="0" [attribute]
	maximumFractionDigits:	/*xtce:*/NonNegativeLongType, // use="optional" [attribute]
	minimumIntegerDigits:	/*xtce:*/NonNegativeLongType, // use="optional" default="1" [attribute]
	maximumIntegerDigits:	/*xtce:*/NonNegativeLongType, // use="optional" [attribute]
	negativeSuffix:	XtceString, // use="optional" default="" [attribute]
	positiveSuffix:	XtceString, // use="optional" default="" [attribute]
	negativePrefix:	XtceString, // use="optional" default="-" [attribute]
	positivePrefix:	XtceString, // use="optional" default="" [attribute]
	showThousandsGrouping:	XtceBoolean, // use="optional" default="false" [attribute]
	notation:	/*xtce:*/FloatingPointNotationType, // use="optional" default="normal" [attribute]
}

#[derive(Debug)]
pub struct OctalType {
<restriction base="string">
<pattern value="0[oO][0-7]+"/>
</restriction>
}

#[derive(Debug)]
pub struct OptionalNameDescriptionType {
<complexContent>
<extension base="/*xtce:*/DescriptionType">
	name:	/*xtce:*/NameType, // use="optional" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ORedConditionsType {
<complexContent>
<extension base="/*xtce:*/BaseConditionsType">
<choice minOccurs="2" maxOccurs="unbounded">
	Condition:	/*xtce:*/ComparisonCheckType, // [element]
	ANDedConditions:	/*xtce:*/ANDedConditionsType, // [element]
</choice>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ParameterSetType {
<choice maxOccurs="unbounded">
	Parameter:	/*xtce:*/ParameterType, // [element]
<appinfo>Need to ensure that the named types actually exist</appinfo>
	ParameterRef:	/*xtce:*/ParameterRefType, // [element]
</choice>
}

#[derive(Debug)]
pub struct RadixType {
<restriction base="string">
<enumeration value="Decimal"/>
<enumeration value="Hexadecimal"/>
<enumeration value="Octal"/>
<enumeration value="Binary"/>
</restriction>
}

#[derive(Debug)]
pub struct RangeFormType {
<restriction base="string">
<enumeration value="outside"/>
<enumeration value="inside"/>
</restriction>
}

#[derive(Debug)]
pub struct ReferenceTimeType {
<choice>
	OffsetFrom:	/*xtce:*/ParameterInstanceRefType, // [element]
	Epoch:	/*xtce:*/EpochType, // [element]
</choice>
}

#[derive(Debug)]
pub struct RelativeTimeType {
<restriction base="duration"/>
}

#[derive(Debug)]
pub struct RepeatType {
<sequence>
	Count:	/*xtce:*/IntegerValueType, // [element]
	Offset:	Option</*xtce:*/IntegerValueType>,
</sequence>
}

#[derive(Debug)]
pub struct ArgumentRepeatType {
<sequence>
	Count:	/*xtce:*/ArgumentIntegerValueType, // [element]
	Offset:	Option</*xtce:*/ArgumentIntegerValueType>,
</sequence>
}

#[derive(Debug)]
pub struct ServiceRefType {
<simpleContent>
<extension base="/*xtce:*/NameReferenceType">
	serviceRef:	/*xtce:*/NameReferenceType, // required
</extension>
</simpleContent>
}

#[derive(Debug)]
pub struct ShortDescriptionType {
<restriction base="string"/>
}

#[derive(Debug)]
pub struct SplinePointType {
	order:	/*xtce:*/NonNegativeLongType, // default="1" [attribute]
	raw:	XtceDouble, // required
	calibrated:	XtceDouble, // required
}

#[derive(Debug)]
pub struct TermType {
	coefficient:	XtceDouble, // required
	exponent:	/*xtce:*/NonNegativeLongType, // required
}

#[derive(Debug)]
pub struct TimeUnitsType {
<restriction base="string">
<enumeration value="seconds"/>
<enumeration value="picoSeconds"/>
<enumeration value="days"/>
<enumeration value="months"/>
<enumeration value="years"/>
</restriction>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct UnitType { // mixed="true"
	power:	XtceDouble, // use="optional" default="1" [attribute]
	factor:	XtceString, // use="optional" default="1" [attribute]
	description:	/*xtce:*/ShortDescriptionType, // use="optional" [attribute]
	form:	/*xtce:*/UnitFormType, // use="optional" default="calibrated" [attribute]
}
*/

#[derive(Debug)]
pub struct ValidationStatusType {
/*
<restriction base="string">
<enumeration value="Unknown"/>
<enumeration value="Working"/>
<enumeration value="Draft"/>
<enumeration value="Test"/>
<enumeration value="Validated"/>
<enumeration value="Released"/>
<enumeration value="Withdrawn"/>
</restriction>
*/
}

/*
#[derive(Debug)]
pub struct ValueEnumerationType {
	value:	XtceLong, // required
	maxValue:	XtceLong, // [attribute]
	label:	XtceString, // required
	shortDescription:	/*xtce:*/ShortDescriptionType, // [attribute]
}


/* ************************************************************* */
/* ******** Types used with alarms *********************** */

#[derive(Debug)]
pub struct AlarmConditionsType {
<sequence>
	WatchAlarm:	Option</*xtce:*/MatchCriteriaType>,
	WarningAlarm:	Option</*xtce:*/MatchCriteriaType>,
	DistressAlarm:	Option</*xtce:*/MatchCriteriaType>,
	CriticalAlarm:	Option</*xtce:*/MatchCriteriaType>,
	SevereAlarm:	Option</*xtce:*/MatchCriteriaType>,
</sequence>
}

#[derive(Debug)]
pub struct AlarmRangesType {
<complexContent>
<extension base="/*xtce:*/BaseAlarmType">
<sequence>
	WatchRange:	Option</*xtce:*/FloatRangeType>,
	WarningRange:	Option</*xtce:*/FloatRangeType>,
	DistressRange:	Option</*xtce:*/FloatRangeType>,
	CriticalRange:	Option</*xtce:*/FloatRangeType>,
	SevereRange:	Option</*xtce:*/FloatRangeType>,
</sequence>
	rangeForm:	/*xtce:*/RangeFormType, // default="outside" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]

#[derive(Debug)]
pub struct AlarmType { // abstract="true"
<complexContent>
<extension base="/*xtce:*/BaseAlarmType">
<sequence>
<choice minOccurs="0">
	AlarmConditions:	/*xtce:*/AlarmConditionsType, // [element]
	CustomAlarm:	/*xtce:*/CustomAlarmType, // [element]
</choice>
</sequence>
	minViolations:	/*xtce:*/PositiveLongType, // default="1" [attribute]
	minConformance:	/*xtce:*/PositiveLongType, // [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct AlarmMultiRangesType {
<complexContent>
<extension base="/*xtce:*/BaseAlarmType">
<sequence>
	Range:	Vec</*xtce:*/MultiRangeType>,
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct BasisType {
<restriction base="string">
<enumeration value="perSecond"/>
<enumeration value="perContainerUpdate"/>
</restriction>
}

#[derive(Debug)]
pub struct BinaryAlarmType {
<complexContent>
<extension base="/*xtce:*/AlarmType"/>
</complexContent>
}

#[derive(Debug)]
pub struct BooleanAlarmType {
<complexContent>
<extension base="/*xtce:*/AlarmType"/>
</complexContent>
}

#[derive(Debug)]
pub struct BinaryContextAlarmListType {
<sequence>
	ContextAlarm:	Vec</*xtce:*/BinaryContextAlarmType>,
</sequence>
}

#[derive(Debug)]
pub struct BinaryContextAlarmType {
<complexContent>
<extension base="/*xtce:*/AlarmType">
<sequence>
	ContextMatch:	/*xtce:*/ContextMatchType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct BooleanContextAlarmType {
<complexContent>
<extension base="/*xtce:*/BooleanAlarmType">
<sequence>
	ContextMatch:	/*xtce:*/ContextMatchType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct BooleanContextAlarmListType {
<sequence>
	ContextAlarm:	Vec</*xtce:*/BooleanContextAlarmType>,
</sequence>
}

#[derive(Debug)]
pub struct ChangeAlarmRangesType {
<complexContent>
<extension base="/*xtce:*/AlarmRangesType">
	changeType:	/*xtce:*/ChangeSpanType, // default="changePerSecond"
	changeBasis:	/*xtce:*/ChangeBasisType, // default="absoluteChange"
	spanOfInterestInSamples:	/*xtce:*/PositiveLongType, // default="1"
	spanOfInterestInSeconds:	XtceDouble, // default="0"
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct ChangeBasisType {
<restriction base="string">
<enumeration value="absoluteChange"/>
<enumeration value="percentageChange"/>
</restriction>
}

#[derive(Debug)]
pub struct ConcernLevelsType {
<restriction base="string">
<enumeration value="normal"/>
<enumeration value="watch"/>
<enumeration value="warning"/>
<enumeration value="distress"/>
<enumeration value="critical"/>
<enumeration value="severe"/>
</restriction>
}

#[derive(Debug)]
pub struct ConsequenceLevelType {
<restriction base="string">
<enumeration value="normal">
</enumeration>
<enumeration value="vital">
</enumeration>
<enumeration value="critical">
</enumeration>
<enumeration value="forbidden">
</enumeration>
<enumeration value="user1">
</enumeration>
<enumeration value="user2">
</enumeration>
</restriction>
}

#[derive(Debug)]
pub struct ChangeSpanType {
<restriction base="string">
<enumeration value="changePerSecond"/>
<enumeration value="changePerSample"/>
</restriction>
}

#[derive(Debug)]
pub struct DiscreteLookupType {
<complexContent>
<extension base="/*xtce:*/MatchCriteriaType">
	value:	XtceLong, // required
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct EnumerationAlarmLevelType {
	alarmLevel:	/*xtce:*/ConcernLevelsType, // required
	enumerationLabel:	XtceString, // required
}

#[derive(Debug)]
pub struct EnumerationAlarmListType {
<sequence>
	EnumerationAlarm:	Vec</*xtce:*/EnumerationAlarmLevelType>,
</sequence>
}

#[derive(Debug)]
pub struct EnumerationContextAlarmType {
<complexContent>
<extension base="/*xtce:*/EnumerationAlarmType">
<sequence>
	ContextMatch:	/*xtce:*/ContextMatchType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct EnumerationAlarmType {
<appinfo>An additional check needs to be performed to ensure that the enumeration values in the alarms are valid enumeration values for the Parameter</appinfo>
<complexContent>
<extension base="/*xtce:*/AlarmType">
<sequence>
	EnumerationAlarmList:	Option</*xtce:*/EnumerationAlarmListType>,
</sequence>
	defaultAlarmLevel:	/*xtce:*/ConcernLevelsType, // default="normal" [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct FloatingPointNotationType {
<restriction base="string">
<enumeration value="normal"/>
<enumeration value="scientific"/>
<enumeration value="engineering"/>
</restriction>
}

#[derive(Debug)]
pub struct FloatRangeType {
<appinfo>Verify that the combination provided is usable.</appinfo>
	minInclusive:	XtceDouble, // [attribute]
	minExclusive:	XtceDouble, // [attribute]
	maxInclusive:	XtceDouble, // [attribute]
	maxExclusive:	XtceDouble, // [attribute]
}

#[derive(Debug)]
pub struct IntegerRangeType {
	minInclusive:	XtceLong, // [attribute]
	maxInclusive:	XtceLong, // [attribute]
}

#[derive(Debug)]
pub struct LinearAdjustmentType {
	slope:	XtceDouble, //
	intercept:	XtceDouble, // default="0"
}

#[derive(Debug)]
pub struct MultiRangeType {
<complexContent>
<extension base="/*xtce:*/FloatRangeType">
	rangeForm:	/*xtce:*/RangeFormType, // default="outside" [attribute]
	level:	/*xtce:*/ConcernLevelsType, // [attribute]
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct NumericAlarmType {
<complexContent>
<extension base="/*xtce:*/AlarmType">
<sequence>
	StaticAlarmRanges:	Option</*xtce:*/AlarmRangesType>,
	ChangeAlarmRanges:	Option</*xtce:*/ChangeAlarmRangesType>,
	AlarmMultiRanges:	Option</*xtce:*/AlarmMultiRangesType>,
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct NumericContextAlarmType {
<complexContent>
<extension base="/*xtce:*/NumericAlarmType">
<sequence>
	ContextMatch:	/*xtce:*/ContextMatchType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct StringAlarmLevelType {
	alarmLevel:	/*xtce:*/ConcernLevelsType, // required
	matchPattern:	XtceString, // required
}

#[derive(Debug)]
pub struct StringAlarmListType {
<sequence>
	StringAlarm:	Vec</*xtce:*/StringAlarmLevelType>,
</sequence>
}

#[derive(Debug)]
pub struct StringAlarmType {
<complexContent>
<extension base="/*xtce:*/AlarmType">
<sequence>
	StringAlarmList:	Option</*xtce:*/StringAlarmListType>,
</sequence>
	defaultAlarmLevel:	/*xtce:*/ConcernLevelsType, // default="normal"
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct StringContextAlarmType {
<complexContent>
<extension base="/*xtce:*/StringAlarmType">
<sequence>
	ContextMatch:	/*xtce:*/ContextMatchType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct StringContextAlarmListType {
<sequence>
	ContextAlarm:	Vec</*xtce:*/StringContextAlarmType>,
</sequence>
}

#[derive(Debug)]
pub struct TimeAlarmType {
<complexContent>
<extension base="/*xtce:*/AlarmType">
<sequence>
	StaticAlarmRanges:	Option</*xtce:*/TimeAlarmRangesType>,
	ChangePerSecondAlarmRanges:	Option</*xtce:*/TimeAlarmRangesType>,
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct TimeAlarmRangesType {
<complexContent>
<extension base="/*xtce:*/AlarmRangesType">
	timeUnits:	/*xtce:*/TimeUnitsType, // default="seconds"
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct TimeContextAlarmListType {
<sequence>
	ContextAlarm:	Vec</*xtce:*/TimeContextAlarmType>,
</sequence>
}

#[derive(Debug)]
pub struct TimeContextAlarmType {
<complexContent>
<extension base="/*xtce:*/TimeAlarmType">
<sequence>
	ContextMatch:	/*xtce:*/ContextMatchType, // [element]
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
pub struct NonNegativeLongType {
<restriction base="long">
<minInclusive value="0"/>
</restriction>
}

#[derive(Debug)]
pub struct PositiveLongType {
<restriction base="long">
<minInclusive value="1"/>
</restriction>
}

#[derive(Debug)]
pub struct UnitFormType {
<restriction base="string">
<enumeration value="calibrated"/>
<enumeration value="uncalibrated"/>
<enumeration value="raw"/>
</restriction>
}
*/ /* common_types.rs */

/* ******** End of Common Types Schema ************** */
/* ************************************************************* */
