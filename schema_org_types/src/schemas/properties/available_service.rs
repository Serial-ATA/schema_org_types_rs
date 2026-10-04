use super::*;
/// <https://schema.org/availableService>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AvailableServiceProperty {
	/// <https://schema.org/MedicalProcedure>
	MedicalProcedure(MedicalProcedure),
	/// <https://schema.org/MedicalTest>
	MedicalTest(MedicalTest),
	/// <https://schema.org/MedicalTherapy>
	MedicalTherapy(MedicalTherapy),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
