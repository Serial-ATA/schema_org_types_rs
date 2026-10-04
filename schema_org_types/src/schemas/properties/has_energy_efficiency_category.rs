use super::*;
/// <https://schema.org/hasEnergyEfficiencyCategory>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasEnergyEfficiencyCategoryProperty {
	/// <https://schema.org/EnergyEfficiencyEnumeration>
	EnergyEfficiencyEnumeration(EnergyEfficiencyEnumeration),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
