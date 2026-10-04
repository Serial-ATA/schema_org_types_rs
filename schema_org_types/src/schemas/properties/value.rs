use super::*;
/// <https://schema.org/value>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ValueProperty {
	/// <https://schema.org/QualitativeValue>
	QualitativeValue(QualitativeValue),
	/// <https://schema.org/StructuredValue>
	StructuredValue(StructuredValue),
	/// <https://schema.org/Boolean>
	Boolean(Boolean),
	/// <https://schema.org/Number>
	Number(Number),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
