use super::*;
/// <https://schema.org/typicalCreditsPerTerm>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TypicalCreditsPerTermProperty {
	/// <https://schema.org/StructuredValue>
	StructuredValue(StructuredValue),
	/// <https://schema.org/Integer>
	Integer(Integer),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
