use super::*;
/// <https://schema.org/returnLabelSource>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReturnLabelSourceProperty {
	/// <https://schema.org/ReturnLabelSourceEnumeration>
	ReturnLabelSourceEnumeration(ReturnLabelSourceEnumeration),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
