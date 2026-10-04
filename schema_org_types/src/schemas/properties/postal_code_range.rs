use super::*;
/// <https://schema.org/postalCodeRange>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PostalCodeRangeProperty {
	/// <https://schema.org/PostalCodeRangeSpecification>
	PostalCodeRangeSpecification(PostalCodeRangeSpecification),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
