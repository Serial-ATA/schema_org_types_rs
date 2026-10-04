use super::*;
/// <https://schema.org/errorCode>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ErrorCodeProperty {
	/// <https://schema.org/DefinedTerm>
	DefinedTerm(DefinedTerm),
	/// <https://schema.org/StatusEnumeration>
	StatusEnumeration(StatusEnumeration),
	/// <https://schema.org/Integer>
	Integer(Integer),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
