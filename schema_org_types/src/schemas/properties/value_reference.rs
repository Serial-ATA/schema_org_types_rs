use super::*;
/// <https://schema.org/valueReference>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ValueReferenceProperty {
	/// <https://schema.org/DefinedTerm>
	DefinedTerm(DefinedTerm),
	/// <https://schema.org/Enumeration>
	Enumeration(Enumeration),
	/// <https://schema.org/PropertyValue>
	PropertyValue(PropertyValue),
	/// <https://schema.org/QualitativeValue>
	QualitativeValue(QualitativeValue),
	/// <https://schema.org/QuantitativeValue>
	QuantitativeValue(QuantitativeValue),
	/// <https://schema.org/StructuredValue>
	StructuredValue(StructuredValue),
	/// <https://schema.org/MeasurementTypeEnumeration>
	MeasurementTypeEnumeration(MeasurementTypeEnumeration),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
