use super::*;
/// <https://schema.org/possibleTreatment>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PossibleTreatmentProperty {
	/// <https://schema.org/Drug>
	Drug(Drug),
	/// <https://schema.org/DrugClass>
	DrugClass(DrugClass),
	/// <https://schema.org/LifestyleModification>
	LifestyleModification(LifestyleModification),
	/// <https://schema.org/MedicalTherapy>
	MedicalTherapy(MedicalTherapy),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
