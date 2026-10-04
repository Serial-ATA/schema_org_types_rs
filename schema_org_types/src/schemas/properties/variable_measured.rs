use super::*;
/// <https://schema.org/variableMeasured>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VariableMeasuredProperty {
	/// <https://schema.org/Property>
	Property(Property),
	/// <https://schema.org/PropertyValue>
	PropertyValue(PropertyValue),
	/// <https://schema.org/StatisticalVariable>
	StatisticalVariable(StatisticalVariable),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
