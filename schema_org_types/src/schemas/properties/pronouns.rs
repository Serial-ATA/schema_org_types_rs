use super::*;
/// <https://schema.org/pronouns>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PronounsProperty {
	/// <https://schema.org/DefinedTerm>
	DefinedTerm(DefinedTerm),
	/// <https://schema.org/StructuredValue>
	StructuredValue(StructuredValue),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
