use super::*;
/// <https://schema.org/sensoryUnit>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SensoryUnitProperty {
	/// <https://schema.org/AnatomicalStructure>
	AnatomicalStructure(AnatomicalStructure),
	/// <https://schema.org/SuperficialAnatomy>
	SuperficialAnatomy(SuperficialAnatomy),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
