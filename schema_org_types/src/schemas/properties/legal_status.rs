use super::*;
/// <https://schema.org/legalStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LegalStatusProperty {
	/// <https://schema.org/DrugLegalStatus>
	DrugLegalStatus(DrugLegalStatus),
	/// <https://schema.org/MedicalEnumeration>
	MedicalEnumeration(MedicalEnumeration),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
