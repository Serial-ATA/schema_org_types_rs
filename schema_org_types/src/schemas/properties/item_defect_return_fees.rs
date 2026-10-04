use super::*;
/// <https://schema.org/itemDefectReturnFees>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemDefectReturnFeesProperty {
	/// <https://schema.org/ReturnFeesEnumeration>
	ReturnFeesEnumeration(ReturnFeesEnumeration),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
