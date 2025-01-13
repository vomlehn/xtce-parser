#[derive(Debug)]
struct ArgumentArgumentRefEntryType {
    argument_ref:   NameReferenceType,
    argument_sequence_entry_type:   ArgumentSequenceEntryType,
}

#[derive(Debug)]
struct ArgumentArrayArgumentRefEntryType {
}

#[derive(Debug)]

#[derive(Debug)]
struct ArgumentSequenceEntryType {
    // Attributes
    argument_ref:                       NameReferenceType,
    ast_entry_for_this_array_instance:  Boolean,
    // Elements
    argument_sequence_entry_type:   Option<ArgumentSequenceEntryType>,
    dimension_list:                 ArgumentDimensionListType,
}

#[derive(Debug)]
struct ArgumentArrayParameterRefEntryType {
<complexContent>
<extension base="xtce:ArgumentSequenceEntryType">
<sequence minOccurs="0">
	DimensionList:	xtce:DimensionListType,
</sequence>
	parameterRef:	xtce:NameReferenceType, // required
<attribute name="lastEntryForThisArrayInstance" type="boolean" default="false"/>
</extension>
</complexContent>
}

#[derive(Debug)]
struct ArgumentContainerRefEntryType {
<complexContent>
<extension base="xtce:ArgumentSequenceEntryType">
	containerRef:	xtce:NameReferenceType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
struct ArgumentContainerSegmentRefEntryType {
<complexContent>
<extension base="xtce:ArgumentSequenceEntryType">
	containerRef:	xtce:NameReferenceType, // required
<attribute name="order" type="xtce:PositiveLongType"/>
	sizeInBits:	xtce:PositiveLongType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
struct ArgumentFixedValueEntryType {
<complexContent>
<extension base="xtce:ArgumentSequenceEntryType">
<attribute name="name" type="string" use="optional">
</attribute>
	binaryValue:	hexBinary, // required
</attribute>
	sizeInBits:	xtce:PositiveLongType, // required
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
struct ArgumentIndirectParameterRefEntryType {
<complexContent>
<extension base="xtce:ArgumentSequenceEntryType">
<sequence>
	ParameterInstance:	xtce:ParameterInstanceRefType,
</sequence>
<attribute name="aliasNameSpace" type="string"/>
</extension>
</complexContent>
}

#[derive(Debug)]
struct ArgumentParameterRefEntryType {
<complexContent>
<extension base="xtce:ArgumentSequenceEntryType">
	parameterRef:	xtce:NameReferenceType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
struct ArgumentParameterSegmentRefEntryType {
<complexContent>
<extension base="xtce:ArgumentSequenceEntryType">
	parameterRef:	xtce:NameReferenceType, // required
<attribute name="order" type="xtce:PositiveLongType"/>
	sizeInBits:	xtce:PositiveLongType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
struct ArgumentStreamSegmentEntryType {
<complexContent>
<extension base="xtce:ArgumentSequenceEntryType">
	streamRef:	xtce:NameReferenceType, // required
<attribute name="order" type="xtce:PositiveLongType"/>
	sizeInBits:	xtce:PositiveLongType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
struct ArrayParameterRefEntryType {
<complexContent>
<extension base="xtce:SequenceEntryType">
<sequence minOccurs="0">
	DimensionList:	xtce:DimensionListType,
</sequence>
	parameterRef:	xtce:NameReferenceType, // required
</extension>
</complexContent>
}
<complexType name="BaseAlarmType" abstract="true">
<sequence>
	AncillaryDataSet:	Option(xtce:AncillaryDataSetType),
</sequence>
<attribute name="name" type="string" use="optional">
</attribute>
<attribute name="shortDescription" type="xtce:ShortDescriptionType">
</attribute>
}

#[derive(Debug)]
struct BaseContainerType {
<sequence>
	RestrictionCriteria:	Option(xtce:RestrictionCriteriaType),
</sequence>
	containerRef:	xtce:NameReferenceType, // required
</attribute>
}
<complexType name="ContainerType" abstract="true" mixed="false">
<complexContent>
<extension base="xtce:NameDescriptionType">
<sequence>
<appinfo>The software should check that any Stream names referenced in the RateInStreamSet actually exist.</appinfo>
	DefaultRateInStream:	Option(xtce:RateInStreamType),
	RateInStreamSet:	Option(xtce:RateInStreamSetType),
	BinaryEncoding:	Option(xtce:BinaryDataEncodingType),
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
struct ContainerRefSetType {
<sequence>
	ContainerRef:	xtce:ContainerRefType,
</sequence>
}

#[derive(Debug)]
struct ContainerRefType {
	containerRef:	xtce:NameReferenceType, // required
</attribute>
}

#[derive(Debug)]
struct ContainerRefEntryType {
<complexContent>
<extension base="xtce:SequenceEntryType">
	containerRef:	xtce:NameReferenceType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
struct ContainerSegmentRefEntryType {
<complexContent>
<extension base="xtce:SequenceEntryType">
	containerRef:	xtce:NameReferenceType, // required
<attribute name="order" type="xtce:PositiveLongType"/>
	sizeInBits:	xtce:PositiveLongType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
struct ContainerSetType {
<choice maxOccurs="unbounded">
	SequenceContainer:	xtce:SequenceContainerType,
</choice>
}
<complexType name="EntryListType" mixed="false">
<choice minOccurs="0" maxOccurs="unbounded">
	ParameterRefEntry:	xtce:ParameterRefEntryType,
	ParameterSegmentRefEntry:	xtce:ParameterSegmentRefEntryType,
	ContainerRefEntry:	xtce:ContainerRefEntryType,
	ContainerSegmentRefEntry:	xtce:ContainerSegmentRefEntryType,
	StreamSegmentEntry:	xtce:StreamSegmentEntryType,
	IndirectParameterRefEntry:	xtce:IndirectParameterRefEntryType,
	ArrayParameterRefEntry:	xtce:ArrayParameterRefEntryType,
</choice>
}

#[derive(Debug)]
struct IndirectParameterRefEntryType {
<complexContent>
<extension base="xtce:SequenceEntryType">
<sequence>
	ParameterInstance:	xtce:ParameterInstanceRefType,
</sequence>
<attribute name="aliasNameSpace" type="string"/>
</extension>
</complexContent>
}

#[derive(Debug)]
struct LocationInContainerInBitsType {
<complexContent>
<extension base="xtce:IntegerValueType">
<attribute name="referenceLocation" type="xtce:ReferenceLocationType" default="previousEntry">
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
struct ArgumentLocationInContainerInBitsType {
<complexContent>
<extension base="xtce:ArgumentIntegerValueType">
<attribute name="referenceLocation" type="xtce:ReferenceLocationType" default="previousEntry"/>
</extension>
</complexContent>
}

#[derive(Debug)]
struct MessageRefType {
	messageRef:	xtce:NameReferenceType, // required
</attribute>
}

#[derive(Debug)]
struct ParameterRefEntryType {
<complexContent>
<extension base="xtce:SequenceEntryType">
	parameterRef:	xtce:NameReferenceType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
struct ParameterSegmentRefEntryType {
<complexContent>
<extension base="xtce:SequenceEntryType">
	parameterRef:	xtce:NameReferenceType, // required
<attribute name="order" type="xtce:PositiveLongType"/>
	sizeInBits:	xtce:PositiveLongType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
struct SequenceContainerType {
<complexContent>
<extension base="xtce:ContainerType">
<sequence>
	EntryList:	xtce:EntryListType,
	BaseContainer:	Option(xtce:BaseContainerType),
</sequence>
<attribute name="abstract" type="boolean" default="false">
</attribute>
<attribute name="idlePattern" type="xtce:FixedIntegerValueType" default="0x0">
</attribute>
</extension>
</complexContent>
}
<complexType name="SequenceEntryType" abstract="true">
<sequence>
	LocationInContainerInBits:	Option(xtce:LocationInContainerInBitsType),
	RepeatEntry:	Option(xtce:RepeatType),
	IncludeCondition:	Option(xtce:MatchCriteriaType),
	TimeAssociation:	Option(xtce:TimeAssociationType),
	AncillaryDataSet:	Option(xtce:AncillaryDataSetType),
</sequence>
<attribute name="shortDescription" type="xtce:ShortDescriptionType" use="optional">
</attribute>
}
<complexType name="ArgumentSequenceEntryType" abstract="true">
<sequence>
	LocationInContainerInBits:	Option(xtce:ArgumentLocationInContainerInBitsType),
	RepeatEntry:	Option(xtce:ArgumentRepeatType),
	IncludeCondition:	Option(xtce:ArgumentMatchCriteriaType),
	AncillaryDataSet:	Option(xtce:AncillaryDataSetType),
</sequence>
<attribute name="shortDescription" type="xtce:ShortDescriptionType" use="optional">
</attribute>
}

#[derive(Debug)]
struct ServiceType {
<complexContent>
<extension base="xtce:NameDescriptionType">
<choice>
	MessageRefSet:	xtce:MessageRefSetType,
	ContainerRefSet:	xtce:ContainerRefSetType,
</choice>
</extension>
</complexContent>
}

#[derive(Debug)]
struct StreamSegmentEntryType {
<complexContent>
<extension base="xtce:SequenceEntryType">
	streamRef:	xtce:NameReferenceType, // required
<attribute name="order" type="xtce:PositiveLongType"/>
	sizeInBits:	xtce:PositiveLongType, // required
</extension>
</complexContent>
}

#[derive(Debug)]
struct MessageType {
<complexContent>
<extension base="xtce:NameDescriptionType">
<sequence>
	MatchCriteria:	xtce:MatchCriteriaType,
	ContainerRef:	xtce:ContainerRefType,
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
struct MessageSetType {
<complexContent>
<extension base="xtce:OptionalNameDescriptionType">
<sequence>
	Message:	xtce:MessageType,
</sequence>
</extension>
</complexContent>
}

#[derive(Debug)]
struct RateInStreamSetType {
<sequence>
	RateInStream:	xtce:RateInStreamWithStreamNameType,
</sequence>
}

#[derive(Debug)]
struct RateInStreamType {
<attribute name="basis" type="xtce:BasisType" default="perSecond">
</attribute>
<attribute name="minimumValue" type="double">
</attribute>
<attribute name="maximumValue" type="double">
</attribute>
}

#[derive(Debug)]
struct RateInStreamWithStreamNameType {
<complexContent>
<extension base="xtce:RateInStreamType">
	streamRef:	xtce:NameReferenceType, // required
</attribute>
</extension>
</complexContent>
}

#[derive(Debug)]
struct ReferenceLocationType {
<restriction base="string">
<enumeration value="containerStart"/>
<enumeration value="containerEnd"/>
<enumeration value="previousEntry"/>
<enumeration value="nextEntry"/>
</restriction>
}

#[derive(Debug)]
struct ReferencePointType {
<restriction base="string">
<enumeration value="start"/>
<enumeration value="end"/>
</restriction>
}


#[derive(Debug)]
struct RestrictionCriteriaType {
<complexContent>
<extension base="xtce:MatchCriteriaType">
<choice>
	NextContainer:	Option(xtce:ContainerRefType),
</choice>
</extension>
</complexContent>
}
