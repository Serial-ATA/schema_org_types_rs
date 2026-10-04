use super::*;
/// <https://schema.org/hasEnergyConsumptionDetails>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasEnergyConsumptionDetailsProperty {
	/// <https://schema.org/EnergyConsumptionDetails>
	EnergyConsumptionDetails(EnergyConsumptionDetails),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
