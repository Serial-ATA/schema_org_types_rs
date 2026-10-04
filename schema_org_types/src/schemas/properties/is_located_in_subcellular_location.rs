use super::*;
/// <https://schema.org/isLocatedInSubcellularLocation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsLocatedInSubcellularLocationProperty {
	/// <https://schema.org/DefinedTerm>
	DefinedTerm(DefinedTerm),
	/// <https://schema.org/PropertyValue>
	PropertyValue(PropertyValue),
	/// <https://schema.org/URL>
	Url(Url),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
